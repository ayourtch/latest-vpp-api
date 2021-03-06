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
// Implementation for udp_encap
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UdpEncap {
	pub table_id: u32,
	pub src_port: u16,
	pub dst_port: u16,
	pub src_ip: Address,
	pub dst_ip: Address,
	pub id: u32,
}
// Implementation for udp_decap
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UdpDecap {
	pub is_ip4: u8,
	pub port: u16,
	pub next_proto: UdpDecapNextProto,
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum UdpDecapNextProto {
	 UDP_API_DECAP_PROTO_IP4=0,
	 UDP_API_DECAP_PROTO_IP6=1,
	 UDP_API_DECAP_PROTO_MPLS=2,
}
impl Default for UdpDecapNextProto {
	fn default() -> Self { UdpDecapNextProto::UDP_API_DECAP_PROTO_IP4 }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(udp_encap_add_f74a60b1)]
pub struct UdpEncapAdd {
	pub client_index: u32,
	pub context: u32,
	pub udp_encap: UdpEncap,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(udp_encap_add_reply_e2fc8294)]
pub struct UdpEncapAddReply {
	pub context: u32,
	pub retval: i32,
	pub id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(udp_encap_del_3a91bde5)]
pub struct UdpEncapDel {
	pub client_index: u32,
	pub context: u32,
	pub id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(udp_encap_del_reply_e8d4e804)]
pub struct UdpEncapDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(udp_encap_dump_51077d14)]
pub struct UdpEncapDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(udp_encap_details_8cfb9c76)]
pub struct UdpEncapDetails {
	pub context: u32,
	pub udp_encap: UdpEncap,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(udp_decap_add_del_d14a4f47)]
pub struct UdpDecapAddDel {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub udp_decap: UdpDecap,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(udp_decap_add_del_reply_e8d4e804)]
pub struct UdpDecapAddDelReply {
	pub context: u32,
	pub retval: i32,
}
