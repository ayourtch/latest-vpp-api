/*
   Autogenerated Data, Do not Edit! 
*/
#![allow(dead_code,unused_mut,unused_variables,unused_must_use,non_camel_case_types,unused_imports,non_snake_case)]
use vpp_api_macros::{VppMessage,VppUnionIdent}; 
use std::convert::TryInto; 
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_message::VppApiMessage;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum;
use crate::ip_types::*; 
use crate::nat_types::*; 
use crate::interface_types::*; 
#[derive(Debug,Serialize, Deserialize, Clone, Copy)] 
pub enum Nat44EiConfigFlags { 
	 NAT44_EI_NONE=0, 
	 NAT44_EI_STATIC_MAPPING_ONLY=1, 
	 NAT44_EI_CONNECTION_TRACKING=2, 
	 NAT44_EI_OUT2IN_DPO=4, 
	 NAT44_EI_ADDR_ONLY_MAPPING=8, 
	 NAT44_EI_IF_INSIDE=16, 
	 NAT44_EI_IF_OUTSIDE=32, 
	 NAT44_EI_STATIC_MAPPING=64, 
} 
impl Default for Nat44EiConfigFlags { 
	fn default() -> Self { Nat44EiConfigFlags::NAT44_EI_NONE }
}
impl AsEnumFlag for Nat44EiConfigFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 0 => Nat44EiConfigFlags::NAT44_EI_NONE, 
			 1 => Nat44EiConfigFlags::NAT44_EI_STATIC_MAPPING_ONLY, 
			 2 => Nat44EiConfigFlags::NAT44_EI_CONNECTION_TRACKING, 
			 4 => Nat44EiConfigFlags::NAT44_EI_OUT2IN_DPO, 
			 8 => Nat44EiConfigFlags::NAT44_EI_ADDR_ONLY_MAPPING, 
			 16 => Nat44EiConfigFlags::NAT44_EI_IF_INSIDE, 
			 32 => Nat44EiConfigFlags::NAT44_EI_IF_OUTSIDE, 
			 64 => Nat44EiConfigFlags::NAT44_EI_STATIC_MAPPING, 
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 8 as u32
	}
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_plugin_enable_disable_bf692144)] 
pub struct Nat44EiPluginEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub inside_vrf : u32, 
	pub outside_vrf : u32, 
	pub users : u32, 
	pub user_memory : u32, 
	pub sessions : u32, 
	pub session_memory : u32, 
	pub user_sessions : u32, 
	pub enable : bool, 
	 pub flags : EnumFlag<Nat44EiConfigFlags>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_plugin_enable_disable_reply_e8d4e804)] 
pub struct Nat44EiPluginEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_show_running_config_51077d14)] 
pub struct Nat44EiShowRunningConfig { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_show_running_config_reply_41b66a81)] 
pub struct Nat44EiShowRunningConfigReply { 
	pub context : u32, 
	pub retval : i32, 
	pub inside_vrf : u32, 
	pub outside_vrf : u32, 
	pub users : u32, 
	pub sessions : u32, 
	pub user_sessions : u32, 
	pub user_buckets : u32, 
	pub translation_buckets : u32, 
	pub forwarding_enabled : bool, 
	pub ipfix_logging_enabled : bool, 
	pub timeouts : NatTimeouts, 
	pub log_level : NatLogLevel, 
	 pub flags : EnumFlag<Nat44EiConfigFlags>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_set_log_level_70076bfe)] 
pub struct Nat44EiSetLogLevel { 
	pub client_index : u32, 
	pub context : u32, 
	pub log_level : NatLogLevel, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_set_log_level_reply_e8d4e804)] 
pub struct Nat44EiSetLogLevelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_set_workers_da926638)] 
pub struct Nat44EiSetWorkers { 
	pub client_index : u32, 
	pub context : u32, 
	pub worker_mask : u64, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_set_workers_reply_e8d4e804)] 
pub struct Nat44EiSetWorkersReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_worker_dump_51077d14)] 
pub struct Nat44EiWorkerDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_worker_details_84bf06fc)] 
pub struct Nat44EiWorkerDetails { 
	pub context : u32, 
	pub worker_index : u32, 
	pub lcore_id : u32, 
	pub name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_ipfix_enable_disable_9af4a2d2)] 
pub struct Nat44EiIpfixEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub domain_id : u32, 
	pub src_port : u16, 
	pub enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_ipfix_enable_disable_reply_e8d4e804)] 
pub struct Nat44EiIpfixEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_set_timeouts_d4746b16)] 
pub struct Nat44EiSetTimeouts { 
	pub client_index : u32, 
	pub context : u32, 
	pub udp : u32, 
	pub tcp_established : u32, 
	pub tcp_transitory : u32, 
	pub icmp : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_set_timeouts_reply_e8d4e804)] 
pub struct Nat44EiSetTimeoutsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_set_addr_and_port_alloc_alg_deeb746f)] 
pub struct Nat44EiSetAddrAndPortAllocAlg { 
	pub client_index : u32, 
	pub context : u32, 
	pub alg : u8, 
	pub psid_offset : u8, 
	pub psid_length : u8, 
	pub psid : u16, 
	pub start_port : u16, 
	pub end_port : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_set_addr_and_port_alloc_alg_reply_e8d4e804)] 
pub struct Nat44EiSetAddrAndPortAllocAlgReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_get_addr_and_port_alloc_alg_51077d14)] 
pub struct Nat44EiGetAddrAndPortAllocAlg { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_get_addr_and_port_alloc_alg_reply_3607a7d0)] 
pub struct Nat44EiGetAddrAndPortAllocAlgReply { 
	pub context : u32, 
	pub retval : i32, 
	pub alg : u8, 
	pub psid_offset : u8, 
	pub psid_length : u8, 
	pub psid : u16, 
	pub start_port : u16, 
	pub end_port : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_set_mss_clamping_25e90abb)] 
pub struct Nat44EiSetMssClamping { 
	pub client_index : u32, 
	pub context : u32, 
	pub mss_value : u16, 
	pub enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_set_mss_clamping_reply_e8d4e804)] 
pub struct Nat44EiSetMssClampingReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_get_mss_clamping_51077d14)] 
pub struct Nat44EiGetMssClamping { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_get_mss_clamping_reply_1c0b2a78)] 
pub struct Nat44EiGetMssClampingReply { 
	pub context : u32, 
	pub retval : i32, 
	pub mss_value : u16, 
	pub enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_ha_set_listener_e4a8cb4e)] 
pub struct Nat44EiHaSetListener { 
	pub client_index : u32, 
	pub context : u32, 
	pub ip_address : Ip4Address, 
	pub port : u16, 
	pub path_mtu : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_ha_set_listener_reply_e8d4e804)] 
pub struct Nat44EiHaSetListenerReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_ha_set_failover_718246af)] 
pub struct Nat44EiHaSetFailover { 
	pub client_index : u32, 
	pub context : u32, 
	pub ip_address : Ip4Address, 
	pub port : u16, 
	pub session_refresh_interval : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_ha_set_failover_reply_e8d4e804)] 
pub struct Nat44EiHaSetFailoverReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_ha_get_listener_51077d14)] 
pub struct Nat44EiHaGetListener { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_ha_get_listener_reply_123ea41f)] 
pub struct Nat44EiHaGetListenerReply { 
	pub context : u32, 
	pub retval : i32, 
	pub ip_address : Ip4Address, 
	pub port : u16, 
	pub path_mtu : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_ha_get_failover_51077d14)] 
pub struct Nat44EiHaGetFailover { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_ha_get_failover_reply_a67d8752)] 
pub struct Nat44EiHaGetFailoverReply { 
	pub context : u32, 
	pub retval : i32, 
	pub ip_address : Ip4Address, 
	pub port : u16, 
	pub session_refresh_interval : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_ha_flush_51077d14)] 
pub struct Nat44EiHaFlush { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_ha_flush_reply_e8d4e804)] 
pub struct Nat44EiHaFlushReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_ha_resync_c8ab9e03)] 
pub struct Nat44EiHaResync { 
	pub client_index : u32, 
	pub context : u32, 
	pub want_resync_event : u8, 
	pub pid : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_ha_resync_reply_e8d4e804)] 
pub struct Nat44EiHaResyncReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_ha_resync_completed_event_fdc598fb)] 
pub struct Nat44EiHaResyncCompletedEvent { 
	pub client_index : u32, 
	pub pid : u32, 
	pub missed_count : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_del_user_99a9f998)] 
pub struct Nat44EiDelUser { 
	pub client_index : u32, 
	pub context : u32, 
	pub ip_address : Ip4Address, 
	pub fib_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_del_user_reply_e8d4e804)] 
pub struct Nat44EiDelUserReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_add_del_address_range_35f21abc)] 
pub struct Nat44EiAddDelAddressRange { 
	pub client_index : u32, 
	pub context : u32, 
	pub first_ip_address : Ip4Address, 
	pub last_ip_address : Ip4Address, 
	pub vrf_id : u32, 
	pub is_add : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_add_del_address_range_reply_e8d4e804)] 
pub struct Nat44EiAddDelAddressRangeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_address_dump_51077d14)] 
pub struct Nat44EiAddressDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_address_details_318f1202)] 
pub struct Nat44EiAddressDetails { 
	pub context : u32, 
	pub ip_address : Ip4Address, 
	pub vrf_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_interface_add_del_feature_63a2db8b)] 
pub struct Nat44EiInterfaceAddDelFeature { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	 pub flags : EnumFlag<Nat44EiConfigFlags>, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_interface_add_del_feature_reply_e8d4e804)] 
pub struct Nat44EiInterfaceAddDelFeatureReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_interface_dump_51077d14)] 
pub struct Nat44EiInterfaceDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_interface_details_f446e508)] 
pub struct Nat44EiInterfaceDetails { 
	pub context : u32, 
	 pub flags : EnumFlag<Nat44EiConfigFlags>, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_interface_add_del_output_feature_63a2db8b)] 
pub struct Nat44EiInterfaceAddDelOutputFeature { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	 pub flags : EnumFlag<Nat44EiConfigFlags>, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_interface_add_del_output_feature_reply_e8d4e804)] 
pub struct Nat44EiInterfaceAddDelOutputFeatureReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_interface_output_feature_dump_51077d14)] 
pub struct Nat44EiInterfaceOutputFeatureDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_interface_output_feature_details_f446e508)] 
pub struct Nat44EiInterfaceOutputFeatureDetails { 
	pub context : u32, 
	 pub flags : EnumFlag<Nat44EiConfigFlags>, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_add_del_output_interface_47d6e753)] 
pub struct Nat44EiAddDelOutputInterface { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_add_del_output_interface_reply_e8d4e804)] 
pub struct Nat44EiAddDelOutputInterfaceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_output_interface_get_f75ba505)] 
pub struct Nat44EiOutputInterfaceGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub cursor : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_output_interface_get_reply_53b48f5d)] 
pub struct Nat44EiOutputInterfaceGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub cursor : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_output_interface_details_0b45011c)] 
pub struct Nat44EiOutputInterfaceDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_add_del_static_mapping_b404b7fe)] 
pub struct Nat44EiAddDelStaticMapping { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	 pub flags : EnumFlag<Nat44EiConfigFlags>, 
	pub local_ip_address : Ip4Address, 
	pub external_ip_address : Ip4Address, 
	pub protocol : u8, 
	pub local_port : u16, 
	pub external_port : u16, 
	pub external_sw_if_index : InterfaceIndex, 
	pub vrf_id : u32, 
	pub tag : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_add_del_static_mapping_reply_e8d4e804)] 
pub struct Nat44EiAddDelStaticMappingReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_static_mapping_dump_51077d14)] 
pub struct Nat44EiStaticMappingDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_static_mapping_details_6b51ca6e)] 
pub struct Nat44EiStaticMappingDetails { 
	pub context : u32, 
	 pub flags : EnumFlag<Nat44EiConfigFlags>, 
	pub local_ip_address : Ip4Address, 
	pub external_ip_address : Ip4Address, 
	pub protocol : u8, 
	pub local_port : u16, 
	pub external_port : u16, 
	pub external_sw_if_index : InterfaceIndex, 
	pub vrf_id : u32, 
	pub tag : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_add_del_identity_mapping_cb8606b9)] 
pub struct Nat44EiAddDelIdentityMapping { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	 pub flags : EnumFlag<Nat44EiConfigFlags>, 
	pub ip_address : Ip4Address, 
	pub protocol : u8, 
	pub port : u16, 
	pub sw_if_index : InterfaceIndex, 
	pub vrf_id : u32, 
	pub tag : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_add_del_identity_mapping_reply_e8d4e804)] 
pub struct Nat44EiAddDelIdentityMappingReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_identity_mapping_dump_51077d14)] 
pub struct Nat44EiIdentityMappingDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_identity_mapping_details_30d53e26)] 
pub struct Nat44EiIdentityMappingDetails { 
	pub context : u32, 
	 pub flags : EnumFlag<Nat44EiConfigFlags>, 
	pub ip_address : Ip4Address, 
	pub protocol : u8, 
	pub port : u16, 
	pub sw_if_index : InterfaceIndex, 
	pub vrf_id : u32, 
	pub tag : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_add_del_interface_addr_883abbcc)] 
pub struct Nat44EiAddDelInterfaceAddr { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub sw_if_index : InterfaceIndex, 
	 pub flags : EnumFlag<Nat44EiConfigFlags>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_add_del_interface_addr_reply_e8d4e804)] 
pub struct Nat44EiAddDelInterfaceAddrReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_interface_addr_dump_51077d14)] 
pub struct Nat44EiInterfaceAddrDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_interface_addr_details_0b45011c)] 
pub struct Nat44EiInterfaceAddrDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_user_dump_51077d14)] 
pub struct Nat44EiUserDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_user_details_355896c2)] 
pub struct Nat44EiUserDetails { 
	pub context : u32, 
	pub vrf_id : u32, 
	pub ip_address : Ip4Address, 
	pub nsessions : u32, 
	pub nstaticsessions : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_user_session_dump_e1899c98)] 
pub struct Nat44EiUserSessionDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub ip_address : Ip4Address, 
	pub vrf_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_user_session_details_19b7c0ac)] 
pub struct Nat44EiUserSessionDetails { 
	pub context : u32, 
	pub outside_ip_address : Ip4Address, 
	pub outside_port : u16, 
	pub inside_ip_address : Ip4Address, 
	pub inside_port : u16, 
	pub protocol : u16, 
	 pub flags : EnumFlag<Nat44EiConfigFlags>, 
	pub last_heard : u64, 
	pub total_bytes : u64, 
	pub total_pkts : u32, 
	pub ext_host_address : Ip4Address, 
	pub ext_host_port : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_del_session_74969ffe)] 
pub struct Nat44EiDelSession { 
	pub client_index : u32, 
	pub context : u32, 
	pub address : Ip4Address, 
	pub protocol : u8, 
	pub port : u16, 
	pub vrf_id : u32, 
	 pub flags : EnumFlag<Nat44EiConfigFlags>, 
	pub ext_host_address : Ip4Address, 
	pub ext_host_port : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_del_session_reply_e8d4e804)] 
pub struct Nat44EiDelSessionReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_forwarding_enable_disable_b3e225d2)] 
pub struct Nat44EiForwardingEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_forwarding_enable_disable_reply_e8d4e804)] 
pub struct Nat44EiForwardingEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_set_fq_options_2399bd71)] 
pub struct Nat44EiSetFqOptions { 
	pub client_index : u32, 
	pub context : u32, 
	pub frame_queue_nelts : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_set_fq_options_reply_e8d4e804)] 
pub struct Nat44EiSetFqOptionsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_show_fq_options_51077d14)] 
pub struct Nat44EiShowFqOptions { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat44_ei_show_fq_options_reply_7213b545)] 
pub struct Nat44EiShowFqOptionsReply { 
	pub context : u32, 
	pub retval : i32, 
	pub frame_queue_nelts : u32, 
} 
