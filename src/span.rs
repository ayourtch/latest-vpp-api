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
#[repr(u32)]
pub enum SpanState {
	 SPAN_STATE_API_DISABLED=0,
	 SPAN_STATE_API_RX=1,
	 SPAN_STATE_API_TX=2,
	 SPAN_STATE_API_RX_TX=3,
}
impl Default for SpanState {
	fn default() -> Self { SpanState::SPAN_STATE_API_DISABLED }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_span_enable_disable_23ddd96b)]
pub struct SwInterfaceSpanEnableDisable {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index_from: InterfaceIndex,
	pub sw_if_index_to: InterfaceIndex,
	pub state: SpanState,
	pub is_l2: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_span_enable_disable_reply_e8d4e804)]
pub struct SwInterfaceSpanEnableDisableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_span_dump_d6cf0c3d)]
pub struct SwInterfaceSpanDump {
	pub client_index: u32,
	pub context: u32,
	pub is_l2: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_span_details_8a20e79f)]
pub struct SwInterfaceSpanDetails {
	pub context: u32,
	pub sw_if_index_from: InterfaceIndex,
	pub sw_if_index_to: InterfaceIndex,
	pub state: SpanState,
	pub is_l2: bool,
}
