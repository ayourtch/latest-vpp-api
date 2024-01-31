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
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LcpItfHostType {
	 LCP_API_ITF_HOST_TAP=0,
	 LCP_API_ITF_HOST_TUN=1,
}
impl Default for LcpItfHostType {
	fn default() -> Self { LcpItfHostType::LCP_API_ITF_HOST_TAP }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_default_ns_set_69749409)]
pub struct LcpDefaultNsSet {
	pub client_index: u32,
	pub context: u32,
	pub netns: FixedSizeString<typenum::U32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_default_ns_set_reply_e8d4e804)]
pub struct LcpDefaultNsSetReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_default_ns_get_51077d14)]
pub struct LcpDefaultNsGet {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_default_ns_get_reply_5102feee)]
pub struct LcpDefaultNsGetReply {
	pub context: u32,
	pub netns: FixedSizeString<typenum::U32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_itf_pair_add_del_40482b80)]
pub struct LcpItfPairAddDel {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub sw_if_index: InterfaceIndex,
	pub host_if_name: FixedSizeString<typenum::U16>,
	pub host_if_type: LcpItfHostType,
	pub netns: FixedSizeString<typenum::U32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_itf_pair_add_del_reply_e8d4e804)]
pub struct LcpItfPairAddDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_itf_pair_add_del_v2_40482b80)]
pub struct LcpItfPairAddDelV2 {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub sw_if_index: InterfaceIndex,
	pub host_if_name: FixedSizeString<typenum::U16>,
	pub host_if_type: LcpItfHostType,
	pub netns: FixedSizeString<typenum::U32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_itf_pair_add_del_v2_reply_39452f52)]
pub struct LcpItfPairAddDelV2Reply {
	pub context: u32,
	pub retval: i32,
	pub host_sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_itf_pair_add_del_v3_40482b80)]
pub struct LcpItfPairAddDelV3 {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub sw_if_index: InterfaceIndex,
	pub host_if_name: FixedSizeString<typenum::U16>,
	pub host_if_type: LcpItfHostType,
	pub netns: FixedSizeString<typenum::U32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_itf_pair_add_del_v3_reply_c2502663)]
pub struct LcpItfPairAddDelV3Reply {
	pub context: u32,
	pub retval: i32,
	pub vif_index: u32,
	pub host_sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_itf_pair_get_f75ba505)]
pub struct LcpItfPairGet {
	pub client_index: u32,
	pub context: u32,
	pub cursor: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_itf_pair_get_reply_53b48f5d)]
pub struct LcpItfPairGetReply {
	pub context: u32,
	pub retval: i32,
	pub cursor: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_itf_pair_get_v2_47250981)]
pub struct LcpItfPairGetV2 {
	pub client_index: u32,
	pub context: u32,
	pub cursor: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_itf_pair_get_v2_reply_53b48f5d)]
pub struct LcpItfPairGetV2Reply {
	pub context: u32,
	pub retval: i32,
	pub cursor: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_itf_pair_details_8b5481af)]
pub struct LcpItfPairDetails {
	pub context: u32,
	pub phy_sw_if_index: InterfaceIndex,
	pub host_sw_if_index: InterfaceIndex,
	pub vif_index: u32,
	pub host_if_name: FixedSizeString<typenum::U16>,
	pub host_if_type: LcpItfHostType,
	pub netns: FixedSizeString<typenum::U32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_itf_pair_replace_begin_51077d14)]
pub struct LcpItfPairReplaceBegin {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_itf_pair_replace_begin_reply_e8d4e804)]
pub struct LcpItfPairReplaceBeginReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_itf_pair_replace_end_51077d14)]
pub struct LcpItfPairReplaceEnd {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(lcp_itf_pair_replace_end_reply_e8d4e804)]
pub struct LcpItfPairReplaceEndReply {
	pub context: u32,
	pub retval: i32,
}
