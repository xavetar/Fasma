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
use Fasma::{
    eSIMD::{
        rolled::{
            extract::{
                alvext_u8, arvext_u8,
                alvextq_u8, arvextq_u8,
                alvext_u16, arvext_u16,
                alvextq_u16, arvextq_u16,
                alvext_u32, arvext_u32,
                alvextq_u32, arvextq_u32,
                alvext_u64, arvext_u64,
                alvextq_u64, arvextq_u64
            }
        }
    }
};

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
use core::{
    mem::{
        transmute
    },
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
            vld1_u64, vld1q_u64
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
        }
    }
};


#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[test]
fn alvext_u8_test() {
    let arr_v: [u8; 8_usize] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let arr_a: [u8; 8_usize] = [0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];

    let v: uint8x8_t = unsafe {
        vld1_u8(arr_v.as_ptr())
    };
    let a: uint8x8_t = unsafe {
        vld1_u8(arr_a.as_ptr())
    };

    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(alvext_u8(v, a, 1)) }, [0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(alvext_u8(v, a, 2)) }, [0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(alvext_u8(v, a, 3)) }, [0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(alvext_u8(v, a, 4)) }, [0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(alvext_u8(v, a, 5)) }, [0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(alvext_u8(v, a, 6)) }, [0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(alvext_u8(v, a, 7)) }, [0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(alvext_u8(v, a, 8)) }, [0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(alvext_u8(v, a, 9)) }, [0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(alvext_u8(v, a, 10)) }, [0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(alvext_u8(v, a, 11)) }, [0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(alvext_u8(v, a, 12)) }, [0x0d, 0x0e, 0x0f, 0x10, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(alvext_u8(v, a, 13)) }, [0x0e, 0x0f, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(alvext_u8(v, a, 14)) }, [0x0f, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(alvext_u8(v, a, 15)) }, [0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(alvext_u8(v, a, 16)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[test]
fn arvext_u8_test() {
    let arr_v: [u8; 8_usize] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let arr_a: [u8; 8_usize] = [0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];

    let v: uint8x8_t = unsafe {
        vld1_u8(arr_v.as_ptr())
    };
    let a: uint8x8_t = unsafe {
        vld1_u8(arr_a.as_ptr())
    };

    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(arvext_u8(v, a, 1)) }, [0x10, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(arvext_u8(v, a, 2)) }, [0x0f, 0x10, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(arvext_u8(v, a, 3)) }, [0x0e, 0x0f, 0x10, 0x01, 0x02, 0x03, 0x04, 0x05]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(arvext_u8(v, a, 4)) }, [0x0d, 0x0e, 0x0f, 0x10, 0x01, 0x02, 0x03, 0x04]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(arvext_u8(v, a, 5)) }, [0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x01, 0x02, 0x03]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(arvext_u8(v, a, 6)) }, [0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x01, 0x02]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(arvext_u8(v, a, 7)) }, [0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x01]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(arvext_u8(v, a, 8)) }, [0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(arvext_u8(v, a, 9)) }, [0x00, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(arvext_u8(v, a, 10)) }, [0x00, 0x00, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(arvext_u8(v, a, 11)) }, [0x00, 0x00, 0x00, 0x09, 0x0a, 0x0b, 0x0c, 0x0d]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(arvext_u8(v, a, 12)) }, [0x00, 0x00, 0x00, 0x00, 0x09, 0x0a, 0x0b, 0x0c]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(arvext_u8(v, a, 13)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x0a, 0x0b]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(arvext_u8(v, a, 14)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x0a]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(arvext_u8(v, a, 15)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09]);
    assert_eq!(unsafe { transmute::<uint8x8_t, [u8; 8_usize]>(arvext_u8(v, a, 16)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[test]
fn alvextq_u8_test() {
    let arr_v: [u8; 16_usize] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
    let arr_a: [u8; 16_usize] = [0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20];

    let v: uint8x16_t = unsafe {
        vld1q_u8(arr_v.as_ptr())
    };
    let a: uint8x16_t = unsafe {
        vld1q_u8(arr_a.as_ptr())
    };

    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 1)) }, [0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 2)) }, [0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 3)) }, [0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 4)) }, [0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 5)) }, [0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 6)) }, [0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 7)) }, [0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 8)) }, [0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 9)) }, [0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 10)) }, [0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 11)) }, [0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 12)) }, [0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 13)) }, [0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 14)) }, [0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 15)) }, [0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 16)) }, [0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 17)) }, [0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 18)) }, [0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 19)) }, [0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 20)) }, [0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 21)) }, [0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 22)) }, [0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 23)) }, [0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 24)) }, [0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 25)) }, [0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 26)) }, [0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 27)) }, [0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 28)) }, [0x1d, 0x1e, 0x1f, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 29)) }, [0x1e, 0x1f, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 30)) }, [0x1f, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 31)) }, [0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(alvextq_u8(v, a, 32)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[test]
fn arvextq_u8_test() {
    let arr_v: [u8; 16_usize] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
    let arr_a: [u8; 16_usize] = [0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20];

    let v: uint8x16_t = unsafe {
        vld1q_u8(arr_v.as_ptr())
    };
    let a: uint8x16_t = unsafe {
        vld1q_u8(arr_a.as_ptr())
    };

    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 1)) }, [0x20, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 2)) }, [0x1f, 0x20, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 3)) }, [0x1e, 0x1f, 0x20, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 4)) }, [0x1d, 0x1e, 0x1f, 0x20, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 5)) }, [0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 6)) }, [0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 7)) }, [0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 8)) }, [0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 9)) }, [0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 10)) }, [0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 11)) }, [0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x01, 0x02, 0x03, 0x04, 0x05]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 12)) }, [0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x01, 0x02, 0x03, 0x04]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 13)) }, [0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x01, 0x02, 0x03]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 14)) }, [0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x01, 0x02]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 15)) }, [0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x01]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 16)) }, [0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 17)) }, [0x00, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 18)) }, [0x00, 0x00, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 19)) }, [0x00, 0x00, 0x00, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 20)) }, [0x00, 0x00, 0x00, 0x00, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 21)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 22)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 23)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 24)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 25)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 26)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 27)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x12, 0x13, 0x14, 0x15]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 28)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x12, 0x13, 0x14]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 29)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x12, 0x13]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 30)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x12]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 31)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11]);
    assert_eq!(unsafe { transmute::<uint8x16_t, [u8; 16_usize]>(arvextq_u8(v, a, 32)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[test]
fn alvext_u16_test() {
    let arr_v: [u16; 4_usize] = [0x01, 0x02, 0x03, 0x04];
    let arr_a: [u16; 4_usize] = [0x05, 0x06, 0x07, 0x08];

    let v: uint16x4_t = unsafe {
        vld1_u16(arr_v.as_ptr())
    };
    let a: uint16x4_t = unsafe {
        vld1_u16(arr_a.as_ptr())
    };

    assert_eq!(unsafe { transmute::<uint16x4_t, [u16; 4_usize]>(alvext_u16(v, a, 1)) }, [0x02, 0x03, 0x04, 0x05]);
    assert_eq!(unsafe { transmute::<uint16x4_t, [u16; 4_usize]>(alvext_u16(v, a, 2)) }, [0x03, 0x04, 0x05, 0x06]);
    assert_eq!(unsafe { transmute::<uint16x4_t, [u16; 4_usize]>(alvext_u16(v, a, 3)) }, [0x04, 0x05, 0x06, 0x07]);
    assert_eq!(unsafe { transmute::<uint16x4_t, [u16; 4_usize]>(alvext_u16(v, a, 4)) }, [0x05, 0x06, 0x07, 0x08]);
    assert_eq!(unsafe { transmute::<uint16x4_t, [u16; 4_usize]>(alvext_u16(v, a, 5)) }, [0x06, 0x07, 0x08, 0x00]);
    assert_eq!(unsafe { transmute::<uint16x4_t, [u16; 4_usize]>(alvext_u16(v, a, 6)) }, [0x07, 0x08, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint16x4_t, [u16; 4_usize]>(alvext_u16(v, a, 7)) }, [0x08, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint16x4_t, [u16; 4_usize]>(alvext_u16(v, a, 8)) }, [0x00, 0x00, 0x00, 0x00]);
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[test]
fn arvext_u16_test() {
    let arr_v: [u16; 4_usize] = [0x01, 0x02, 0x03, 0x04];
    let arr_a: [u16; 4_usize] = [0x05, 0x06, 0x07, 0x08];

    let v: uint16x4_t = unsafe {
        vld1_u16(arr_v.as_ptr())
    };
    let a: uint16x4_t = unsafe {
        vld1_u16(arr_a.as_ptr())
    };

    assert_eq!(unsafe { transmute::<uint16x4_t, [u16; 4_usize]>(arvext_u16(v, a, 1)) }, [0x08, 0x01, 0x02, 0x03]);
    assert_eq!(unsafe { transmute::<uint16x4_t, [u16; 4_usize]>(arvext_u16(v, a, 2)) }, [0x07, 0x08, 0x01, 0x02]);
    assert_eq!(unsafe { transmute::<uint16x4_t, [u16; 4_usize]>(arvext_u16(v, a, 3)) }, [0x06, 0x07, 0x08, 0x01]);
    assert_eq!(unsafe { transmute::<uint16x4_t, [u16; 4_usize]>(arvext_u16(v, a, 4)) }, [0x05, 0x06, 0x07, 0x08]);
    assert_eq!(unsafe { transmute::<uint16x4_t, [u16; 4_usize]>(arvext_u16(v, a, 5)) }, [0x00, 0x05, 0x06, 0x07]);
    assert_eq!(unsafe { transmute::<uint16x4_t, [u16; 4_usize]>(arvext_u16(v, a, 6)) }, [0x00, 0x00, 0x05, 0x06]);
    assert_eq!(unsafe { transmute::<uint16x4_t, [u16; 4_usize]>(arvext_u16(v, a, 7)) }, [0x00, 0x00, 0x00, 0x05]);
    assert_eq!(unsafe { transmute::<uint16x4_t, [u16; 4_usize]>(arvext_u16(v, a, 8)) }, [0x00, 0x00, 0x00, 0x00]);
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[test]
fn alvextq_u16_test() {
    let arr_v: [u16; 8_usize] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let arr_a: [u16; 8_usize] = [0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];

    let v: uint16x8_t = unsafe {
        vld1q_u16(arr_v.as_ptr())
    };
    let a: uint16x8_t = unsafe {
        vld1q_u16(arr_a.as_ptr())
    };

    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(alvextq_u16(v, a, 1)) }, [0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(alvextq_u16(v, a, 2)) }, [0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(alvextq_u16(v, a, 3)) }, [0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(alvextq_u16(v, a, 4)) }, [0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(alvextq_u16(v, a, 5)) }, [0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(alvextq_u16(v, a, 6)) }, [0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(alvextq_u16(v, a, 7)) }, [0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(alvextq_u16(v, a, 8)) }, [0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(alvextq_u16(v, a, 9)) }, [0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x00]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(alvextq_u16(v, a, 10)) }, [0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(alvextq_u16(v, a, 11)) }, [0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(alvextq_u16(v, a, 12)) }, [0x0d, 0x0e, 0x0f, 0x10, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(alvextq_u16(v, a, 13)) }, [0x0e, 0x0f, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(alvextq_u16(v, a, 14)) }, [0x0f, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(alvextq_u16(v, a, 15)) }, [0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(alvextq_u16(v, a, 16)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[test]
fn arvextq_u16_test() {
    let arr_v: [u16; 8_usize] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let arr_a: [u16; 8_usize] = [0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];

    let v: uint16x8_t = unsafe {
        vld1q_u16(arr_v.as_ptr())
    };
    let a: uint16x8_t = unsafe {
        vld1q_u16(arr_a.as_ptr())
    };

    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(arvextq_u16(v, a, 1)) }, [0x10, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(arvextq_u16(v, a, 2)) }, [0x0f, 0x10, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(arvextq_u16(v, a, 3)) }, [0x0e, 0x0f, 0x10, 0x01, 0x02, 0x03, 0x04, 0x05]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(arvextq_u16(v, a, 4)) }, [0x0d, 0x0e, 0x0f, 0x10, 0x01, 0x02, 0x03, 0x04]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(arvextq_u16(v, a, 5)) }, [0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x01, 0x02, 0x03]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(arvextq_u16(v, a, 6)) }, [0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x01, 0x02]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(arvextq_u16(v, a, 7)) }, [0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x01]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(arvextq_u16(v, a, 8)) }, [0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(arvextq_u16(v, a, 9)) }, [0x00, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(arvextq_u16(v, a, 10)) }, [0x00, 0x00, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(arvextq_u16(v, a, 11)) }, [0x00, 0x00, 0x00, 0x09, 0x0a, 0x0b, 0x0c, 0x0d]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(arvextq_u16(v, a, 12)) }, [0x00, 0x00, 0x00, 0x00, 0x09, 0x0a, 0x0b, 0x0c]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(arvextq_u16(v, a, 13)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x0a, 0x0b]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(arvextq_u16(v, a, 14)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x0a]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(arvextq_u16(v, a, 15)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09]);
    assert_eq!(unsafe { transmute::<uint16x8_t, [u16; 8_usize]>(arvextq_u16(v, a, 16)) }, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[test]
fn alvext_u32_test() {
    let arr_v: [u32; 2_usize] = [0x01, 0x02];
    let arr_a: [u32; 2_usize] = [0x03, 0x04];

    let v: uint32x2_t = unsafe {
        vld1_u32(arr_v.as_ptr())
    };
    let a: uint32x2_t = unsafe {
        vld1_u32(arr_a.as_ptr())
    };

    assert_eq!(unsafe { transmute::<uint32x2_t, [u32; 2_usize]>(alvext_u32(v, a, 1)) }, [0x02, 0x03]);
    assert_eq!(unsafe { transmute::<uint32x2_t, [u32; 2_usize]>(alvext_u32(v, a, 2)) }, [0x03, 0x04]);
    assert_eq!(unsafe { transmute::<uint32x2_t, [u32; 2_usize]>(alvext_u32(v, a, 3)) }, [0x04, 0x00]);
    assert_eq!(unsafe { transmute::<uint32x2_t, [u32; 2_usize]>(alvext_u32(v, a, 4)) }, [0x00, 0x00]);
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[test]
fn arvext_u32_test() {
    let arr_v: [u32; 2_usize] = [0x01, 0x02];
    let arr_a: [u32; 2_usize] = [0x03, 0x04];

    let v: uint32x2_t = unsafe {
        vld1_u32(arr_v.as_ptr())
    };
    let a: uint32x2_t = unsafe {
        vld1_u32(arr_a.as_ptr())
    };

    assert_eq!(unsafe { transmute::<uint32x2_t, [u32; 2_usize]>(arvext_u32(v, a, 1)) }, [0x04, 0x01]);
    assert_eq!(unsafe { transmute::<uint32x2_t, [u32; 2_usize]>(arvext_u32(v, a, 2)) }, [0x03, 0x04]);
    assert_eq!(unsafe { transmute::<uint32x2_t, [u32; 2_usize]>(arvext_u32(v, a, 3)) }, [0x00, 0x03]);
    assert_eq!(unsafe { transmute::<uint32x2_t, [u32; 2_usize]>(arvext_u32(v, a, 4)) }, [0x00, 0x00]);
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[test]
fn alvextq_u32_test() {
    let arr_v: [u32; 4_usize] = [0x01, 0x02, 0x03, 0x04];
    let arr_a: [u32; 4_usize] = [0x05, 0x06, 0x07, 0x08];

    let v: uint32x4_t = unsafe {
        vld1q_u32(arr_v.as_ptr())
    };
    let a: uint32x4_t = unsafe {
        vld1q_u32(arr_a.as_ptr())
    };

    assert_eq!(unsafe { transmute::<uint32x4_t, [u32; 4_usize]>(alvextq_u32(v, a, 1)) }, [0x02, 0x03, 0x04, 0x05]);
    assert_eq!(unsafe { transmute::<uint32x4_t, [u32; 4_usize]>(alvextq_u32(v, a, 2)) }, [0x03, 0x04, 0x05, 0x06]);
    assert_eq!(unsafe { transmute::<uint32x4_t, [u32; 4_usize]>(alvextq_u32(v, a, 3)) }, [0x04, 0x05, 0x06, 0x07]);
    assert_eq!(unsafe { transmute::<uint32x4_t, [u32; 4_usize]>(alvextq_u32(v, a, 4)) }, [0x05, 0x06, 0x07, 0x08]);
    assert_eq!(unsafe { transmute::<uint32x4_t, [u32; 4_usize]>(alvextq_u32(v, a, 5)) }, [0x06, 0x07, 0x08, 0x00]);
    assert_eq!(unsafe { transmute::<uint32x4_t, [u32; 4_usize]>(alvextq_u32(v, a, 6)) }, [0x07, 0x08, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint32x4_t, [u32; 4_usize]>(alvextq_u32(v, a, 7)) }, [0x08, 0x00, 0x00, 0x00]);
    assert_eq!(unsafe { transmute::<uint32x4_t, [u32; 4_usize]>(alvextq_u32(v, a, 8)) }, [0x00, 0x00, 0x00, 0x00]);
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[test]
fn arvextq_u32_test() {
    let arr_v: [u32; 4_usize] = [0x01, 0x02, 0x03, 0x04];
    let arr_a: [u32; 4_usize] = [0x05, 0x06, 0x07, 0x08];

    let v: uint32x4_t = unsafe {
        vld1q_u32(arr_v.as_ptr())
    };
    let a: uint32x4_t = unsafe {
        vld1q_u32(arr_a.as_ptr())
    };

    assert_eq!(unsafe { transmute::<uint32x4_t, [u32; 4_usize]>(arvextq_u32(v, a, 1)) }, [0x08, 0x01, 0x02, 0x03]);
    assert_eq!(unsafe { transmute::<uint32x4_t, [u32; 4_usize]>(arvextq_u32(v, a, 2)) }, [0x07, 0x08, 0x01, 0x02]);
    assert_eq!(unsafe { transmute::<uint32x4_t, [u32; 4_usize]>(arvextq_u32(v, a, 3)) }, [0x06, 0x07, 0x08, 0x01]);
    assert_eq!(unsafe { transmute::<uint32x4_t, [u32; 4_usize]>(arvextq_u32(v, a, 4)) }, [0x05, 0x06, 0x07, 0x08]);
    assert_eq!(unsafe { transmute::<uint32x4_t, [u32; 4_usize]>(arvextq_u32(v, a, 5)) }, [0x00, 0x05, 0x06, 0x07]);
    assert_eq!(unsafe { transmute::<uint32x4_t, [u32; 4_usize]>(arvextq_u32(v, a, 6)) }, [0x00, 0x00, 0x05, 0x06]);
    assert_eq!(unsafe { transmute::<uint32x4_t, [u32; 4_usize]>(arvextq_u32(v, a, 7)) }, [0x00, 0x00, 0x00, 0x05]);
    assert_eq!(unsafe { transmute::<uint32x4_t, [u32; 4_usize]>(arvextq_u32(v, a, 8)) }, [0x00, 0x00, 0x00, 0x00]);
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[test]
fn alvext_u64_test() {
    let arr_v: [u64; 1_usize] = [0x01];
    let arr_a: [u64; 1_usize] = [0x02];

    let v: uint64x1_t = unsafe {
        vld1_u64(arr_v.as_ptr())
    };
    let a: uint64x1_t = unsafe {
        vld1_u64(arr_a.as_ptr())
    };

    assert_eq!(unsafe { transmute::<uint64x1_t, [u64; 1_usize]>(alvext_u64(v, a, 1)) }, [0x02]);
    assert_eq!(unsafe { transmute::<uint64x1_t, [u64; 1_usize]>(alvext_u64(v, a, 2)) }, [0x00]);
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[test]
fn arvext_u64_test() {
    let arr_v: [u64; 1_usize] = [0x01];
    let arr_a: [u64; 1_usize] = [0x02];

    let v: uint64x1_t = unsafe {
        vld1_u64(arr_v.as_ptr())
    };
    let a: uint64x1_t = unsafe {
        vld1_u64(arr_a.as_ptr())
    };

    assert_eq!(unsafe { transmute::<uint64x1_t, [u64; 1_usize]>(arvext_u64(v, a, 1)) }, [0x02]);
    assert_eq!(unsafe { transmute::<uint64x1_t, [u64; 1_usize]>(arvext_u64(v, a, 2)) }, [0x00]);
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[test]
fn alvextq_u64_test() {
    let arr_v: [u64; 2_usize] = [0x01, 0x02];
    let arr_a: [u64; 2_usize] = [0x03, 0x04];

    let v: uint64x2_t = unsafe {
        vld1q_u64(arr_v.as_ptr())
    };
    let a: uint64x2_t = unsafe {
        vld1q_u64(arr_a.as_ptr())
    };

    assert_eq!(unsafe { transmute::<uint64x2_t, [u64; 2_usize]>(alvextq_u64(v, a, 1)) }, [0x02, 0x03]);
    assert_eq!(unsafe { transmute::<uint64x2_t, [u64; 2_usize]>(alvextq_u64(v, a, 2)) }, [0x03, 0x04]);
    assert_eq!(unsafe { transmute::<uint64x2_t, [u64; 2_usize]>(alvextq_u64(v, a, 3)) }, [0x04, 0x00]);
    assert_eq!(unsafe { transmute::<uint64x2_t, [u64; 2_usize]>(alvextq_u64(v, a, 4)) }, [0x00, 0x00]);
}

#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[test]
fn arvextq_u64_test() {
    let arr_v: [u64; 2_usize] = [0x01, 0x02];
    let arr_a: [u64; 2_usize] = [0x03, 0x04];

    let v: uint64x2_t = unsafe {
        vld1q_u64(arr_v.as_ptr())
    };
    let a: uint64x2_t = unsafe {
        vld1q_u64(arr_a.as_ptr())
    };

    assert_eq!(unsafe { transmute::<uint64x2_t, [u64; 2_usize]>(arvextq_u64(v, a, 1)) }, [0x04, 0x01]);
    assert_eq!(unsafe { transmute::<uint64x2_t, [u64; 2_usize]>(arvextq_u64(v, a, 2)) }, [0x03, 0x04]);
    assert_eq!(unsafe { transmute::<uint64x2_t, [u64; 2_usize]>(arvextq_u64(v, a, 3)) }, [0x00, 0x03]);
    assert_eq!(unsafe { transmute::<uint64x2_t, [u64; 2_usize]>(arvextq_u64(v, a, 4)) }, [0x00, 0x00]);
}