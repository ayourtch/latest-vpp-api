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
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)] 
pub enum VirtioNetFeaturesFirst32 { 
	 VIRTIO_NET_F_API_CSUM=1, 
	 VIRTIO_NET_F_API_GUEST_CSUM=2, 
	 VIRTIO_NET_F_API_GUEST_TSO4=128, 
	 VIRTIO_NET_F_API_GUEST_TSO6=256, 
	 VIRTIO_NET_F_API_GUEST_UFO=1024, 
	 VIRTIO_NET_F_API_HOST_TSO4=2048, 
	 VIRTIO_NET_F_API_HOST_TSO6=4096, 
	 VIRTIO_NET_F_API_HOST_UFO=16384, 
	 VIRTIO_NET_F_API_MRG_RXBUF=32768, 
	 VIRTIO_NET_F_API_CTRL_VQ=131072, 
	 VIRTIO_NET_F_API_GUEST_ANNOUNCE=2097152, 
	 VIRTIO_NET_F_API_MQ=4194304, 
	 VHOST_F_API_LOG_ALL=67108864, 
	 VIRTIO_F_API_ANY_LAYOUT=134217728, 
	 VIRTIO_F_API_INDIRECT_DESC=268435456, 
	 VHOST_USER_F_API_PROTOCOL_FEATURES=1073741824, 
} 
impl Default for VirtioNetFeaturesFirst32 { 
	fn default() -> Self { VirtioNetFeaturesFirst32::VIRTIO_NET_F_API_CSUM }
}
impl AsEnumFlag for VirtioNetFeaturesFirst32 {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 1 => VirtioNetFeaturesFirst32::VIRTIO_NET_F_API_CSUM, 
			 2 => VirtioNetFeaturesFirst32::VIRTIO_NET_F_API_GUEST_CSUM, 
			 128 => VirtioNetFeaturesFirst32::VIRTIO_NET_F_API_GUEST_TSO4, 
			 256 => VirtioNetFeaturesFirst32::VIRTIO_NET_F_API_GUEST_TSO6, 
			 1024 => VirtioNetFeaturesFirst32::VIRTIO_NET_F_API_GUEST_UFO, 
			 2048 => VirtioNetFeaturesFirst32::VIRTIO_NET_F_API_HOST_TSO4, 
			 4096 => VirtioNetFeaturesFirst32::VIRTIO_NET_F_API_HOST_TSO6, 
			 16384 => VirtioNetFeaturesFirst32::VIRTIO_NET_F_API_HOST_UFO, 
			 32768 => VirtioNetFeaturesFirst32::VIRTIO_NET_F_API_MRG_RXBUF, 
			 131072 => VirtioNetFeaturesFirst32::VIRTIO_NET_F_API_CTRL_VQ, 
			 2097152 => VirtioNetFeaturesFirst32::VIRTIO_NET_F_API_GUEST_ANNOUNCE, 
			 4194304 => VirtioNetFeaturesFirst32::VIRTIO_NET_F_API_MQ, 
			 67108864 => VirtioNetFeaturesFirst32::VHOST_F_API_LOG_ALL, 
			 134217728 => VirtioNetFeaturesFirst32::VIRTIO_F_API_ANY_LAYOUT, 
			 268435456 => VirtioNetFeaturesFirst32::VIRTIO_F_API_INDIRECT_DESC, 
			 1073741824 => VirtioNetFeaturesFirst32::VHOST_USER_F_API_PROTOCOL_FEATURES, 
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 32 as u32
	}
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum VirtioNetFeaturesLast32 { 
	 VIRTIO_F_API_VERSION_1=1, 
} 
impl Default for VirtioNetFeaturesLast32 { 
	fn default() -> Self { VirtioNetFeaturesLast32::VIRTIO_F_API_VERSION_1 }
}
