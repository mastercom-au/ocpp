//! The [Request] trait simplifies writing responses to requests by filling in the
//! parts of a response which are simply copied from the request, as required by
//! the OCPP specification.
//!
//! Similarly, there are sometimes fields which must be copied with a response,
//! which would otherwise be verbose to initialize

/// The Request trait, see the [`crate::common::request`](crate::common::request) module
pub trait Request<Args, Response> {
    /// Given an instance of a request, build an appropriate response
    fn build_response(&self, args: Args) -> Response;
}
