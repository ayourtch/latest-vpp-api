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
// Implementation for pnat_match_tuple 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct PnatMatchTuple { 
	pub src : Ip4Address, 
	pub dst : Ip4Address, 
	pub proto : IpProto, 
	pub sport : u16, 
	pub dport : u16, 
	pub mask : PnatMask, 
} 
// Implementation for pnat_rewrite_tuple 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct PnatRewriteTuple { 
	pub src : Ip4Address, 
	pub dst : Ip4Address, 
	pub sport : u16, 
	pub dport : u16, 
	pub mask : PnatMask, 
	pub from_offset : u8, 
	pub to_offset : u8, 
	pub clear_offset : u8, 
} 
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)] 
pub enum PnatMask { 
	 PNAT_SA=1, 
	 PNAT_DA=2, 
	 PNAT_SPORT=4, 
	 PNAT_DPORT=8, 
	 PNAT_COPY_BYTE=16, 
	 PNAT_CLEAR_BYTE=32, 
	 PNAT_PROTO=64, 
} 
impl Default for PnatMask { 
	fn default() -> Self { PnatMask::PNAT_SA }
}
impl AsEnumFlag for PnatMask {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 1 => PnatMask::PNAT_SA, 
			 2 => PnatMask::PNAT_DA, 
			 4 => PnatMask::PNAT_SPORT, 
			 8 => PnatMask::PNAT_DPORT, 
			 16 => PnatMask::PNAT_COPY_BYTE, 
			 32 => PnatMask::PNAT_CLEAR_BYTE, 
			 64 => PnatMask::PNAT_PROTO, 
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 32 as u32
	}
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum PnatAttachmentPoint { 
	 PNAT_IP4_INPUT=0, 
	 PNAT_IP4_OUTPUT=1, 
	 PNAT_ATTACHMENT_POINT_MAX=2, 
} 
impl Default for PnatAttachmentPoint { 
	fn default() -> Self { PnatAttachmentPoint::PNAT_IP4_INPUT }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pnat_binding_add_946ee0b7)] 
pub struct PnatBindingAdd { 
	pub client_index : u32, 
	pub context : u32, 
	pub mach : PnatMatchTuple, 
	pub rewrite : PnatRewriteTuple, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pnat_binding_add_reply_4cd980a7)] 
pub struct PnatBindingAddReply { 
	pub context : u32, 
	pub retval : i32, 
	pub binding_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pnat_binding_add_v2_946ee0b7)] 
pub struct PnatBindingAddV2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub mach : PnatMatchTuple, 
	pub rewrite : PnatRewriteTuple, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pnat_binding_add_v2_reply_4cd980a7)] 
pub struct PnatBindingAddV2Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub binding_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pnat_binding_del_9259df7b)] 
pub struct PnatBindingDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub binding_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pnat_binding_del_reply_e8d4e804)] 
pub struct PnatBindingDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pnat_binding_attach_6e074232)] 
pub struct PnatBindingAttach { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub attachment : PnatAttachmentPoint, 
	pub binding_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pnat_binding_attach_reply_e8d4e804)] 
pub struct PnatBindingAttachReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pnat_binding_detach_6e074232)] 
pub struct PnatBindingDetach { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub attachment : PnatAttachmentPoint, 
	pub binding_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pnat_binding_detach_reply_e8d4e804)] 
pub struct PnatBindingDetachReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pnat_bindings_get_f75ba505)] 
pub struct PnatBindingsGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub cursor : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pnat_bindings_get_reply_53b48f5d)] 
pub struct PnatBindingsGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub cursor : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pnat_bindings_details_08fb2815)] 
pub struct PnatBindingsDetails { 
	pub context : u32, 
	pub mach : PnatMatchTuple, 
	pub rewrite : PnatRewriteTuple, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pnat_interfaces_get_f75ba505)] 
pub struct PnatInterfacesGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub cursor : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pnat_interfaces_get_reply_53b48f5d)] 
pub struct PnatInterfacesGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub cursor : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(pnat_interfaces_details_4cb09493)] 
pub struct PnatInterfacesDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub enabled : FixedSizeArray<bool, typenum::U2>, 
	pub lookup_mask : FixedSizeArray<PnatMask, typenum::U2>, 
} 
