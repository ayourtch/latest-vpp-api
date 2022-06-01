/*
   Autogenerated Data, Do not Edit!
*/
#![allow(dead_code,unused_mut,unused_variables,unused_must_use,non_camel_case_types,unused_imports,non_snake_case)]
use vpp_api_macros::{VppMessage,VppUnionIdent};
use std::convert::TryInto;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use vpp_api_encoding::typ::*;
pub use vpp_api_encoding;
use vpp_api_message::VppApiMessage;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum;
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CryptoDispatchMode {
	 CRYPTO_ASYNC_DISPATCH_POLLING=0,
	 CRYPTO_ASYNC_DISPATCH_INTERRUPT=1,
}
impl Default for CryptoDispatchMode {
	fn default() -> Self { CryptoDispatchMode::CRYPTO_ASYNC_DISPATCH_POLLING }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CryptoOpClassType {
	 CRYPTO_API_OP_SIMPLE=0,
	 CRYPTO_API_OP_CHAINED=1,
	 CRYPTO_API_OP_BOTH=2,
}
impl Default for CryptoOpClassType {
	fn default() -> Self { CryptoOpClassType::CRYPTO_API_OP_SIMPLE }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(crypto_set_async_dispatch_5ca4adc0)]
pub struct CryptoSetAsyncDispatch {
	pub client_index : u32,
	pub context : u32,
	pub mode : CryptoDispatchMode,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(crypto_set_async_dispatch_reply_e8d4e804)]
pub struct CryptoSetAsyncDispatchReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(crypto_set_handler_ce9ad00d)]
pub struct CryptoSetHandler {
	pub client_index : u32,
	pub context : u32,
	pub alg_name : FixedSizeString<typenum::U32>,
	pub engine : FixedSizeString<typenum::U16>,
	pub oct : CryptoOpClassType,
	pub is_async : u8,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(crypto_set_handler_reply_e8d4e804)]
pub struct CryptoSetHandlerReply {
	pub context : u32,
	pub retval : i32,
}
