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
// Implementation for vrrp_vr_key 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct VrrpVrKey { 
	pub sw_if_index : InterfaceIndex, 
	pub vr_id : u8, 
	pub is_ipv6 : u8, 
} 
// Implementation for vrrp_vr_conf 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct VrrpVrConf { 
	pub sw_if_index : InterfaceIndex, 
	pub vr_id : u8, 
	pub priority : u8, 
	pub interval : u16, 
	pub flags : EnumFlag<VrrpVrFlags>, 
} 
// Implementation for vrrp_vr_tracking 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct VrrpVrTracking { 
	pub interfaces_dec : u32, 
	pub priority : u8, 
} 
// Implementation for vrrp_vr_runtime 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct VrrpVrRuntime { 
	pub state : VrrpVrState, 
	pub master_adv_int : u16, 
	pub skew : u16, 
	pub master_down_int : u16, 
	pub mac : MacAddress, 
	pub tracking : VrrpVrTracking, 
} 
// Implementation for vrrp_vr_track_if 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct VrrpVrTrackIf { 
	pub sw_if_index : InterfaceIndex, 
	pub priority : u8, 
} 
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)] 
pub enum VrrpVrFlags { 
	 VRRP_API_VR_PREEMPT=1, 
	 VRRP_API_VR_ACCEPT=2, 
	 VRRP_API_VR_UNICAST=4, 
	 VRRP_API_VR_IPV6=8, 
} 
impl Default for VrrpVrFlags { 
	fn default() -> Self { VrrpVrFlags::VRRP_API_VR_PREEMPT }
}
impl AsEnumFlag for VrrpVrFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 1 => VrrpVrFlags::VRRP_API_VR_PREEMPT, 
			 2 => VrrpVrFlags::VRRP_API_VR_ACCEPT, 
			 4 => VrrpVrFlags::VRRP_API_VR_UNICAST, 
			 8 => VrrpVrFlags::VRRP_API_VR_IPV6, 
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 32 as u32
	}
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum VrrpVrState { 
	 VRRP_API_VR_STATE_INIT=0, 
	 VRRP_API_VR_STATE_BACKUP=1, 
	 VRRP_API_VR_STATE_MASTER=2, 
	 VRRP_API_VR_STATE_INTF_DOWN=3, 
} 
impl Default for VrrpVrState { 
	fn default() -> Self { VrrpVrState::VRRP_API_VR_STATE_INIT }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_add_del_c5cf15aa)] 
pub struct VrrpVrAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : u8, 
	pub sw_if_index : InterfaceIndex, 
	pub vr_id : u8, 
	pub priority : u8, 
	pub interval : u16, 
	 pub flags : EnumFlag<VrrpVrFlags>, 
	pub n_addrs : u8, 
	pub addrs : VariableSizeArray<Address>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_add_del_reply_e8d4e804)] 
pub struct VrrpVrAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_update_0b51e2f4)] 
pub struct VrrpVrUpdate { 
	pub client_index : u32, 
	pub context : u32, 
	pub vrrp_index : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub vr_id : u8, 
	pub priority : u8, 
	pub interval : u16, 
	 pub flags : EnumFlag<VrrpVrFlags>, 
	pub n_addrs : u8, 
	pub addrs : VariableSizeArray<Address>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_update_reply_5317d608)] 
pub struct VrrpVrUpdateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub vrrp_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_del_6029baa1)] 
pub struct VrrpVrDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub vrrp_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_del_reply_e8d4e804)] 
pub struct VrrpVrDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_dump_f9e6675e)] 
pub struct VrrpVrDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_details_46edcebd)] 
pub struct VrrpVrDetails { 
	pub context : u32, 
	pub config : VrrpVrConf, 
	pub runtime : VrrpVrRuntime, 
	pub n_addrs : u8, 
	pub addrs : VariableSizeArray<Address>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_start_stop_0662a3b7)] 
pub struct VrrpVrStartStop { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub vr_id : u8, 
	pub is_ipv6 : u8, 
	pub is_start : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_start_stop_reply_e8d4e804)] 
pub struct VrrpVrStartStopReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_set_peers_20bec71f)] 
pub struct VrrpVrSetPeers { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub vr_id : u8, 
	pub is_ipv6 : u8, 
	pub n_addrs : u8, 
	pub addrs : VariableSizeArray<Address>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_set_peers_reply_e8d4e804)] 
pub struct VrrpVrSetPeersReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_peer_dump_6fa3f7c4)] 
pub struct VrrpVrPeerDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_ipv6 : u8, 
	pub vr_id : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_peer_details_3d99c108)] 
pub struct VrrpVrPeerDetails { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub vr_id : u8, 
	pub is_ipv6 : u8, 
	pub n_peer_addrs : u8, 
	pub peer_addrs : VariableSizeArray<Address>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_track_if_add_del_d67df299)] 
pub struct VrrpVrTrackIfAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_ipv6 : u8, 
	pub vr_id : u8, 
	pub is_add : u8, 
	pub n_ifs : u8, 
	pub ifs : VariableSizeArray<VrrpVrTrackIf>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_track_if_add_del_reply_e8d4e804)] 
pub struct VrrpVrTrackIfAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_track_if_dump_a34dfc6d)] 
pub struct VrrpVrTrackIfDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub is_ipv6 : u8, 
	pub vr_id : u8, 
	pub dump_all : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_track_if_details_73c36f81)] 
pub struct VrrpVrTrackIfDetails { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub vr_id : u8, 
	pub is_ipv6 : u8, 
	pub n_ifs : u8, 
	pub ifs : VariableSizeArray<VrrpVrTrackIf>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(vrrp_vr_event_c1fea6a5)] 
pub struct VrrpVrEvent { 
	pub client_index : u32, 
	pub pid : u32, 
	pub vr : VrrpVrKey, 
	pub old_state : VrrpVrState, 
	pub new_state : VrrpVrState, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(want_vrrp_vr_events_c5e2af94)] 
pub struct WantVrrpVrEvents { 
	pub client_index : u32, 
	pub context : u32, 
	pub enable_disable : bool, 
	pub pid : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(want_vrrp_vr_events_reply_e8d4e804)] 
pub struct WantVrrpVrEventsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
