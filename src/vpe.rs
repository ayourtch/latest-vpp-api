/*
   Autogenerated Data, Do not Edit! 
*/
#![allow(dead_code,unused_mut,unused_variables,unused_must_use,non_camel_case_types,unused_imports,non_snake_case)]
use vpp_api_macros::{VppMessage,VppUnionIdent}; 
use std::convert::TryInto; 
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum;
use crate::vpe_types::*; 
// Implementation for thread_data 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct ThreadData { 
	pub id : u32, 
	pub name : FixedSizeString<typenum::U64>, 
	pub typ : FixedSizeString<typenum::U64>, 
	pub pid : u32, 
	pub cpu_id : u32, 
	pub core : u32, 
	pub cpu_socket : u32, 
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
#[message_name_and_crc(cli_23bfbfff)] 
pub struct Cli { 
	pub client_index : u32, 
	pub context : u32, 
	pub cmd_in_shmem : u64, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cli_inband_f8377302)] 
pub struct CliInband { 
	pub client_index : u32, 
	pub context : u32, 
	pub cmd : VariableSizeString, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cli_reply_06d68297)] 
pub struct CliReply { 
	pub context : u32, 
	pub retval : i32, 
	pub reply_in_shmem : u64, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(cli_inband_reply_05879051)] 
pub struct CliInbandReply { 
	pub context : u32, 
	pub retval : i32, 
	pub reply : VariableSizeString, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(get_node_index_f1984c64)] 
pub struct GetNodeIndex { 
	pub client_index : u32, 
	pub context : u32, 
	pub node_name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(get_node_index_reply_a8600b89)] 
pub struct GetNodeIndexReply { 
	pub context : u32, 
	pub retval : i32, 
	pub node_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(add_node_next_2457116d)] 
pub struct AddNodeNext { 
	pub client_index : u32, 
	pub context : u32, 
	pub node_name : FixedSizeString<typenum::U64>, 
	pub next_name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(add_node_next_reply_2ed75f32)] 
pub struct AddNodeNextReply { 
	pub context : u32, 
	pub retval : i32, 
	pub next_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_version_51077d14)] 
pub struct ShowVersion { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_version_reply_c919bde1)] 
pub struct ShowVersionReply { 
	pub context : u32, 
	pub retval : i32, 
	pub program : FixedSizeString<typenum::U32>, 
	pub version : FixedSizeString<typenum::U32>, 
	pub build_date : FixedSizeString<typenum::U32>, 
	pub build_directory : FixedSizeString<typenum::U256>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_threads_51077d14)] 
pub struct ShowThreads { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_threads_reply_efd78e83)] 
pub struct ShowThreadsReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub thread_data : VariableSizeArray<ThreadData>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(get_node_graph_51077d14)] 
pub struct GetNodeGraph { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(get_node_graph_reply_06d68297)] 
pub struct GetNodeGraphReply { 
	pub context : u32, 
	pub retval : i32, 
	pub reply_in_shmem : u64, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(get_next_index_2457116d)] 
pub struct GetNextIndex { 
	pub client_index : u32, 
	pub context : u32, 
	pub node_name : FixedSizeString<typenum::U64>, 
	pub next_name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(get_next_index_reply_2ed75f32)] 
pub struct GetNextIndexReply { 
	pub context : u32, 
	pub retval : i32, 
	pub next_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(log_dump_6ab31753)] 
pub struct LogDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub start_timestamp : Timestamp, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(log_details_03d61cc0)] 
pub struct LogDetails { 
	pub context : u32, 
	pub timestamp : Timestamp, 
	pub level : LogLevel, 
	pub msg_class : FixedSizeString<typenum::U32>, 
	pub message : FixedSizeString<typenum::U256>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_vpe_system_time_51077d14)] 
pub struct ShowVpeSystemTime { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(show_vpe_system_time_reply_7ffd8193)] 
pub struct ShowVpeSystemTimeReply { 
	pub context : u32, 
	pub retval : i32, 
	pub vpe_system_time : Timestamp, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(get_f64_endian_value_809fcd44)] 
pub struct GetF64EndianValue { 
	pub client_index : u32, 
	pub context : u32, 
	pub f64_one : f64, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(get_f64_endian_value_reply_7e02e404)] 
pub struct GetF64EndianValueReply { 
	pub context : u32, 
	pub retval : u32, 
	pub f64_one_result : f64, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(get_f64_increment_by_one_b64f027e)] 
pub struct GetF64IncrementByOne { 
	pub client_index : u32, 
	pub context : u32, 
	pub f64_value : f64, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(get_f64_increment_by_one_reply_d25dbaa3)] 
pub struct GetF64IncrementByOneReply { 
	pub context : u32, 
	pub retval : u32, 
	pub f64_value : f64, 
} 
