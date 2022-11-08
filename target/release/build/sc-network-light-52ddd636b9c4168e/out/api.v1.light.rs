/// A pair of arbitrary bytes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pair {
    /// The first element of the pair.
    #[prost(bytes="vec", tag="1")]
    pub fst: ::prost::alloc::vec::Vec<u8>,
    /// The second element of the pair.
    #[prost(bytes="vec", tag="2")]
    pub snd: ::prost::alloc::vec::Vec<u8>,
}
/// Enumerate all possible light client request messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof="request::Request", tags="1, 2, 4")]
    pub request: ::core::option::Option<request::Request>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag="1")]
        RemoteCallRequest(super::RemoteCallRequest),
        #[prost(message, tag="2")]
        RemoteReadRequest(super::RemoteReadRequest),
        /// Note: ids 3 and 5 were used in the past. It would be preferable to not re-use them.
        #[prost(message, tag="4")]
        RemoteReadChildRequest(super::RemoteReadChildRequest),
    }
}
/// Enumerate all possible light client response messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof="response::Response", tags="1, 2")]
    pub response: ::core::option::Option<response::Response>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag="1")]
        RemoteCallResponse(super::RemoteCallResponse),
        /// Note: ids 3 and 4 were used in the past. It would be preferable to not re-use them.
        #[prost(message, tag="2")]
        RemoteReadResponse(super::RemoteReadResponse),
    }
}
/// Remote call request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteCallRequest {
    /// Block at which to perform call.
    #[prost(bytes="vec", tag="2")]
    pub block: ::prost::alloc::vec::Vec<u8>,
    /// Method name.
    #[prost(string, tag="3")]
    pub method: ::prost::alloc::string::String,
    /// Call data.
    #[prost(bytes="vec", tag="4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Remote call response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteCallResponse {
    /// Execution proof.
    #[prost(bytes="vec", tag="2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
}
/// Remote storage read request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteReadRequest {
    /// Block at which to perform call.
    #[prost(bytes="vec", tag="2")]
    pub block: ::prost::alloc::vec::Vec<u8>,
    /// Storage keys.
    #[prost(bytes="vec", repeated, tag="3")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Remote read response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteReadResponse {
    /// Read proof.
    #[prost(bytes="vec", tag="2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
}
/// Remote storage read child request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteReadChildRequest {
    /// Block at which to perform call.
    #[prost(bytes="vec", tag="2")]
    pub block: ::prost::alloc::vec::Vec<u8>,
    /// Child Storage key, this is relative
    /// to the child type storage location.
    #[prost(bytes="vec", tag="3")]
    pub storage_key: ::prost::alloc::vec::Vec<u8>,
    /// Storage keys.
    #[prost(bytes="vec", repeated, tag="6")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
