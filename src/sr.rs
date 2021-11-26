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
use crate::interface_types::*; 
use crate::sr_types::*; 
// Implementation for srv6_sid_list 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct Srv6SidList { 
	pub num_sids : u8, 
	pub weight : u32, 
	pub sids : FixedSizeArray<Ip6Address, typenum::U16>, 
} 
// Implementation for srv6_sid_list_with_sl_index 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct Srv6SidListWithSlIndex { 
	pub num_sids : u8, 
	pub weight : u32, 
	pub sl_index : u32, 
	pub sids : FixedSizeArray<Ip6Address, typenum::U16>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_localsid_add_del_5a36c324)] 
pub struct SrLocalsidAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_del : bool, 
	pub localsid : Ip6Address, 
	pub end_psp : bool, 
	pub behavior : SrBehavior, 
	pub sw_if_index : InterfaceIndex, 
	pub vlan_index : u32, 
	pub fib_table : u32, 
	pub nh_addr : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_localsid_add_del_reply_e8d4e804)] 
pub struct SrLocalsidAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_policy_add_44ac92e8)] 
pub struct SrPolicyAdd { 
	pub client_index : u32, 
	pub context : u32, 
	pub bsid_addr : Ip6Address, 
	pub weight : u32, 
	pub is_encap : bool, 
	pub is_spray : bool, 
	pub fib_table : u32, 
	pub sids : Srv6SidList, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_policy_add_reply_e8d4e804)] 
pub struct SrPolicyAddReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_policy_mod_b97bb56e)] 
pub struct SrPolicyMod { 
	pub client_index : u32, 
	pub context : u32, 
	pub bsid_addr : Ip6Address, 
	pub sr_policy_index : u32, 
	pub fib_table : u32, 
	pub operation : SrPolicyOp, 
	pub sl_index : u32, 
	pub weight : u32, 
	pub sids : Srv6SidList, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_policy_mod_reply_e8d4e804)] 
pub struct SrPolicyModReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_policy_del_cb4d48d5)] 
pub struct SrPolicyDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub bsid_addr : Ip6Address, 
	pub sr_policy_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_policy_del_reply_e8d4e804)] 
pub struct SrPolicyDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_set_encap_source_d3bad5e1)] 
pub struct SrSetEncapSource { 
	pub client_index : u32, 
	pub context : u32, 
	pub encaps_source : Ip6Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_set_encap_source_reply_e8d4e804)] 
pub struct SrSetEncapSourceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_set_encap_hop_limit_aa75d7d0)] 
pub struct SrSetEncapHopLimit { 
	pub client_index : u32, 
	pub context : u32, 
	pub hop_limit : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_set_encap_hop_limit_reply_e8d4e804)] 
pub struct SrSetEncapHopLimitReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_steering_add_del_e46b0a0f)] 
pub struct SrSteeringAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_del : bool, 
	pub bsid_addr : Ip6Address, 
	pub sr_policy_index : u32, 
	pub table_id : u32, 
	pub prefix : Prefix, 
	pub sw_if_index : InterfaceIndex, 
	pub traffic_type : SrSteer, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_steering_add_del_reply_e8d4e804)] 
pub struct SrSteeringAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_localsids_dump_51077d14)] 
pub struct SrLocalsidsDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_localsids_details_2e9221b9)] 
pub struct SrLocalsidsDetails { 
	pub context : u32, 
	pub addr : Ip6Address, 
	pub end_psp : bool, 
	pub behavior : SrBehavior, 
	pub fib_table : u32, 
	pub vlan_index : u32, 
	pub xconnect_nh_addr : Address, 
	pub xconnect_iface_or_vrf_table : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_policies_dump_51077d14)] 
pub struct SrPoliciesDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_policies_details_db6ff2a1)] 
pub struct SrPoliciesDetails { 
	pub context : u32, 
	pub bsid : Ip6Address, 
	pub is_spray : bool, 
	pub is_encap : bool, 
	pub fib_table : u32, 
	pub num_sid_lists : u8, 
	pub sid_lists : VariableSizeArray<Srv6SidList>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_policies_with_sl_index_dump_51077d14)] 
pub struct SrPoliciesWithSlIndexDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_policies_with_sl_index_details_ca2e9bc8)] 
pub struct SrPoliciesWithSlIndexDetails { 
	pub context : u32, 
	pub bsid : Ip6Address, 
	pub is_spray : bool, 
	pub is_encap : bool, 
	pub fib_table : u32, 
	pub num_sid_lists : u8, 
	pub sid_lists : VariableSizeArray<Srv6SidListWithSlIndex>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_steering_pol_dump_51077d14)] 
pub struct SrSteeringPolDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sr_steering_pol_details_d41258c9)] 
pub struct SrSteeringPolDetails { 
	pub context : u32, 
	pub traffic_type : SrSteer, 
	pub fib_table : u32, 
	pub prefix : Prefix, 
	pub sw_if_index : InterfaceIndex, 
	pub bsid : Ip6Address, 
} 
