#![no_main]
use libfuzzer_sys::fuzz_target;
use smartswap_core::orderbook::OrderBook;
use smartswap_core::types::AddOrderRequest;

fuzz_target!(|data: (String, String, String, String, String)| {
    let (base, quote, amount, price, side) = data;
    let mut ob = OrderBook::new();
    let req = AddOrderRequest {
        base,
        quote,
        amount,
        price,
        side,
    };
    let _ = ob.add_order(req);
    // Цель — поймать любые паники/краши на add_order
}); 