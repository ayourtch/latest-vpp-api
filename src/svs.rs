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
#[message_name_and_crc(svs_plugin_get_version_51077d14)]
pub struct SvsPluginGetVersion {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(svs_plugin_get_version_reply_9b32cf86)]
pub struct SvsPluginGetVersionReply {
	pub context: u32,
	pub major: u32,
	pub minor: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(svs_table_add_del_7d21cb2a)]
pub struct SvsTableAddDel {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub af: AddressFamily,
	pub table_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(svs_table_add_del_reply_e8d4e804)]
pub struct SvsTableAddDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(svs_route_add_del_e49bc63c)]
pub struct SvsRouteAddDel {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub prefix: Prefix,
	pub table_id: u32,
	pub source_table_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(svs_route_add_del_reply_e8d4e804)]
pub struct SvsRouteAddDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(svs_enable_disable_634b89d2)]
pub struct SvsEnableDisable {
	pub client_index: u32,
	pub context: u32,
	pub is_enable: bool,
	pub af: AddressFamily,
	pub table_id: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(svs_enable_disable_reply_e8d4e804)]
pub struct SvsEnableDisableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(svs_dump_51077d14)]
pub struct SvsDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(svs_details_6282cd55)]
pub struct SvsDetails {
	pub context: u32,
	pub table_id: u32,
	pub sw_if_index: InterfaceIndex,
	pub af: AddressFamily,
}
