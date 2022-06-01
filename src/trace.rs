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
#[message_name_and_crc(trace_profile_add_de08aa6d)]
pub struct TraceProfileAdd {
	pub client_index : u32,
	pub context : u32,
	pub trace_type : u8,
	pub num_elts : u8,
	pub trace_tsp : u8,
	pub node_id : u32,
	pub app_data : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(trace_profile_add_reply_e8d4e804)]
pub struct TraceProfileAddReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(trace_profile_del_51077d14)]
pub struct TraceProfileDel {
	pub client_index : u32,
	pub context : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(trace_profile_del_reply_e8d4e804)]
pub struct TraceProfileDelReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(trace_profile_show_config_51077d14)]
pub struct TraceProfileShowConfig {
	pub client_index : u32,
	pub context : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(trace_profile_show_config_reply_0f1d374c)]
pub struct TraceProfileShowConfigReply {
	pub context : u32,
	pub retval : i32,
	pub trace_type : u8,
	pub num_elts : u8,
	pub trace_tsp : u8,
	pub node_id : u32,
	pub app_data : u32,
}
