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
// Implementation for module_version 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct ModuleVersion { 
	pub major : u32, 
	pub minor : u32, 
	pub patch : u32, 
	pub name : FixedSizeString<typenum::U64>, 
} 
// Implementation for message_table_entry 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct MessageTableEntry { 
	pub index : u16, 
	pub name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(memclnt_create_9c5e1c2f)] 
pub struct MemclntCreate { 
	pub context : u32, 
	pub ctx_quota : i32, 
	pub input_queue : u64, 
	pub name : FixedSizeString<typenum::U64>, 
	pub api_versions : FixedSizeArray<u32, typenum::U8>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(memclnt_create_reply_42ec4560)] 
pub struct MemclntCreateReply { 
	pub context : u32, 
	pub response : i32, 
	pub handle : u64, 
	pub index : u32, 
	pub message_table : u64, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(memclnt_delete_7e1c04e3)] 
pub struct MemclntDelete { 
	pub index : u32, 
	pub handle : u64, 
	pub do_cleanup : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(memclnt_delete_reply_3d3b6312)] 
pub struct MemclntDeleteReply { 
	pub response : i32, 
	pub handle : u64, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(rx_thread_exit_c3a3a452)] 
pub struct RxThreadExit { 
	pub dummy : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(memclnt_rx_thread_suspend_c3a3a452)] 
pub struct MemclntRxThreadSuspend { 
	pub dummy : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(memclnt_read_timeout_c3a3a452)] 
pub struct MemclntReadTimeout { 
	pub dummy : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(rpc_call_7e8a2c95)] 
pub struct RpcCall { 
	pub client_index : u32, 
	pub context : u32, 
	pub function : u64, 
	pub multicast : u8, 
	pub need_barrier_sync : u8, 
	pub send_reply : u8, 
	pub data_len : u32, 
	pub data : VariableSizeArray<u8>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(rpc_call_reply_e8d4e804)] 
pub struct RpcCallReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(get_first_msg_id_ebf79a66)] 
pub struct GetFirstMsgId { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(get_first_msg_id_reply_7d337472)] 
pub struct GetFirstMsgIdReply { 
	pub context : u32, 
	pub retval : i32, 
	pub first_msg_id : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(api_versions_51077d14)] 
pub struct ApiVersions { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(api_versions_reply_5f0d99d6)] 
pub struct ApiVersionsReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub api_versions : VariableSizeArray<ModuleVersion>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(trace_plugin_msg_ids_f476d3ce)] 
pub struct TracePluginMsgIds { 
	pub client_index : u32, 
	pub context : u32, 
	pub plugin_name : FixedSizeString<typenum::U128>, 
	pub first_msg_id : u16, 
	pub last_msg_id : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sockclnt_create_455fb9c4)] 
pub struct SockclntCreate { 
	pub context : u32, 
	pub name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sockclnt_create_reply_35166268)] 
pub struct SockclntCreateReply { 
	pub client_index : u32, 
	pub context : u32, 
	pub response : i32, 
	pub index : u32, 
	pub count : u16, 
	pub message_table : VariableSizeArray<MessageTableEntry>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sockclnt_delete_8ac76db6)] 
pub struct SockclntDelete { 
	pub client_index : u32, 
	pub context : u32, 
	pub index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sockclnt_delete_reply_8f38b1ee)] 
pub struct SockclntDeleteReply { 
	pub context : u32, 
	pub response : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sock_init_shm_51646d92)] 
pub struct SockInitShm { 
	pub client_index : u32, 
	pub context : u32, 
	pub requested_size : u32, 
	pub nitems : u8, 
	pub configs : VariableSizeArray<u64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sock_init_shm_reply_e8d4e804)] 
pub struct SockInitShmReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(memclnt_keepalive_51077d14)] 
pub struct MemclntKeepalive { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(memclnt_keepalive_reply_e8d4e804)] 
pub struct MemclntKeepaliveReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(control_ping_51077d14)] 
pub struct ControlPing { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(control_ping_reply_f6b0b8ca)] 
pub struct ControlPingReply { 
	pub context : u32, 
	pub retval : i32, 
	pub client_index : u32, 
	pub vpe_pid : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(memclnt_create_v2_c4bd4882)] 
pub struct MemclntCreateV2 { 
	pub context : u32, 
	pub ctx_quota : i32, 
	pub input_queue : u64, 
	pub name : FixedSizeString<typenum::U64>, 
	pub api_versions : FixedSizeArray<u32, typenum::U8>, 
	pub keepalive : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(memclnt_create_v2_reply_42ec4560)] 
pub struct MemclntCreateV2Reply { 
	pub context : u32, 
	pub response : i32, 
	pub handle : u64, 
	pub index : u32, 
	pub message_table : u64, 
} 
