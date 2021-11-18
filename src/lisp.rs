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
// Implementation for lisp_adjacency 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct LispAdjacency { 
	pub reid : Eid, 
	pub leid : Eid, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum LispLocatorSetFilter { 
	 LISP_LOCATOR_SET_FILTER_API_ALL=0, 
	 LISP_LOCATOR_SET_FILTER_API_LOCAL=1, 
	 LISP_LOCATOR_SET_FILTER_API_REMOTE=2, 
} 
impl Default for LispLocatorSetFilter { 
	fn default() -> Self { LispLocatorSetFilter::LISP_LOCATOR_SET_FILTER_API_ALL }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_add_del_locator_set_6fcd6471)] 
pub struct LispAddDelLocatorSet { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub locator_set_name : FixedSizeString<typenum::U64>, 
	pub locator_num : u32, 
	pub locators : VariableSizeArray<LocalLocator>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_add_del_locator_set_reply_b6666db4)] 
pub struct LispAddDelLocatorSetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub ls_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_add_del_locator_af4d8f13)] 
pub struct LispAddDelLocator { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub locator_set_name : FixedSizeString<typenum::U64>, 
	pub sw_if_index : InterfaceIndex, 
	pub priority : u8, 
	pub weight : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_add_del_locator_reply_e8d4e804)] 
pub struct LispAddDelLocatorReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_add_del_local_eid_4e5a83a2)] 
pub struct LispAddDelLocalEid { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub eid : Eid, 
	pub locator_set_name : FixedSizeString<typenum::U64>, 
	pub vni : u32, 
	pub key : HmacKey, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_add_del_local_eid_reply_e8d4e804)] 
pub struct LispAddDelLocalEidReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_add_del_map_server_ce19e32d)] 
pub struct LispAddDelMapServer { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub ip_address : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_add_del_map_server_reply_e8d4e804)] 
pub struct LispAddDelMapServerReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_add_del_map_resolver_ce19e32d)] 
pub struct LispAddDelMapResolver { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub ip_address : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_add_del_map_resolver_reply_e8d4e804)] 
pub struct LispAddDelMapResolverReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_enable_disable_c264d7bf)] 
pub struct LispEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_enable_disable_reply_e8d4e804)] 
pub struct LispEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_pitr_set_locator_set_486e2b76)] 
pub struct LispPitrSetLocatorSet { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub ls_name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_pitr_set_locator_set_reply_e8d4e804)] 
pub struct LispPitrSetLocatorSetReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_use_petr_d87dbad9)] 
pub struct LispUsePetr { 
	pub client_index : u32, 
	pub context : u32, 
	pub ip_address : Address, 
	pub is_add : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_use_petr_reply_e8d4e804)] 
pub struct LispUsePetrReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_lisp_use_petr_51077d14)] 
pub struct ShowLispUsePetr { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_lisp_use_petr_reply_22b9a4b0)] 
pub struct ShowLispUsePetrReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_petr_enable : bool, 
	pub ip_address : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_lisp_rloc_probe_state_51077d14)] 
pub struct ShowLispRlocProbeState { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_lisp_rloc_probe_state_reply_e33a377b)] 
pub struct ShowLispRlocProbeStateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_enabled : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_rloc_probe_enable_disable_c264d7bf)] 
pub struct LispRlocProbeEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_rloc_probe_enable_disable_reply_e8d4e804)] 
pub struct LispRlocProbeEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_map_register_enable_disable_c264d7bf)] 
pub struct LispMapRegisterEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_map_register_enable_disable_reply_e8d4e804)] 
pub struct LispMapRegisterEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_lisp_map_register_state_51077d14)] 
pub struct ShowLispMapRegisterState { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_lisp_map_register_state_reply_e33a377b)] 
pub struct ShowLispMapRegisterStateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_enabled : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_map_request_mode_f43c26ae)] 
pub struct LispMapRequestMode { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_src_dst : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_map_request_mode_reply_e8d4e804)] 
pub struct LispMapRequestModeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_lisp_map_request_mode_51077d14)] 
pub struct ShowLispMapRequestMode { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_lisp_map_request_mode_reply_5b05038e)] 
pub struct ShowLispMapRequestModeReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_src_dst : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_add_del_remote_mapping_6d5c789e)] 
pub struct LispAddDelRemoteMapping { 
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
#[message_name_and_crc(lisp_add_del_remote_mapping_reply_e8d4e804)] 
pub struct LispAddDelRemoteMappingReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_add_del_adjacency_2ce0e6f6)] 
pub struct LispAddDelAdjacency { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub vni : u32, 
	pub reid : Eid, 
	pub leid : Eid, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_add_del_adjacency_reply_e8d4e804)] 
pub struct LispAddDelAdjacencyReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_add_del_map_request_itr_rlocs_6be88e45)] 
pub struct LispAddDelMapRequestItrRlocs { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub locator_set_name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_add_del_map_request_itr_rlocs_reply_e8d4e804)] 
pub struct LispAddDelMapRequestItrRlocsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_eid_table_add_del_map_9481416b)] 
pub struct LispEidTableAddDelMap { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub vni : u32, 
	pub dp_table : u32, 
	pub is_l2 : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_eid_table_add_del_map_reply_e8d4e804)] 
pub struct LispEidTableAddDelMapReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_locator_dump_b954fad7)] 
pub struct LispLocatorDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub ls_index : u32, 
	pub ls_name : FixedSizeString<typenum::U64>, 
	pub is_index_set : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_locator_details_2c620ffe)] 
pub struct LispLocatorDetails { 
	pub context : u32, 
	pub local : u8, 
	pub sw_if_index : InterfaceIndex, 
	pub ip_address : Address, 
	pub priority : u8, 
	pub weight : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_locator_set_details_5b33a105)] 
pub struct LispLocatorSetDetails { 
	pub context : u32, 
	pub ls_index : u32, 
	pub ls_name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_locator_set_dump_c2cb5922)] 
pub struct LispLocatorSetDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub filter : LispLocatorSetFilter, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_eid_table_details_1c29f792)] 
pub struct LispEidTableDetails { 
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
#[message_name_and_crc(lisp_eid_table_dump_629468b5)] 
pub struct LispEidTableDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub eid_set : u8, 
	pub prefix_length : u8, 
	pub vni : u32, 
	pub eid : Eid, 
	pub filter : LispLocatorSetFilter, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_adjacencies_get_reply_807257bf)] 
pub struct LispAdjacenciesGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub adjacencies : VariableSizeArray<LispAdjacency>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_adjacencies_get_8d1f2fe9)] 
pub struct LispAdjacenciesGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub vni : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_eid_table_map_details_0b6859e2)] 
pub struct LispEidTableMapDetails { 
	pub context : u32, 
	pub vni : u32, 
	pub dp_table : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_eid_table_map_dump_d6cf0c3d)] 
pub struct LispEidTableMapDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_l2 : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_eid_table_vni_dump_51077d14)] 
pub struct LispEidTableVniDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_eid_table_vni_details_64abc01e)] 
pub struct LispEidTableVniDetails { 
	pub context : u32, 
	pub vni : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_map_resolver_details_3e78fc57)] 
pub struct LispMapResolverDetails { 
	pub context : u32, 
	pub ip_address : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_map_resolver_dump_51077d14)] 
pub struct LispMapResolverDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_map_server_details_3e78fc57)] 
pub struct LispMapServerDetails { 
	pub context : u32, 
	pub ip_address : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_map_server_dump_51077d14)] 
pub struct LispMapServerDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_lisp_status_51077d14)] 
pub struct ShowLispStatus { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_lisp_status_reply_9e8f10c0)] 
pub struct ShowLispStatusReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_lisp_enabled : bool, 
	pub is_gpe_enabled : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_get_map_request_itr_rlocs_51077d14)] 
pub struct LispGetMapRequestItrRlocs { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(lisp_get_map_request_itr_rlocs_reply_76580f3a)] 
pub struct LispGetMapRequestItrRlocsReply { 
	pub context : u32, 
	pub retval : i32, 
	pub locator_set_name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_lisp_pitr_51077d14)] 
pub struct ShowLispPitr { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_lisp_pitr_reply_27aa69b1)] 
pub struct ShowLispPitrReply { 
	pub context : u32, 
	pub retval : i32, 
	pub is_enabled : bool, 
	pub locator_set_name : FixedSizeString<typenum::U64>, 
} 
