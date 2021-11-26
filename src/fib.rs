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
use crate::fib_types::*; 
// Implementation for fib_source 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct FibSource { 
	pub priority : u8, 
	pub id : u8, 
	pub name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(fib_source_add_b3ac2aec)] 
pub struct FibSourceAdd { 
	pub client_index : u32, 
	pub context : u32, 
	pub src : FibSource, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(fib_source_add_reply_604fd6f1)] 
pub struct FibSourceAddReply { 
	pub context : u32, 
	pub retval : i32, 
	pub id : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(fib_source_dump_51077d14)] 
pub struct FibSourceDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(fib_source_details_8668acdb)] 
pub struct FibSourceDetails { 
	pub context : u32, 
	pub src : FibSource, 
} 
