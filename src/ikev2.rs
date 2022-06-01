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
use crate::ikev2_types::*;
use crate::interface_types::*;
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_plugin_get_version_51077d14)]
pub struct Ikev2PluginGetVersion {
	pub client_index : u32,
	pub context : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_plugin_get_version_reply_9b32cf86)]
pub struct Ikev2PluginGetVersionReply {
	pub context : u32,
	pub major : u32,
	pub minor : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_dump_51077d14)]
pub struct Ikev2ProfileDump {
	pub client_index : u32,
	pub context : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_details_670d01d9)]
pub struct Ikev2ProfileDetails {
	pub context : u32,
	pub profile : Ikev2Profile,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_sa_dump_51077d14)]
pub struct Ikev2SaDump {
	pub client_index : u32,
	pub context : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_sa_details_937c22d5)]
pub struct Ikev2SaDetails {
	pub context : u32,
	pub retval : i32,
	pub sa : Ikev2Sa,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_child_sa_dump_01eab609)]
pub struct Ikev2ChildSaDump {
	pub client_index : u32,
	pub context : u32,
	pub sa_index : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_child_sa_details_ff67741f)]
pub struct Ikev2ChildSaDetails {
	pub context : u32,
	pub retval : i32,
	pub child_sa : Ikev2ChildSa,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_nonce_get_7fe9ad51)]
pub struct Ikev2NonceGet {
	pub client_index : u32,
	pub context : u32,
	pub is_initiator : bool,
	pub sa_index : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_nonce_get_reply_1b37a342)]
pub struct Ikev2NonceGetReply {
	pub context : u32,
	pub retval : i32,
	pub data_len : u32,
	pub nonce : VariableSizeArray<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_traffic_selector_dump_a7385e33)]
pub struct Ikev2TrafficSelectorDump {
	pub client_index : u32,
	pub context : u32,
	pub is_initiator : bool,
	pub sa_index : u32,
	pub child_sa_index : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_traffic_selector_details_518cb06f)]
pub struct Ikev2TrafficSelectorDetails {
	pub context : u32,
	pub retval : i32,
	pub ts : Ikev2Ts,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_add_del_2c925b55)]
pub struct Ikev2ProfileAddDel {
	pub client_index : u32,
	pub context : u32,
	pub name : FixedSizeString<typenum::U64>,
	pub is_add : bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_add_del_reply_e8d4e804)]
pub struct Ikev2ProfileAddDelReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_set_auth_642c97cd)]
pub struct Ikev2ProfileSetAuth {
	pub client_index : u32,
	pub context : u32,
	pub name : FixedSizeString<typenum::U64>,
	pub auth_method : u8,
	pub is_hex : bool,
	pub data_len : u32,
	pub data : VariableSizeArray<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_set_auth_reply_e8d4e804)]
pub struct Ikev2ProfileSetAuthReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_set_id_4d7e2418)]
pub struct Ikev2ProfileSetId {
	pub client_index : u32,
	pub context : u32,
	pub name : FixedSizeString<typenum::U64>,
	pub is_local : bool,
	pub id_type : u8,
	pub data_len : u32,
	pub data : VariableSizeArray<u8>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_set_id_reply_e8d4e804)]
pub struct Ikev2ProfileSetIdReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_disable_natt_ebf79a66)]
pub struct Ikev2ProfileDisableNatt {
	pub client_index : u32,
	pub context : u32,
	pub name : FixedSizeString<typenum::U64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_disable_natt_reply_e8d4e804)]
pub struct Ikev2ProfileDisableNattReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_set_ts_8eb8cfd1)]
pub struct Ikev2ProfileSetTs {
	pub client_index : u32,
	pub context : u32,
	pub name : FixedSizeString<typenum::U64>,
	pub ts : Ikev2Ts,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_set_ts_reply_e8d4e804)]
pub struct Ikev2ProfileSetTsReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_set_local_key_799b69ec)]
pub struct Ikev2SetLocalKey {
	pub client_index : u32,
	pub context : u32,
	pub key_file : FixedSizeString<typenum::U256>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_set_local_key_reply_e8d4e804)]
pub struct Ikev2SetLocalKeyReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_set_tunnel_interface_ca67182c)]
pub struct Ikev2SetTunnelInterface {
	pub client_index : u32,
	pub context : u32,
	pub name : FixedSizeString<typenum::U64>,
	pub sw_if_index : InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_set_tunnel_interface_reply_e8d4e804)]
pub struct Ikev2SetTunnelInterfaceReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_set_responder_a2055df1)]
pub struct Ikev2SetResponder {
	pub client_index : u32,
	pub context : u32,
	pub name : FixedSizeString<typenum::U64>,
	pub responder : Ikev2Responder,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_set_responder_reply_e8d4e804)]
pub struct Ikev2SetResponderReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_set_responder_hostname_350d6949)]
pub struct Ikev2SetResponderHostname {
	pub client_index : u32,
	pub context : u32,
	pub name : FixedSizeString<typenum::U64>,
	pub hostname : FixedSizeString<typenum::U64>,
	pub sw_if_index : InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_set_responder_hostname_reply_e8d4e804)]
pub struct Ikev2SetResponderHostnameReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_set_ike_transforms_076d7378)]
pub struct Ikev2SetIkeTransforms {
	pub client_index : u32,
	pub context : u32,
	pub name : FixedSizeString<typenum::U64>,
	pub tr : Ikev2IkeTransforms,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_set_ike_transforms_reply_e8d4e804)]
pub struct Ikev2SetIkeTransformsReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_set_esp_transforms_a63dc205)]
pub struct Ikev2SetEspTransforms {
	pub client_index : u32,
	pub context : u32,
	pub name : FixedSizeString<typenum::U64>,
	pub tr : Ikev2EspTransforms,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_set_esp_transforms_reply_e8d4e804)]
pub struct Ikev2SetEspTransformsReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_set_sa_lifetime_7039feaa)]
pub struct Ikev2SetSaLifetime {
	pub client_index : u32,
	pub context : u32,
	pub name : FixedSizeString<typenum::U64>,
	pub lifetime : u64,
	pub lifetime_jitter : u32,
	pub handover : u32,
	pub lifetime_maxdata : u64,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_set_sa_lifetime_reply_e8d4e804)]
pub struct Ikev2SetSaLifetimeReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_initiate_sa_init_ebf79a66)]
pub struct Ikev2InitiateSaInit {
	pub client_index : u32,
	pub context : u32,
	pub name : FixedSizeString<typenum::U64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_initiate_sa_init_reply_e8d4e804)]
pub struct Ikev2InitiateSaInitReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_initiate_del_ike_sa_8d125bdd)]
pub struct Ikev2InitiateDelIkeSa {
	pub client_index : u32,
	pub context : u32,
	pub ispi : u64,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_initiate_del_ike_sa_reply_e8d4e804)]
pub struct Ikev2InitiateDelIkeSaReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_initiate_del_child_sa_7f004d2e)]
pub struct Ikev2InitiateDelChildSa {
	pub client_index : u32,
	pub context : u32,
	pub ispi : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_initiate_del_child_sa_reply_e8d4e804)]
pub struct Ikev2InitiateDelChildSaReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_initiate_rekey_child_sa_7f004d2e)]
pub struct Ikev2InitiateRekeyChildSa {
	pub client_index : u32,
	pub context : u32,
	pub ispi : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_initiate_rekey_child_sa_reply_e8d4e804)]
pub struct Ikev2InitiateRekeyChildSaReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_set_udp_encap_ebf79a66)]
pub struct Ikev2ProfileSetUdpEncap {
	pub client_index : u32,
	pub context : u32,
	pub name : FixedSizeString<typenum::U64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_set_udp_encap_reply_e8d4e804)]
pub struct Ikev2ProfileSetUdpEncapReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_set_ipsec_udp_port_615ce758)]
pub struct Ikev2ProfileSetIpsecUdpPort {
	pub client_index : u32,
	pub context : u32,
	pub is_set : u8,
	pub port : u16,
	pub name : FixedSizeString<typenum::U64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_set_ipsec_udp_port_reply_e8d4e804)]
pub struct Ikev2ProfileSetIpsecUdpPortReply {
	pub context : u32,
	pub retval : i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_set_liveness_6bdf4d65)]
pub struct Ikev2ProfileSetLiveness {
	pub client_index : u32,
	pub context : u32,
	pub period : u32,
	pub max_retries : u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(ikev2_profile_set_liveness_reply_e8d4e804)]
pub struct Ikev2ProfileSetLivenessReply {
	pub context : u32,
	pub retval : i32,
}
