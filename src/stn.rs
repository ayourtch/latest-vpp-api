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
use crate::interface_types::*;
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(stn_add_del_rule_224c6edd)]
pub struct StnAddDelRule {
	pub client_index: u32,
	pub context: u32,
	pub ip_address: Address,
	pub sw_if_index: InterfaceIndex,
	pub is_add: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(stn_add_del_rule_reply_e8d4e804)]
pub struct StnAddDelRuleReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(stn_rules_dump_51077d14)]
pub struct StnRulesDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(stn_rules_details_a51935a6)]
pub struct StnRulesDetails {
	pub context: u32,
	pub ip_address: Address,
	pub sw_if_index: InterfaceIndex,
}
