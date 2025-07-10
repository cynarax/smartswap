use proptest::prelude::*;
use smartswap_core::orderbook::OrderBook;
use smartswap_core::types::AddOrderRequest;

proptest! {
    #[test]
    fn prop_add_remove(amount in 1u32..10_000, price in 1u32..1_000_000) {
        let mut ob = OrderBook::new();
        let req = AddOrderRequest {
            base: "ETH".into(),
            quote: "USDT".into(),
            amount: amount.to_string(),
            price: price.to_string(),
            side: "Buy".into(),
        };
        let id = ob.add_order(req).unwrap();
        assert!(ob.delete_order(id));
        assert_eq!(ob.get_orders().len(), 0);
    }
} 