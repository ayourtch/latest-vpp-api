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
use crate::interface_types::*; 
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)] 
pub enum FlowprobeWhichFlags { 
	 FLOWPROBE_WHICH_FLAG_IP4=1, 
	 FLOWPROBE_WHICH_FLAG_L2=2, 
	 FLOWPROBE_WHICH_FLAG_IP6=4, 
} 
impl Default for FlowprobeWhichFlags { 
	fn default() -> Self { FlowprobeWhichFlags::FLOWPROBE_WHICH_FLAG_IP4 }
}
impl AsEnumFlag for FlowprobeWhichFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 1 => FlowprobeWhichFlags::FLOWPROBE_WHICH_FLAG_IP4, 
			 2 => FlowprobeWhichFlags::FLOWPROBE_WHICH_FLAG_L2, 
			 4 => FlowprobeWhichFlags::FLOWPROBE_WHICH_FLAG_IP6, 
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 8 as u32
	}
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum FlowprobeWhich { 
	 FLOWPROBE_WHICH_IP4=0, 
	 FLOWPROBE_WHICH_IP6=1, 
	 FLOWPROBE_WHICH_L2=2, 
} 
impl Default for FlowprobeWhich { 
	fn default() -> Self { FlowprobeWhich::FLOWPROBE_WHICH_IP4 }
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)] 
pub enum FlowprobeRecordFlags { 
	 FLOWPROBE_RECORD_FLAG_L2=1, 
	 FLOWPROBE_RECORD_FLAG_L3=2, 
	 FLOWPROBE_RECORD_FLAG_L4=4, 
} 
impl Default for FlowprobeRecordFlags { 
	fn default() -> Self { FlowprobeRecordFlags::FLOWPROBE_RECORD_FLAG_L2 }
}
impl AsEnumFlag for FlowprobeRecordFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 1 => FlowprobeRecordFlags::FLOWPROBE_RECORD_FLAG_L2, 
			 2 => FlowprobeRecordFlags::FLOWPROBE_RECORD_FLAG_L3, 
			 4 => FlowprobeRecordFlags::FLOWPROBE_RECORD_FLAG_L4, 
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 8 as u32
	}
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum FlowprobeDirection { 
	 FLOWPROBE_DIRECTION_RX=0, 
	 FLOWPROBE_DIRECTION_TX=1, 
	 FLOWPROBE_DIRECTION_BOTH=2, 
} 
impl Default for FlowprobeDirection { 
	fn default() -> Self { FlowprobeDirection::FLOWPROBE_DIRECTION_RX }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(flowprobe_tx_interface_add_del_b782c976)] 
pub struct FlowprobeTxInterfaceAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	 pub which : EnumFlag<FlowprobeWhichFlags>, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(flowprobe_tx_interface_add_del_reply_e8d4e804)] 
pub struct FlowprobeTxInterfaceAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(flowprobe_interface_add_del_3420739c)] 
pub struct FlowprobeInterfaceAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub which : FlowprobeWhich, 
	pub direction : FlowprobeDirection, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(flowprobe_interface_add_del_reply_e8d4e804)] 
pub struct FlowprobeInterfaceAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(flowprobe_interface_dump_f9e6675e)] 
pub struct FlowprobeInterfaceDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(flowprobe_interface_details_427d77e0)] 
pub struct FlowprobeInterfaceDetails { 
	pub context : u32, 
	pub which : FlowprobeWhich, 
	pub direction : FlowprobeDirection, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(flowprobe_params_baa46c09)] 
pub struct FlowprobeParams { 
	pub client_index : u32, 
	pub context : u32, 
	 pub record_flags : EnumFlag<FlowprobeRecordFlags>, 
	pub active_timer : u32, 
	pub passive_timer : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(flowprobe_params_reply_e8d4e804)] 
pub struct FlowprobeParamsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(flowprobe_set_params_baa46c09)] 
pub struct FlowprobeSetParams { 
	pub client_index : u32, 
	pub context : u32, 
	 pub record_flags : EnumFlag<FlowprobeRecordFlags>, 
	pub active_timer : u32, 
	pub passive_timer : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(flowprobe_set_params_reply_e8d4e804)] 
pub struct FlowprobeSetParamsReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(flowprobe_get_params_51077d14)] 
pub struct FlowprobeGetParams { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(flowprobe_get_params_reply_f350d621)] 
pub struct FlowprobeGetParamsReply { 
	pub context : u32, 
	pub retval : i32, 
	 pub record_flags : EnumFlag<FlowprobeRecordFlags>, 
	pub active_timer : u32, 
	pub passive_timer : u32, 
} 
