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
// Implementation for mac_entry
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MacEntry {
	pub sw_if_index: InterfaceIndex,
	pub mac_addr: MacAddress,
	pub action: MacEventAction,
	pub flags: u8,
}
// Implementation for bridge_domain_sw_if
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BridgeDomainSwIf {
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub shg: u8,
}
// Implementation for bd_ip_mac
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BdIpMac {
	pub bd_id: u32,
	pub ip: Address,
	pub mac: MacAddress,
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum MacEventAction {
	 MAC_EVENT_ACTION_API_ADD=0,
	 MAC_EVENT_ACTION_API_DELETE=1,
	 MAC_EVENT_ACTION_API_MOVE=2,
}
impl Default for MacEventAction {
	fn default() -> Self { MacEventAction::MAC_EVENT_ACTION_API_ADD }
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum BdFlags {
	 BRIDGE_API_FLAG_NONE=0,
	 BRIDGE_API_FLAG_LEARN=1,
	 BRIDGE_API_FLAG_FWD=2,
	 BRIDGE_API_FLAG_FLOOD=4,
	 BRIDGE_API_FLAG_UU_FLOOD=8,
	 BRIDGE_API_FLAG_ARP_TERM=16,
	 BRIDGE_API_FLAG_ARP_UFWD=32,
}
impl Default for BdFlags {
	fn default() -> Self { BdFlags::BRIDGE_API_FLAG_NONE }
}
impl AsEnumFlag for BdFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 0 => BdFlags::BRIDGE_API_FLAG_NONE,
			 1 => BdFlags::BRIDGE_API_FLAG_LEARN,
			 2 => BdFlags::BRIDGE_API_FLAG_FWD,
			 4 => BdFlags::BRIDGE_API_FLAG_FLOOD,
			 8 => BdFlags::BRIDGE_API_FLAG_UU_FLOOD,
			 16 => BdFlags::BRIDGE_API_FLAG_ARP_TERM,
			 32 => BdFlags::BRIDGE_API_FLAG_ARP_UFWD,
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 32 as u32
	}
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum L2PortType {
	 L2_API_PORT_TYPE_NORMAL=0,
	 L2_API_PORT_TYPE_BVI=1,
	 L2_API_PORT_TYPE_UU_FWD=2,
}
impl Default for L2PortType {
	fn default() -> Self { L2PortType::L2_API_PORT_TYPE_NORMAL }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_xconnect_details_472b6b67)]
pub struct L2XconnectDetails {
	pub context: u32,
	pub rx_sw_if_index: InterfaceIndex,
	pub tx_sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_xconnect_dump_51077d14)]
pub struct L2XconnectDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_fib_table_details_a44ef6b8)]
pub struct L2FibTableDetails {
	pub context: u32,
	pub bd_id: u32,
	pub mac: MacAddress,
	pub sw_if_index: InterfaceIndex,
	pub static_mac: bool,
	pub filter_mac: bool,
	pub bvi_mac: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_fib_table_dump_c25fdce6)]
pub struct L2FibTableDump {
	pub client_index: u32,
	pub context: u32,
	pub bd_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_fib_clear_table_51077d14)]
pub struct L2FibClearTable {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_fib_clear_table_reply_e8d4e804)]
pub struct L2FibClearTableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2fib_flush_all_51077d14)]
pub struct L2fibFlushAll {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2fib_flush_all_reply_e8d4e804)]
pub struct L2fibFlushAllReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2fib_flush_bd_c25fdce6)]
pub struct L2fibFlushBd {
	pub client_index: u32,
	pub context: u32,
	pub bd_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2fib_flush_bd_reply_e8d4e804)]
pub struct L2fibFlushBdReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2fib_flush_int_f9e6675e)]
pub struct L2fibFlushInt {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2fib_flush_int_reply_e8d4e804)]
pub struct L2fibFlushIntReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2fib_add_del_eddda487)]
pub struct L2fibAddDel {
	pub client_index: u32,
	pub context: u32,
	pub mac: MacAddress,
	pub bd_id: u32,
	pub sw_if_index: InterfaceIndex,
	pub is_add: bool,
	pub static_mac: bool,
	pub filter_mac: bool,
	pub bvi_mac: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2fib_add_del_reply_e8d4e804)]
pub struct L2fibAddDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(want_l2_macs_events_9aabdfde)]
pub struct WantL2MacsEvents {
	pub client_index: u32,
	pub context: u32,
	pub learn_limit: u32,
	pub scan_delay: u8,
	pub max_macs_in_event: u8,
	pub enable_disable: bool,
	pub pid: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(want_l2_macs_events_reply_e8d4e804)]
pub struct WantL2MacsEventsReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(want_l2_macs_events2_cc1377b0)]
pub struct WantL2MacsEvents2 {
	pub client_index: u32,
	pub context: u32,
	pub max_macs_in_event: u8,
	pub enable_disable: bool,
	pub pid: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(want_l2_macs_events2_reply_e8d4e804)]
pub struct WantL2MacsEvents2Reply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2fib_set_scan_delay_a3b968a4)]
pub struct L2fibSetScanDelay {
	pub client_index: u32,
	pub context: u32,
	pub scan_delay: u16,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2fib_set_scan_delay_reply_e8d4e804)]
pub struct L2fibSetScanDelayReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_macs_event_44b8fd64)]
pub struct L2MacsEvent {
	pub client_index: u32,
	pub pid: u32,
	pub n_macs: u32,
	pub mac: VariableSizeArray<MacEntry>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_flags_fc41cfe8)]
pub struct L2Flags {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub is_set: bool,
	pub feature_bitmap: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_flags_reply_29b2a2b3)]
pub struct L2FlagsReply {
	pub context: u32,
	pub retval: i32,
	pub resulting_feature_bitmap: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bridge_domain_set_mac_age_b537ad7b)]
pub struct BridgeDomainSetMacAge {
	pub client_index: u32,
	pub context: u32,
	pub bd_id: u32,
	pub mac_age: u8,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bridge_domain_set_mac_age_reply_e8d4e804)]
pub struct BridgeDomainSetMacAgeReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bridge_domain_set_default_learn_limit_f097ffce)]
pub struct BridgeDomainSetDefaultLearnLimit {
	pub client_index: u32,
	pub context: u32,
	pub learn_limit: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bridge_domain_set_default_learn_limit_reply_e8d4e804)]
pub struct BridgeDomainSetDefaultLearnLimitReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bridge_domain_set_learn_limit_89c52b5f)]
pub struct BridgeDomainSetLearnLimit {
	pub client_index: u32,
	pub context: u32,
	pub bd_id: u32,
	pub learn_limit: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bridge_domain_set_learn_limit_reply_e8d4e804)]
pub struct BridgeDomainSetLearnLimitReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bridge_domain_add_del_600b7170)]
pub struct BridgeDomainAddDel {
	pub client_index: u32,
	pub context: u32,
	pub bd_id: u32,
	pub flood: bool,
	pub uu_flood: bool,
	pub forward: bool,
	pub learn: bool,
	pub arp_term: bool,
	pub arp_ufwd: bool,
	pub mac_age: u8,
	pub bd_tag: FixedSizeString<typenum::U64>,
	pub is_add: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bridge_domain_add_del_reply_e8d4e804)]
pub struct BridgeDomainAddDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bridge_domain_dump_74396a43)]
pub struct BridgeDomainDump {
	pub client_index: u32,
	pub context: u32,
	pub bd_id: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bridge_domain_details_0fa506fd)]
pub struct BridgeDomainDetails {
	pub context: u32,
	pub bd_id: u32,
	pub flood: bool,
	pub uu_flood: bool,
	pub forward: bool,
	pub learn: bool,
	pub arp_term: bool,
	pub arp_ufwd: bool,
	pub mac_age: u8,
	pub bd_tag: FixedSizeString<typenum::U64>,
	pub bvi_sw_if_index: InterfaceIndex,
	pub uu_fwd_sw_if_index: InterfaceIndex,
	pub n_sw_ifs: u32,
	pub sw_if_details: VariableSizeArray<BridgeDomainSwIf>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bridge_flags_1b0c5fbd)]
pub struct BridgeFlags {
	pub client_index: u32,
	pub context: u32,
	pub bd_id: u32,
	pub is_set: bool,
	 pub flags: EnumFlag<BdFlags>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bridge_flags_reply_29b2a2b3)]
pub struct BridgeFlagsReply {
	pub context: u32,
	pub retval: i32,
	pub resulting_feature_bitmap: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_interface_vlan_tag_rewrite_62cc0bbc)]
pub struct L2InterfaceVlanTagRewrite {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub vtr_op: u32,
	pub push_dot1q: u32,
	pub tag1: u32,
	pub tag2: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_interface_vlan_tag_rewrite_reply_e8d4e804)]
pub struct L2InterfaceVlanTagRewriteReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_interface_pbb_tag_rewrite_38e802a8)]
pub struct L2InterfacePbbTagRewrite {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub vtr_op: u32,
	pub outer_tag: u16,
	pub b_dmac: MacAddress,
	pub b_smac: MacAddress,
	pub b_vlanid: u16,
	pub i_sid: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_interface_pbb_tag_rewrite_reply_e8d4e804)]
pub struct L2InterfacePbbTagRewriteReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_patch_add_del_a1f6a6f3)]
pub struct L2PatchAddDel {
	pub client_index: u32,
	pub context: u32,
	pub rx_sw_if_index: InterfaceIndex,
	pub tx_sw_if_index: InterfaceIndex,
	pub is_add: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_patch_add_del_reply_e8d4e804)]
pub struct L2PatchAddDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_l2_xconnect_4fa28a85)]
pub struct SwInterfaceSetL2Xconnect {
	pub client_index: u32,
	pub context: u32,
	pub rx_sw_if_index: InterfaceIndex,
	pub tx_sw_if_index: InterfaceIndex,
	pub enable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_l2_xconnect_reply_e8d4e804)]
pub struct SwInterfaceSetL2XconnectReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_l2_bridge_d0678b13)]
pub struct SwInterfaceSetL2Bridge {
	pub client_index: u32,
	pub context: u32,
	pub rx_sw_if_index: InterfaceIndex,
	pub bd_id: u32,
	pub port_type: L2PortType,
	pub shg: u8,
	pub enable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_l2_bridge_reply_e8d4e804)]
pub struct SwInterfaceSetL2BridgeReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bd_ip_mac_add_del_0257c869)]
pub struct BdIpMacAddDel {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub entry: BdIpMac,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bd_ip_mac_add_del_reply_e8d4e804)]
pub struct BdIpMacAddDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bd_ip_mac_flush_c25fdce6)]
pub struct BdIpMacFlush {
	pub client_index: u32,
	pub context: u32,
	pub bd_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bd_ip_mac_flush_reply_e8d4e804)]
pub struct BdIpMacFlushReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bd_ip_mac_details_545af86a)]
pub struct BdIpMacDetails {
	pub context: u32,
	pub entry: BdIpMac,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bd_ip_mac_dump_c25fdce6)]
pub struct BdIpMacDump {
	pub client_index: u32,
	pub context: u32,
	pub bd_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_interface_efp_filter_5501adee)]
pub struct L2InterfaceEfpFilter {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub enable_disable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_interface_efp_filter_reply_e8d4e804)]
pub struct L2InterfaceEfpFilterReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_vpath_ae6cfcfb)]
pub struct SwInterfaceSetVpath {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub enable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_set_vpath_reply_e8d4e804)]
pub struct SwInterfaceSetVpathReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bvi_create_f5398559)]
pub struct BviCreate {
	pub client_index: u32,
	pub context: u32,
	pub mac: MacAddress,
	pub user_instance: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bvi_create_reply_5383d31f)]
pub struct BviCreateReply {
	pub context: u32,
	pub retval: i32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bvi_delete_f9e6675e)]
pub struct BviDelete {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(bvi_delete_reply_e8d4e804)]
pub struct BviDeleteReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(want_l2_arp_term_events_3ec6d6c2)]
pub struct WantL2ArpTermEvents {
	pub client_index: u32,
	pub context: u32,
	pub enable: bool,
	pub pid: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(want_l2_arp_term_events_reply_e8d4e804)]
pub struct WantL2ArpTermEventsReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(l2_arp_term_event_6963e07a)]
pub struct L2ArpTermEvent {
	pub client_index: u32,
	pub pid: u32,
	pub ip: Address,
	pub sw_if_index: InterfaceIndex,
	pub mac: MacAddress,
}
