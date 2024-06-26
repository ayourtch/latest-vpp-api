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
use crate::tunnel_types::*;
use crate::interface_types::*;
// Implementation for key
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Key {
	pub length: u8,
	pub data: FixedSizeArray<u8, typenum::U128>,
}
// Implementation for ipsec_spd_entry
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IpsecSpdEntry {
	pub spd_id: u32,
	pub priority: i32,
	pub is_outbound: bool,
	pub sa_id: u32,
	pub policy: IpsecSpdAction,
	pub protocol: u8,
	pub remote_address_start: Address,
	pub remote_address_stop: Address,
	pub local_address_start: Address,
	pub local_address_stop: Address,
	pub remote_port_start: u16,
	pub remote_port_stop: u16,
	pub local_port_start: u16,
	pub local_port_stop: u16,
}
// Implementation for ipsec_spd_entry_v2
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IpsecSpdEntryV2 {
	pub spd_id: u32,
	pub priority: i32,
	pub is_outbound: bool,
	pub sa_id: u32,
	pub policy: IpsecSpdAction,
	pub protocol: u8,
	pub remote_address_start: Address,
	pub remote_address_stop: Address,
	pub local_address_start: Address,
	pub local_address_stop: Address,
	pub remote_port_start: u16,
	pub remote_port_stop: u16,
	pub local_port_start: u16,
	pub local_port_stop: u16,
}
// Implementation for ipsec_sad_entry
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IpsecSadEntry {
	pub sad_id: u32,
	pub spi: u32,
	pub protocol: IpsecProto,
	pub crypto_algorithm: IpsecCryptoAlg,
	pub crypto_key: Key,
	pub integrity_algorithm: IpsecIntegAlg,
	pub integrity_key: Key,
	pub flags: EnumFlag<IpsecSadFlags>,
	pub tunnel_src: Address,
	pub tunnel_dst: Address,
	pub tx_table_id: u32,
	pub salt: u32,
	pub udp_src_port: u16,
	pub udp_dst_port: u16,
}
// Implementation for ipsec_sad_entry_v2
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IpsecSadEntryV2 {
	pub sad_id: u32,
	pub spi: u32,
	pub protocol: IpsecProto,
	pub crypto_algorithm: IpsecCryptoAlg,
	pub crypto_key: Key,
	pub integrity_algorithm: IpsecIntegAlg,
	pub integrity_key: Key,
	pub flags: EnumFlag<IpsecSadFlags>,
	pub tunnel_src: Address,
	pub tunnel_dst: Address,
	pub tunnel_flags: EnumFlag<TunnelEncapDecapFlags>,
	pub dscp: IpDscp,
	pub tx_table_id: u32,
	pub salt: u32,
	pub udp_src_port: u16,
	pub udp_dst_port: u16,
}
// Implementation for ipsec_sad_entry_v3
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IpsecSadEntryV3 {
	pub sad_id: u32,
	pub spi: u32,
	pub protocol: IpsecProto,
	pub crypto_algorithm: IpsecCryptoAlg,
	pub crypto_key: Key,
	pub integrity_algorithm: IpsecIntegAlg,
	pub integrity_key: Key,
	pub flags: EnumFlag<IpsecSadFlags>,
	pub tunnel: Tunnel,
	pub salt: u32,
	pub udp_src_port: u16,
	pub udp_dst_port: u16,
}
// Implementation for ipsec_sad_entry_v4
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IpsecSadEntryV4 {
	pub sad_id: u32,
	pub spi: u32,
	pub protocol: IpsecProto,
	pub crypto_algorithm: IpsecCryptoAlg,
	pub crypto_key: Key,
	pub integrity_algorithm: IpsecIntegAlg,
	pub integrity_key: Key,
	pub flags: EnumFlag<IpsecSadFlags>,
	pub tunnel: Tunnel,
	pub salt: u32,
	pub udp_src_port: u16,
	pub udp_dst_port: u16,
	pub anti_replay_window_size: u32,
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum IpsecCryptoAlg {
	 IPSEC_API_CRYPTO_ALG_NONE=0,
	 IPSEC_API_CRYPTO_ALG_AES_CBC_128=1,
	 IPSEC_API_CRYPTO_ALG_AES_CBC_192=2,
	 IPSEC_API_CRYPTO_ALG_AES_CBC_256=3,
	 IPSEC_API_CRYPTO_ALG_AES_CTR_128=4,
	 IPSEC_API_CRYPTO_ALG_AES_CTR_192=5,
	 IPSEC_API_CRYPTO_ALG_AES_CTR_256=6,
	 IPSEC_API_CRYPTO_ALG_AES_GCM_128=7,
	 IPSEC_API_CRYPTO_ALG_AES_GCM_192=8,
	 IPSEC_API_CRYPTO_ALG_AES_GCM_256=9,
	 IPSEC_API_CRYPTO_ALG_DES_CBC=10,
	 IPSEC_API_CRYPTO_ALG_3DES_CBC=11,
	 IPSEC_API_CRYPTO_ALG_CHACHA20_POLY1305=12,
	 IPSEC_API_CRYPTO_ALG_AES_NULL_GMAC_128=13,
	 IPSEC_API_CRYPTO_ALG_AES_NULL_GMAC_192=14,
	 IPSEC_API_CRYPTO_ALG_AES_NULL_GMAC_256=15,
}
impl Default for IpsecCryptoAlg {
	fn default() -> Self { IpsecCryptoAlg::IPSEC_API_CRYPTO_ALG_NONE }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum IpsecIntegAlg {
	 IPSEC_API_INTEG_ALG_NONE=0,
	 IPSEC_API_INTEG_ALG_MD5_96=1,
	 IPSEC_API_INTEG_ALG_SHA1_96=2,
	 IPSEC_API_INTEG_ALG_SHA_256_96=3,
	 IPSEC_API_INTEG_ALG_SHA_256_128=4,
	 IPSEC_API_INTEG_ALG_SHA_384_192=5,
	 IPSEC_API_INTEG_ALG_SHA_512_256=6,
}
impl Default for IpsecIntegAlg {
	fn default() -> Self { IpsecIntegAlg::IPSEC_API_INTEG_ALG_NONE }
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum IpsecSadFlags {
	 IPSEC_API_SAD_FLAG_NONE=0,
	 IPSEC_API_SAD_FLAG_USE_ESN=1,
	 IPSEC_API_SAD_FLAG_USE_ANTI_REPLAY=2,
	 IPSEC_API_SAD_FLAG_IS_TUNNEL=4,
	 IPSEC_API_SAD_FLAG_IS_TUNNEL_V6=8,
	 IPSEC_API_SAD_FLAG_UDP_ENCAP=16,
	 IPSEC_API_SAD_FLAG_IS_INBOUND=64,
	 IPSEC_API_SAD_FLAG_ASYNC=128,
}
impl Default for IpsecSadFlags {
	fn default() -> Self { IpsecSadFlags::IPSEC_API_SAD_FLAG_NONE }
}
impl AsEnumFlag for IpsecSadFlags {
	 fn as_u32(data: &Self) -> u32{
		 *data as u32
	 }
	 fn from_u32(data: u32) -> Self{
		 match data{
			 0 => IpsecSadFlags::IPSEC_API_SAD_FLAG_NONE,
			 1 => IpsecSadFlags::IPSEC_API_SAD_FLAG_USE_ESN,
			 2 => IpsecSadFlags::IPSEC_API_SAD_FLAG_USE_ANTI_REPLAY,
			 4 => IpsecSadFlags::IPSEC_API_SAD_FLAG_IS_TUNNEL,
			 8 => IpsecSadFlags::IPSEC_API_SAD_FLAG_IS_TUNNEL_V6,
			 16 => IpsecSadFlags::IPSEC_API_SAD_FLAG_UDP_ENCAP,
			 64 => IpsecSadFlags::IPSEC_API_SAD_FLAG_IS_INBOUND,
			 128 => IpsecSadFlags::IPSEC_API_SAD_FLAG_ASYNC,
			_ => panic!("Invalid Enum Descriminant")
		 }
	 }
	 fn size_of_enum_flag() -> u32{
		 32 as u32
	}
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum IpsecProto {
	 IPSEC_API_PROTO_ESP=50,
	 IPSEC_API_PROTO_AH=51,
}
impl Default for IpsecProto {
	fn default() -> Self { IpsecProto::IPSEC_API_PROTO_ESP }
}
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum IpsecSpdAction {
	 IPSEC_API_SPD_ACTION_BYPASS=0,
	 IPSEC_API_SPD_ACTION_DISCARD=1,
	 IPSEC_API_SPD_ACTION_RESOLVE=2,
	 IPSEC_API_SPD_ACTION_PROTECT=3,
}
impl Default for IpsecSpdAction {
	fn default() -> Self { IpsecSpdAction::IPSEC_API_SPD_ACTION_BYPASS }
}
