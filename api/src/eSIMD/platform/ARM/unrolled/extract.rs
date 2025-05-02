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

#[cfg(all(target_arch = "arm", target_feature = "v7", target_feature = "neon"))]
use core::{
    arch::{
        arm::{
            uint8x8_t, uint8x16_t,
            uint16x4_t, uint16x8_t,
            uint32x2_t, uint32x4_t,
            uint64x1_t, uint64x2_t,
            vdup_n_u8, vdupq_n_u8,
            vdup_n_u16, vdupq_n_u16,
            vdup_n_u32, vdupq_n_u32,
            vdup_n_u64, vdupq_n_u64,
            vext_u8, vextq_u8,
            vext_u16, vextq_u16,
            vext_u32, vextq_u32,
            vext_u64, vextq_u64
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
            vdup_n_u8, vdupq_n_u8,
            vdup_n_u16, vdupq_n_u16,
            vdup_n_u32, vdupq_n_u32,
            vdup_n_u64, vdupq_n_u64,
            vext_u8, vextq_u8,
            vext_u16, vextq_u16,
            vext_u32, vextq_u32,
            vext_u64, vextq_u64
        }
    }
};


/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08]```
/// - Addition: ```[09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
///
/// - Vext'ed left by 1 byte: ```[02, 03, 04, 05, 06, 07, 08, 09]```
/// - Vext'ed left by 2 bytes: ```[03, 04, 05, 06, 07, 08, 09, 0a]```
/// - Vext'ed left by 3 bytes: ```[04, 05, 06, 07, 08, 09, 0a, 0b]```
/// - Vext'ed left by 4 bytes: ```[05, 06, 07, 08, 09, 0a, 0b, 0c]```
/// - Vext'ed left by 5 bytes: ```[06, 07, 08, 09, 0a, 0b, 0c, 0d]```
/// - Vext'ed left by 6 bytes: ```[07, 08, 09, 0a, 0b, 0c, 0d, 0e]```
/// - Vext'ed left by 7 bytes: ```[08, 09, 0a, 0b, 0c, 0d, 0e, 0f]```
/// - Vext'ed left by 8 bytes: ```[09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Vext'ed left by 9 bytes: ```[0a, 0b, 0c, 0d, 0e, 0f, 10, 00]```
/// - Vext'ed left by 10 bytes: ```[0b, 0c, 0d, 0e, 0f, 10, 00, 00]```
/// - Vext'ed left by 11 bytes: ```[0c, 0d, 0e, 0f, 10, 00, 00, 00]```
/// - Vext'ed left by 12 bytes: ```[0d, 0e, 0f, 10, 00, 00, 00, 00]```
/// - Vext'ed left by 13 bytes: ```[0e, 0f, 10, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 14 bytes: ```[0f, 10, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 15 bytes: ```[10, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 16 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[inline]
pub unsafe fn alvext_u8<const N: i32>(vector: uint8x8_t, addition: uint8x8_t) -> uint8x8_t {
    return match N {
        0x00 => vector,
        0x01 => vext_u8::<0x01>(vector, addition),
        0x02 => vext_u8::<0x02>(vector, addition),
        0x03 => vext_u8::<0x03>(vector, addition),
        0x04 => vext_u8::<0x04>(vector, addition),
        0x05 => vext_u8::<0x05>(vector, addition),
        0x06 => vext_u8::<0x06>(vector, addition),
        0x07 => vext_u8::<0x07>(vector, addition),
        0x08 => addition,
        0x09 => vext_u8::<0x01>(addition, vdup_n_u8(0)),
        0x0A => vext_u8::<0x02>(addition, vdup_n_u8(0)),
        0x0B => vext_u8::<0x03>(addition, vdup_n_u8(0)),
        0x0C => vext_u8::<0x04>(addition, vdup_n_u8(0)),
        0x0D => vext_u8::<0x05>(addition, vdup_n_u8(0)),
        0x0E => vext_u8::<0x06>(addition, vdup_n_u8(0)),
        0x0F => vext_u8::<0x07>(addition, vdup_n_u8(0)),
        _ => vdup_n_u8(0),
    };
}

/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08]```
/// - Addition: ```[09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
///
/// - Vext'ed right by 1 byte: ```[10, 01, 02, 03, 04, 05, 06, 07]```
/// - Vext'ed right by 2 bytes: ```[0f, 10, 01, 02, 03, 04, 05, 06]```
/// - Vext'ed right by 3 bytes: ```[0e, 0f, 10, 01, 02, 03, 04, 05]```
/// - Vext'ed right by 4 bytes: ```[0d, 0e, 0f, 10, 01, 02, 03, 04]```
/// - Vext'ed right by 5 bytes: ```[0c, 0d, 0e, 0f, 10, 01, 02, 03]```
/// - Vext'ed right by 6 bytes: ```[0b, 0c, 0d, 0e, 0f, 10, 01, 02]```
/// - Vext'ed right by 7 bytes: ```[0a, 0b, 0c, 0d, 0e, 0f, 10, 01]```
/// - Vext'ed right by 8 bytes: ```[09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Vext'ed right by 9 bytes: ```[00, 09, 0a, 0b, 0c, 0d, 0e, 0f]```
/// - Vext'ed right by 10 bytes: ```[00, 00, 09, 0a, 0b, 0c, 0d, 0e]```
/// - Vext'ed right by 11 bytes: ```[00, 00, 00, 09, 0a, 0b, 0c, 0d]```
/// - Vext'ed right by 12 bytes: ```[00, 00, 00, 00, 09, 0a, 0b, 0c]```
/// - Vext'ed right by 13 bytes: ```[00, 00, 00, 00, 00, 09, 0a, 0b]```
/// - Vext'ed right by 14 bytes: ```[00, 00, 00, 00, 00, 00, 09, 0a]```
/// - Vext'ed right by 15 bytes: ```[00, 00, 00, 00, 00, 00, 00, 09]```
/// - Vext'ed right by 16 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[inline]
pub unsafe fn arvext_u8<const N: i32>(vector: uint8x8_t, addition: uint8x8_t) -> uint8x8_t {
    return match N {
        0x00 => vector,
        0x01 => vext_u8::<0x07>(addition, vector),
        0x02 => vext_u8::<0x06>(addition, vector),
        0x03 => vext_u8::<0x05>(addition, vector),
        0x04 => vext_u8::<0x04>(addition, vector),
        0x05 => vext_u8::<0x03>(addition, vector),
        0x06 => vext_u8::<0x02>(addition, vector),
        0x07 => vext_u8::<0x01>(addition, vector),
        0x08 => addition,
        0x09 => vext_u8::<0x07>(vdup_n_u8(0), addition),
        0x0A => vext_u8::<0x06>(vdup_n_u8(0), addition),
        0x0B => vext_u8::<0x05>(vdup_n_u8(0), addition),
        0x0C => vext_u8::<0x04>(vdup_n_u8(0), addition),
        0x0D => vext_u8::<0x03>(vdup_n_u8(0), addition),
        0x0E => vext_u8::<0x02>(vdup_n_u8(0), addition),
        0x0F => vext_u8::<0x01>(vdup_n_u8(0), addition),
        _ => vdup_n_u8(0),
    };
}

/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Addition: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
///
/// - Vext'ed left by 1 byte: ```[02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11]```
/// - Vext'ed left by 2 bytes: ```[03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12]```
/// - Vext'ed left by 3 bytes: ```[04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13]```
/// - Vext'ed left by 4 bytes: ```[05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14]```
/// - Vext'ed left by 5 bytes: ```[06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15]```
/// - Vext'ed left by 6 bytes: ```[07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16]```
/// - Vext'ed left by 7 bytes: ```[08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17]```
/// - Vext'ed left by 8 bytes: ```[09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18]```
/// - Vext'ed left by 9 bytes: ```[0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]```
/// - Vext'ed left by 10 bytes: ```[0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a]```
/// - Vext'ed left by 11 bytes: ```[0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b]```
/// - Vext'ed left by 12 bytes: ```[0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c]```
/// - Vext'ed left by 13 bytes: ```[0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d]```
/// - Vext'ed left by 14 bytes: ```[0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e]```
/// - Vext'ed left by 15 bytes: ```[10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f]```
/// - Vext'ed left by 16 bytes: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
/// - Vext'ed left by 17 bytes: ```[12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 00]```
/// - Vext'ed left by 18 bytes: ```[13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 00, 00]```
/// - Vext'ed left by 19 bytes: ```[14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 00, 00, 00]```
/// - Vext'ed left by 20 bytes: ```[15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 00, 00, 00, 00]```
/// - Vext'ed left by 21 bytes: ```[16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 22 bytes: ```[17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 23 bytes: ```[18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 24 bytes: ```[19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 25 bytes: ```[1a, 1b, 1c, 1d, 1e, 1f, 20, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 26 bytes: ```[1b, 1c, 1d, 1e, 1f, 20, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 27 bytes: ```[1c, 1d, 1e, 1f, 20, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 28 bytes: ```[1d, 1e, 1f, 20, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 29 bytes: ```[1e, 1f, 20, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 30 bytes: ```[1f, 20, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 31 bytes: ```[20, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 32 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[inline]
pub unsafe fn alvextq_u8<const N: i32>(vector: uint8x16_t, addition: uint8x16_t) -> uint8x16_t {
    return match N {
        0x00 => vector,
        0x01 => vextq_u8::<0x01>(vector, addition),
        0x02 => vextq_u8::<0x02>(vector, addition),
        0x03 => vextq_u8::<0x03>(vector, addition),
        0x04 => vextq_u8::<0x04>(vector, addition),
        0x05 => vextq_u8::<0x05>(vector, addition),
        0x06 => vextq_u8::<0x06>(vector, addition),
        0x07 => vextq_u8::<0x07>(vector, addition),
        0x08 => vextq_u8::<0x08>(vector, addition),
        0x09 => vextq_u8::<0x09>(vector, addition),
        0x0A => vextq_u8::<0x0A>(vector, addition),
        0x0B => vextq_u8::<0x0B>(vector, addition),
        0x0C => vextq_u8::<0x0C>(vector, addition),
        0x0D => vextq_u8::<0x0D>(vector, addition),
        0x0E => vextq_u8::<0x0E>(vector, addition),
        0x0F => vextq_u8::<0x0F>(vector, addition),
        0x10 => addition,
        0x11 => vextq_u8::<0x01>(addition, vdupq_n_u8(0)),
        0x12 => vextq_u8::<0x02>(addition, vdupq_n_u8(0)),
        0x13 => vextq_u8::<0x03>(addition, vdupq_n_u8(0)),
        0x14 => vextq_u8::<0x04>(addition, vdupq_n_u8(0)),
        0x15 => vextq_u8::<0x05>(addition, vdupq_n_u8(0)),
        0x16 => vextq_u8::<0x06>(addition, vdupq_n_u8(0)),
        0x17 => vextq_u8::<0x07>(addition, vdupq_n_u8(0)),
        0x18 => vextq_u8::<0x08>(addition, vdupq_n_u8(0)),
        0x19 => vextq_u8::<0x09>(addition, vdupq_n_u8(0)),
        0x1A => vextq_u8::<0x0A>(addition, vdupq_n_u8(0)),
        0x1B => vextq_u8::<0x0B>(addition, vdupq_n_u8(0)),
        0x1C => vextq_u8::<0x0C>(addition, vdupq_n_u8(0)),
        0x1D => vextq_u8::<0x0D>(addition, vdupq_n_u8(0)),
        0x1E => vextq_u8::<0x0E>(addition, vdupq_n_u8(0)),
        0x1F => vextq_u8::<0x0F>(addition, vdupq_n_u8(0)),
        _ => vdupq_n_u8(0),
    };
}

/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Addition: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
///
/// - Vext'ed right by 1 byte: ```[20, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f]```
/// - Vext'ed right by 2 bytes: ```[1f, 20, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e]```
/// - Vext'ed right by 3 bytes: ```[1e, 1f, 20, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d]```
/// - Vext'ed right by 4 bytes: ```[1d, 1e, 1f, 20, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c]```
/// - Vext'ed right by 5 bytes: ```[1c, 1d, 1e, 1f, 20, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b]```
/// - Vext'ed right by 6 bytes: ```[1b, 1c, 1d, 1e, 1f, 20, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a]```
/// - Vext'ed right by 7 bytes: ```[1a, 1b, 1c, 1d, 1e, 1f, 20, 01, 02, 03, 04, 05, 06, 07, 08, 09]```
/// - Vext'ed right by 8 bytes: ```[19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 01, 02, 03, 04, 05, 06, 07, 08]```
/// - Vext'ed right by 9 bytes: ```[18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 01, 02, 03, 04, 05, 06, 07]```
/// - Vext'ed right by 10 bytes: ```[17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 01, 02, 03, 04, 05, 06]```
/// - Vext'ed right by 11 bytes: ```[16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 01, 02, 03, 04, 05]```
/// - Vext'ed right by 12 bytes: ```[15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 01, 02, 03, 04]```
/// - Vext'ed right by 13 bytes: ```[14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 01, 02, 03]```
/// - Vext'ed right by 14 bytes: ```[13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 01, 02]```
/// - Vext'ed right by 15 bytes: ```[12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 01]```
/// - Vext'ed right by 16 bytes: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
/// - Vext'ed right by 17 bytes: ```[00, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f]```
/// - Vext'ed right by 18 bytes: ```[00, 00, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e]```
/// - Vext'ed right by 19 bytes: ```[00, 00, 00, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d]```
/// - Vext'ed right by 20 bytes: ```[00, 00, 00, 00, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c]```
/// - Vext'ed right by 21 bytes: ```[00, 00, 00, 00, 00, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b]```
/// - Vext'ed right by 22 bytes: ```[00, 00, 00, 00, 00, 00, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a]```
/// - Vext'ed right by 23 bytes: ```[00, 00, 00, 00, 00, 00, 00, 11, 12, 13, 14, 15, 16, 17, 18, 19]```
/// - Vext'ed right by 24 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 11, 12, 13, 14, 15, 16, 17, 18]```
/// - Vext'ed right by 25 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 11, 12, 13, 14, 15, 16, 17]```
/// - Vext'ed right by 26 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 11, 12, 13, 14, 15, 16]```
/// - Vext'ed right by 27 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 11, 12, 13, 14, 15]```
/// - Vext'ed right by 28 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 11, 12, 13, 14]```
/// - Vext'ed right by 29 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 11, 12, 13]```
/// - Vext'ed right by 30 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 11, 12]```
/// - Vext'ed right by 31 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 11]```
/// - Vext'ed right by 32 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[inline]
pub unsafe fn arvextq_u8<const N: i32>(vector: uint8x16_t, addition: uint8x16_t) -> uint8x16_t {
    return match N {
        0x00 => vector,
        0x01 => vextq_u8::<0x0F>(addition, vector),
        0x02 => vextq_u8::<0x0E>(addition, vector),
        0x03 => vextq_u8::<0x0D>(addition, vector),
        0x04 => vextq_u8::<0x0C>(addition, vector),
        0x05 => vextq_u8::<0x0B>(addition, vector),
        0x06 => vextq_u8::<0x0A>(addition, vector),
        0x07 => vextq_u8::<0x09>(addition, vector),
        0x08 => vextq_u8::<0x08>(addition, vector),
        0x09 => vextq_u8::<0x07>(addition, vector),
        0x0A => vextq_u8::<0x06>(addition, vector),
        0x0B => vextq_u8::<0x05>(addition, vector),
        0x0C => vextq_u8::<0x04>(addition, vector),
        0x0D => vextq_u8::<0x03>(addition, vector),
        0x0E => vextq_u8::<0x02>(addition, vector),
        0x0F => vextq_u8::<0x01>(addition, vector),
        0x10 => addition,
        0x11 => vextq_u8::<0x0F>(vdupq_n_u8(0), addition),
        0x12 => vextq_u8::<0x0E>(vdupq_n_u8(0), addition),
        0x13 => vextq_u8::<0x0D>(vdupq_n_u8(0), addition),
        0x14 => vextq_u8::<0x0C>(vdupq_n_u8(0), addition),
        0x15 => vextq_u8::<0x0B>(vdupq_n_u8(0), addition),
        0x16 => vextq_u8::<0x0A>(vdupq_n_u8(0), addition),
        0x17 => vextq_u8::<0x09>(vdupq_n_u8(0), addition),
        0x18 => vextq_u8::<0x08>(vdupq_n_u8(0), addition),
        0x19 => vextq_u8::<0x07>(vdupq_n_u8(0), addition),
        0x1A => vextq_u8::<0x06>(vdupq_n_u8(0), addition),
        0x1B => vextq_u8::<0x05>(vdupq_n_u8(0), addition),
        0x1C => vextq_u8::<0x04>(vdupq_n_u8(0), addition),
        0x1D => vextq_u8::<0x03>(vdupq_n_u8(0), addition),
        0x1E => vextq_u8::<0x02>(vdupq_n_u8(0), addition),
        0x1F => vextq_u8::<0x01>(vdupq_n_u8(0), addition),
        _ => vdupq_n_u8(0),
    };
}

/// - Source: ```[01, 02, 03, 04]```
/// - Addition: ```[05, 06, 07, 08]```
///
/// - Vext'ed left by 1 pair: ```[02, 03, 04, 05]```
/// - Vext'ed left by 2 pairs: ```[03, 04, 05, 06]```
/// - Vext'ed left by 3 pairs: ```[04, 05, 06, 07]```
/// - Vext'ed left by 4 pairs: ```[05, 06, 07, 08]```
/// - Vext'ed left by 5 pairs: ```[06, 07, 08, 00]```
/// - Vext'ed left by 6 pairs: ```[07, 08, 00, 00]```
/// - Vext'ed left by 7 pairs: ```[08, 00, 00, 00]```
/// - Vext'ed left by 8 pairs: ```[00, 00, 00, 00]```
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[inline]
pub unsafe fn alvext_u16<const N: i32>(vector: uint16x4_t, addition: uint16x4_t) -> uint16x4_t {
    return match N {
        0x00 => vector,
        0x01 => vext_u16::<0x01>(vector, addition),
        0x02 => vext_u16::<0x02>(vector, addition),
        0x03 => vext_u16::<0x03>(vector, addition),
        0x04 => addition,
        0x05 => vext_u16::<0x01>(addition, vdup_n_u16(0)),
        0x06 => vext_u16::<0x02>(addition, vdup_n_u16(0)),
        0x07 => vext_u16::<0x03>(addition, vdup_n_u16(0)),
        _ => vdup_n_u16(0),
    };
}

/// - Source: ```[01, 02, 03, 04]```
/// - Addition: ```[05, 06, 07, 08]```
///
/// - Vext'ed right by 1 pair: ```[08, 01, 02, 03]```
/// - Vext'ed right by 2 pairs: ```[07, 08, 01, 02]```
/// - Vext'ed right by 3 pairs: ```[06, 07, 08, 01]```
/// - Vext'ed right by 4 pairs: ```[05, 06, 07, 08]```
/// - Vext'ed right by 5 pairs: ```[00, 05, 06, 07]```
/// - Vext'ed right by 6 pairs: ```[00, 00, 05, 06]```
/// - Vext'ed right by 7 pairs: ```[00, 00, 00, 05]```
/// - Vext'ed right by 8 pairs: ```[00, 00, 00, 00]```
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[inline]
pub unsafe fn arvext_u16<const N: i32>(vector: uint16x4_t, addition: uint16x4_t) -> uint16x4_t {
    return match N {
        0x00 => vector,
        0x01 => vext_u16::<0x03>(addition, vector),
        0x02 => vext_u16::<0x02>(addition, vector),
        0x03 => vext_u16::<0x01>(addition, vector),
        0x04 => addition,
        0x05 => vext_u16::<0x03>(vdup_n_u16(0), addition),
        0x06 => vext_u16::<0x02>(vdup_n_u16(0), addition),
        0x07 => vext_u16::<0x01>(vdup_n_u16(0), addition),
        _ => vdup_n_u16(0),
    };
}

/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08]```
/// - Addition: ```[09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
///
/// - Vext'ed left by 1 pair: ```[02, 03, 04, 05, 06, 07, 08, 09]```
/// - Vext'ed left by 2 pairs: ```[03, 04, 05, 06, 07, 08, 09, 0a]```
/// - Vext'ed left by 3 pairs: ```[04, 05, 06, 07, 08, 09, 0a, 0b]```
/// - Vext'ed left by 4 pairs: ```[05, 06, 07, 08, 09, 0a, 0b, 0c]```
/// - Vext'ed left by 5 pairs: ```[06, 07, 08, 09, 0a, 0b, 0c, 0d]```
/// - Vext'ed left by 6 pairs: ```[07, 08, 09, 0a, 0b, 0c, 0d, 0e]```
/// - Vext'ed left by 7 pairs: ```[08, 09, 0a, 0b, 0c, 0d, 0e, 0f]```
/// - Vext'ed left by 8 pairs: ```[09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Vext'ed left by 9 pairs: ```[0a, 0b, 0c, 0d, 0e, 0f, 10, 00]```
/// - Vext'ed left by 10 pairs: ```[0b, 0c, 0d, 0e, 0f, 10, 00, 00]```
/// - Vext'ed left by 11 pairs: ```[0c, 0d, 0e, 0f, 10, 00, 00, 00]```
/// - Vext'ed left by 12 pairs: ```[0d, 0e, 0f, 10, 00, 00, 00, 00]```
/// - Vext'ed left by 13 pairs: ```[0e, 0f, 10, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 14 pairs: ```[0f, 10, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 15 pairs: ```[10, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed left by 16 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[inline]
pub unsafe fn alvextq_u16<const N: i32>(vector: uint16x8_t, addition: uint16x8_t) -> uint16x8_t {
    return match N {
        0x00 => vector,
        0x01 => vextq_u16::<0x01>(vector, addition),
        0x02 => vextq_u16::<0x02>(vector, addition),
        0x03 => vextq_u16::<0x03>(vector, addition),
        0x04 => vextq_u16::<0x04>(vector, addition),
        0x05 => vextq_u16::<0x05>(vector, addition),
        0x06 => vextq_u16::<0x06>(vector, addition),
        0x07 => vextq_u16::<0x07>(vector, addition),
        0x08 => addition,
        0x09 => vextq_u16::<0x01>(addition, vdupq_n_u16(0)),
        0x0A => vextq_u16::<0x02>(addition, vdupq_n_u16(0)),
        0x0B => vextq_u16::<0x03>(addition, vdupq_n_u16(0)),
        0x0C => vextq_u16::<0x04>(addition, vdupq_n_u16(0)),
        0x0D => vextq_u16::<0x05>(addition, vdupq_n_u16(0)),
        0x0E => vextq_u16::<0x06>(addition, vdupq_n_u16(0)),
        0x0F => vextq_u16::<0x07>(addition, vdupq_n_u16(0)),
        _ => vdupq_n_u16(0),
    };
}

/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08]```
/// - Addition: ```[09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
///
/// - Vext'ed right by 1 pair: ```[10, 01, 02, 03, 04, 05, 06, 07]```
/// - Vext'ed right by 2 pairs: ```[0f, 10, 01, 02, 03, 04, 05, 06]```
/// - Vext'ed right by 3 pairs: ```[0e, 0f, 10, 01, 02, 03, 04, 05]```
/// - Vext'ed right by 4 pairs: ```[0d, 0e, 0f, 10, 01, 02, 03, 04]```
/// - Vext'ed right by 5 pairs: ```[0c, 0d, 0e, 0f, 10, 01, 02, 03]```
/// - Vext'ed right by 6 pairs: ```[0b, 0c, 0d, 0e, 0f, 10, 01, 02]```
/// - Vext'ed right by 7 pairs: ```[0a, 0b, 0c, 0d, 0e, 0f, 10, 01]```
/// - Vext'ed right by 8 pairs: ```[09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Vext'ed right by 9 pairs: ```[00, 09, 0a, 0b, 0c, 0d, 0e, 0f]```
/// - Vext'ed right by 10 pairs: ```[00, 00, 09, 0a, 0b, 0c, 0d, 0e]```
/// - Vext'ed right by 11 pairs: ```[00, 00, 00, 09, 0a, 0b, 0c, 0d]```
/// - Vext'ed right by 12 pairs: ```[00, 00, 00, 00, 09, 0a, 0b, 0c]```
/// - Vext'ed right by 13 pairs: ```[00, 00, 00, 00, 00, 09, 0a, 0b]```
/// - Vext'ed right by 14 pairs: ```[00, 00, 00, 00, 00, 00, 09, 0a]```
/// - Vext'ed right by 15 pairs: ```[00, 00, 00, 00, 00, 00, 00, 09]```
/// - Vext'ed right by 16 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[inline]
pub unsafe fn arvextq_u16<const N: i32>(vector: uint16x8_t, addition: uint16x8_t) -> uint16x8_t {
    return match N {
        0x00 => vector,
        0x01 => vextq_u16::<0x07>(addition, vector),
        0x02 => vextq_u16::<0x06>(addition, vector),
        0x03 => vextq_u16::<0x05>(addition, vector),
        0x04 => vextq_u16::<0x04>(addition, vector),
        0x05 => vextq_u16::<0x03>(addition, vector),
        0x06 => vextq_u16::<0x02>(addition, vector),
        0x07 => vextq_u16::<0x01>(addition, vector),
        0x08 => addition,
        0x09 => vextq_u16::<0x07>(vdupq_n_u16(0), addition),
        0x0A => vextq_u16::<0x06>(vdupq_n_u16(0), addition),
        0x0B => vextq_u16::<0x05>(vdupq_n_u16(0), addition),
        0x0C => vextq_u16::<0x04>(vdupq_n_u16(0), addition),
        0x0D => vextq_u16::<0x03>(vdupq_n_u16(0), addition),
        0x0E => vextq_u16::<0x02>(vdupq_n_u16(0), addition),
        0x0F => vextq_u16::<0x01>(vdupq_n_u16(0), addition),
        _ => vdupq_n_u16(0),
    };
}

/// - Source: ```[01, 02]```
/// - Addition: ```[03, 04]```
///
/// - Vext'ed left by 1 pair: ```[02, 03]```
/// - Vext'ed left by 2 pairs: ```[03, 04]```
/// - Vext'ed left by 3 pairs: ```[04, 00]```
/// - Vext'ed left by 4 pairs: ```[00, 00]```
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[inline]
pub unsafe fn alvext_u32<const N: i32>(vector: uint32x2_t, addition: uint32x2_t) -> uint32x2_t {
    return match N {
        0x00 => vector,
        0x01 => vext_u32::<0x01>(vector, addition),
        0x02 => addition,
        0x03 => vext_u32::<0x01>(addition, vdup_n_u32(0)),
        _ => vdup_n_u32(0),
    };
}

/// - Source: ```[01, 02]```
/// - Addition: ```[03, 04]```
///
/// - Vext'ed right by 1 pair: ```[04, 01]```
/// - Vext'ed right by 2 pairs: ```[03, 04]```
/// - Vext'ed right by 3 pairs: ```[00, 03]```
/// - Vext'ed right by 4 pairs: ```[00, 00]```
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[inline]
pub unsafe fn arvext_u32<const N: i32>(vector: uint32x2_t, addition: uint32x2_t) -> uint32x2_t {
    return match N {
        0x00 => vector,
        0x01 => vext_u32::<0x01>(addition, vector),
        0x02 => addition,
        0x03 => vext_u32::<0x01>(vdup_n_u32(0), addition),
        _ => vdup_n_u32(0),
    };
}

/// - Source: ```[01, 02, 03, 04]```
/// - Addition: ```[05, 06, 07, 08]```
///
/// - Vext'ed left by 1 pair: ```[02, 03, 04, 05]```
/// - Vext'ed left by 2 pairs: ```[03, 04, 05, 06]```
/// - Vext'ed left by 3 pairs: ```[04, 05, 06, 07]```
/// - Vext'ed left by 4 pairs: ```[05, 06, 07, 08]```
/// - Vext'ed left by 5 pairs: ```[06, 07, 08, 00]```
/// - Vext'ed left by 6 pairs: ```[07, 08, 00, 00]```
/// - Vext'ed left by 7 pairs: ```[08, 00, 00, 00]```
/// - Vext'ed left by 8 pairs: ```[00, 00, 00, 00]```
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[inline]
pub unsafe fn alvextq_u32<const N: i32>(vector: uint32x4_t, addition: uint32x4_t) -> uint32x4_t {
    return match N {
        0x00 => vector,
        0x01 => vextq_u32::<0x01>(vector, addition),
        0x02 => vextq_u32::<0x02>(vector, addition),
        0x03 => vextq_u32::<0x03>(vector, addition),
        0x04 => addition,
        0x05 => vextq_u32::<0x01>(addition, vdupq_n_u32(0)),
        0x06 => vextq_u32::<0x02>(addition, vdupq_n_u32(0)),
        0x07 => vextq_u32::<0x03>(addition, vdupq_n_u32(0)),
        _ => vdupq_n_u32(0),
    };
}

/// - Source: ```[01, 02, 03, 04]```
/// - Addition: ```[05, 06, 07, 08]```
///
/// - Vext'ed right by 1 pair: ```[08, 01, 02, 03]```
/// - Vext'ed right by 2 pairs: ```[07, 08, 01, 02]```
/// - Vext'ed right by 3 pairs: ```[06, 07, 08, 01]```
/// - Vext'ed right by 4 pairs: ```[05, 06, 07, 08]```
/// - Vext'ed right by 5 pairs: ```[00, 05, 06, 07]```
/// - Vext'ed right by 6 pairs: ```[00, 00, 05, 06]```
/// - Vext'ed right by 7 pairs: ```[00, 00, 00, 05]```
/// - Vext'ed right by 8 pairs: ```[00, 00, 00, 00]```
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[inline]
pub unsafe fn arvextq_u32<const N: i32>(vector: uint32x4_t, addition: uint32x4_t) -> uint32x4_t {
    return match N {
        0x00 => vector,
        0x01 => vextq_u32::<0x03>(addition, vector),
        0x02 => vextq_u32::<0x02>(addition, vector),
        0x03 => vextq_u32::<0x01>(addition, vector),
        0x04 => addition,
        0x05 => vextq_u32::<0x03>(vdupq_n_u32(0), addition),
        0x06 => vextq_u32::<0x02>(vdupq_n_u32(0), addition),
        0x07 => vextq_u32::<0x01>(vdupq_n_u32(0), addition),
        _ => vdupq_n_u32(0),
    };
}

/// - Source: ```[01]```
/// - Addition: ```[02]```
///
/// - Vext'ed left by 1 pair: ```[02]```
/// - Vext'ed left by 2 pairs: ```[00]```
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[inline]
pub unsafe fn alvext_u64<const N: i32>(vector: uint64x1_t, addition: uint64x1_t) -> uint64x1_t {
    return match N {
        0x00 => vector,
        0x01 => addition,
        _ => vdup_n_u64(0),
    };
}

/// - Source: ```[01]```
/// - Addition: ```[02]```
///
/// - Vext'ed right by 1 pair: ```[02]```
/// - Vext'ed right by 2 pairs: ```[00]```
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[inline]
pub unsafe fn arvext_u64<const N: i32>(vector: uint64x1_t, addition: uint64x1_t) -> uint64x1_t {
    return match N {
        0x00 => vector,
        0x01 => addition,
        _ => vdup_n_u64(0),
    };
}

/// - Source: ```[01, 02]```
/// - Addition: ```[03, 04]```
///
/// - Vext'ed left by 1 pair: ```[02, 03]```
/// - Vext'ed left by 2 pairs: ```[03, 04]```
/// - Vext'ed left by 3 pairs: ```[04, 00]```
/// - Vext'ed left by 4 pairs: ```[00, 00]```
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[inline]
pub unsafe fn alvextq_u64<const N: i32>(vector: uint64x2_t, addition: uint64x2_t) -> uint64x2_t {
    return match N {
        0x00 => vector,
        0x01 => vextq_u64::<0x01>(vector, addition),
        0x02 => addition,
        0x03 => vextq_u64::<0x01>(addition, vdupq_n_u64(0)),
        _ => vdupq_n_u64(0),
    };
}

/// - Source: ```[01, 02]```
/// - Addition: ```[03, 04]```
///
/// - Vext'ed right by 1 pair: ```[04, 01]```
/// - Vext'ed right by 2 pairs: ```[03, 04]```
/// - Vext'ed right by 3 pairs: ```[00, 03]```
/// - Vext'ed right by 4 pairs: ```[00, 00]```
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
#[inline]
pub unsafe fn arvextq_u64<const N: i32>(vector: uint64x2_t, addition: uint64x2_t) -> uint64x2_t {
    return match N {
        0x00 => vector,
        0x01 => vextq_u64::<0x01>(addition, vector),
        0x02 => addition,
        0x03 => vextq_u64::<0x01>(vdupq_n_u64(0), addition),
        _ => vdupq_n_u64(0),
    };
}