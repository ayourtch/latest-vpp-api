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
use crate::sr_types::*;
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sr_mpls_policy_add_a1a70c70)]
pub struct SrMplsPolicyAdd {
	pub client_index: u32,
	pub context: u32,
	pub bsid: u32,
	pub weight: u32,
	pub is_spray: bool,
	pub n_segments: u8,
	pub segments: VariableSizeArray<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sr_mpls_policy_add_reply_e8d4e804)]
pub struct SrMplsPolicyAddReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sr_mpls_policy_mod_88482c17)]
pub struct SrMplsPolicyMod {
	pub client_index: u32,
	pub context: u32,
	pub bsid: u32,
	pub operation: SrPolicyOp,
	pub sl_index: u32,
	pub weight: u32,
	pub n_segments: u8,
	pub segments: VariableSizeArray<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sr_mpls_policy_mod_reply_e8d4e804)]
pub struct SrMplsPolicyModReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sr_mpls_policy_del_e29d34fa)]
pub struct SrMplsPolicyDel {
	pub client_index: u32,
	pub context: u32,
	pub bsid: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sr_mpls_policy_del_reply_e8d4e804)]
pub struct SrMplsPolicyDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sr_mpls_steering_add_del_64acff63)]
pub struct SrMplsSteeringAddDel {
	pub client_index: u32,
	pub context: u32,
	pub is_del: bool,
	pub bsid: u32,
	pub table_id: u32,
	pub prefix: Prefix,
	pub mask_width: u32,
	pub next_hop: Address,
	pub color: u32,
	pub co_bits: u8,
	pub vpn_label: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sr_mpls_steering_add_del_reply_e8d4e804)]
pub struct SrMplsSteeringAddDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sr_mpls_policy_assign_endpoint_color_0e7eb978)]
pub struct SrMplsPolicyAssignEndpointColor {
	pub client_index: u32,
	pub context: u32,
	pub bsid: u32,
	pub endpoint: Address,
	pub color: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sr_mpls_policy_assign_endpoint_color_reply_e8d4e804)]
pub struct SrMplsPolicyAssignEndpointColorReply {
	pub context: u32,
	pub retval: i32,
}
