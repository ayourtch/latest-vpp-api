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
// Implementation for pci_address 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct PciAddress { 
	pub domain : u16, 
	pub bus : u8, 
	pub slot : u8, 
	pub function : u8, 
} 
