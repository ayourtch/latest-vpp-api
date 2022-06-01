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
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum NodeFlag {
	 NODE_FLAG_FRAME_NO_FREE_AFTER_DISPATCH=1,
	 NODE_FLAG_IS_OUTPUT=2,
	 NODE_FLAG_IS_DROP=4,
	 NODE_FLAG_IS_PUNT=8,
	 NODE_FLAG_IS_HANDOFF=16,
	 NODE_FLAG_TRACE=32,
	 NODE_FLAG_SWITCH_FROM_INTERRUPT_TO_POLLING_MODE=64,
	 NODE_FLAG_SWITCH_FROM_POLLING_TO_INTERRUPT_MODE=128,
	 NODE_FLAG_TRACE_SUPPORTED=256,
}
impl Default for NodeFlag {
	fn default() -> Self { NodeFlag::NODE_FLAG_FRAME_NO_FREE_AFTER_DISPATCH }
}
impl AsEnumFlag for NodeFlag {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 1 => NodeFlag::NODE_FLAG_FRAME_NO_FREE_AFTER_DISPATCH,
			 2 => NodeFlag::NODE_FLAG_IS_OUTPUT,
			 4 => NodeFlag::NODE_FLAG_IS_DROP,
			 8 => NodeFlag::NODE_FLAG_IS_PUNT,
			 16 => NodeFlag::NODE_FLAG_IS_HANDOFF,
			 32 => NodeFlag::NODE_FLAG_TRACE,
			 64 => NodeFlag::NODE_FLAG_SWITCH_FROM_INTERRUPT_TO_POLLING_MODE,
			 128 => NodeFlag::NODE_FLAG_SWITCH_FROM_POLLING_TO_INTERRUPT_MODE,
			 256 => NodeFlag::NODE_FLAG_TRACE_SUPPORTED,
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 32 as u32
	}
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(graph_node_get_39c8792e)]
pub struct GraphNodeGet {
	pub client_index : u32,
	pub context : u32,
	pub cursor : u32,
	pub index : u32,
	pub name : FixedSizeString<typenum::U64>,
	 pub flags : EnumFlag<NodeFlag>,
	pub want_arcs : bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(graph_node_get_reply_53b48f5d)]
pub struct GraphNodeGetReply {
	pub context : u32,
	pub retval : i32,
	pub cursor : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(graph_node_details_ac762018)]
pub struct GraphNodeDetails {
	pub context : u32,
	pub index : u32,
	pub name : FixedSizeString<typenum::U64>,
	 pub flags : EnumFlag<NodeFlag>,
	pub n_arcs : u32,
	pub arcs_out : VariableSizeArray<u32>,
}
