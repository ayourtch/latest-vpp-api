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
use crate::ip_types::*;
use crate::fib_types::*;
use crate::interface_types::*;
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum UrpfMode {
	 URPF_API_MODE_OFF=0,
	 URPF_API_MODE_LOOSE=1,
	 URPF_API_MODE_STRICT=2,
}
impl Default for UrpfMode {
	fn default() -> Self { UrpfMode::URPF_API_MODE_OFF }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(urpf_update_cc274cd1)]
pub struct UrpfUpdate {
	pub client_index : u32,
	pub context : u32,
	pub is_input : bool,
	pub mode : UrpfMode,
	pub af : AddressFamily,
	pub sw_if_index : InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(urpf_update_reply_e8d4e804)]
pub struct UrpfUpdateReply {
	pub context : u32,
	pub retval : i32,
}
