/*
   Autogenerated Data, Do not Edit! 
*/
#![allow(dead_code,unused_mut,unused_variables,unused_must_use,non_camel_case_types,unused_imports,non_snake_case)]
use vpp_api_macros::{VppMessage,VppUnionIdent}; 
use std::convert::TryInto; 
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use crate::VppNamedMessage;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum;
use crate::ip_types::*; 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum SyslogSeverity { 
	 SYSLOG_API_SEVERITY_EMERG=0, 
	 SYSLOG_API_SEVERITY_ALERT=1, 
	 SYSLOG_API_SEVERITY_CRIT=2, 
	 SYSLOG_API_SEVERITY_ERR=3, 
	 SYSLOG_API_SEVERITY_WARN=4, 
	 SYSLOG_API_SEVERITY_NOTICE=5, 
	 SYSLOG_API_SEVERITY_INFO=6, 
	 SYSLOG_API_SEVERITY_DBG=7, 
} 
impl Default for SyslogSeverity { 
	fn default() -> Self { SyslogSeverity::SYSLOG_API_SEVERITY_EMERG }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(syslog_set_sender_b8011d0b)] 
pub struct SyslogSetSender { 
	pub client_index : u32, 
	pub context : u32, 
	pub src_address : Ip4Address, 
	pub collector_address : Ip4Address, 
	pub collector_port : u16, 
	pub vrf_id : u32, 
	pub max_msg_size : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(syslog_set_sender_reply_e8d4e804)] 
pub struct SyslogSetSenderReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(syslog_get_sender_51077d14)] 
pub struct SyslogGetSender { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(syslog_get_sender_reply_424cfa4e)] 
pub struct SyslogGetSenderReply { 
	pub context : u32, 
	pub retval : i32, 
	pub src_address : Ip4Address, 
	pub collector_address : Ip4Address, 
	pub collector_port : u16, 
	pub vrf_id : u32, 
	pub max_msg_size : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(syslog_set_filter_571348c3)] 
pub struct SyslogSetFilter { 
	pub client_index : u32, 
	pub context : u32, 
	pub severity : SyslogSeverity, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(syslog_set_filter_reply_e8d4e804)] 
pub struct SyslogSetFilterReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(syslog_get_filter_51077d14)] 
pub struct SyslogGetFilter { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(syslog_get_filter_reply_eb1833f8)] 
pub struct SyslogGetFilterReply { 
	pub context : u32, 
	pub retval : i32, 
	pub severity : SyslogSeverity, 
} 
