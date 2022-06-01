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
// Implementation for qos_store
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QosStore {
	pub sw_if_index: InterfaceIndex,
	pub input_source: QosSource,
	pub value: u8,
}
// Implementation for qos_record
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QosRecord {
	pub sw_if_index: InterfaceIndex,
	pub input_source: QosSource,
}
// Implementation for qos_egress_map_row
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QosEgressMapRow {
	pub outputs: FixedSizeArray<u8, typenum::U256>,
}
// Implementation for qos_egress_map
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QosEgressMap {
	pub id: u32,
	pub rows: FixedSizeArray<QosEgressMapRow, typenum::U4>,
}
// Implementation for qos_mark
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QosMark {
	pub sw_if_index: u32,
	pub map_id: u32,
	pub output_source: QosSource,
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum QosSource {
	 QOS_API_SOURCE_EXT=0,
	 QOS_API_SOURCE_VLAN=1,
	 QOS_API_SOURCE_MPLS=2,
	 QOS_API_SOURCE_IP=3,
}
impl Default for QosSource {
	fn default() -> Self { QosSource::QOS_API_SOURCE_EXT }
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_store_enable_disable_f3abcc8b)]
pub struct QosStoreEnableDisable {
	pub client_index: u32,
	pub context: u32,
	pub enable: bool,
	pub store: QosStore,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_store_enable_disable_reply_e8d4e804)]
pub struct QosStoreEnableDisableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_store_dump_51077d14)]
pub struct QosStoreDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_store_details_3ee0aad7)]
pub struct QosStoreDetails {
	pub context: u32,
	pub store: QosStore,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_record_enable_disable_2f1a4a38)]
pub struct QosRecordEnableDisable {
	pub client_index: u32,
	pub context: u32,
	pub enable: bool,
	pub record: QosRecord,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_record_enable_disable_reply_e8d4e804)]
pub struct QosRecordEnableDisableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_record_dump_51077d14)]
pub struct QosRecordDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_record_details_a425d4d3)]
pub struct QosRecordDetails {
	pub context: u32,
	pub record: QosRecord,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_egress_map_update_6d1c065f)]
pub struct QosEgressMapUpdate {
	pub client_index: u32,
	pub context: u32,
	pub map: QosEgressMap,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_egress_map_update_reply_e8d4e804)]
pub struct QosEgressMapUpdateReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_egress_map_delete_3a91bde5)]
pub struct QosEgressMapDelete {
	pub client_index: u32,
	pub context: u32,
	pub id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_egress_map_delete_reply_e8d4e804)]
pub struct QosEgressMapDeleteReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_egress_map_dump_51077d14)]
pub struct QosEgressMapDump {
	pub client_index: u32,
	pub context: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_egress_map_details_46c5653c)]
pub struct QosEgressMapDetails {
	pub context: u32,
	pub map: QosEgressMap,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_mark_enable_disable_1a010f74)]
pub struct QosMarkEnableDisable {
	pub client_index: u32,
	pub context: u32,
	pub enable: bool,
	pub mark: QosMark,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_mark_enable_disable_reply_e8d4e804)]
pub struct QosMarkEnableDisableReply {
	pub context: u32,
	pub retval: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_mark_dump_f9e6675e)]
pub struct QosMarkDump {
	pub client_index: u32,
	pub context: u32,
	pub sw_if_index: InterfaceIndex,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_mark_details_89fe81a9)]
pub struct QosMarkDetails {
	pub context: u32,
	pub mark: QosMark,
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)]
#[message_name_and_crc(qos_mark_details_reply_e8d4e804)]
pub struct QosMarkDetailsReply {
	pub context: u32,
	pub retval: i32,
}
