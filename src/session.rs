/*
   Autogenerated Data, Do not Edit! 
*/
#![allow(dead_code,unused_mut,unused_variables,unused_must_use,non_camel_case_types,unused_imports,non_snake_case)]
use vpp_api_macros::{VppMessage,VppUnionIdent}; 
use std::convert::TryInto; 
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use crate::VppNamedMessage;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum;
use crate::ip_types::*; 
use crate::interface_types::*; 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum TransportProto { 
	 TRANSPORT_PROTO_API_TCP=1, 
	 TRANSPORT_PROTO_API_UDP=2, 
	 TRANSPORT_PROTO_API_NONE=3, 
	 TRANSPORT_PROTO_API_TLS=4, 
	 TRANSPORT_PROTO_API_QUIC=5, 
} 
impl Default for TransportProto { 
	fn default() -> Self { TransportProto::TRANSPORT_PROTO_API_TCP }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum SessionRuleScope { 
	 SESSION_RULE_SCOPE_API_GLOBAL=0, 
	 SESSION_RULE_SCOPE_API_LOCAL=1, 
	 SESSION_RULE_SCOPE_API_BOTH=2, 
} 
impl Default for SessionRuleScope { 
	fn default() -> Self { SessionRuleScope::SESSION_RULE_SCOPE_API_GLOBAL }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(app_attach_5f4a260d)] 
pub struct AppAttach { 
	pub client_index : u32, 
	pub context : u32, 
	pub options : FixedSizeArray<u64, typenum::U18>, 
	pub namespace_id : VariableSizeString, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(app_attach_reply_5c89c3b0)] 
pub struct AppAttachReply { 
	pub context : u32, 
	pub retval : i32, 
	pub app_mq : u64, 
	pub vpp_ctrl_mq : u64, 
	pub vpp_ctrl_mq_thread : u8, 
	pub app_index : u32, 
	pub n_fds : u8, 
	pub fd_flags : u8, 
	pub segment_size : u32, 
	pub segment_handle : u64, 
	pub segment_name : VariableSizeString, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(application_detach_51077d14)] 
pub struct ApplicationDetach { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(application_detach_reply_e8d4e804)] 
pub struct ApplicationDetachReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(app_add_cert_key_pair_02eb8016)] 
pub struct AppAddCertKeyPair { 
	pub client_index : u32, 
	pub context : u32, 
	pub cert_len : u16, 
	pub certkey_len : u16, 
	pub certkey : VariableSizeArray<u8>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(app_add_cert_key_pair_reply_b42958d0)] 
pub struct AppAddCertKeyPairReply { 
	pub context : u32, 
	pub retval : i32, 
	pub index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(app_del_cert_key_pair_8ac76db6)] 
pub struct AppDelCertKeyPair { 
	pub client_index : u32, 
	pub context : u32, 
	pub index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(app_del_cert_key_pair_reply_e8d4e804)] 
pub struct AppDelCertKeyPairReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(application_tls_cert_add_3f5cfe45)] 
pub struct ApplicationTlsCertAdd { 
	pub client_index : u32, 
	pub context : u32, 
	pub app_index : u32, 
	pub cert_len : u16, 
	pub cert : VariableSizeArray<u8>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(application_tls_cert_add_reply_e8d4e804)] 
pub struct ApplicationTlsCertAddReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(application_tls_key_add_5eaf70cd)] 
pub struct ApplicationTlsKeyAdd { 
	pub client_index : u32, 
	pub context : u32, 
	pub app_index : u32, 
	pub key_len : u16, 
	pub key : VariableSizeArray<u8>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(application_tls_key_add_reply_e8d4e804)] 
pub struct ApplicationTlsKeyAddReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(app_worker_add_del_753253dc)] 
pub struct AppWorkerAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub app_index : u32, 
	pub wrk_index : u32, 
	pub is_add : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(app_worker_add_del_reply_5735ffe7)] 
pub struct AppWorkerAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub wrk_index : u32, 
	pub app_event_queue_address : u64, 
	pub n_fds : u8, 
	pub fd_flags : u8, 
	pub segment_handle : u64, 
	pub is_add : bool, 
	pub segment_name : VariableSizeString, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(session_enable_disable_c264d7bf)] 
pub struct SessionEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(session_enable_disable_reply_e8d4e804)] 
pub struct SessionEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(session_sapi_enable_disable_c264d7bf)] 
pub struct SessionSapiEnableDisable { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_enable : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(session_sapi_enable_disable_reply_e8d4e804)] 
pub struct SessionSapiEnableDisableReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(app_namespace_add_del_6306aecb)] 
pub struct AppNamespaceAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub secret : u64, 
	pub sw_if_index : InterfaceIndex, 
	pub ip4_fib_id : u32, 
	pub ip6_fib_id : u32, 
	pub namespace_id : VariableSizeString, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(app_namespace_add_del_v2_ee0755cf)] 
pub struct AppNamespaceAddDelV2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub secret : u64, 
	pub sw_if_index : InterfaceIndex, 
	pub ip4_fib_id : u32, 
	pub ip6_fib_id : u32, 
	pub namespace_id : FixedSizeString<typenum::U64>, 
	pub netns : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(app_namespace_add_del_v3_8a7e40a1)] 
pub struct AppNamespaceAddDelV3 { 
	pub client_index : u32, 
	pub context : u32, 
	pub secret : u64, 
	pub is_add : bool, 
	pub sw_if_index : InterfaceIndex, 
	pub ip4_fib_id : u32, 
	pub ip6_fib_id : u32, 
	pub namespace_id : FixedSizeString<typenum::U64>, 
	pub netns : FixedSizeString<typenum::U64>, 
	pub sock_name : VariableSizeString, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(app_namespace_add_del_reply_85137120)] 
pub struct AppNamespaceAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub appns_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(app_namespace_add_del_v2_reply_85137120)] 
pub struct AppNamespaceAddDelV2Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub appns_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(app_namespace_add_del_v3_reply_85137120)] 
pub struct AppNamespaceAddDelV3Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub appns_index : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(session_rule_add_del_e4895422)] 
pub struct SessionRuleAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub transport_proto : TransportProto, 
	pub lcl : Prefix, 
	pub rmt : Prefix, 
	pub lcl_port : u16, 
	pub rmt_port : u16, 
	pub action_index : u32, 
	pub is_add : bool, 
	pub appns_index : u32, 
	pub scope : SessionRuleScope, 
	pub tag : FixedSizeString<typenum::U64>, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(session_rule_add_del_reply_e8d4e804)] 
pub struct SessionRuleAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(session_rules_dump_51077d14)] 
pub struct SessionRulesDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(session_rules_details_28d71830)] 
pub struct SessionRulesDetails { 
	pub context : u32, 
	pub transport_proto : TransportProto, 
	pub lcl : Prefix, 
	pub rmt : Prefix, 
	pub lcl_port : u16, 
	pub rmt_port : u16, 
	pub action_index : u32, 
	pub appns_index : u32, 
	pub scope : SessionRuleScope, 
	pub tag : FixedSizeString<typenum::U64>, 
} 
