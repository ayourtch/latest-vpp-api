/*
   Autogenerated Data, Do not Edit! 
*/
#![allow(dead_code,unused_mut,unused_variables,unused_must_use,non_camel_case_types,unused_imports,non_snake_case)]
use vpp_api_macros::{VppMessage,VppUnionIdent}; 
use std::convert::TryInto; 
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum;
use crate::ip_types::*; 
use crate::lisp_types::*; 
use crate::interface_types::*; 
use crate::ethernet_types::*; 
// Implementation for gpe_locator 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct GpeLocator { 
	pub weight : u8, 
	pub addr : Address, 
} 
// Implementation for gpe_fwd_entry 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct GpeFwdEntry { 
	pub fwd_entry_index : u32, 
	pub dp_table : u32, 
	pub leid : Eid, 
	pub reid : Eid, 
	pub vni : u32, 
	pub action : u8, 
} 
// Implementation for gpe_native_fwd_rpath 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct GpeNativeFwdRpath { 
	pub fib_index : u32, 
	pub nh_sw_if_index : InterfaceIndex, 
	pub nh_addr : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_add_del_fwd_entry_f0847644)] 
pub struct GpeAddDelFwdEntry { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub rmt_eid : Eid, 
	pub lcl_eid : Eid, 
	pub vni : u32, 
	pub dp_table : u32, 
	pub action : u8, 
	pub loc_num : u32, 
	pub locs : VariableSizeArray<GpeLocator>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_add_del_fwd_entry_reply_efe5f176)] 
pub struct GpeAddDelFwdEntryReply { 
	pub context : u32, 
	pub retval : i32, 
	pub fwd_entry_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_enable_disable_c264d7bf)] 
pub struct GpeEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_enable_disable_reply_e8d4e804)] 
pub struct GpeEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_add_del_iface_3ccff273)] 
pub struct GpeAddDelIface { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub is_l2 : bool, 
	pub dp_table : u32, 
	pub vni : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_add_del_iface_reply_e8d4e804)] 
pub struct GpeAddDelIfaceReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_fwd_entry_vnis_get_51077d14)] 
pub struct GpeFwdEntryVnisGet { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_fwd_entry_vnis_get_reply_aa70da20)] 
pub struct GpeFwdEntryVnisGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub vnis : VariableSizeArray<u32>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_fwd_entries_get_8d1f2fe9)] 
pub struct GpeFwdEntriesGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub vni : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_fwd_entries_get_reply_c4844876)] 
pub struct GpeFwdEntriesGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub entries : VariableSizeArray<GpeFwdEntry>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_fwd_entry_path_dump_39bce980)] 
pub struct GpeFwdEntryPathDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub fwd_entry_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_fwd_entry_path_details_483df51a)] 
pub struct GpeFwdEntryPathDetails { 
	pub context : u32, 
	pub lcl_loc : GpeLocator, 
	pub rmt_loc : GpeLocator, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_set_encap_mode_bd819eac)] 
pub struct GpeSetEncapMode { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_vxlan : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_set_encap_mode_reply_e8d4e804)] 
pub struct GpeSetEncapModeReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_get_encap_mode_51077d14)] 
pub struct GpeGetEncapMode { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_get_encap_mode_reply_36e3f7ca)] 
pub struct GpeGetEncapModeReply { 
	pub context : u32, 
	pub retval : i32, 
	pub encap_mode : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_add_del_native_fwd_rpath_43fc8b54)] 
pub struct GpeAddDelNativeFwdRpath { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub table_id : u32, 
	pub nh_sw_if_index : InterfaceIndex, 
	pub nh_addr : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_add_del_native_fwd_rpath_reply_e8d4e804)] 
pub struct GpeAddDelNativeFwdRpathReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_native_fwd_rpaths_get_f652ceb4)] 
pub struct GpeNativeFwdRpathsGet { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_ip4 : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(gpe_native_fwd_rpaths_get_reply_7a1ca5a2)] 
pub struct GpeNativeFwdRpathsGetReply { 
	pub context : u32, 
	pub retval : i32, 
	pub count : u32, 
	pub entries : VariableSizeArray<GpeNativeFwdRpath>, 
} 
