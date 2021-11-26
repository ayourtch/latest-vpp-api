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
#[message_name_and_crc(tls_openssl_set_engine_e34d95c1)] 
pub struct TlsOpensslSetEngine { 
	pub client_index : u32, 
	pub context : u32, 
	pub async_enable : u32, 
	pub engine : FixedSizeArray<u8, typenum::U64>, 
	pub algorithm : FixedSizeArray<u8, typenum::U64>, 
	pub ciphers : FixedSizeArray<u8, typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(tls_openssl_set_engine_reply_e8d4e804)] 
pub struct TlsOpensslSetEngineReply { 
	pub context : u32, 
	pub retval : i32, 
} 
