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
use crate::vpe_types::*;
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(show_version_51077d14)]
pub struct ShowVersion {
	pub client_index : u32,
	pub context : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(show_version_reply_c919bde1)]
pub struct ShowVersionReply {
	pub context : u32,
	pub retval : i32,
	pub program : FixedSizeString<typenum::U32>,
	pub version : FixedSizeString<typenum::U32>,
	pub build_date : FixedSizeString<typenum::U32>,
	pub build_directory : FixedSizeString<typenum::U256>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(show_vpe_system_time_51077d14)]
pub struct ShowVpeSystemTime {
	pub client_index : u32,
	pub context : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(show_vpe_system_time_reply_7ffd8193)]
pub struct ShowVpeSystemTimeReply {
	pub context : u32,
	pub retval : i32,
	pub vpe_system_time : Timestamp,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(log_dump_6ab31753)]
pub struct LogDump {
	pub client_index : u32,
	pub context : u32,
	pub start_timestamp : Timestamp,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(log_details_03d61cc0)]
pub struct LogDetails {
	pub context : u32,
	pub timestamp : Timestamp,
	pub level : LogLevel,
	pub msg_class : FixedSizeString<typenum::U32>,
	pub message : FixedSizeString<typenum::U256>,
}
