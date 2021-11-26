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
use crate::interface_types::*; 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum ClassifyAction { 
	 CLASSIFY_API_ACTION_NONE=0, 
	 CLASSIFY_API_ACTION_SET_IP4_FIB_INDEX=1, 
	 CLASSIFY_API_ACTION_SET_IP6_FIB_INDEX=2, 
	 CLASSIFY_API_ACTION_SET_METADATA=3, 
} 
impl Default for ClassifyAction { 
	fn default() -> Self { ClassifyAction::CLASSIFY_API_ACTION_NONE }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum PolicerClassifyTable { 
	 POLICER_CLASSIFY_API_TABLE_IP4=1, 
	 POLICER_CLASSIFY_API_TABLE_IP6=2, 
	 POLICER_CLASSIFY_API_TABLE_L2=3, 
} 
impl Default for PolicerClassifyTable { 
	fn default() -> Self { PolicerClassifyTable::POLICER_CLASSIFY_API_TABLE_IP4 }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum FlowClassifyTable { 
	 FLOW_CLASSIFY_API_TABLE_IP4=1, 
	 FLOW_CLASSIFY_API_TABLE_IP6=2, 
} 
impl Default for FlowClassifyTable { 
	fn default() -> Self { FlowClassifyTable::FLOW_CLASSIFY_API_TABLE_IP4 }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_add_del_table_6849e39e)] 
pub struct ClassifyAddDelTable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub del_chain : bool, 
	pub table_index : u32, 
	pub nbuckets : u32, 
	pub memory_size : u32, 
	pub skip_n_vectors : u32, 
	pub match_n_vectors : u32, 
	pub next_table_index : u32, 
	pub miss_next_index : u32, 
	pub current_data_flag : u8, 
	pub current_data_offset : i16, 
	pub mask_len : u32, 
	pub mask : VariableSizeArray<u8>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_add_del_table_reply_05486349)] 
pub struct ClassifyAddDelTableReply { 
	pub context : u32, 
	pub retval : i32, 
	pub new_table_index : u32, 
	pub skip_n_vectors : u32, 
	pub match_n_vectors : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_add_del_session_f20879f0)] 
pub struct ClassifyAddDelSession { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub table_index : u32, 
	pub hit_next_index : u32, 
	pub opaque_index : u32, 
	pub advance : i32, 
	pub action : ClassifyAction, 
	pub metadata : u32, 
	pub match_len : u32, 
	pub mach : VariableSizeArray<u8>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_add_del_session_reply_e8d4e804)] 
pub struct ClassifyAddDelSessionReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(policer_classify_set_interface_de7ad708)] 
pub struct PolicerClassifySetInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub ip4_table_index : u32, 
	pub ip6_table_index : u32, 
	pub l2_table_index : u32, 
	pub is_add : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(policer_classify_set_interface_reply_e8d4e804)] 
pub struct PolicerClassifySetInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(policer_classify_dump_6bfe6603)] 
pub struct PolicerClassifyDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub typ : PolicerClassifyTable, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(policer_classify_details_dfd08765)] 
pub struct PolicerClassifyDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub table_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_table_ids_51077d14)] 
pub struct ClassifyTableIds { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_table_ids_reply_d1d20e1d)] 
pub struct ClassifyTableIdsReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub ids : VariableSizeArray<u32>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_table_by_interface_f9e6675e)] 
pub struct ClassifyTableByInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_table_by_interface_reply_ed4197db)] 
pub struct ClassifyTableByInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
	pub l2_table_id : u32, 
	pub ip4_table_id : u32, 
	pub ip6_table_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_table_info_0cca2cd9)] 
pub struct ClassifyTableInfo { 
	pub client_index : u32, 
	pub context : u32, 
	pub table_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_table_info_reply_4a573c0e)] 
pub struct ClassifyTableInfoReply { 
	pub context : u32, 
	pub retval : i32, 
	pub table_id : u32, 
	pub nbuckets : u32, 
	pub match_n_vectors : u32, 
	pub skip_n_vectors : u32, 
	pub active_sessions : u32, 
	pub next_table_index : u32, 
	pub miss_next_index : u32, 
	pub mask_length : u32, 
	pub mask : VariableSizeArray<u8>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_session_dump_0cca2cd9)] 
pub struct ClassifySessionDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub table_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_session_details_60e3ef94)] 
pub struct ClassifySessionDetails { 
	pub context : u32, 
	pub retval : i32, 
	pub table_id : u32, 
	pub hit_next_index : u32, 
	pub advance : i32, 
	pub opaque_index : u32, 
	pub match_length : u32, 
	pub mach : VariableSizeArray<u8>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(flow_classify_set_interface_b6192f1c)] 
pub struct FlowClassifySetInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub ip4_table_index : u32, 
	pub ip6_table_index : u32, 
	pub is_add : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(flow_classify_set_interface_reply_e8d4e804)] 
pub struct FlowClassifySetInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(flow_classify_dump_8a6ad43d)] 
pub struct FlowClassifyDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub typ : FlowClassifyTable, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(flow_classify_details_dfd08765)] 
pub struct FlowClassifyDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub table_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_set_interface_ip_table_e0b097c7)] 
pub struct ClassifySetInterfaceIpTable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_ipv6 : bool, 
	pub sw_if_index : InterfaceIndex, 
	pub table_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_set_interface_ip_table_reply_e8d4e804)] 
pub struct ClassifySetInterfaceIpTableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_set_interface_l2_tables_5a6ddf65)] 
pub struct ClassifySetInterfaceL2Tables { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub ip4_table_index : u32, 
	pub ip6_table_index : u32, 
	pub other_table_index : u32, 
	pub is_input : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_set_interface_l2_tables_reply_e8d4e804)] 
pub struct ClassifySetInterfaceL2TablesReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(input_acl_set_interface_de7ad708)] 
pub struct InputAclSetInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub ip4_table_index : u32, 
	pub ip6_table_index : u32, 
	pub l2_table_index : u32, 
	pub is_add : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(input_acl_set_interface_reply_e8d4e804)] 
pub struct InputAclSetInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(punt_acl_add_del_a93bf3a0)] 
pub struct PuntAclAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub ip4_table_index : u32, 
	pub ip6_table_index : u32, 
	pub is_add : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(punt_acl_add_del_reply_e8d4e804)] 
pub struct PuntAclAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(output_acl_set_interface_de7ad708)] 
pub struct OutputAclSetInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub ip4_table_index : u32, 
	pub ip6_table_index : u32, 
	pub l2_table_index : u32, 
	pub is_add : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(output_acl_set_interface_reply_e8d4e804)] 
pub struct OutputAclSetInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_pcap_lookup_table_e1b4cc6b)] 
pub struct ClassifyPcapLookupTable { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub skip_n_vectors : u32, 
	pub match_n_vectors : u32, 
	pub mask_len : u32, 
	pub mask : VariableSizeArray<u8>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_pcap_lookup_table_reply_9c6c6773)] 
pub struct ClassifyPcapLookupTableReply { 
	pub context : u32, 
	pub retval : i32, 
	pub table_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_pcap_set_table_006051b3)] 
pub struct ClassifyPcapSetTable { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub table_index : u32, 
	pub sort_masks : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_pcap_set_table_reply_9c6c6773)] 
pub struct ClassifyPcapSetTableReply { 
	pub context : u32, 
	pub retval : i32, 
	pub table_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_pcap_get_tables_f9e6675e)] 
pub struct ClassifyPcapGetTables { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_pcap_get_tables_reply_5f5bc9e6)] 
pub struct ClassifyPcapGetTablesReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub indices : VariableSizeArray<u32>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_trace_lookup_table_3f7b72e4)] 
pub struct ClassifyTraceLookupTable { 
	pub client_index : u32, 
	pub context : u32, 
	pub skip_n_vectors : u32, 
	pub match_n_vectors : u32, 
	pub mask_len : u32, 
	pub mask : VariableSizeArray<u8>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_trace_lookup_table_reply_9c6c6773)] 
pub struct ClassifyTraceLookupTableReply { 
	pub context : u32, 
	pub retval : i32, 
	pub table_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_trace_set_table_3909b55a)] 
pub struct ClassifyTraceSetTable { 
	pub client_index : u32, 
	pub context : u32, 
	pub table_index : u32, 
	pub sort_masks : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_trace_set_table_reply_9c6c6773)] 
pub struct ClassifyTraceSetTableReply { 
	pub context : u32, 
	pub retval : i32, 
	pub table_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_trace_get_tables_51077d14)] 
pub struct ClassifyTraceGetTables { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(classify_trace_get_tables_reply_5f5bc9e6)] 
pub struct ClassifyTraceGetTablesReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub indices : VariableSizeArray<u32>, 
} 
