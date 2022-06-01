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
pub enum SrPolicyOp {
	 SR_POLICY_OP_API_NONE=0,
	 SR_POLICY_OP_API_ADD=1,
	 SR_POLICY_OP_API_DEL=2,
	 SR_POLICY_OP_API_MOD=3,
}
impl Default for SrPolicyOp {
	fn default() -> Self { SrPolicyOp::SR_POLICY_OP_API_NONE }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SrBehavior {
	 SR_BEHAVIOR_API_END=1,
	 SR_BEHAVIOR_API_X=2,
	 SR_BEHAVIOR_API_T=3,
	 SR_BEHAVIOR_API_D_FIRST=4,
	 SR_BEHAVIOR_API_DX2=5,
	 SR_BEHAVIOR_API_DX6=6,
	 SR_BEHAVIOR_API_DX4=7,
	 SR_BEHAVIOR_API_DT6=8,
	 SR_BEHAVIOR_API_DT4=9,
	 SR_BEHAVIOR_API_LAST=10,
}
impl Default for SrBehavior {
	fn default() -> Self { SrBehavior::SR_BEHAVIOR_API_END }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SrSteer {
	 SR_STEER_API_L2=2,
	 SR_STEER_API_IPV4=4,
	 SR_STEER_API_IPV6=6,
}
impl Default for SrSteer {
	fn default() -> Self { SrSteer::SR_STEER_API_L2 }
}
