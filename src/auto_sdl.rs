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
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(auto_sdl_config_14f30db8)]
pub struct AutoSdlConfig {
	pub client_index: u32,
	pub context: u32,
	pub threshold: u32,
	pub remove_timeout: u32,
	pub enable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(auto_sdl_config_reply_e8d4e804)]
pub struct AutoSdlConfigReply {
	pub context: u32,
	pub retval: i32,
}