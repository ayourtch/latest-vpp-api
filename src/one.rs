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
use crate::lisp_types::*; 
use crate::interface_types::*; 
use crate::ethernet_types::*; 
// Implementation for one_l2_arp_entry 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct OneL2ArpEntry { 
	pub mac : MacAddress, 
	pub ip4 : Ip4Address, 
} 
// Implementation for one_ndp_entry 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct OneNdpEntry { 
	pub mac : MacAddress, 
	pub ip6 : Ip6Address, 
} 
// Implementation for one_adjacency 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct OneAdjacency { 
	pub reid : Eid, 
	pub leid : Eid, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum OneMapMode { 
	 ONE_MAP_MODE_API_DST_ONLY=0, 
	 ONE_MAP_MODE_API_SRC_DST=1, 
} 
impl Default for OneMapMode { 
	fn default() -> Self { OneMapMode::ONE_MAP_MODE_API_DST_ONLY }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum OneFilter { 
	 ONE_FILTER_API_ALL=0, 
	 ONE_FILTER_API_LOCAL=1, 
	 ONE_FILTER_API_REMOTE=2, 
} 
impl Default for OneFilter { 
	fn default() -> Self { OneFilter::ONE_FILTER_API_ALL }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_locator_set_6fcd6471)] 
pub struct OneAddDelLocatorSet { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub locator_set_name : FixedSizeString<typenum::U64>, 
	pub locator_num : u32, 
	pub locators : VariableSizeArray<LocalLocator>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_locator_set_reply_b6666db4)] 
pub struct OneAddDelLocatorSetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub ls_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_locator_af4d8f13)] 
pub struct OneAddDelLocator { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub locator_set_name : FixedSizeString<typenum::U64>, 
	pub sw_if_index : InterfaceIndex, 
	pub priority : u8, 
	pub weight : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_locator_reply_e8d4e804)] 
pub struct OneAddDelLocatorReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_local_eid_4e5a83a2)] 
pub struct OneAddDelLocalEid { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub eid : Eid, 
	pub locator_set_name : FixedSizeString<typenum::U64>, 
	pub vni : u32, 
	pub key : HmacKey, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_local_eid_reply_e8d4e804)] 
pub struct OneAddDelLocalEidReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_map_register_set_ttl_dd59f1f3)] 
pub struct OneMapRegisterSetTtl { 
	pub client_index : u32, 
	pub context : u32, 
	pub ttl : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_map_register_set_ttl_reply_e8d4e804)] 
pub struct OneMapRegisterSetTtlReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_map_register_ttl_51077d14)] 
pub struct ShowOneMapRegisterTtl { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_map_register_ttl_reply_fa83dd66)] 
pub struct ShowOneMapRegisterTtlReply { 
	pub context : u32, 
	pub retval : i32, 
	pub ttl : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_map_server_ce19e32d)] 
pub struct OneAddDelMapServer { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub ip_address : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_map_server_reply_e8d4e804)] 
pub struct OneAddDelMapServerReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_map_resolver_ce19e32d)] 
pub struct OneAddDelMapResolver { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub ip_address : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_map_resolver_reply_e8d4e804)] 
pub struct OneAddDelMapResolverReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_enable_disable_c264d7bf)] 
pub struct OneEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_enable_disable_reply_e8d4e804)] 
pub struct OneEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_nsh_set_locator_set_486e2b76)] 
pub struct OneNshSetLocatorSet { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub ls_name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_nsh_set_locator_set_reply_e8d4e804)] 
pub struct OneNshSetLocatorSetReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_pitr_set_locator_set_486e2b76)] 
pub struct OnePitrSetLocatorSet { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub ls_name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_pitr_set_locator_set_reply_e8d4e804)] 
pub struct OnePitrSetLocatorSetReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_use_petr_d87dbad9)] 
pub struct OneUsePetr { 
	pub client_index : u32, 
	pub context : u32, 
	pub ip_address : Address, 
	pub is_add : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_use_petr_reply_e8d4e804)] 
pub struct OneUsePetrReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_use_petr_51077d14)] 
pub struct ShowOneUsePetr { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_use_petr_reply_84a03528)] 
pub struct ShowOneUsePetrReply { 
	pub context : u32, 
	pub retval : i32, 
	pub status : bool, 
	pub ip_address : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_rloc_probe_state_51077d14)] 
pub struct ShowOneRlocProbeState { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_rloc_probe_state_reply_f15abb16)] 
pub struct ShowOneRlocProbeStateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_rloc_probe_enable_disable_c264d7bf)] 
pub struct OneRlocProbeEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_rloc_probe_enable_disable_reply_e8d4e804)] 
pub struct OneRlocProbeEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_map_register_enable_disable_c264d7bf)] 
pub struct OneMapRegisterEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_map_register_enable_disable_reply_e8d4e804)] 
pub struct OneMapRegisterEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_map_register_state_51077d14)] 
pub struct ShowOneMapRegisterState { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_map_register_state_reply_f15abb16)] 
pub struct ShowOneMapRegisterStateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_map_request_mode_ffa5d2f5)] 
pub struct OneMapRequestMode { 
	pub client_index : u32, 
	pub context : u32, 
	pub mode : OneMapMode, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_map_request_mode_reply_e8d4e804)] 
pub struct OneMapRequestModeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_map_request_mode_51077d14)] 
pub struct ShowOneMapRequestMode { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_map_request_mode_reply_d41f3c1d)] 
pub struct ShowOneMapRequestModeReply { 
	pub context : u32, 
	pub retval : i32, 
	pub mode : OneMapMode, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_remote_mapping_6d5c789e)] 
pub struct OneAddDelRemoteMapping { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub is_src_dst : bool, 
	pub del_all : bool, 
	pub vni : u32, 
	pub action : u8, 
	pub deid : Eid, 
	pub seid : Eid, 
	pub rloc_num : u32, 
	pub rlocs : VariableSizeArray<RemoteLocator>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_remote_mapping_reply_e8d4e804)] 
pub struct OneAddDelRemoteMappingReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_l2_arp_entry_1aa5e8b3)] 
pub struct OneAddDelL2ArpEntry { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub bd : u32, 
	pub entry : OneL2ArpEntry, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_l2_arp_entry_reply_e8d4e804)] 
pub struct OneAddDelL2ArpEntryReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_l2_arp_entries_get_4d418cf4)] 
pub struct OneL2ArpEntriesGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub bd : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_l2_arp_entries_get_reply_b0dd200f)] 
pub struct OneL2ArpEntriesGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub entries : VariableSizeArray<OneL2ArpEntry>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_ndp_entry_0f8a287c)] 
pub struct OneAddDelNdpEntry { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub bd : u32, 
	pub entry : OneNdpEntry, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_ndp_entry_reply_e8d4e804)] 
pub struct OneAddDelNdpEntryReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_ndp_entries_get_4d418cf4)] 
pub struct OneNdpEntriesGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub bd : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_ndp_entries_get_reply_70719b1a)] 
pub struct OneNdpEntriesGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub entries : VariableSizeArray<OneNdpEntry>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_set_transport_protocol_07b6b85f)] 
pub struct OneSetTransportProtocol { 
	pub client_index : u32, 
	pub context : u32, 
	pub protocol : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_set_transport_protocol_reply_e8d4e804)] 
pub struct OneSetTransportProtocolReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_get_transport_protocol_51077d14)] 
pub struct OneGetTransportProtocol { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_get_transport_protocol_reply_62a28eb3)] 
pub struct OneGetTransportProtocolReply { 
	pub context : u32, 
	pub retval : i32, 
	pub protocol : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_ndp_bd_get_51077d14)] 
pub struct OneNdpBdGet { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_ndp_bd_get_reply_221ac888)] 
pub struct OneNdpBdGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub bridge_domains : VariableSizeArray<u32>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_l2_arp_bd_get_51077d14)] 
pub struct OneL2ArpBdGet { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_l2_arp_bd_get_reply_221ac888)] 
pub struct OneL2ArpBdGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub bridge_domains : VariableSizeArray<u32>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_adjacency_9e830312)] 
pub struct OneAddDelAdjacency { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : u8, 
	pub vni : u32, 
	pub reid : Eid, 
	pub leid : Eid, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_adjacency_reply_e8d4e804)] 
pub struct OneAddDelAdjacencyReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_map_request_itr_rlocs_6be88e45)] 
pub struct OneAddDelMapRequestItrRlocs { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub locator_set_name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_add_del_map_request_itr_rlocs_reply_e8d4e804)] 
pub struct OneAddDelMapRequestItrRlocsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_eid_table_add_del_map_9481416b)] 
pub struct OneEidTableAddDelMap { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub vni : u32, 
	pub dp_table : u32, 
	pub is_l2 : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_eid_table_add_del_map_reply_e8d4e804)] 
pub struct OneEidTableAddDelMapReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_locator_dump_9b11076c)] 
pub struct OneLocatorDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub ls_index : u32, 
	pub ls_name : FixedSizeString<typenum::U64>, 
	pub is_index_set : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_locator_details_2c620ffe)] 
pub struct OneLocatorDetails { 
	pub context : u32, 
	pub local : u8, 
	pub sw_if_index : InterfaceIndex, 
	pub ip_address : Address, 
	pub priority : u8, 
	pub weight : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_locator_set_details_5b33a105)] 
pub struct OneLocatorSetDetails { 
	pub context : u32, 
	pub ls_index : u32, 
	pub ls_name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_locator_set_dump_71190768)] 
pub struct OneLocatorSetDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub filter : OneFilter, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_eid_table_details_1c29f792)] 
pub struct OneEidTableDetails { 
	pub context : u32, 
	pub locator_set_index : u32, 
	pub action : u8, 
	pub is_local : bool, 
	pub is_src_dst : bool, 
	pub vni : u32, 
	pub deid : Eid, 
	pub seid : Eid, 
	pub ttl : u32, 
	pub authoritative : u8, 
	pub key : HmacKey, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_eid_table_dump_bd190269)] 
pub struct OneEidTableDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub eid_set : bool, 
	pub vni : u32, 
	pub eid : Eid, 
	pub filter : OneFilter, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_adjacencies_get_reply_085bab89)] 
pub struct OneAdjacenciesGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub adjacencies : VariableSizeArray<OneAdjacency>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_adjacencies_get_8d1f2fe9)] 
pub struct OneAdjacenciesGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub vni : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_eid_table_map_details_0b6859e2)] 
pub struct OneEidTableMapDetails { 
	pub context : u32, 
	pub vni : u32, 
	pub dp_table : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_eid_table_map_dump_d6cf0c3d)] 
pub struct OneEidTableMapDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_l2 : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_eid_table_vni_dump_51077d14)] 
pub struct OneEidTableVniDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_eid_table_vni_details_64abc01e)] 
pub struct OneEidTableVniDetails { 
	pub context : u32, 
	pub vni : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_map_resolver_details_3e78fc57)] 
pub struct OneMapResolverDetails { 
	pub context : u32, 
	pub ip_address : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_map_resolver_dump_51077d14)] 
pub struct OneMapResolverDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_map_server_details_3e78fc57)] 
pub struct OneMapServerDetails { 
	pub context : u32, 
	pub ip_address : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_map_server_dump_51077d14)] 
pub struct OneMapServerDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_status_51077d14)] 
pub struct ShowOneStatus { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_status_reply_961bb25b)] 
pub struct ShowOneStatusReply { 
	pub context : u32, 
	pub retval : i32, 
	pub feature_status : bool, 
	pub gpe_status : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_get_map_request_itr_rlocs_51077d14)] 
pub struct OneGetMapRequestItrRlocs { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_get_map_request_itr_rlocs_reply_76580f3a)] 
pub struct OneGetMapRequestItrRlocsReply { 
	pub context : u32, 
	pub retval : i32, 
	pub locator_set_name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_nsh_mapping_51077d14)] 
pub struct ShowOneNshMapping { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_nsh_mapping_reply_46478c02)] 
pub struct ShowOneNshMappingReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_set : bool, 
	pub locator_set_name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_pitr_51077d14)] 
pub struct ShowOnePitr { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_pitr_reply_a2d1a49f)] 
pub struct ShowOnePitrReply { 
	pub context : u32, 
	pub retval : i32, 
	pub status : bool, 
	pub locator_set_name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_stats_dump_51077d14)] 
pub struct OneStatsDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_stats_details_2eb74678)] 
pub struct OneStatsDetails { 
	pub context : u32, 
	pub vni : u32, 
	pub deid : Eid, 
	pub seid : Eid, 
	pub rloc : Address, 
	pub lloc : Address, 
	pub pkt_count : u32, 
	pub bytes : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_stats_flush_51077d14)] 
pub struct OneStatsFlush { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_stats_flush_reply_e8d4e804)] 
pub struct OneStatsFlushReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_stats_enable_disable_c264d7bf)] 
pub struct OneStatsEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_stats_enable_disable_reply_e8d4e804)] 
pub struct OneStatsEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_stats_enable_disable_51077d14)] 
pub struct ShowOneStatsEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_stats_enable_disable_reply_f15abb16)] 
pub struct ShowOneStatsEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_map_register_fallback_threshold_f7d4a475)] 
pub struct OneMapRegisterFallbackThreshold { 
	pub client_index : u32, 
	pub context : u32, 
	pub value : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_map_register_fallback_threshold_reply_e8d4e804)] 
pub struct OneMapRegisterFallbackThresholdReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_map_register_fallback_threshold_51077d14)] 
pub struct ShowOneMapRegisterFallbackThreshold { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_one_map_register_fallback_threshold_reply_c93a9113)] 
pub struct ShowOneMapRegisterFallbackThresholdReply { 
	pub context : u32, 
	pub retval : i32, 
	pub value : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_enable_disable_xtr_mode_c264d7bf)] 
pub struct OneEnableDisableXtrMode { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_enable_disable_xtr_mode_reply_e8d4e804)] 
pub struct OneEnableDisableXtrModeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_show_xtr_mode_51077d14)] 
pub struct OneShowXtrMode { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_show_xtr_mode_reply_f15abb16)] 
pub struct OneShowXtrModeReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_enable_disable_petr_mode_c264d7bf)] 
pub struct OneEnableDisablePetrMode { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_enable_disable_petr_mode_reply_e8d4e804)] 
pub struct OneEnableDisablePetrModeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_show_petr_mode_51077d14)] 
pub struct OneShowPetrMode { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_show_petr_mode_reply_f15abb16)] 
pub struct OneShowPetrModeReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_enable_disable_pitr_mode_c264d7bf)] 
pub struct OneEnableDisablePitrMode { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_enable_disable_pitr_mode_reply_e8d4e804)] 
pub struct OneEnableDisablePitrModeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_show_pitr_mode_51077d14)] 
pub struct OneShowPitrMode { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(one_show_pitr_mode_reply_f15abb16)] 
pub struct OneShowPitrModeReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_enable : bool, 
} 
