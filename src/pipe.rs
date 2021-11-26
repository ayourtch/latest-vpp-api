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
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pipe_create_bb263bd3)] 
pub struct PipeCreate { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_specified : bool, 
	pub user_instance : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pipe_create_reply_b7ce310c)] 
pub struct PipeCreateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
	pub pipe_sw_if_index : FixedSizeArray<InterfaceIndex, typenum::U2>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pipe_delete_f9e6675e)] 
pub struct PipeDelete { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pipe_delete_reply_e8d4e804)] 
pub struct PipeDeleteReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pipe_dump_51077d14)] 
pub struct PipeDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pipe_details_c52b799d)] 
pub struct PipeDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub pipe_sw_if_index : FixedSizeArray<InterfaceIndex, typenum::U2>, 
	pub instance : u32, 
} 
