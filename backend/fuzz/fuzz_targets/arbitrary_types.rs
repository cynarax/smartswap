//! Реализация Arbitrary для AddOrderRequest и других DTO через newtype-обёртки.

use backend::handlers::types::AddOrderRequest;
use arbitrary::Arbitrary;

/// Newtype-обёртка для AddOrderRequest, чтобы обойти orphan rule
#[derive(Debug, Clone)]
pub struct FuzzAddOrderRequest(pub AddOrderRequest);

impl<'a> Arbitrary<'a> for FuzzAddOrderRequest {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(FuzzAddOrderRequest(AddOrderRequest {
            base: String::arbitrary(u)?,
            quote: String::arbitrary(u)?,
            amount: String::arbitrary(u)?,
            price: String::arbitrary(u)?,
            side: String::arbitrary(u)?,
        }))
    }
}

impl From<FuzzAddOrderRequest> for AddOrderRequest {
    fn from(fuzz_req: FuzzAddOrderRequest) -> Self {
        fuzz_req.0
    }
} 