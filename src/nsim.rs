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
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nsim_cross_connect_enable_disable_9c3ead86)]
pub struct NsimCrossConnectEnableDisable {
	pub client_index: u32,
	pub context: u32,
	pub enable_disable: bool,
	pub sw_if_index0: InterfaceIndex,
	pub sw_if_index1: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nsim_cross_connect_enable_disable_reply_e8d4e804)]
pub struct NsimCrossConnectEnableDisableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nsim_output_feature_enable_disable_3865946c)]
pub struct NsimOutputFeatureEnableDisable {
	pub client_index: u32,
	pub context: u32,
	pub enable_disable: bool,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nsim_output_feature_enable_disable_reply_e8d4e804)]
pub struct NsimOutputFeatureEnableDisableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nsim_configure_16ed400f)]
pub struct NsimConfigure {
	pub client_index: u32,
	pub context: u32,
	pub delay_in_usec: u32,
	pub average_packet_size: u32,
	pub bandwidth_in_bits_per_second: u64,
	pub packets_per_drop: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nsim_configure_reply_e8d4e804)]
pub struct NsimConfigureReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nsim_configure2_64de8ed3)]
pub struct NsimConfigure2 {
	pub client_index: u32,
	pub context: u32,
	pub delay_in_usec: u32,
	pub average_packet_size: u32,
	pub bandwidth_in_bits_per_second: u64,
	pub packets_per_drop: u32,
	pub packets_per_reorder: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nsim_configure2_reply_e8d4e804)]
pub struct NsimConfigure2Reply {
	pub context: u32,
	pub retval: i32,
}
