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
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vxlan_gpe_add_del_tunnel_a645b2b0)] 
pub struct VxlanGpeAddDelTunnel { 
	pub client_index : u32, 
	pub context : u32, 
	pub local : Address, 
	pub remote : Address, 
	pub mcast_sw_if_index : InterfaceIndex, 
	pub encap_vrf_id : u32, 
	pub decap_vrf_id : u32, 
	pub protocol : IpProto, 
	pub vni : u32, 
	pub is_add : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vxlan_gpe_add_del_tunnel_v2_d62fdb35)] 
pub struct VxlanGpeAddDelTunnelV2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub local : Address, 
	pub remote : Address, 
	pub local_port : u16, 
	pub remote_port : u16, 
	pub mcast_sw_if_index : InterfaceIndex, 
	pub encap_vrf_id : u32, 
	pub decap_vrf_id : u32, 
	pub protocol : IpProto, 
	pub vni : u32, 
	pub is_add : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vxlan_gpe_add_del_tunnel_reply_5383d31f)] 
pub struct VxlanGpeAddDelTunnelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vxlan_gpe_add_del_tunnel_v2_reply_5383d31f)] 
pub struct VxlanGpeAddDelTunnelV2Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vxlan_gpe_tunnel_dump_f9e6675e)] 
pub struct VxlanGpeTunnelDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vxlan_gpe_tunnel_v2_dump_f9e6675e)] 
pub struct VxlanGpeTunnelV2Dump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vxlan_gpe_tunnel_details_0968fc8b)] 
pub struct VxlanGpeTunnelDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub local : Address, 
	pub remote : Address, 
	pub vni : u32, 
	pub protocol : IpProto, 
	pub mcast_sw_if_index : InterfaceIndex, 
	pub encap_vrf_id : u32, 
	pub decap_vrf_id : u32, 
	pub is_ipv6 : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vxlan_gpe_tunnel_v2_details_06be4870)] 
pub struct VxlanGpeTunnelV2Details { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub local : Address, 
	pub remote : Address, 
	pub local_port : u16, 
	pub remote_port : u16, 
	pub vni : u32, 
	pub protocol : IpProto, 
	pub mcast_sw_if_index : InterfaceIndex, 
	pub encap_vrf_id : u32, 
	pub decap_vrf_id : u32, 
	pub is_ipv6 : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sw_interface_set_vxlan_gpe_bypass_65247409)] 
pub struct SwInterfaceSetVxlanGpeBypass { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_ipv6 : bool, 
	pub enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sw_interface_set_vxlan_gpe_bypass_reply_e8d4e804)] 
pub struct SwInterfaceSetVxlanGpeBypassReply { 
	pub context : u32, 
	pub retval : i32, 
} 
