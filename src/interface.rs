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
use crate::ethernet_types::*;
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_flags_f5aec1b8)]
pub struct SwInterfaceSetFlags {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	 pub flags: EnumFlag<IfStatusFlags>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_flags_reply_e8d4e804)]
pub struct SwInterfaceSetFlagsReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_promisc_d40860d4)]
pub struct SwInterfaceSetPromisc {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub promisc_on: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_promisc_reply_e8d4e804)]
pub struct SwInterfaceSetPromiscReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(hw_interface_set_mtu_e6746899)]
pub struct HwInterfaceSetMtu {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub mtu: u16,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(hw_interface_set_mtu_reply_e8d4e804)]
pub struct HwInterfaceSetMtuReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_mtu_5cbe85e5)]
pub struct SwInterfaceSetMtu {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub mtu: FixedSizeArray<u32, typenum::U4>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_mtu_reply_e8d4e804)]
pub struct SwInterfaceSetMtuReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_ip_directed_broadcast_ae6cfcfb)]
pub struct SwInterfaceSetIpDirectedBroadcast {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub enable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_ip_directed_broadcast_reply_e8d4e804)]
pub struct SwInterfaceSetIpDirectedBroadcastReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_event_2d3d95a7)]
pub struct SwInterfaceEvent {
	pub client_index: u32,
	pub pid: u32,
	pub sw_if_index: InterfaceIndex,
	 pub flags: EnumFlag<IfStatusFlags>,
	pub deleted: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(want_interface_events_476f5a08)]
pub struct WantInterfaceEvents {
	pub client_index: u32,
	pub context: u32,
	pub enable_disable: u32,
	pub pid: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(want_interface_events_reply_e8d4e804)]
pub struct WantInterfaceEventsReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_details_6c221fc7)]
pub struct SwInterfaceDetails {
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub sup_sw_if_index: u32,
	pub l2_address: MacAddress,
	 pub flags: EnumFlag<IfStatusFlags>,
	pub typ: IfType,
	pub link_duplex: LinkDuplex,
	pub link_speed: u32,
	pub link_mtu: u16,
	pub mtu: FixedSizeArray<u32, typenum::U4>,
	pub sub_id: u32,
	pub sub_number_of_tags: u8,
	pub sub_outer_vlan_id: u16,
	pub sub_inner_vlan_id: u16,
	 pub sub_if_flags: EnumFlag<SubIfFlags>,
	pub vtr_op: u32,
	pub vtr_push_dot1q: u32,
	pub vtr_tag1: u32,
	pub vtr_tag2: u32,
	pub outer_tag: u16,
	pub b_dmac: MacAddress,
	pub b_smac: MacAddress,
	pub b_vlanid: u16,
	pub i_sid: u32,
	pub interface_name: FixedSizeString<typenum::U64>,
	pub interface_dev_type: FixedSizeString<typenum::U64>,
	pub tag: FixedSizeString<typenum::U64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_dump_aa610c27)]
pub struct SwInterfaceDump {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub name_filter_valid: bool,
	pub name_filter: VariableSizeString,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_add_del_address_5463d73b)]
pub struct SwInterfaceAddDelAddress {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub is_add: bool,
	pub del_all: bool,
	pub prefix: AddressWithPrefix,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_add_del_address_reply_e8d4e804)]
pub struct SwInterfaceAddDelAddressReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_address_replace_begin_51077d14)]
pub struct SwInterfaceAddressReplaceBegin {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_address_replace_begin_reply_e8d4e804)]
pub struct SwInterfaceAddressReplaceBeginReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_address_replace_end_51077d14)]
pub struct SwInterfaceAddressReplaceEnd {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_address_replace_end_reply_e8d4e804)]
pub struct SwInterfaceAddressReplaceEndReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_table_df42a577)]
pub struct SwInterfaceSetTable {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub is_ipv6: bool,
	pub vrf_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_table_reply_e8d4e804)]
pub struct SwInterfaceSetTableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_get_table_2d033de4)]
pub struct SwInterfaceGetTable {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub is_ipv6: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_get_table_reply_a6eb0109)]
pub struct SwInterfaceGetTableReply {
	pub context: u32,
	pub retval: i32,
	pub vrf_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_unnumbered_154a6439)]
pub struct SwInterfaceSetUnnumbered {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub unnumbered_sw_if_index: InterfaceIndex,
	pub is_add: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_unnumbered_reply_e8d4e804)]
pub struct SwInterfaceSetUnnumberedReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_clear_stats_f9e6675e)]
pub struct SwInterfaceClearStats {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_clear_stats_reply_e8d4e804)]
pub struct SwInterfaceClearStatsReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_tag_add_del_426f8bc1)]
pub struct SwInterfaceTagAddDel {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub sw_if_index: InterfaceIndex,
	pub tag: FixedSizeString<typenum::U64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_tag_add_del_reply_e8d4e804)]
pub struct SwInterfaceTagAddDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_add_del_mac_address_638bb9f4)]
pub struct SwInterfaceAddDelMacAddress {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: u32,
	pub addr: MacAddress,
	pub is_add: u8,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_add_del_mac_address_reply_e8d4e804)]
pub struct SwInterfaceAddDelMacAddressReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_mac_address_c536e7eb)]
pub struct SwInterfaceSetMacAddress {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub mac_address: MacAddress,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_mac_address_reply_e8d4e804)]
pub struct SwInterfaceSetMacAddressReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_get_mac_address_f9e6675e)]
pub struct SwInterfaceGetMacAddress {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_get_mac_address_reply_40ef2c08)]
pub struct SwInterfaceGetMacAddressReply {
	pub context: u32,
	pub retval: i32,
	pub mac_address: MacAddress,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_rx_mode_b04d1cfe)]
pub struct SwInterfaceSetRxMode {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub queue_id_valid: bool,
	pub queue_id: u32,
	pub mode: RxMode,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_rx_mode_reply_e8d4e804)]
pub struct SwInterfaceSetRxModeReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_rx_placement_db65f3c9)]
pub struct SwInterfaceSetRxPlacement {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub queue_id: u32,
	pub worker_id: u32,
	pub is_main: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_rx_placement_reply_e8d4e804)]
pub struct SwInterfaceSetRxPlacementReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_tx_placement_4e0cd5ff)]
pub struct SwInterfaceSetTxPlacement {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub queue_id: u32,
	pub array_size: u32,
	pub threads: VariableSizeArray<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_tx_placement_reply_e8d4e804)]
pub struct SwInterfaceSetTxPlacementReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_interface_name_45a1d548)]
pub struct SwInterfaceSetInterfaceName {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub name: FixedSizeString<typenum::U64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_interface_name_reply_e8d4e804)]
pub struct SwInterfaceSetInterfaceNameReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_rx_placement_dump_f9e6675e)]
pub struct SwInterfaceRxPlacementDump {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_rx_placement_details_9e44a7ce)]
pub struct SwInterfaceRxPlacementDetails {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub queue_id: u32,
	pub worker_id: u32,
	pub mode: RxMode,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_tx_placement_get_47250981)]
pub struct SwInterfaceTxPlacementGet {
	pub client_index: u32,
	pub context: u32,
	pub cursor: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_tx_placement_get_reply_53b48f5d)]
pub struct SwInterfaceTxPlacementGetReply {
	pub context: u32,
	pub retval: i32,
	pub cursor: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_tx_placement_details_00381a2e)]
pub struct SwInterfaceTxPlacementDetails {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub queue_id: u32,
	pub shared: u8,
	pub array_size: u32,
	pub threads: VariableSizeArray<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(interface_name_renumber_2b8858b8)]
pub struct InterfaceNameRenumber {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub new_show_dev_instance: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(interface_name_renumber_reply_e8d4e804)]
pub struct InterfaceNameRenumberReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(create_subif_790ca755)]
pub struct CreateSubif {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub sub_id: u32,
	 pub sub_if_flags: EnumFlag<SubIfFlags>,
	pub outer_vlan_id: u16,
	pub inner_vlan_id: u16,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(create_subif_reply_5383d31f)]
pub struct CreateSubifReply {
	pub context: u32,
	pub retval: i32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(create_vlan_subif_af34ac8b)]
pub struct CreateVlanSubif {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub vlan_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(create_vlan_subif_reply_5383d31f)]
pub struct CreateVlanSubifReply {
	pub context: u32,
	pub retval: i32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(delete_subif_f9e6675e)]
pub struct DeleteSubif {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(delete_subif_reply_e8d4e804)]
pub struct DeleteSubifReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(create_loopback_42bb5d22)]
pub struct CreateLoopback {
	pub client_index: u32,
	pub context: u32,
	pub mac_address: MacAddress,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(create_loopback_reply_5383d31f)]
pub struct CreateLoopbackReply {
	pub context: u32,
	pub retval: i32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(create_loopback_instance_d36a3ee2)]
pub struct CreateLoopbackInstance {
	pub client_index: u32,
	pub context: u32,
	pub mac_address: MacAddress,
	pub is_specified: bool,
	pub user_instance: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(create_loopback_instance_reply_5383d31f)]
pub struct CreateLoopbackInstanceReply {
	pub context: u32,
	pub retval: i32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(delete_loopback_f9e6675e)]
pub struct DeleteLoopback {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(delete_loopback_reply_e8d4e804)]
pub struct DeleteLoopbackReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(collect_detailed_interface_stats_5501adee)]
pub struct CollectDetailedInterfaceStats {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub enable_disable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(collect_detailed_interface_stats_reply_e8d4e804)]
pub struct CollectDetailedInterfaceStatsReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(pcap_trace_on_cb39e968)]
pub struct PcapTraceOn {
	pub client_index: u32,
	pub context: u32,
	pub capture_rx: bool,
	pub capture_tx: bool,
	pub capture_drop: bool,
	pub filter: bool,
	pub preallocate_data: bool,
	pub free_data: bool,
	pub max_packets: u32,
	pub max_bytes_per_packet: u32,
	pub sw_if_index: InterfaceIndex,
	pub error: FixedSizeString<typenum::U128>,
	pub filename: FixedSizeString<typenum::U64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(pcap_trace_on_reply_e8d4e804)]
pub struct PcapTraceOnReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(pcap_trace_off_51077d14)]
pub struct PcapTraceOff {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(pcap_trace_off_reply_e8d4e804)]
pub struct PcapTraceOffReply {
	pub context: u32,
	pub retval: i32,
}
