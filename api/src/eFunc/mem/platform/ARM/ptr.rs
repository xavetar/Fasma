/*
 * ╔═════════════════════════════════════════════════════════════════════════════════════╗
 * ║                                 ANTI-VIRUS LICENSE                                  ║
 * ║                                                                                     ║
 * ║                          Code Shielded from Viral Threats                           ║
 * ╟─────────────────────────────────────────────────────────────────────────────────────╢
 * ║  Copyright Notice                                                                   ║
 * ║                                                                                     ║
 * ║  Copyright (c) 2025 Stanislav Mikhailov (xavetar)                                   ║
 * ╟─────────────────────────────────────────────────────────────────────────────────────╢
 * ║  License Terms                                                                      ║
 * ║                                                                                     ║
 * ║  Licensed under the Anti-Virus License Agreement.                                   ║
 * ║  This file may not be used except in compliance with the License.                   ║
 * ║                                                                                     ║
 * ║  The License is included within the project distribution.                           ║
 * ║  If this file is obtained without the accompanying License, it must be deleted.     ║
 * ╟─────────────────────────────────────────────────────────────────────────────────────╢
 * ║  Warranty Disclaimer                                                                ║
 * ║                                                                                     ║
 * ║  Unless required by applicable law or agreed to in writing, software                ║
 * ║  distributed under the License is distributed on an "AS IS" BASIS, WITHOUT          ║
 * ║  WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.                   ║
 * ║                                                                                     ║
 * ║  See the License for the specific language governing permissions and limitations    ║
 * ║  under the License.                                                                 ║
 * ╚═════════════════════════════════════════════════════════════════════════════════════╝
 */

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
use crate::{
    eSIMD::{
        rolled::{
            extract::{
                alvext_u8, alvextq_u8,
                alvext_u16, alvextq_u16,
                alvext_u32, alvextq_u32,
                alvext_u64, alvextq_u64
            }
        }
    }
};

#[cfg(all(target_arch = "arm", target_feature = "v7", target_feature = "neon"))]
use core::{
    arch::{
        arm::{
            uint8x8_t, uint8x16_t,
            uint16x4_t, uint16x8_t,
            uint32x2_t, uint32x4_t,
            uint64x1_t, uint64x2_t,
            vld1_u8, vld1q_u8,
            vld1_u16, vld1q_u16,
            vld1_u32, vld1q_u32,
            vld1_u64, vld1q_u64,
            vst1_u8, vst1q_u8,
            vst1_u16, vst1q_u16,
            vst1_u32, vst1q_u32,
            vst1_u64, vst1q_u64
        }
    }
};

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use core::{
    arch::{
        aarch64::{
            uint8x8_t, uint8x16_t,
            uint16x4_t, uint16x8_t,
            uint32x2_t, uint32x4_t,
            uint64x1_t, uint64x2_t,
            vld1_u8, vld1q_u8,
            vld1_u16, vld1q_u16,
            vld1_u32, vld1q_u32,
            vld1_u64, vld1q_u64,
            vst1_u8, vst1q_u8,
            vst1_u16, vst1q_u16,
            vst1_u32, vst1q_u32,
            vst1_u64, vst1q_u64
        }
    }
};

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
macro_rules! align_remainder_8 {
    (
        $functor_name:ident,
        $pointer_type:ty,
        $substitution_type:ty,
        $load_function:ident,
        $store_function:ident,
        $align_function:ident,
        $total_register_items:expr
    ) => {
        pub unsafe fn $functor_name(data_ptr: *const $pointer_type, substitution_data: $substitution_type, length: usize) -> $substitution_type {
            let remainder: usize = length % $total_register_items;
            
            if remainder != 0_usize {
                let (mut index, mut buffer): (usize, $substitution_type) = (0_usize, substitution_data);

                if length >= $total_register_items {
                    $store_function(buffer.as_mut_ptr(), $align_function($load_function(data_ptr.add(length - $total_register_items)), $load_function(substitution_data.as_ptr()), $total_register_items - remainder));
                } else {
                    while index < length { buffer[index] = *data_ptr.add(index); index += 1_usize; }
                }
    
                return buffer;
            } else { 
                return substitution_data;
            }
        }
    };
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
macro_rules! align_remainder_16 {
    (
        $functor_name:ident,
        $pointer_type:ty,
        $substitution_type:ty,
        $load_function:ident,
        $load_half_function:ident,
        $store_function:ident,
        $store_half_function:ident,
        $align_function:ident,
        $total_register_items:expr,
        $half_register_items:expr
    ) => {
        pub unsafe fn $functor_name(data_ptr: *const $pointer_type, substitution_data: $substitution_type, length: usize) -> $substitution_type {
            let remainder: usize = length % $total_register_items;
            
            if remainder != 0_usize {
                let (mut index, mut buffer): (usize, $substitution_type) = (0_usize, substitution_data);

                if length >= $total_register_items {
                    $store_function(buffer.as_mut_ptr(), $align_function($load_function(data_ptr.add(length - $total_register_items)), $load_function(substitution_data.as_ptr()), $total_register_items - remainder));
                } else {
                    if remainder >= $half_register_items {
                        $store_half_function(buffer.as_mut_ptr(), $load_half_function(data_ptr)); index += $half_register_items;
                    }
    
                    while index < length { buffer[index] = *data_ptr.add(index); index += 1_usize; }
                }
                
                return buffer;
            } else { 
                return substitution_data;
            }
        }
    };
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
align_remainder_8!(align_remainder_8x8, u8, [u8; 8_usize], vld1_u8, vst1_u8, alvext_u8, 8_usize);
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
align_remainder_8!(align_remainder_16x4, u16, [u16; 4_usize], vld1_u16, vst1_u16, alvext_u16, 4_usize);
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
align_remainder_8!(align_remainder_32x2, u32, [u32; 2_usize], vld1_u32, vst1_u32, alvext_u32, 2_usize);
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
align_remainder_8!(align_remainder_64x1, u64, [u64; 1_usize], vld1_u64, vst1_u64, alvext_u64, 1_usize);

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
align_remainder_16!(align_remainder_8x16, u8, [u8; 16_usize], vld1q_u8, vld1_u8, vst1q_u8, vst1_u8, alvextq_u8, 16_usize, 8_usize);
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
align_remainder_16!(align_remainder_16x8, u16, [u16; 8_usize], vld1q_u16, vld1_u16, vst1q_u16, vst1_u16, alvextq_u16, 8_usize, 4_usize);
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
align_remainder_16!(align_remainder_32x4, u32, [u32; 4_usize], vld1q_u32, vld1_u32, vst1q_u32, vst1_u32, alvextq_u32, 4_usize, 2_usize);
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
align_remainder_16!(align_remainder_64x2, u64, [u64; 2_usize], vld1q_u64, vld1_u64, vst1q_u64, vst1_u64, alvextq_u64, 2_usize, 1_usize);
