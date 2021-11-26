/*
   Autogenerated Data, Do not Edit! 
*/
#![allow(dead_code,unused_mut,unused_variables,unused_must_use,non_camel_case_types,unused_imports,non_snake_case)]
use vpp_api_macros::{VppMessage,VppUnionIdent}; 
use std::convert::TryInto; 
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_message::VppApiMessage;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum;
use crate::ip_types::*; 
use crate::ethernet_types::*; 
// Implementation for acl_rule 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct AclRule { 
	pub is_permit : AclAction, 
	pub src_prefix : Prefix, 
	pub dst_prefix : Prefix, 
	pub proto : IpProto, 
	pub srcport_or_icmptype_first : u16, 
	pub srcport_or_icmptype_last : u16, 
	pub dstport_or_icmpcode_first : u16, 
	pub dstport_or_icmpcode_last : u16, 
	pub tcp_flags_mask : u8, 
	pub tcp_flags_value : u8, 
} 
// Implementation for macip_acl_rule 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct MacipAclRule { 
	pub is_permit : AclAction, 
	pub src_mac : MacAddress, 
	pub src_mac_mask : MacAddress, 
	pub src_prefix : Prefix, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum AclAction { 
	 ACL_ACTION_API_DENY=0, 
	 ACL_ACTION_API_PERMIT=1, 
	 ACL_ACTION_API_PERMIT_REFLECT=2, 
} 
impl Default for AclAction { 
	fn default() -> Self { AclAction::ACL_ACTION_API_DENY }
}
