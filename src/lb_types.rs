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
// Implementation for lb_vip 
#[derive(Debug, Clone, Serialize, Deserialize, Default)] 
pub struct LbVip { 
	pub pfx : AddressWithPrefix, 
	pub protocol : IpProto, 
	pub port : u16, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum LbSrvType { 
	 LB_API_SRV_TYPE_CLUSTERIP=0, 
	 LB_API_SRV_TYPE_NODEPORT=1, 
	 LB_API_SRV_N_TYPES=2, 
} 
impl Default for LbSrvType { 
	fn default() -> Self { LbSrvType::LB_API_SRV_TYPE_CLUSTERIP }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum LbEncapType { 
	 LB_API_ENCAP_TYPE_GRE4=0, 
	 LB_API_ENCAP_TYPE_GRE6=1, 
	 LB_API_ENCAP_TYPE_L3DSR=2, 
	 LB_API_ENCAP_TYPE_NAT4=3, 
	 LB_API_ENCAP_TYPE_NAT6=4, 
	 LB_API_ENCAP_N_TYPES=5, 
} 
impl Default for LbEncapType { 
	fn default() -> Self { LbEncapType::LB_API_ENCAP_TYPE_GRE4 }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum LbLkpTypeT { 
	 LB_API_LKP_SAME_IP_PORT=0, 
	 LB_API_LKP_DIFF_IP_PORT=1, 
	 LB_API_LKP_ALL_PORT_IP=2, 
	 LB_API_LKP_N_TYPES=3, 
} 
impl Default for LbLkpTypeT { 
	fn default() -> Self { LbLkpTypeT::LB_API_LKP_SAME_IP_PORT }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum LbVipType { 
	 LB_API_VIP_TYPE_IP6_GRE6=0, 
	 LB_API_VIP_TYPE_IP6_GRE4=1, 
	 LB_API_VIP_TYPE_IP4_GRE6=2, 
	 LB_API_VIP_TYPE_IP4_GRE4=3, 
	 LB_API_VIP_TYPE_IP4_L3DSR=4, 
	 LB_API_VIP_TYPE_IP4_NAT4=5, 
	 LB_API_VIP_TYPE_IP6_NAT6=6, 
	 LB_API_VIP_N_TYPES=7, 
} 
impl Default for LbVipType { 
	fn default() -> Self { LbVipType::LB_API_VIP_TYPE_IP6_GRE6 }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum LbNatProtocol { 
	 LB_API_NAT_PROTOCOL_UDP=6, 
	 LB_API_NAT_PROTOCOL_TCP=23, 
	 LB_API_NAT_PROTOCOL_ANY=4294967295, 
} 
impl Default for LbNatProtocol { 
	fn default() -> Self { LbNatProtocol::LB_API_NAT_PROTOCOL_UDP }
}
