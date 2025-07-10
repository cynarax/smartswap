use smartswap_core::orderbook::OrderBook;
use smartswap_core::types::AddOrderRequest;
use smartswap_core::swap_engine::SwapEngine;
use smartswap_core::price_source::PriceSource;

#[test]
fn e2e_add_and_execute_order() {
    // Тест базового сценария: добавить, исполнить и удалить ордер
    let mut ob = OrderBook::new();
    let req = AddOrderRequest {
        base: "WBNB".into(),
        quote: "USDT".into(),
        amount: "100".into(),
        price: "3200".into(),
        side: "Buy".into(),
    };
    let id = ob.add_order(req).unwrap();
    assert_eq!(ob.get_orders().len(), 1);
    let order = ob.get_orders().front().cloned().unwrap();
    let amount = order.amount;
    let price = order.price;
    let result = SwapEngine::get_quote(amount, price);
    assert!(result.is_ok(), "Quote calculation failed");
    assert!(ob.delete_order(id));
    assert_eq!(ob.get_orders().len(), 0);
}

#[cfg(feature = "uniswap")]
#[tokio::test]
async fn test_pricing_uniswap_pancake() {
    use smartswap_core::price_source::uniswap_v2::UniswapV2PriceSource;
    use ethers::types::Address;

    let source = UniswapV2PriceSource {
        rpc_url: "https://bsc-dataseed.binance.org/".to_string(),
        pool_address: "0x16b9a82891338f9ba80e2d6970fdda79d1eb0dae".parse::<Address>().unwrap(),
        token0_symbol: "WBNB",
        token1_symbol: "USDT",
        decimals0: 18, // WBNB
        decimals1: 18, // USDT (BSC)
    };
    let price = source.get_price("WBNB", "USDT", None).await
        .expect("Uniswap price fetch failed");
    assert!(price > 0.0, "PancakeSwap price should be > 0, got {}", price);
} 