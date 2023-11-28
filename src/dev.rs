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
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum DevFlags {
	 VL_API_DEV_FLAG_NO_STATS=1,
}
impl Default for DevFlags {
	fn default() -> Self { DevFlags::VL_API_DEV_FLAG_NO_STATS }
}
impl AsEnumFlag for DevFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 1 => DevFlags::VL_API_DEV_FLAG_NO_STATS,
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 32 as u32
	}
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum DevPortFlags {
	 VL_API_DEV_PORT_FLAG_INTERRUPT_MODE=1,
}
impl Default for DevPortFlags {
	fn default() -> Self { DevPortFlags::VL_API_DEV_PORT_FLAG_INTERRUPT_MODE }
}
impl AsEnumFlag for DevPortFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 1 => DevPortFlags::VL_API_DEV_PORT_FLAG_INTERRUPT_MODE,
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 32 as u32
	}
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(dev_attach_44b725fc)]
pub struct DevAttach {
	pub client_index: u32,
	pub context: u32,
	pub device_id: FixedSizeString<typenum::U48>,
	pub driver_name: FixedSizeString<typenum::U16>,
	 pub flags: EnumFlag<DevFlags>,
	pub args: VariableSizeString,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(dev_attach_reply_6082b181)]
pub struct DevAttachReply {
	pub context: u32,
	pub dev_index: u32,
	pub retval: i32,
	pub error_string: VariableSizeString,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(dev_detach_afae52d6)]
pub struct DevDetach {
	pub client_index: u32,
	pub context: u32,
	pub dev_index: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(dev_detach_reply_c8d74455)]
pub struct DevDetachReply {
	pub context: u32,
	pub retval: i32,
	pub error_string: VariableSizeString,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(dev_create_port_if_1eb00d01)]
pub struct DevCreatePortIf {
	pub client_index: u32,
	pub context: u32,
	pub dev_index: u32,
	pub intf_name: FixedSizeString<typenum::U32>,
	pub num_rx_queues: u16,
	pub num_tx_queues: u16,
	pub rx_queue_size: u16,
	pub tx_queue_size: u16,
	pub port_id: u16,
	 pub flags: EnumFlag<DevPortFlags>,
	pub args: VariableSizeString,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(dev_create_port_if_reply_243c2374)]
pub struct DevCreatePortIfReply {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: u32,
	pub retval: i32,
	pub error_string: VariableSizeString,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(dev_remove_port_if_529cb13f)]
pub struct DevRemovePortIf {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(dev_remove_port_if_reply_c8d74455)]
pub struct DevRemovePortIfReply {
	pub context: u32,
	pub retval: i32,
	pub error_string: VariableSizeString,
}
