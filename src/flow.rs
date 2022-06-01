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
use crate::flow_types::*;
use crate::interface_types::*;
use crate::ethernet_types::*;
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(flow_add_f946ed84)]
pub struct FlowAdd {
	pub client_index: u32,
	pub context: u32,
	pub flow: FlowRule,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(flow_add_v2_5b757558)]
pub struct FlowAddV2 {
	pub client_index: u32,
	pub context: u32,
	pub flow: FlowRuleV2,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(flow_add_reply_8587dc85)]
pub struct FlowAddReply {
	pub context: u32,
	pub retval: i32,
	pub flow_index: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(flow_add_v2_reply_8587dc85)]
pub struct FlowAddV2Reply {
	pub context: u32,
	pub retval: i32,
	pub flow_index: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(flow_del_b6b9b02c)]
pub struct FlowDel {
	pub client_index: u32,
	pub context: u32,
	pub flow_index: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(flow_del_reply_e8d4e804)]
pub struct FlowDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(flow_enable_2024be69)]
pub struct FlowEnable {
	pub client_index: u32,
	pub context: u32,
	pub flow_index: u32,
	pub hw_if_index: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(flow_enable_reply_e8d4e804)]
pub struct FlowEnableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(flow_disable_2024be69)]
pub struct FlowDisable {
	pub client_index: u32,
	pub context: u32,
	pub flow_index: u32,
	pub hw_if_index: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(flow_disable_reply_e8d4e804)]
pub struct FlowDisableReply {
	pub context: u32,
	pub retval: i32,
}
