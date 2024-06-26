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
use crate::nat_types::*;
use crate::interface_types::*;
// Implementation for nat44_lb_addr_port
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Nat44LbAddrPort {
	pub addr: Ip4Address,
	pub port: u16,
	pub probability: u8,
	pub vrf_id: u32,
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Nat44ConfigFlags {
	 NAT44_IS_ENDPOINT_INDEPENDENT=0,
	 NAT44_IS_ENDPOINT_DEPENDENT=1,
	 NAT44_IS_STATIC_MAPPING_ONLY=2,
	 NAT44_IS_CONNECTION_TRACKING=4,
	 NAT44_IS_OUT2IN_DPO=8,
}
impl Default for Nat44ConfigFlags {
	fn default() -> Self { Nat44ConfigFlags::NAT44_IS_ENDPOINT_INDEPENDENT }
}
impl AsEnumFlag for Nat44ConfigFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 0 => Nat44ConfigFlags::NAT44_IS_ENDPOINT_INDEPENDENT,
			 1 => Nat44ConfigFlags::NAT44_IS_ENDPOINT_DEPENDENT,
			 2 => Nat44ConfigFlags::NAT44_IS_STATIC_MAPPING_ONLY,
			 4 => Nat44ConfigFlags::NAT44_IS_CONNECTION_TRACKING,
			 8 => Nat44ConfigFlags::NAT44_IS_OUT2IN_DPO,
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 8 as u32
	}
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_plugin_enable_disable_be17f8dd)]
pub struct Nat44EdPluginEnableDisable {
	pub client_index: u32,
	pub context: u32,
	pub inside_vrf: u32,
	pub outside_vrf: u32,
	pub sessions: u32,
	pub session_memory: u32,
	pub enable: bool,
	 pub flags: EnumFlag<Nat44ConfigFlags>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_plugin_enable_disable_reply_e8d4e804)]
pub struct Nat44EdPluginEnableDisableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_forwarding_enable_disable_b3e225d2)]
pub struct Nat44ForwardingEnableDisable {
	pub client_index: u32,
	pub context: u32,
	pub enable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_forwarding_enable_disable_reply_e8d4e804)]
pub struct Nat44ForwardingEnableDisableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat_ipfix_enable_disable_9af4a2d2)]
pub struct NatIpfixEnableDisable {
	pub client_index: u32,
	pub context: u32,
	pub domain_id: u32,
	pub src_port: u16,
	pub enable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat_ipfix_enable_disable_reply_e8d4e804)]
pub struct NatIpfixEnableDisableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat_set_timeouts_d4746b16)]
pub struct NatSetTimeouts {
	pub client_index: u32,
	pub context: u32,
	pub udp: u32,
	pub tcp_established: u32,
	pub tcp_transitory: u32,
	pub icmp: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat_set_timeouts_reply_e8d4e804)]
pub struct NatSetTimeoutsReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_set_session_limit_8899bbb1)]
pub struct Nat44SetSessionLimit {
	pub client_index: u32,
	pub context: u32,
	pub session_limit: u32,
	pub vrf_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_set_session_limit_reply_e8d4e804)]
pub struct Nat44SetSessionLimitReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_show_running_config_51077d14)]
pub struct Nat44ShowRunningConfig {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_show_running_config_reply_93d8e267)]
pub struct Nat44ShowRunningConfigReply {
	pub context: u32,
	pub retval: i32,
	pub inside_vrf: u32,
	pub outside_vrf: u32,
	pub users: u32,
	pub sessions: u32,
	pub user_sessions: u32,
	pub user_buckets: u32,
	pub translation_buckets: u32,
	pub forwarding_enabled: bool,
	pub ipfix_logging_enabled: bool,
	pub timeouts: NatTimeouts,
	pub log_level: NatLogLevel,
	 pub flags: EnumFlag<Nat44ConfigFlags>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat_set_workers_da926638)]
pub struct NatSetWorkers {
	pub client_index: u32,
	pub context: u32,
	pub worker_mask: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat_set_workers_reply_e8d4e804)]
pub struct NatSetWorkersReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat_worker_dump_51077d14)]
pub struct NatWorkerDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat_worker_details_84bf06fc)]
pub struct NatWorkerDetails {
	pub context: u32,
	pub worker_index: u32,
	pub lcore_id: u32,
	pub name: FixedSizeString<typenum::U64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_add_del_vrf_table_08330904)]
pub struct Nat44EdAddDelVrfTable {
	pub client_index: u32,
	pub context: u32,
	pub table_vrf_id: u32,
	pub is_add: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_add_del_vrf_table_reply_e8d4e804)]
pub struct Nat44EdAddDelVrfTableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_add_del_vrf_route_59187407)]
pub struct Nat44EdAddDelVrfRoute {
	pub client_index: u32,
	pub context: u32,
	pub table_vrf_id: u32,
	pub vrf_id: u32,
	pub is_add: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_add_del_vrf_route_reply_e8d4e804)]
pub struct Nat44EdAddDelVrfRouteReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_vrf_tables_dump_51077d14)]
pub struct Nat44EdVrfTablesDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_vrf_tables_details_7b264e4f)]
pub struct Nat44EdVrfTablesDetails {
	pub context: u32,
	pub table_vrf_id: u32,
	pub n_vrf_ids: u32,
	pub vrf_ids: VariableSizeArray<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_vrf_tables_v2_dump_51077d14)]
pub struct Nat44EdVrfTablesV2Dump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_vrf_tables_v2_details_7b264e4f)]
pub struct Nat44EdVrfTablesV2Details {
	pub context: u32,
	pub table_vrf_id: u32,
	pub n_vrf_ids: u32,
	pub vrf_ids: VariableSizeArray<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat_set_mss_clamping_25e90abb)]
pub struct NatSetMssClamping {
	pub client_index: u32,
	pub context: u32,
	pub mss_value: u16,
	pub enable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat_set_mss_clamping_reply_e8d4e804)]
pub struct NatSetMssClampingReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat_get_mss_clamping_51077d14)]
pub struct NatGetMssClamping {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat_get_mss_clamping_reply_1c0b2a78)]
pub struct NatGetMssClampingReply {
	pub context: u32,
	pub retval: i32,
	pub mss_value: u16,
	pub enable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_set_fq_options_2399bd71)]
pub struct Nat44EdSetFqOptions {
	pub client_index: u32,
	pub context: u32,
	pub frame_queue_nelts: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_set_fq_options_reply_e8d4e804)]
pub struct Nat44EdSetFqOptionsReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_show_fq_options_51077d14)]
pub struct Nat44EdShowFqOptions {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_show_fq_options_reply_7213b545)]
pub struct Nat44EdShowFqOptionsReply {
	pub context: u32,
	pub retval: i32,
	pub frame_queue_nelts: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_add_del_interface_addr_4aed50c0)]
pub struct Nat44AddDelInterfaceAddr {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub sw_if_index: InterfaceIndex,
	 pub flags: EnumFlag<NatConfigFlags>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_add_del_interface_addr_reply_e8d4e804)]
pub struct Nat44AddDelInterfaceAddrReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_interface_addr_dump_51077d14)]
pub struct Nat44InterfaceAddrDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_interface_addr_details_e4aca9ca)]
pub struct Nat44InterfaceAddrDetails {
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	 pub flags: EnumFlag<NatConfigFlags>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_add_del_address_range_6f2b8055)]
pub struct Nat44AddDelAddressRange {
	pub client_index: u32,
	pub context: u32,
	pub first_ip_address: Ip4Address,
	pub last_ip_address: Ip4Address,
	pub vrf_id: u32,
	pub is_add: bool,
	 pub flags: EnumFlag<NatConfigFlags>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_add_del_address_range_reply_e8d4e804)]
pub struct Nat44AddDelAddressRangeReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_address_dump_51077d14)]
pub struct Nat44AddressDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_address_details_0d1beac1)]
pub struct Nat44AddressDetails {
	pub context: u32,
	pub ip_address: Ip4Address,
	 pub flags: EnumFlag<NatConfigFlags>,
	pub vrf_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_interface_add_del_feature_f3699b83)]
pub struct Nat44InterfaceAddDelFeature {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	 pub flags: EnumFlag<NatConfigFlags>,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_interface_add_del_feature_reply_e8d4e804)]
pub struct Nat44InterfaceAddDelFeatureReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_interface_dump_51077d14)]
pub struct Nat44InterfaceDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_interface_details_5d286289)]
pub struct Nat44InterfaceDetails {
	pub context: u32,
	 pub flags: EnumFlag<NatConfigFlags>,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_add_del_output_interface_47d6e753)]
pub struct Nat44EdAddDelOutputInterface {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_add_del_output_interface_reply_e8d4e804)]
pub struct Nat44EdAddDelOutputInterfaceReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_output_interface_get_f75ba505)]
pub struct Nat44EdOutputInterfaceGet {
	pub client_index: u32,
	pub context: u32,
	pub cursor: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_output_interface_get_reply_53b48f5d)]
pub struct Nat44EdOutputInterfaceGetReply {
	pub context: u32,
	pub retval: i32,
	pub cursor: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_ed_output_interface_details_0b45011c)]
pub struct Nat44EdOutputInterfaceDetails {
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_add_del_static_mapping_5ae5f03e)]
pub struct Nat44AddDelStaticMapping {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	 pub flags: EnumFlag<NatConfigFlags>,
	pub local_ip_address: Ip4Address,
	pub external_ip_address: Ip4Address,
	pub protocol: u8,
	pub local_port: u16,
	pub external_port: u16,
	pub external_sw_if_index: InterfaceIndex,
	pub vrf_id: u32,
	pub tag: FixedSizeString<typenum::U64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_add_del_static_mapping_reply_e8d4e804)]
pub struct Nat44AddDelStaticMappingReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_add_del_static_mapping_v2_5e205f1a)]
pub struct Nat44AddDelStaticMappingV2 {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub match_pool: bool,
	 pub flags: EnumFlag<NatConfigFlags>,
	pub pool_ip_address: Ip4Address,
	pub local_ip_address: Ip4Address,
	pub external_ip_address: Ip4Address,
	pub protocol: u8,
	pub local_port: u16,
	pub external_port: u16,
	pub external_sw_if_index: InterfaceIndex,
	pub vrf_id: u32,
	pub tag: FixedSizeString<typenum::U64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_add_del_static_mapping_v2_reply_e8d4e804)]
pub struct Nat44AddDelStaticMappingV2Reply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_static_mapping_dump_51077d14)]
pub struct Nat44StaticMappingDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_static_mapping_details_06cb40b2)]
pub struct Nat44StaticMappingDetails {
	pub context: u32,
	 pub flags: EnumFlag<NatConfigFlags>,
	pub local_ip_address: Ip4Address,
	pub external_ip_address: Ip4Address,
	pub protocol: u8,
	pub local_port: u16,
	pub external_port: u16,
	pub external_sw_if_index: InterfaceIndex,
	pub vrf_id: u32,
	pub tag: FixedSizeString<typenum::U64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_add_del_identity_mapping_02faaa22)]
pub struct Nat44AddDelIdentityMapping {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	 pub flags: EnumFlag<NatConfigFlags>,
	pub ip_address: Ip4Address,
	pub protocol: u8,
	pub port: u16,
	pub sw_if_index: InterfaceIndex,
	pub vrf_id: u32,
	pub tag: FixedSizeString<typenum::U64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_add_del_identity_mapping_reply_e8d4e804)]
pub struct Nat44AddDelIdentityMappingReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_identity_mapping_dump_51077d14)]
pub struct Nat44IdentityMappingDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_identity_mapping_details_2a52a030)]
pub struct Nat44IdentityMappingDetails {
	pub context: u32,
	 pub flags: EnumFlag<NatConfigFlags>,
	pub ip_address: Ip4Address,
	pub protocol: u8,
	pub port: u16,
	pub sw_if_index: InterfaceIndex,
	pub vrf_id: u32,
	pub tag: FixedSizeString<typenum::U64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_add_del_lb_static_mapping_4f68ee9d)]
pub struct Nat44AddDelLbStaticMapping {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	 pub flags: EnumFlag<NatConfigFlags>,
	pub external_addr: Ip4Address,
	pub external_port: u16,
	pub protocol: u8,
	pub affinity: u32,
	pub tag: FixedSizeString<typenum::U64>,
	pub local_num: u32,
	pub locals: VariableSizeArray<Nat44LbAddrPort>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_add_del_lb_static_mapping_reply_e8d4e804)]
pub struct Nat44AddDelLbStaticMappingReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_lb_static_mapping_add_del_local_7ca47547)]
pub struct Nat44LbStaticMappingAddDelLocal {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub external_addr: Ip4Address,
	pub external_port: u16,
	pub protocol: u8,
	pub local: Nat44LbAddrPort,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_lb_static_mapping_add_del_local_reply_e8d4e804)]
pub struct Nat44LbStaticMappingAddDelLocalReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_lb_static_mapping_dump_51077d14)]
pub struct Nat44LbStaticMappingDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_lb_static_mapping_details_ed5ce876)]
pub struct Nat44LbStaticMappingDetails {
	pub context: u32,
	pub external_addr: Ip4Address,
	pub external_port: u16,
	pub protocol: u8,
	 pub flags: EnumFlag<NatConfigFlags>,
	pub affinity: u32,
	pub tag: FixedSizeString<typenum::U64>,
	pub local_num: u32,
	pub locals: VariableSizeArray<Nat44LbAddrPort>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_del_session_15a5bf8c)]
pub struct Nat44DelSession {
	pub client_index: u32,
	pub context: u32,
	pub address: Ip4Address,
	pub protocol: u8,
	pub port: u16,
	pub vrf_id: u32,
	 pub flags: EnumFlag<NatConfigFlags>,
	pub ext_host_address: Ip4Address,
	pub ext_host_port: u16,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_del_session_reply_e8d4e804)]
pub struct Nat44DelSessionReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_user_dump_51077d14)]
pub struct Nat44UserDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_user_details_355896c2)]
pub struct Nat44UserDetails {
	pub context: u32,
	pub vrf_id: u32,
	pub ip_address: Ip4Address,
	pub nsessions: u32,
	pub nstaticsessions: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_user_session_dump_e1899c98)]
pub struct Nat44UserSessionDump {
	pub client_index: u32,
	pub context: u32,
	pub ip_address: Ip4Address,
	pub vrf_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_user_session_details_2cf6e16d)]
pub struct Nat44UserSessionDetails {
	pub context: u32,
	pub outside_ip_address: Ip4Address,
	pub outside_port: u16,
	pub inside_ip_address: Ip4Address,
	pub inside_port: u16,
	pub protocol: u16,
	 pub flags: EnumFlag<NatConfigFlags>,
	pub last_heard: u64,
	pub total_bytes: u64,
	pub total_pkts: u32,
	pub ext_host_address: Ip4Address,
	pub ext_host_port: u16,
	pub ext_host_nat_address: Ip4Address,
	pub ext_host_nat_port: u16,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_user_session_v2_dump_e1899c98)]
pub struct Nat44UserSessionV2Dump {
	pub client_index: u32,
	pub context: u32,
	pub ip_address: Ip4Address,
	pub vrf_id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_user_session_v2_details_fd42b729)]
pub struct Nat44UserSessionV2Details {
	pub context: u32,
	pub outside_ip_address: Ip4Address,
	pub outside_port: u16,
	pub inside_ip_address: Ip4Address,
	pub inside_port: u16,
	pub protocol: u16,
	 pub flags: EnumFlag<NatConfigFlags>,
	pub last_heard: u64,
	pub total_bytes: u64,
	pub total_pkts: u32,
	pub ext_host_address: Ip4Address,
	pub ext_host_port: u16,
	pub ext_host_nat_address: Ip4Address,
	pub ext_host_nat_port: u16,
	pub is_timed_out: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_user_session_v3_details_edae926e)]
pub struct Nat44UserSessionV3Details {
	pub context: u32,
	pub outside_ip_address: Ip4Address,
	pub outside_port: u16,
	pub inside_ip_address: Ip4Address,
	pub inside_port: u16,
	pub protocol: u16,
	 pub flags: EnumFlag<NatConfigFlags>,
	pub last_heard: u64,
	pub time_since_last_heard: u64,
	pub total_bytes: u64,
	pub total_pkts: u32,
	pub ext_host_address: Ip4Address,
	pub ext_host_port: u16,
	pub ext_host_nat_address: Ip4Address,
	pub ext_host_nat_port: u16,
	pub is_timed_out: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(nat44_user_session_v3_dump_e1899c98)]
pub struct Nat44UserSessionV3Dump {
	pub client_index: u32,
	pub context: u32,
	pub ip_address: Ip4Address,
	pub vrf_id: u32,
}
