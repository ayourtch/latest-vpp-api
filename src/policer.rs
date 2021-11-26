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
use crate::policer_types::*; 
use crate::interface_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(policer_bind_dcf516f9)] 
pub struct PolicerBind { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<typenum::U64>, 
	pub worker_index : u32, 
	pub bind_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(policer_bind_reply_e8d4e804)] 
pub struct PolicerBindReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(policer_input_233f0ef5)] 
pub struct PolicerInput { 
	pub client_index : u32, 
	pub context : u32, 
	pub name : FixedSizeString<typenum::U64>, 
	pub sw_if_index : InterfaceIndex, 
	pub apply : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(policer_input_reply_e8d4e804)] 
pub struct PolicerInputReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(policer_add_del_2b31dd38)] 
pub struct PolicerAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub name : FixedSizeString<typenum::U64>, 
	pub cir : u32, 
	pub eir : u32, 
	pub cb : u64, 
	pub eb : u64, 
	pub rate_type : Sse2QosRateType, 
	pub round_type : Sse2QosRoundType, 
	pub typ : Sse2QosPolicerType, 
	pub color_aware : bool, 
	pub conform_action : Sse2QosAction, 
	pub exceed_action : Sse2QosAction, 
	pub violate_action : Sse2QosAction, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(policer_add_del_reply_a177cef2)] 
pub struct PolicerAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub policer_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(policer_dump_35f1ae0f)] 
pub struct PolicerDump { 
	pub client_index : u32, 
	pub context : u32, 
	pub match_name_valid : bool, 
	pub match_name : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(policer_details_72d0e248)] 
pub struct PolicerDetails { 
	pub context : u32, 
	pub name : FixedSizeString<typenum::U64>, 
	pub cir : u32, 
	pub eir : u32, 
	pub cb : u64, 
	pub eb : u64, 
	pub rate_type : Sse2QosRateType, 
	pub round_type : Sse2QosRoundType, 
	pub typ : Sse2QosPolicerType, 
	pub conform_action : Sse2QosAction, 
	pub exceed_action : Sse2QosAction, 
	pub violate_action : Sse2QosAction, 
	pub single_rate : bool, 
	pub color_aware : bool, 
	pub scale : u32, 
	pub cir_tokens_per_period : u32, 
	pub pir_tokens_per_period : u32, 
	pub current_limit : u32, 
	pub current_bucket : u32, 
	pub extended_limit : u32, 
	pub extended_bucket : u32, 
	pub last_update_time : u64, 
} 
