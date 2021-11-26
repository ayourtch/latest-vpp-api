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
use crate::pci_types::*; 
use crate::interface_types::*; 
use crate::ethernet_types::*; 
#[derive(Debug,Serialize, Deserialize, Clone, Copy)] 
pub enum VirtioFlags { 
	 VIRTIO_API_FLAG_GSO=1, 
	 VIRTIO_API_FLAG_CSUM_OFFLOAD=2, 
	 VIRTIO_API_FLAG_GRO_COALESCE=4, 
	 VIRTIO_API_FLAG_PACKED=8, 
	 VIRTIO_API_FLAG_IN_ORDER=16, 
	 VIRTIO_API_FLAG_BUFFERING=32, 
} 
impl Default for VirtioFlags { 
	fn default() -> Self { VirtioFlags::VIRTIO_API_FLAG_GSO }
}
impl AsEnumFlag for VirtioFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 1 => VirtioFlags::VIRTIO_API_FLAG_GSO, 
			 2 => VirtioFlags::VIRTIO_API_FLAG_CSUM_OFFLOAD, 
			 4 => VirtioFlags::VIRTIO_API_FLAG_GRO_COALESCE, 
			 8 => VirtioFlags::VIRTIO_API_FLAG_PACKED, 
			 16 => VirtioFlags::VIRTIO_API_FLAG_IN_ORDER, 
			 32 => VirtioFlags::VIRTIO_API_FLAG_BUFFERING, 
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 32 as u32
	}
}
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(virtio_pci_create_1944f8db)] 
pub struct VirtioPciCreate { 
	pub client_index : u32, 
	pub context : u32, 
	pub pci_addr : PciAddress, 
	pub use_random_mac : bool, 
	pub mac_address : MacAddress, 
	pub gso_enabled : bool, 
	pub checksum_offload_enabled : bool, 
	pub features : u64, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(virtio_pci_create_reply_5383d31f)] 
pub struct VirtioPciCreateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(virtio_pci_create_v2_5d096e1a)] 
pub struct VirtioPciCreateV2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub pci_addr : PciAddress, 
	pub use_random_mac : bool, 
	pub mac_address : MacAddress, 
	 pub virtio_flags : EnumFlag<VirtioFlags>, 
	pub features : u64, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(virtio_pci_create_v2_reply_5383d31f)] 
pub struct VirtioPciCreateV2Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(virtio_pci_delete_f9e6675e)] 
pub struct VirtioPciDelete { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(virtio_pci_delete_reply_e8d4e804)] 
pub struct VirtioPciDeleteReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sw_interface_virtio_pci_dump_51077d14)] 
pub struct SwInterfaceVirtioPciDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, VppMessage)] 
#[message_name_and_crc(sw_interface_virtio_pci_details_6ca9c167)] 
pub struct SwInterfaceVirtioPciDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub pci_addr : PciAddress, 
	pub mac_addr : MacAddress, 
	pub tx_ring_sz : u16, 
	pub rx_ring_sz : u16, 
	pub features : u64, 
} 
