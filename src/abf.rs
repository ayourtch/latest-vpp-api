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
use crate::fib_types::*; 
use crate::interface_types::*; 
// Implementation for abf_policy 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct AbfPolicy { 
	pub policy_id : u32, 
	pub acl_index : u32, 
	pub n_paths : u8, 
	pub paths : VariableSizeArray<FibPath>, 
} 
// Implementation for abf_itf_attach 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct AbfItfAttach { 
	pub policy_id : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub priority : u32, 
	pub is_ipv6 : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(abf_plugin_get_version_51077d14)] 
pub struct AbfPluginGetVersion { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(abf_plugin_get_version_reply_9b32cf86)] 
pub struct AbfPluginGetVersionReply { 
	pub context : u32, 
	pub major : u32, 
	pub minor : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(abf_policy_add_del_c6131197)] 
pub struct AbfPolicyAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub policy : AbfPolicy, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(abf_policy_add_del_reply_e8d4e804)] 
pub struct AbfPolicyAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(abf_policy_details_b7487fa4)] 
pub struct AbfPolicyDetails { 
	pub context : u32, 
	pub policy : AbfPolicy, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(abf_policy_dump_51077d14)] 
pub struct AbfPolicyDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(abf_itf_attach_add_del_25c8621b)] 
pub struct AbfItfAttachAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub attach : AbfItfAttach, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(abf_itf_attach_add_del_reply_e8d4e804)] 
pub struct AbfItfAttachAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(abf_itf_attach_details_7819523e)] 
pub struct AbfItfAttachDetails { 
	pub context : u32, 
	pub attach : AbfItfAttach, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(abf_itf_attach_dump_51077d14)] 
pub struct AbfItfAttachDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
