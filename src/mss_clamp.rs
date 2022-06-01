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
use crate::interface_types::*;
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MssClampDir {
	 MSS_CLAMP_DIR_NONE=0,
	 MSS_CLAMP_DIR_RX=1,
	 MSS_CLAMP_DIR_TX=2,
}
impl Default for MssClampDir {
	fn default() -> Self { MssClampDir::MSS_CLAMP_DIR_NONE }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(mss_clamp_enable_disable_d31b44e3)]
pub struct MssClampEnableDisable {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub ipv4_mss: u16,
	pub ipv6_mss: u16,
	pub ipv4_direction: MssClampDir,
	pub ipv6_direction: MssClampDir,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(mss_clamp_enable_disable_reply_e8d4e804)]
pub struct MssClampEnableDisableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(mss_clamp_get_47250981)]
pub struct MssClampGet {
	pub client_index: u32,
	pub context: u32,
	pub cursor: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(mss_clamp_get_reply_53b48f5d)]
pub struct MssClampGetReply {
	pub context: u32,
	pub retval: i32,
	pub cursor: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(mss_clamp_details_d3a4de61)]
pub struct MssClampDetails {
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub ipv4_mss: u16,
	pub ipv6_mss: u16,
	pub ipv4_direction: MssClampDir,
	pub ipv6_direction: MssClampDir,
}
