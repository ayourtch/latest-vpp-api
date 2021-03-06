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
use crate::acl_types::*;
use crate::interface_types::*;
use crate::ethernet_types::*;
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_plugin_get_version_51077d14)]
pub struct AclPluginGetVersion {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_plugin_get_version_reply_9b32cf86)]
pub struct AclPluginGetVersionReply {
	pub context: u32,
	pub major: u32,
	pub minor: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_plugin_control_ping_51077d14)]
pub struct AclPluginControlPing {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_plugin_control_ping_reply_f6b0b8ca)]
pub struct AclPluginControlPingReply {
	pub context: u32,
	pub retval: i32,
	pub client_index: u32,
	pub vpe_pid: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_plugin_get_conn_table_max_entries_51077d14)]
pub struct AclPluginGetConnTableMaxEntries {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_plugin_get_conn_table_max_entries_reply_7a096d3d)]
pub struct AclPluginGetConnTableMaxEntriesReply {
	pub context: u32,
	pub conn_table_max_entries: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_add_replace_ee5c2f18)]
pub struct AclAddReplace {
	pub client_index: u32,
	pub context: u32,
	pub acl_index: u32,
	pub tag: FixedSizeString<typenum::U64>,
	pub count: u32,
	pub r: VariableSizeArray<AclRule>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_add_replace_reply_ac407b0c)]
pub struct AclAddReplaceReply {
	pub context: u32,
	pub acl_index: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_del_ef34fea4)]
pub struct AclDel {
	pub client_index: u32,
	pub context: u32,
	pub acl_index: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_del_reply_e8d4e804)]
pub struct AclDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_interface_add_del_4b54bebd)]
pub struct AclInterfaceAddDel {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub is_input: bool,
	pub sw_if_index: InterfaceIndex,
	pub acl_index: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_interface_add_del_reply_e8d4e804)]
pub struct AclInterfaceAddDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_interface_set_acl_list_473982bd)]
pub struct AclInterfaceSetAclList {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub count: u8,
	pub n_input: u8,
	pub acls: VariableSizeArray<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_interface_set_acl_list_reply_e8d4e804)]
pub struct AclInterfaceSetAclListReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_dump_ef34fea4)]
pub struct AclDump {
	pub client_index: u32,
	pub context: u32,
	pub acl_index: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_details_95babae0)]
pub struct AclDetails {
	pub context: u32,
	pub acl_index: u32,
	pub tag: FixedSizeString<typenum::U64>,
	pub count: u32,
	pub r: VariableSizeArray<AclRule>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_interface_list_dump_f9e6675e)]
pub struct AclInterfaceListDump {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_interface_list_details_e695d256)]
pub struct AclInterfaceListDetails {
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub count: u8,
	pub n_input: u8,
	pub acls: VariableSizeArray<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(macip_acl_add_ce6fbad0)]
pub struct MacipAclAdd {
	pub client_index: u32,
	pub context: u32,
	pub tag: FixedSizeString<typenum::U64>,
	pub count: u32,
	pub r: VariableSizeArray<MacipAclRule>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(macip_acl_add_reply_ac407b0c)]
pub struct MacipAclAddReply {
	pub context: u32,
	pub acl_index: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(macip_acl_add_replace_2a461dd4)]
pub struct MacipAclAddReplace {
	pub client_index: u32,
	pub context: u32,
	pub acl_index: u32,
	pub tag: FixedSizeString<typenum::U64>,
	pub count: u32,
	pub r: VariableSizeArray<MacipAclRule>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(macip_acl_add_replace_reply_ac407b0c)]
pub struct MacipAclAddReplaceReply {
	pub context: u32,
	pub acl_index: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(macip_acl_del_ef34fea4)]
pub struct MacipAclDel {
	pub client_index: u32,
	pub context: u32,
	pub acl_index: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(macip_acl_del_reply_e8d4e804)]
pub struct MacipAclDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(macip_acl_interface_add_del_4b8690b1)]
pub struct MacipAclInterfaceAddDel {
	pub client_index: u32,
	pub context: u32,
	pub is_add: bool,
	pub sw_if_index: InterfaceIndex,
	pub acl_index: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(macip_acl_interface_add_del_reply_e8d4e804)]
pub struct MacipAclInterfaceAddDelReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(macip_acl_dump_ef34fea4)]
pub struct MacipAclDump {
	pub client_index: u32,
	pub context: u32,
	pub acl_index: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(macip_acl_details_27135b59)]
pub struct MacipAclDetails {
	pub context: u32,
	pub acl_index: u32,
	pub tag: FixedSizeString<typenum::U64>,
	pub count: u32,
	pub r: VariableSizeArray<MacipAclRule>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(macip_acl_interface_get_51077d14)]
pub struct MacipAclInterfaceGet {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(macip_acl_interface_get_reply_accf9b05)]
pub struct MacipAclInterfaceGetReply {
	pub context: u32,
	pub count: u32,
	pub acls: VariableSizeArray<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(macip_acl_interface_list_dump_f9e6675e)]
pub struct MacipAclInterfaceListDump {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(macip_acl_interface_list_details_a0c5d56d)]
pub struct MacipAclInterfaceListDetails {
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub count: u8,
	pub acls: VariableSizeArray<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_interface_set_etype_whitelist_3f5c2d2d)]
pub struct AclInterfaceSetEtypeWhitelist {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub count: u8,
	pub n_input: u8,
	pub whitelist: VariableSizeArray<u16>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_interface_set_etype_whitelist_reply_e8d4e804)]
pub struct AclInterfaceSetEtypeWhitelistReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_interface_etype_whitelist_dump_f9e6675e)]
pub struct AclInterfaceEtypeWhitelistDump {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_interface_etype_whitelist_details_cc2bfded)]
pub struct AclInterfaceEtypeWhitelistDetails {
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
	pub count: u8,
	pub n_input: u8,
	pub whitelist: VariableSizeArray<u16>,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_stats_intf_counters_enable_b3e225d2)]
pub struct AclStatsIntfCountersEnable {
	pub client_index: u32,
	pub context: u32,
	pub enable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_stats_intf_counters_enable_reply_e8d4e804)]
pub struct AclStatsIntfCountersEnableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_plugin_use_hash_lookup_set_b3e225d2)]
pub struct AclPluginUseHashLookupSet {
	pub client_index: u32,
	pub context: u32,
	pub enable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_plugin_use_hash_lookup_set_reply_e8d4e804)]
pub struct AclPluginUseHashLookupSetReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_plugin_use_hash_lookup_get_51077d14)]
pub struct AclPluginUseHashLookupGet {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(acl_plugin_use_hash_lookup_get_reply_5392ad31)]
pub struct AclPluginUseHashLookupGetReply {
	pub context: u32,
	pub enable: bool,
}
