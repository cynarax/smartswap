use async_trait::async_trait;
use reqwest;
use serde_json;
use std::any::Any;

#[async_trait]
pub trait PriceSource: Send + Sync + 'static {
    async fn get_price(&self, from: &str, to: &str, amount: Option<&str>) -> Result<f64, String>;
    fn as_any(&self) -> &dyn Any;
}

// --- Mock --- //
pub struct MockPriceSource;

#[async_trait]
impl PriceSource for MockPriceSource {
    async fn get_price(&self, from: &str, to: &str, _amount: Option<&str>) -> Result<f64, String> {
        match (from, to) {
            ("ETH", "USDT") => Ok(3200.0),
            ("USDT", "ETH") => Ok(1.0 / 3200.0),
            ("WBTC", "USDT") => Ok(67000.0),
            ("USDT", "WBTC") => Ok(1.0 / 67000.0),
            _ => Err(format!("No price for {from}/{to}")),
        }
    }
    fn as_any(&self) -> &dyn Any { self }
}

// --- CoinGecko --- //
pub struct CoinGeckoPriceSource;

#[async_trait]
impl PriceSource for CoinGeckoPriceSource {
    async fn get_price(&self, from: &str, to: &str, _amount: Option<&str>) -> Result<f64, String> {
        let from_id = map_token(from);
        let to_id = map_token(to);
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={from_id}&vs_currencies={to_id}"
        );
        let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
        let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        let price = json.get(&from_id)
            .and_then(|x| x.get(&to_id))
            .and_then(|x| x.as_f64())
            .ok_or("Price not found".to_string())?;
        Ok(price)
    }
    fn as_any(&self) -> &dyn Any { self }
}

// --- Token mapping for CoinGecko --- //
fn map_token(symbol: &str) -> String {
    match symbol.to_ascii_uppercase().as_str() {
        "ETH" => "ethereum".to_string(),
        "WBTC" => "wrapped-bitcoin".to_string(),
        "USDT" => "tether".to_string(),
        "BNB" => "binancecoin".to_string(),
        _ => symbol.to_ascii_lowercase(),
    }
}

// --- Uniswap поддержка через feature ---
#[cfg(feature = "uniswap")]
pub mod uniswap_v2 {
    use async_trait::async_trait;
    use ethers::prelude::*;
    use rust_decimal::Decimal;
    use rust_decimal::prelude::ToPrimitive;
    use std::sync::Arc;


    abigen!(
        UniswapV2Pair,
        "abi/UniswapV2Pair.json"
    );

    pub struct UniswapV2PriceSource {
        pub rpc_url: String,
        pub pool_address: Address,
        pub token0_symbol: &'static str,
        pub token1_symbol: &'static str,
        pub decimals0: u8,
        pub decimals1: u8,
    }

    #[async_trait]
    impl super::PriceSource for UniswapV2PriceSource {
        async fn get_price(&self, from: &str, to: &str, _amount: Option<&str>) -> Result<f64, String> {
            let provider = Provider::<Http>::try_from(self.rpc_url.clone())
                .map_err(|e| format!("Provider error: {e}"))?;
            let client = Arc::new(provider);

            let pair = UniswapV2Pair::new(self.pool_address, client);
            let (reserve0, reserve1, _) = pair.get_reserves().call().await
                .map_err(|e| format!("Get reserves failed: {e}"))?;

            // Универсальная поддержка любых пар
            if from == self.token0_symbol && to == self.token1_symbol {
                let r0 = Decimal::from(reserve0);
                let r1 = Decimal::from(reserve1);
                let price = (r1 / Decimal::new(10u64.pow(self.decimals1 as u32) as i64, 0))
                    / (r0 / Decimal::new(10u64.pow(self.decimals0 as u32) as i64, 0));
                Ok(price.to_f64().unwrap())
            } else if from == self.token1_symbol && to == self.token0_symbol {
                let r0 = Decimal::from(reserve0);
                let r1 = Decimal::from(reserve1);
                let price = (r0 / Decimal::new(10u64.pow(self.decimals0 as u32) as i64, 0))
                    / (r1 / Decimal::new(10u64.pow(self.decimals1 as u32) as i64, 0));
                Ok(price.to_f64().unwrap())
            } else {
                Err("Pair not supported".to_string())
            }
        }
        fn as_any(&self) -> &dyn std::any::Any { self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;

    #[test]
    fn test_mock_price_source_supported_pairs() {
        let mock = MockPriceSource;
        let price = block_on(mock.get_price("ETH", "USDT", None)).unwrap();
        assert_eq!(price, 3200.0);
        let price = block_on(mock.get_price("USDT", "ETH", None)).unwrap();
        assert!((price - (1.0 / 3200.0)).abs() < 1e-8);
    }

    #[test]
    fn test_mock_price_source_unsupported_pair() {
        let mock = MockPriceSource;
        let res = block_on(mock.get_price("DOGE", "USDT", None));
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_coingecko_price_source_unknown_pair() {
        let cg = CoinGeckoPriceSource;
        let res = cg.get_price("UNKNOWN", "USDT", None).await;
        assert!(res.is_err());
    }
}