/// type = request, meta is event byt, type = sync , type = terminate

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VlcMsg {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub vlc_meta: ::prost::alloc::vec::Vec<u8>,
}
/// type = vlc
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZMessage {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub msg_meta: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VlcMeta {
    #[prost(message, optional, tag = "1")]
    pub clock_state: ::core::option::Option<Clock>,
    #[prost(bytes = "vec", tag = "2")]
    pub event_meta: ::prost::alloc::vec::Vec<u8>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Clock {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub value: u64,
    #[prost(message, repeated, tag = "3")]
    pub ancestors: ::prost::alloc::vec::Vec<Clock>,
}
