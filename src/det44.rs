/*
   Autogenerated Data, Do not Edit! 
*/
#![allow(dead_code,unused_mut,unused_variables,unused_must_use,non_camel_case_types,unused_imports,non_snake_case)]
use vpp_api_macros::{VppMessage,VppUnionIdent}; 
use std::convert::TryInto; 
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use crate::VppNamedMessage;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum;
use crate::ip_types::*; 
use crate::nat_types::*; 
use crate::interface_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_plugin_enable_disable_617b6bf8)] 
pub struct Det44PluginEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub inside_vrf : u32, 
	pub outside_vrf : u32, 
	pub enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_plugin_enable_disable_reply_e8d4e804)] 
pub struct Det44PluginEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_interface_add_del_feature_dc17a836)] 
pub struct Det44InterfaceAddDelFeature { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub is_inside : bool, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_interface_add_del_feature_reply_e8d4e804)] 
pub struct Det44InterfaceAddDelFeatureReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_interface_dump_51077d14)] 
pub struct Det44InterfaceDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_interface_details_e60cc5be)] 
pub struct Det44InterfaceDetails { 
	pub context : u32, 
	pub is_inside : bool, 
	pub is_outside : bool, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_add_del_map_1150a190)] 
pub struct Det44AddDelMap { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub in_addr : Ip4Address, 
	pub in_plen : u8, 
	pub out_addr : Ip4Address, 
	pub out_plen : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_add_del_map_reply_e8d4e804)] 
pub struct Det44AddDelMapReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_forward_7f8a89cd)] 
pub struct Det44Forward { 
	pub client_index : u32, 
	pub context : u32, 
	pub in_addr : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_forward_reply_a8ccbdc0)] 
pub struct Det44ForwardReply { 
	pub context : u32, 
	pub retval : i32, 
	pub out_port_lo : u16, 
	pub out_port_hi : u16, 
	pub out_addr : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_reverse_a7573fe1)] 
pub struct Det44Reverse { 
	pub client_index : u32, 
	pub context : u32, 
	pub out_port : u16, 
	pub out_addr : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_reverse_reply_34066d48)] 
pub struct Det44ReverseReply { 
	pub context : u32, 
	pub retval : i32, 
	pub in_addr : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_map_dump_51077d14)] 
pub struct Det44MapDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_map_details_ad91dc83)] 
pub struct Det44MapDetails { 
	pub context : u32, 
	pub in_addr : Ip4Address, 
	pub in_plen : u8, 
	pub out_addr : Ip4Address, 
	pub out_plen : u8, 
	pub sharing_ratio : u32, 
	pub ports_per_host : u16, 
	pub ses_num : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_close_session_out_f6b259d1)] 
pub struct Det44CloseSessionOut { 
	pub client_index : u32, 
	pub context : u32, 
	pub out_addr : Ip4Address, 
	pub out_port : u16, 
	pub ext_addr : Ip4Address, 
	pub ext_port : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_close_session_out_reply_e8d4e804)] 
pub struct Det44CloseSessionOutReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_close_session_in_3c68e073)] 
pub struct Det44CloseSessionIn { 
	pub client_index : u32, 
	pub context : u32, 
	pub in_addr : Ip4Address, 
	pub in_port : u16, 
	pub ext_addr : Ip4Address, 
	pub ext_port : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_close_session_in_reply_e8d4e804)] 
pub struct Det44CloseSessionInReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_session_dump_e45a3af7)] 
pub struct Det44SessionDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub user_addr : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_session_details_27f3c171)] 
pub struct Det44SessionDetails { 
	pub context : u32, 
	pub in_port : u16, 
	pub ext_addr : Ip4Address, 
	pub ext_port : u16, 
	pub out_port : u16, 
	pub state : u8, 
	pub expire : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_set_timeouts_d4746b16)] 
pub struct Det44SetTimeouts { 
	pub client_index : u32, 
	pub context : u32, 
	pub udp : u32, 
	pub tcp_established : u32, 
	pub tcp_transitory : u32, 
	pub icmp : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_set_timeouts_reply_e8d4e804)] 
pub struct Det44SetTimeoutsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_get_timeouts_51077d14)] 
pub struct Det44GetTimeouts { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(det44_get_timeouts_reply_3c4df4e1)] 
pub struct Det44GetTimeoutsReply { 
	pub context : u32, 
	pub retval : i32, 
	pub udp : u32, 
	pub tcp_established : u32, 
	pub tcp_transitory : u32, 
	pub icmp : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat_det_add_del_map_1150a190)] 
pub struct NatDetAddDelMap { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub in_addr : Ip4Address, 
	pub in_plen : u8, 
	pub out_addr : Ip4Address, 
	pub out_plen : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat_det_add_del_map_reply_e8d4e804)] 
pub struct NatDetAddDelMapReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat_det_forward_7f8a89cd)] 
pub struct NatDetForward { 
	pub client_index : u32, 
	pub context : u32, 
	pub in_addr : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat_det_forward_reply_a8ccbdc0)] 
pub struct NatDetForwardReply { 
	pub context : u32, 
	pub retval : i32, 
	pub out_port_lo : u16, 
	pub out_port_hi : u16, 
	pub out_addr : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat_det_reverse_a7573fe1)] 
pub struct NatDetReverse { 
	pub client_index : u32, 
	pub context : u32, 
	pub out_port : u16, 
	pub out_addr : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat_det_reverse_reply_34066d48)] 
pub struct NatDetReverseReply { 
	pub context : u32, 
	pub retval : i32, 
	pub in_addr : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat_det_map_dump_51077d14)] 
pub struct NatDetMapDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat_det_map_details_ad91dc83)] 
pub struct NatDetMapDetails { 
	pub context : u32, 
	pub in_addr : Ip4Address, 
	pub in_plen : u8, 
	pub out_addr : Ip4Address, 
	pub out_plen : u8, 
	pub sharing_ratio : u32, 
	pub ports_per_host : u16, 
	pub ses_num : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat_det_close_session_out_f6b259d1)] 
pub struct NatDetCloseSessionOut { 
	pub client_index : u32, 
	pub context : u32, 
	pub out_addr : Ip4Address, 
	pub out_port : u16, 
	pub ext_addr : Ip4Address, 
	pub ext_port : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat_det_close_session_out_reply_e8d4e804)] 
pub struct NatDetCloseSessionOutReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat_det_close_session_in_3c68e073)] 
pub struct NatDetCloseSessionIn { 
	pub client_index : u32, 
	pub context : u32, 
	pub in_addr : Ip4Address, 
	pub in_port : u16, 
	pub ext_addr : Ip4Address, 
	pub ext_port : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat_det_close_session_in_reply_e8d4e804)] 
pub struct NatDetCloseSessionInReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat_det_session_dump_e45a3af7)] 
pub struct NatDetSessionDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub user_addr : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(nat_det_session_details_27f3c171)] 
pub struct NatDetSessionDetails { 
	pub context : u32, 
	pub in_port : u16, 
	pub ext_addr : Ip4Address, 
	pub ext_port : u16, 
	pub out_port : u16, 
	pub state : u8, 
	pub expire : u32, 
} 
