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
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SrMobileNhtype {
	 SRV6_NHTYPE_API_NONE=0,
	 SRV6_NHTYPE_API_IPV4=1,
	 SRV6_NHTYPE_API_IPV6=2,
	 SRV6_NHTYPE_API_NON_IP=3,
}
impl Default for SrMobileNhtype {
	fn default() -> Self { SrMobileNhtype::SRV6_NHTYPE_API_NONE }
}