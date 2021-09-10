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
use crate::fib_types::*; 
// Implementation for mfib_path 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct MfibPath { 
	pub itf_flags : EnumFlag<MfibItfFlags>, 
	pub path : FibPath, 
} 
#[derive(Debug,Serialize, Deserialize, Clone, Copy)] 
pub enum MfibEntryFlags { 
	 MFIB_API_ENTRY_FLAG_NONE=0, 
	 MFIB_API_ENTRY_FLAG_SIGNAL=1, 
	 MFIB_API_ENTRY_FLAG_DROP=2, 
	 MFIB_API_ENTRY_FLAG_CONNECTED=4, 
	 MFIB_API_ENTRY_FLAG_ACCEPT_ALL_ITF=8, 
} 
impl Default for MfibEntryFlags { 
	fn default() -> Self { MfibEntryFlags::MFIB_API_ENTRY_FLAG_NONE }
}
impl AsEnumFlag for MfibEntryFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 0 => MfibEntryFlags::MFIB_API_ENTRY_FLAG_NONE, 
			 1 => MfibEntryFlags::MFIB_API_ENTRY_FLAG_SIGNAL, 
			 2 => MfibEntryFlags::MFIB_API_ENTRY_FLAG_DROP, 
			 4 => MfibEntryFlags::MFIB_API_ENTRY_FLAG_CONNECTED, 
			 8 => MfibEntryFlags::MFIB_API_ENTRY_FLAG_ACCEPT_ALL_ITF, 
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 32 as u32
	}
}
#[derive(Debug,Serialize, Deserialize, Clone, Copy)] 
pub enum MfibItfFlags { 
	 MFIB_API_ITF_FLAG_NONE=0, 
	 MFIB_API_ITF_FLAG_NEGATE_SIGNAL=1, 
	 MFIB_API_ITF_FLAG_ACCEPT=2, 
	 MFIB_API_ITF_FLAG_FORWARD=4, 
	 MFIB_API_ITF_FLAG_SIGNAL_PRESENT=8, 
	 MFIB_API_ITF_FLAG_DONT_PRESERVE=16, 
} 
impl Default for MfibItfFlags { 
	fn default() -> Self { MfibItfFlags::MFIB_API_ITF_FLAG_NONE }
}
impl AsEnumFlag for MfibItfFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 0 => MfibItfFlags::MFIB_API_ITF_FLAG_NONE, 
			 1 => MfibItfFlags::MFIB_API_ITF_FLAG_NEGATE_SIGNAL, 
			 2 => MfibItfFlags::MFIB_API_ITF_FLAG_ACCEPT, 
			 4 => MfibItfFlags::MFIB_API_ITF_FLAG_FORWARD, 
			 8 => MfibItfFlags::MFIB_API_ITF_FLAG_SIGNAL_PRESENT, 
			 16 => MfibItfFlags::MFIB_API_ITF_FLAG_DONT_PRESERVE, 
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 32 as u32
	}
}
