//! Adapters: talk to external providers over HTTP and implement the domain ports.
//! This is the only crate besides `app` that knows about `reqwest`.

mod httpjson;
pub mod identity;
mod origin;
pub mod payments;

pub use httpjson::{HttpJsonError, MAX_BODY_BYTES};
pub use origin::{Origin, OriginError};
