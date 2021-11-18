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
// Implementation for cnat_endpoint 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct CnatEndpoint { 
	pub addr : Address, 
	pub sw_if_index : InterfaceIndex, 
	pub if_af : AddressFamily, 
	pub port : u16, 
} 
// Implementation for cnat_endpoint_tuple 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct CnatEndpointTuple { 
	pub dst_ep : CnatEndpoint, 
	pub src_ep : CnatEndpoint, 
	pub flags : u8, 
} 
// Implementation for cnat_translation 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct CnatTranslation { 
	pub vip : CnatEndpoint, 
	pub id : u32, 
	pub ip_proto : IpProto, 
	pub is_real_ip : u8, 
	pub flags : u8, 
	pub lb_type : CnatLbType, 
	pub n_paths : u32, 
	pub paths : VariableSizeArray<CnatEndpointTuple>, 
} 
// Implementation for cnat_session 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct CnatSession { 
	pub src : CnatEndpoint, 
	pub dst : CnatEndpoint, 
	pub new : CnatEndpoint, 
	pub ip_proto : IpProto, 
	pub location : u8, 
	pub timestamp : f64, 
} 
#[derive(Debug,Serialize, Deserialize, Clone, Copy)] 
pub enum CnatTranslationFlags { 
	 CNAT_TRANSLATION_ALLOC_PORT=1, 
} 
impl Default for CnatTranslationFlags { 
	fn default() -> Self { CnatTranslationFlags::CNAT_TRANSLATION_ALLOC_PORT }
}
impl AsEnumFlag for CnatTranslationFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 1 => CnatTranslationFlags::CNAT_TRANSLATION_ALLOC_PORT, 
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 8 as u32
	}
}
#[derive(Debug,Serialize, Deserialize, Clone, Copy)] 
pub enum CnatEndpointTupleFlags { 
	 CNAT_EPT_NO_NAT=1, 
} 
impl Default for CnatEndpointTupleFlags { 
	fn default() -> Self { CnatEndpointTupleFlags::CNAT_EPT_NO_NAT }
}
impl AsEnumFlag for CnatEndpointTupleFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 1 => CnatEndpointTupleFlags::CNAT_EPT_NO_NAT, 
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 8 as u32
	}
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum CnatLbType { 
	 CNAT_LB_TYPE_DEFAULT=0, 
	 CNAT_LB_TYPE_MAGLEV=1, 
} 
impl Default for CnatLbType { 
	fn default() -> Self { CnatLbType::CNAT_LB_TYPE_DEFAULT }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum CnatSnatPolicyTable { 
	 CNAT_POLICY_INCLUDE_V4=0, 
	 CNAT_POLICY_INCLUDE_V6=1, 
	 CNAT_POLICY_POD=2, 
} 
impl Default for CnatSnatPolicyTable { 
	fn default() -> Self { CnatSnatPolicyTable::CNAT_POLICY_INCLUDE_V4 }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum CnatSnatPolicies { 
	 CNAT_POLICY_NONE=0, 
	 CNAT_POLICY_IF_PFX=1, 
	 CNAT_POLICY_K8S=2, 
} 
impl Default for CnatSnatPolicies { 
	fn default() -> Self { CnatSnatPolicies::CNAT_POLICY_NONE }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_translation_update_cd5aedf5)] 
pub struct CnatTranslationUpdate { 
	pub client_index : u32, 
	pub context : u32, 
	pub translation : CnatTranslation, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_translation_update_reply_e2fc8294)] 
pub struct CnatTranslationUpdateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_translation_del_3a91bde5)] 
pub struct CnatTranslationDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_translation_del_reply_e8d4e804)] 
pub struct CnatTranslationDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_translation_details_347e1f16)] 
pub struct CnatTranslationDetails { 
	pub context : u32, 
	pub translation : CnatTranslation, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_translation_dump_51077d14)] 
pub struct CnatTranslationDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_session_purge_51077d14)] 
pub struct CnatSessionPurge { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_session_purge_reply_e8d4e804)] 
pub struct CnatSessionPurgeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_session_details_7e5017c7)] 
pub struct CnatSessionDetails { 
	pub context : u32, 
	pub session : CnatSession, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_session_dump_51077d14)] 
pub struct CnatSessionDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_set_snat_addresses_d997e96c)] 
pub struct CnatSetSnatAddresses { 
	pub client_index : u32, 
	pub context : u32, 
	pub snat_ip4 : Ip4Address, 
	pub snat_ip6 : Ip6Address, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_set_snat_addresses_reply_e8d4e804)] 
pub struct CnatSetSnatAddressesReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_get_snat_addresses_51077d14)] 
pub struct CnatGetSnatAddresses { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_get_snat_addresses_reply_879513c1)] 
pub struct CnatGetSnatAddressesReply { 
	pub context : u32, 
	pub retval : i32, 
	pub id : u32, 
	pub snat_ip4 : Ip4Address, 
	pub snat_ip6 : Ip6Address, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_snat_policy_add_del_exclude_pfx_e26dd79a)] 
pub struct CnatSnatPolicyAddDelExcludePfx { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : u8, 
	pub prefix : Prefix, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_snat_policy_add_del_exclude_pfx_reply_e8d4e804)] 
pub struct CnatSnatPolicyAddDelExcludePfxReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_snat_policy_add_del_if_6828deca)] 
pub struct CnatSnatPolicyAddDelIf { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_add : u8, 
	pub table : CnatSnatPolicyTable, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_snat_policy_add_del_if_reply_e8d4e804)] 
pub struct CnatSnatPolicyAddDelIfReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_set_snat_policy_d3e6eaf4)] 
pub struct CnatSetSnatPolicy { 
	pub client_index : u32, 
	pub context : u32, 
	pub policy : CnatSnatPolicies, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cnat_set_snat_policy_reply_e8d4e804)] 
pub struct CnatSetSnatPolicyReply { 
	pub context : u32, 
	pub retval : i32, 
} 
