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
use crate::interface_types::*;
// Implementation for sw_if_counters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SwIfCounters {
	pub packets_rx: u64,
	pub packets_tx: u64,
	pub bytes_rx: u64,
	pub bytes_tx: u64,
}
// Implementation for tunnel_metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TunnelMetrics {
	pub sw_if_index: InterfaceIndex,
	pub reserved: u32,
	pub counters: SwIfCounters,
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum GtpuForwardingType {
	 GTPU_API_FORWARDING_NONE=0,
	 GTPU_API_FORWARDING_BAD_HEADER=1,
	 GTPU_API_FORWARDING_UNKNOWN_TEID=2,
	 GTPU_API_FORWARDING_UNKNOWN_TYPE=4,
}
impl Default for GtpuForwardingType {
	fn default() -> Self { GtpuForwardingType::GTPU_API_FORWARDING_NONE }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum GtpuDecapNextType {
	 GTPU_API_DECAP_NEXT_DROP=0,
	 GTPU_API_DECAP_NEXT_L2=1,
	 GTPU_API_DECAP_NEXT_IP4=2,
	 GTPU_API_DECAP_NEXT_IP6=3,
}
impl Default for GtpuDecapNextType {
	fn default() -> Self { GtpuDecapNextType::GTPU_API_DECAP_NEXT_DROP }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(gtpu_add_del_tunnel_ca983a2b)]
pub struct GtpuAddDelTunnel {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub src_address: Address,
	pub dst_address: Address,
	pub mcast_sw_if_index: InterfaceIndex,
	pub encap_vrf_id: u32,
	pub decap_next_index: u32,
	pub teid: u32,
	pub tteid: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(gtpu_add_del_tunnel_reply_5383d31f)]
pub struct GtpuAddDelTunnelReply {
	pub context: u32,
	pub retval: i32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(gtpu_add_del_tunnel_v2_a0c30713)]
pub struct GtpuAddDelTunnelV2 {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub src_address: Address,
	pub dst_address: Address,
	pub mcast_sw_if_index: InterfaceIndex,
	pub encap_vrf_id: u32,
	pub decap_next_index: GtpuDecapNextType,
	pub teid: u32,
	pub tteid: u32,
	pub pdu_extension: bool,
	pub qfi: u8,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(gtpu_add_del_tunnel_v2_reply_62b41304)]
pub struct GtpuAddDelTunnelV2Reply {
	pub context: u32,
	pub retval: i32,
	pub sw_if_index: InterfaceIndex,
	pub counters: SwIfCounters,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(gtpu_tunnel_update_tteid_79f33816)]
pub struct GtpuTunnelUpdateTteid {
	pub client_index: u32,
	pub context: u32,
	pub dst_address: Address,
	pub encap_vrf_id: u32,
	pub teid: u32,
	pub tteid: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(gtpu_tunnel_update_tteid_reply_e8d4e804)]
pub struct GtpuTunnelUpdateTteidReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(gtpu_tunnel_dump_f9e6675e)]
pub struct GtpuTunnelDump {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(gtpu_tunnel_details_27f434ae)]
pub struct GtpuTunnelDetails {
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub src_address: Address,
	pub dst_address: Address,
	pub mcast_sw_if_index: InterfaceIndex,
	pub encap_vrf_id: u32,
	pub decap_next_index: u32,
	pub teid: u32,
	pub tteid: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(gtpu_tunnel_v2_dump_f9e6675e)]
pub struct GtpuTunnelV2Dump {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(gtpu_tunnel_v2_details_8bf4ba92)]
pub struct GtpuTunnelV2Details {
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub src_address: Address,
	pub dst_address: Address,
	pub mcast_sw_if_index: InterfaceIndex,
	pub encap_vrf_id: u32,
	pub decap_next_index: GtpuDecapNextType,
	pub teid: u32,
	pub tteid: u32,
	pub pdu_extension: bool,
	pub qfi: u8,
	pub is_forwarding: bool,
	pub forwarding_type: GtpuForwardingType,
	pub counters: SwIfCounters,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_gtpu_bypass_65247409)]
pub struct SwInterfaceSetGtpuBypass {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub is_ipv6: bool,
	pub enable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_gtpu_bypass_reply_e8d4e804)]
pub struct SwInterfaceSetGtpuBypassReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(gtpu_offload_rx_f0b08786)]
pub struct GtpuOffloadRx {
	pub client_index: u32,
	pub context: u32,
	pub hw_if_index: u32,
	pub sw_if_index: u32,
	pub enable: u8,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(gtpu_offload_rx_reply_e8d4e804)]
pub struct GtpuOffloadRxReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(gtpu_add_del_forward_c6ccce13)]
pub struct GtpuAddDelForward {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub dst_address: Address,
	pub forwarding_type: GtpuForwardingType,
	pub encap_vrf_id: u32,
	pub decap_next_index: GtpuDecapNextType,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(gtpu_add_del_forward_reply_5383d31f)]
pub struct GtpuAddDelForwardReply {
	pub context: u32,
	pub retval: i32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(gtpu_get_transfer_counts_61410788)]
pub struct GtpuGetTransferCounts {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index_start: InterfaceIndex,
	pub capacity: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(gtpu_get_transfer_counts_reply_e35f04bc)]
pub struct GtpuGetTransferCountsReply {
	pub context: u32,
	pub retval: i32,
	pub count: u32,
	pub tunnels: VariableSizeArray<TunnelMetrics>,
}
