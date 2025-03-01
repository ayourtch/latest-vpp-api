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
use crate::mfib_types::*;
use crate::interface_types::*;
use crate::ethernet_types::*;
// Implementation for ip_table
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IpTable {
	pub table_id: u32,
	pub is_ip6: bool,
	pub name: FixedSizeString<typenum::U64>,
}
// Implementation for ip_route
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IpRoute {
	pub table_id: u32,
	pub stats_index: u32,
	pub prefix: Prefix,
	pub n_paths: u8,
	pub paths: VariableSizeArray<FibPath>,
}
// Implementation for ip_route_v2
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IpRouteV2 {
	pub table_id: u32,
	pub stats_index: u32,
	pub prefix: Prefix,
	pub n_paths: u8,
	pub src: u8,
	pub paths: VariableSizeArray<FibPath>,
}
// Implementation for ip_mroute
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IpMroute {
	pub table_id: u32,
	pub entry_flags: EnumFlag<MfibEntryFlags>,
	pub rpf_id: u32,
	pub prefix: Mprefix,
	pub n_paths: u8,
	pub paths: VariableSizeArray<MfibPath>,
}
// Implementation for punt_redirect
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PuntRedirect {
	pub rx_sw_if_index: InterfaceIndex,
	pub tx_sw_if_index: InterfaceIndex,
	pub nh: Address,
}
// Implementation for punt_redirect_v2
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PuntRedirectV2 {
	pub rx_sw_if_index: InterfaceIndex,
	pub af: AddressFamily,
	pub n_paths: u32,
	pub paths: VariableSizeArray<FibPath>,
}
// Implementation for ip_path_mtu
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IpPathMtu {
	pub client_index: u32,
	pub context: u32,
	pub table_id: u32,
	pub nh: Address,
	pub path_mtu: u16,
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum IpReassType {
	 IP_REASS_TYPE_FULL=0,
	 IP_REASS_TYPE_SHALLOW_VIRTUAL=1,
}
impl Default for IpReassType {
	fn default() -> Self { IpReassType::IP_REASS_TYPE_FULL }
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum IpFlowHashConfig {
	 IP_API_FLOW_HASH_SRC_IP=1,
	 IP_API_FLOW_HASH_DST_IP=2,
	 IP_API_FLOW_HASH_SRC_PORT=4,
	 IP_API_FLOW_HASH_DST_PORT=8,
	 IP_API_FLOW_HASH_PROTO=16,
	 IP_API_FLOW_HASH_REVERSE=32,
	 IP_API_FLOW_HASH_SYMETRIC=64,
	 IP_API_FLOW_HASH_FLOW_LABEL=128,
}
impl Default for IpFlowHashConfig {
	fn default() -> Self { IpFlowHashConfig::IP_API_FLOW_HASH_SRC_IP }
}
impl AsEnumFlag for IpFlowHashConfig {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 1 => IpFlowHashConfig::IP_API_FLOW_HASH_SRC_IP,
			 2 => IpFlowHashConfig::IP_API_FLOW_HASH_DST_IP,
			 4 => IpFlowHashConfig::IP_API_FLOW_HASH_SRC_PORT,
			 8 => IpFlowHashConfig::IP_API_FLOW_HASH_DST_PORT,
			 16 => IpFlowHashConfig::IP_API_FLOW_HASH_PROTO,
			 32 => IpFlowHashConfig::IP_API_FLOW_HASH_REVERSE,
			 64 => IpFlowHashConfig::IP_API_FLOW_HASH_SYMETRIC,
			 128 => IpFlowHashConfig::IP_API_FLOW_HASH_FLOW_LABEL,
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 32 as u32
	}
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum IpFlowHashConfigV2 {
	 IP_API_V2_FLOW_HASH_SRC_IP=1,
	 IP_API_V2_FLOW_HASH_DST_IP=2,
	 IP_API_V2_FLOW_HASH_SRC_PORT=4,
	 IP_API_V2_FLOW_HASH_DST_PORT=8,
	 IP_API_V2_FLOW_HASH_PROTO=16,
	 IP_API_V2_FLOW_HASH_REVERSE=32,
	 IP_API_V2_FLOW_HASH_SYMETRIC=64,
	 IP_API_V2_FLOW_HASH_FLOW_LABEL=128,
	 IP_API_V2_FLOW_HASH_GTPV1_TEID=256,
}
impl Default for IpFlowHashConfigV2 {
	fn default() -> Self { IpFlowHashConfigV2::IP_API_V2_FLOW_HASH_SRC_IP }
}
impl AsEnumFlag for IpFlowHashConfigV2 {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 1 => IpFlowHashConfigV2::IP_API_V2_FLOW_HASH_SRC_IP,
			 2 => IpFlowHashConfigV2::IP_API_V2_FLOW_HASH_DST_IP,
			 4 => IpFlowHashConfigV2::IP_API_V2_FLOW_HASH_SRC_PORT,
			 8 => IpFlowHashConfigV2::IP_API_V2_FLOW_HASH_DST_PORT,
			 16 => IpFlowHashConfigV2::IP_API_V2_FLOW_HASH_PROTO,
			 32 => IpFlowHashConfigV2::IP_API_V2_FLOW_HASH_REVERSE,
			 64 => IpFlowHashConfigV2::IP_API_V2_FLOW_HASH_SYMETRIC,
			 128 => IpFlowHashConfigV2::IP_API_V2_FLOW_HASH_FLOW_LABEL,
			 256 => IpFlowHashConfigV2::IP_API_V2_FLOW_HASH_GTPV1_TEID,
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 32 as u32
	}
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_table_add_del_0ffdaec0)]
pub struct IpTableAddDel {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub table: IpTable,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_table_add_del_reply_e8d4e804)]
pub struct IpTableAddDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_table_add_del_v2_14e5081f)]
pub struct IpTableAddDelV2 {
	pub client_index: u32,
	pub context: u32,
	pub table: IpTable,
	pub create_mfib: bool,
	pub is_add: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_table_add_del_v2_reply_e8d4e804)]
pub struct IpTableAddDelV2Reply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_table_allocate_b9d2e09e)]
pub struct IpTableAllocate {
	pub client_index: u32,
	pub context: u32,
	pub table: IpTable,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_table_allocate_reply_1728303a)]
pub struct IpTableAllocateReply {
	pub context: u32,
	pub retval: i32,
	pub table: IpTable,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_table_dump_51077d14)]
pub struct IpTableDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_table_replace_begin_b9d2e09e)]
pub struct IpTableReplaceBegin {
	pub client_index: u32,
	pub context: u32,
	pub table: IpTable,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_table_replace_begin_reply_e8d4e804)]
pub struct IpTableReplaceBeginReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_table_replace_end_b9d2e09e)]
pub struct IpTableReplaceEnd {
	pub client_index: u32,
	pub context: u32,
	pub table: IpTable,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_table_replace_end_reply_e8d4e804)]
pub struct IpTableReplaceEndReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_table_flush_b9d2e09e)]
pub struct IpTableFlush {
	pub client_index: u32,
	pub context: u32,
	pub table: IpTable,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_table_flush_reply_e8d4e804)]
pub struct IpTableFlushReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_table_details_c79fca0f)]
pub struct IpTableDetails {
	pub context: u32,
	pub table: IpTable,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_route_add_del_b8ecfe0d)]
pub struct IpRouteAddDel {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub is_multipath: bool,
	pub route: IpRoute,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_route_add_del_v2_521ef330)]
pub struct IpRouteAddDelV2 {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub is_multipath: bool,
	pub route: IpRouteV2,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_route_add_del_reply_1992deab)]
pub struct IpRouteAddDelReply {
	pub context: u32,
	pub retval: i32,
	pub stats_index: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_route_add_del_v2_reply_1992deab)]
pub struct IpRouteAddDelV2Reply {
	pub context: u32,
	pub retval: i32,
	pub stats_index: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_route_dump_b9d2e09e)]
pub struct IpRouteDump {
	pub client_index: u32,
	pub context: u32,
	pub table: IpTable,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_route_v2_dump_d16f72e6)]
pub struct IpRouteV2Dump {
	pub client_index: u32,
	pub context: u32,
	pub src: u8,
	pub table: IpTable,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_route_details_bda8f315)]
pub struct IpRouteDetails {
	pub context: u32,
	pub route: IpRoute,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_route_v2_details_b09aa6c0)]
pub struct IpRouteV2Details {
	pub context: u32,
	pub route: IpRouteV2,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_route_lookup_710d6471)]
pub struct IpRouteLookup {
	pub client_index: u32,
	pub context: u32,
	pub table_id: u32,
	pub exact: u8,
	pub prefix: Prefix,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_route_lookup_v2_710d6471)]
pub struct IpRouteLookupV2 {
	pub client_index: u32,
	pub context: u32,
	pub table_id: u32,
	pub exact: u8,
	pub prefix: Prefix,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_route_lookup_reply_5d8febcb)]
pub struct IpRouteLookupReply {
	pub context: u32,
	pub retval: i32,
	pub route: IpRoute,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_route_lookup_v2_reply_84cc9e03)]
pub struct IpRouteLookupV2Reply {
	pub context: u32,
	pub retval: i32,
	pub route: IpRouteV2,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(set_ip_flow_hash_084ee09e)]
pub struct SetIpFlowHash {
	pub client_index: u32,
	pub context: u32,
	pub vrf_id: u32,
	pub is_ipv6: bool,
	pub src: bool,
	pub dst: bool,
	pub sport: bool,
	pub dport: bool,
	pub proto: bool,
	pub reverse: bool,
	pub symmetric: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(set_ip_flow_hash_reply_e8d4e804)]
pub struct SetIpFlowHashReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(set_ip_flow_hash_v2_6d132100)]
pub struct SetIpFlowHashV2 {
	pub client_index: u32,
	pub context: u32,
	pub table_id: u32,
	pub af: AddressFamily,
	pub flow_hash_config: IpFlowHashConfig,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(set_ip_flow_hash_v2_reply_e8d4e804)]
pub struct SetIpFlowHashV2Reply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(set_ip_flow_hash_v3_b7876e07)]
pub struct SetIpFlowHashV3 {
	pub client_index: u32,
	pub context: u32,
	pub table_id: u32,
	pub af: AddressFamily,
	pub flow_hash_config: IpFlowHashConfigV2,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(set_ip_flow_hash_v3_reply_e8d4e804)]
pub struct SetIpFlowHashV3Reply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(set_ip_flow_hash_router_id_03e4f48e)]
pub struct SetIpFlowHashRouterId {
	pub client_index: u32,
	pub context: u32,
	pub router_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(set_ip_flow_hash_router_id_reply_e8d4e804)]
pub struct SetIpFlowHashRouterIdReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_ip6_enable_disable_ae6cfcfb)]
pub struct SwInterfaceIp6EnableDisable {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub enable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_ip6_enable_disable_reply_e8d4e804)]
pub struct SwInterfaceIp6EnableDisableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_ip4_enable_disable_ae6cfcfb)]
pub struct SwInterfaceIp4EnableDisable {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub enable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_ip4_enable_disable_reply_e8d4e804)]
pub struct SwInterfaceIp4EnableDisableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_mtable_dump_51077d14)]
pub struct IpMtableDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_mtable_details_b9d2e09e)]
pub struct IpMtableDetails {
	pub client_index: u32,
	pub context: u32,
	pub table: IpTable,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_mroute_add_del_0dd7e790)]
pub struct IpMrouteAddDel {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub is_multipath: bool,
	pub route: IpMroute,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_mroute_add_del_reply_1992deab)]
pub struct IpMrouteAddDelReply {
	pub context: u32,
	pub retval: i32,
	pub stats_index: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_mroute_dump_b9d2e09e)]
pub struct IpMrouteDump {
	pub client_index: u32,
	pub context: u32,
	pub table: IpTable,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_mroute_details_c5cb23fc)]
pub struct IpMrouteDetails {
	pub context: u32,
	pub route: IpMroute,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_address_details_ee29b797)]
pub struct IpAddressDetails {
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub prefix: AddressWithPrefix,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_address_dump_2d033de4)]
pub struct IpAddressDump {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub is_ipv6: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_unnumbered_details_cc59bd42)]
pub struct IpUnnumberedDetails {
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub ip_sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_unnumbered_dump_f9e6675e)]
pub struct IpUnnumberedDump {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_details_eb152d07)]
pub struct IpDetails {
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub is_ipv6: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_dump_98d231ca)]
pub struct IpDump {
	pub client_index: u32,
	pub context: u32,
	pub is_ipv6: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(mfib_signal_dump_51077d14)]
pub struct MfibSignalDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(mfib_signal_details_6f4a4cfb)]
pub struct MfibSignalDetails {
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub table_id: u32,
	pub prefix: Mprefix,
	pub ip_packet_len: u16,
	pub ip_packet_data: FixedSizeArray<u8, typenum::U256>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_punt_police_db867cea)]
pub struct IpPuntPolice {
	pub client_index: u32,
	pub context: u32,
	pub policer_index: u32,
	pub is_add: bool,
	pub is_ip6: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_punt_police_reply_e8d4e804)]
pub struct IpPuntPoliceReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_punt_redirect_6580f635)]
pub struct IpPuntRedirect {
	pub client_index: u32,
	pub context: u32,
	pub punt: PuntRedirect,
	pub is_add: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_punt_redirect_reply_e8d4e804)]
pub struct IpPuntRedirectReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_punt_redirect_dump_2d033de4)]
pub struct IpPuntRedirectDump {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub is_ipv6: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_punt_redirect_details_2cef63e7)]
pub struct IpPuntRedirectDetails {
	pub context: u32,
	pub punt: PuntRedirect,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(add_del_ip_punt_redirect_v2_9e804227)]
pub struct AddDelIpPuntRedirectV2 {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub punt: PuntRedirectV2,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(add_del_ip_punt_redirect_v2_reply_e8d4e804)]
pub struct AddDelIpPuntRedirectV2Reply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_punt_redirect_v2_dump_d817a484)]
pub struct IpPuntRedirectV2Dump {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub af: AddressFamily,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_punt_redirect_v2_details_7ba42e1d)]
pub struct IpPuntRedirectV2Details {
	pub context: u32,
	pub punt: PuntRedirectV2,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_container_proxy_add_del_7df1dff1)]
pub struct IpContainerProxyAddDel {
	pub client_index: u32,
	pub context: u32,
	pub pfx: Prefix,
	pub sw_if_index: InterfaceIndex,
	pub is_add: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_container_proxy_add_del_reply_e8d4e804)]
pub struct IpContainerProxyAddDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_container_proxy_dump_51077d14)]
pub struct IpContainerProxyDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_container_proxy_details_a8085523)]
pub struct IpContainerProxyDetails {
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub prefix: Prefix,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_source_and_port_range_check_add_del_92a067e3)]
pub struct IpSourceAndPortRangeCheckAddDel {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub prefix: Prefix,
	pub number_of_ranges: u8,
	pub low_ports: FixedSizeArray<u16, typenum::U32>,
	pub high_ports: FixedSizeArray<u16, typenum::U32>,
	pub vrf_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_source_and_port_range_check_add_del_reply_e8d4e804)]
pub struct IpSourceAndPortRangeCheckAddDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_source_and_port_range_check_interface_add_del_e1ba8987)]
pub struct IpSourceAndPortRangeCheckInterfaceAddDel {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub sw_if_index: InterfaceIndex,
	pub tcp_in_vrf_id: u32,
	pub tcp_out_vrf_id: u32,
	pub udp_in_vrf_id: u32,
	pub udp_out_vrf_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_source_and_port_range_check_interface_add_del_reply_e8d4e804)]
pub struct IpSourceAndPortRangeCheckInterfaceAddDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_ip6_set_link_local_address_1c10f15f)]
pub struct SwInterfaceIp6SetLinkLocalAddress {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub ip: Ip6Address,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_ip6_set_link_local_address_reply_e8d4e804)]
pub struct SwInterfaceIp6SetLinkLocalAddressReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_ip6_get_link_local_address_f9e6675e)]
pub struct SwInterfaceIp6GetLinkLocalAddress {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(sw_interface_ip6_get_link_local_address_reply_d16b7130)]
pub struct SwInterfaceIp6GetLinkLocalAddressReply {
	pub context: u32,
	pub retval: i32,
	pub ip: Ip6Address,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ioam_enable_51ccd868)]
pub struct IoamEnable {
	pub client_index: u32,
	pub context: u32,
	pub id: u16,
	pub seqno: bool,
	pub analyse: bool,
	pub pot_enable: bool,
	pub trace_enable: bool,
	pub node_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ioam_enable_reply_e8d4e804)]
pub struct IoamEnableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ioam_disable_6b16a45e)]
pub struct IoamDisable {
	pub client_index: u32,
	pub context: u32,
	pub id: u16,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ioam_disable_reply_e8d4e804)]
pub struct IoamDisableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_reassembly_set_16467d25)]
pub struct IpReassemblySet {
	pub client_index: u32,
	pub context: u32,
	pub timeout_ms: u32,
	pub max_reassemblies: u32,
	pub max_reassembly_length: u32,
	pub expire_walk_interval_ms: u32,
	pub is_ip6: bool,
	pub typ: IpReassType,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_reassembly_set_reply_e8d4e804)]
pub struct IpReassemblySetReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_reassembly_get_ea13ff63)]
pub struct IpReassemblyGet {
	pub client_index: u32,
	pub context: u32,
	pub is_ip6: bool,
	pub typ: IpReassType,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_reassembly_get_reply_d5eb8d34)]
pub struct IpReassemblyGetReply {
	pub context: u32,
	pub retval: i32,
	pub timeout_ms: u32,
	pub max_reassemblies: u32,
	pub max_reassembly_length: u32,
	pub expire_walk_interval_ms: u32,
	pub is_ip6: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_reassembly_enable_disable_eb77968d)]
pub struct IpReassemblyEnableDisable {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub enable_ip4: bool,
	pub enable_ip6: bool,
	pub typ: IpReassType,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_reassembly_enable_disable_reply_e8d4e804)]
pub struct IpReassemblyEnableDisableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_local_reass_enable_disable_34e2ccc4)]
pub struct IpLocalReassEnableDisable {
	pub client_index: u32,
	pub context: u32,
	pub enable_ip4: bool,
	pub enable_ip6: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_local_reass_enable_disable_reply_e8d4e804)]
pub struct IpLocalReassEnableDisableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_local_reass_get_51077d14)]
pub struct IpLocalReassGet {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_local_reass_get_reply_3e93a702)]
pub struct IpLocalReassGetReply {
	pub context: u32,
	pub retval: i32,
	pub ip4_is_enabled: bool,
	pub ip6_is_enabled: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_path_mtu_update_10bbe5cb)]
pub struct IpPathMtuUpdate {
	pub client_index: u32,
	pub context: u32,
	pub pmtu: IpPathMtu,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_path_mtu_update_reply_e8d4e804)]
pub struct IpPathMtuUpdateReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_path_mtu_get_f75ba505)]
pub struct IpPathMtuGet {
	pub client_index: u32,
	pub context: u32,
	pub cursor: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_path_mtu_get_reply_53b48f5d)]
pub struct IpPathMtuGetReply {
	pub context: u32,
	pub retval: i32,
	pub cursor: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_path_mtu_details_ac9539a7)]
pub struct IpPathMtuDetails {
	pub context: u32,
	pub pmtu: IpPathMtu,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_path_mtu_replace_begin_51077d14)]
pub struct IpPathMtuReplaceBegin {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_path_mtu_replace_begin_reply_e8d4e804)]
pub struct IpPathMtuReplaceBeginReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_path_mtu_replace_end_51077d14)]
pub struct IpPathMtuReplaceEnd {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ip_path_mtu_replace_end_reply_e8d4e804)]
pub struct IpPathMtuReplaceEndReply {
	pub context: u32,
	pub retval: i32,
}
