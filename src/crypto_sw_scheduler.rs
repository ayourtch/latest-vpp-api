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
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(crypto_sw_scheduler_set_worker_b4274502)] 
pub struct CryptoSwSchedulerSetWorker { 
	pub client_index : u32, 
	pub context : u32, 
	pub worker_index : u32, 
	pub crypto_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(crypto_sw_scheduler_set_worker_reply_e8d4e804)] 
pub struct CryptoSwSchedulerSetWorkerReply { 
	pub context : u32, 
	pub retval : i32, 
} 
