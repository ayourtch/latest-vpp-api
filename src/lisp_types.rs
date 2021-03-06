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
// Implementation for local_locator
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalLocator {
	pub sw_if_index: InterfaceIndex,
	pub priority: u8,
	pub weight: u8,
}
// Implementation for remote_locator
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoteLocator {
	pub priority: u8,
	pub weight: u8,
	pub ip_address: Address,
}
// Implementation for nsh
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Nsh {
	pub spi: u32,
	pub si: u8,
}
// Implementation for eid
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Eid {
	pub typ: EidType,
	pub address: EidAddress,
}
// Implementation for hmac_key
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HmacKey {
	pub id: HmacKeyId,
	pub key: FixedSizeArray<u8, typenum::U64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, VppUnionIdent)]
#[types(Prefix:18)]
#[types(MacAddress:6)]
#[types(Nsh:5)]
pub struct EidAddress(FixedSizeArray<u8, typenum::U18>);
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EidType {
	 EID_TYPE_API_PREFIX=0,
	 EID_TYPE_API_MAC=1,
	 EID_TYPE_API_NSH=2,
}
impl Default for EidType {
	fn default() -> Self { EidType::EID_TYPE_API_PREFIX }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum HmacKeyId {
	 KEY_ID_API_HMAC_NO_KEY=0,
	 KEY_ID_API_HMAC_SHA_1_96=1,
	 KEY_ID_API_HMAC_SHA_256_128=2,
}
impl Default for HmacKeyId {
	fn default() -> Self { HmacKeyId::KEY_ID_API_HMAC_NO_KEY }
}
