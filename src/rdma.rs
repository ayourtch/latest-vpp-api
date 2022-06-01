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
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum RdmaMode {
	 RDMA_API_MODE_AUTO=0,
	 RDMA_API_MODE_IBV=1,
	 RDMA_API_MODE_DV=2,
}
impl Default for RdmaMode {
	fn default() -> Self { RdmaMode::RDMA_API_MODE_AUTO }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum RdmaRss4 {
	 RDMA_API_RSS4_AUTO=0,
	 RDMA_API_RSS4_IP=1,
	 RDMA_API_RSS4_IP_UDP=2,
	 RDMA_API_RSS4_IP_TCP=3,
}
impl Default for RdmaRss4 {
	fn default() -> Self { RdmaRss4::RDMA_API_RSS4_AUTO }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum RdmaRss6 {
	 RDMA_API_RSS6_AUTO=0,
	 RDMA_API_RSS6_IP=1,
	 RDMA_API_RSS6_IP_UDP=2,
	 RDMA_API_RSS6_IP_TCP=3,
}
impl Default for RdmaRss6 {
	fn default() -> Self { RdmaRss6::RDMA_API_RSS6_AUTO }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(rdma_create_076fe418)]
pub struct RdmaCreate {
	pub client_index : u32,
	pub context : u32,
	pub host_if : FixedSizeString<typenum::U64>,
	pub name : FixedSizeString<typenum::U64>,
	pub rxq_num : u16,
	pub rxq_size : u16,
	pub txq_size : u16,
	pub mode : RdmaMode,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(rdma_create_v2_5826a4f3)]
pub struct RdmaCreateV2 {
	pub client_index : u32,
	pub context : u32,
	pub host_if : FixedSizeString<typenum::U64>,
	pub name : FixedSizeString<typenum::U64>,
	pub rxq_num : u16,
	pub rxq_size : u16,
	pub txq_size : u16,
	pub mode : RdmaMode,
	pub no_multi_seg : bool,
	pub max_pktlen : u16,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(rdma_create_v3_c6287ea8)]
pub struct RdmaCreateV3 {
	pub client_index : u32,
	pub context : u32,
	pub host_if : FixedSizeString<typenum::U64>,
	pub name : FixedSizeString<typenum::U64>,
	pub rxq_num : u16,
	pub rxq_size : u16,
	pub txq_size : u16,
	pub mode : RdmaMode,
	pub no_multi_seg : bool,
	pub max_pktlen : u16,
	pub rss4 : RdmaRss4,
	pub rss6 : RdmaRss6,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(rdma_create_reply_5383d31f)]
pub struct RdmaCreateReply {
	pub context : u32,
	pub retval : i32,
	pub sw_if_index : InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(rdma_create_v2_reply_5383d31f)]
pub struct RdmaCreateV2Reply {
	pub context : u32,
	pub retval : i32,
	pub sw_if_index : InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(rdma_create_v3_reply_5383d31f)]
pub struct RdmaCreateV3Reply {
	pub context : u32,
	pub retval : i32,
	pub sw_if_index : InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(rdma_delete_f9e6675e)]
pub struct RdmaDelete {
	pub client_index : u32,
	pub context : u32,
	pub sw_if_index : InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(rdma_delete_reply_e8d4e804)]
pub struct RdmaDeleteReply {
	pub context : u32,
	pub retval : i32,
}
