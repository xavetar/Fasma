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

use core::{
    arch::{
        arm::{
            uint8x8_t, uint8x16_t,
            uint16x4_t, uint16x8_t,
            uint32x2_t, uint32x4_t,
            vdup_n_u8, vdupq_n_u8,
            vdup_n_u16, vdupq_n_u16,
            vdup_n_u32, vdupq_n_u32,
            vext_u8, vextq_u8,
            vext_u16, vextq_u16,
            vext_u32, vextq_u32
        }
    }
};

#[target_feature(enable = "neon")]
pub unsafe fn alvext_u8(vector: uint8x8_t, addition: uint8x8_t, shift: usize) -> uint8x8_t {
    return match shift {
        0x00 => vext_u8::<0x00>(vector, addition),
        0x01 => vext_u8::<0x01>(vector, addition),
        0x02 => vext_u8::<0x02>(vector, addition),
        0x03 => vext_u8::<0x03>(vector, addition),
        0x04 => vext_u8::<0x04>(vector, addition),
        0x05 => vext_u8::<0x05>(vector, addition),
        0x06 => vext_u8::<0x06>(vector, addition),
        0x07 => vext_u8::<0x07>(vector, addition),
        _ => vdup_n_u8(0),
    };
}

#[target_feature(enable = "neon")]
pub unsafe fn arvext_u8(vector: uint8x8_t, addition: uint8x8_t, shift: usize) -> uint8x8_t {
    return match shift {
        0x00 => vext_u8::<0x00>(addition, vector),
        0x01 => vext_u8::<0x01>(addition, vector),
        0x02 => vext_u8::<0x02>(addition, vector),
        0x03 => vext_u8::<0x03>(addition, vector),
        0x04 => vext_u8::<0x04>(addition, vector),
        0x05 => vext_u8::<0x05>(addition, vector),
        0x06 => vext_u8::<0x06>(addition, vector),
        0x07 => vext_u8::<0x07>(addition, vector),
        _ => vdup_n_u8(0),
    };
}

#[target_feature(enable = "neon")]
pub unsafe fn alvextq_u8(vector: uint8x16_t, addition: uint8x16_t, shift: usize) -> uint8x16_t {
    return match shift {
        0x00 => vextq_u8::<0x00>(vector, addition),
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
        _ => vdupq_n_u8(0),
    };
}

#[target_feature(enable = "neon")]
pub unsafe fn arvextq_u8(vector: uint8x16_t, addition: uint8x16_t, shift: usize) -> uint8x16_t {
    return match shift {
        0x00 => vextq_u8::<0x00>(addition, vector),
        0x01 => vextq_u8::<0x01>(addition, vector),
        0x02 => vextq_u8::<0x02>(addition, vector),
        0x03 => vextq_u8::<0x03>(addition, vector),
        0x04 => vextq_u8::<0x04>(addition, vector),
        0x05 => vextq_u8::<0x05>(addition, vector),
        0x06 => vextq_u8::<0x06>(addition, vector),
        0x07 => vextq_u8::<0x07>(addition, vector),
        0x08 => vextq_u8::<0x08>(addition, vector),
        0x09 => vextq_u8::<0x09>(addition, vector),
        0x0A => vextq_u8::<0x0A>(addition, vector),
        0x0B => vextq_u8::<0x0B>(addition, vector),
        0x0C => vextq_u8::<0x0C>(addition, vector),
        0x0D => vextq_u8::<0x0D>(addition, vector),
        0x0E => vextq_u8::<0x0E>(addition, vector),
        0x0F => vextq_u8::<0x0F>(addition, vector),
        _ => vdupq_n_u8(0),
    };
}

#[target_feature(enable = "neon")]
pub unsafe fn alvext_u16(vector: uint16x4_t, addition: uint16x4_t, shift: usize) -> uint16x4_t {
    return match shift {
        0x00 => vext_u16::<0x00>(vector, addition),
        0x01 => vext_u16::<0x01>(vector, addition),
        0x02 => vext_u16::<0x02>(vector, addition),
        0x03 => vext_u16::<0x03>(vector, addition),
        _ => vdup_n_u16(0),
    };
}

#[target_feature(enable = "neon")]
pub unsafe fn arvext_u16(vector: uint16x4_t, addition: uint16x4_t, shift: usize) -> uint16x4_t {
    return match shift {
        0x00 => vext_u16::<0x00>(addition, vector),
        0x01 => vext_u16::<0x01>(addition, vector),
        0x02 => vext_u16::<0x02>(addition, vector),
        0x03 => vext_u16::<0x03>(addition, vector),
        _ => vdup_n_u16(0),
    };
}

#[target_feature(enable = "neon")]
pub unsafe fn alvextq_u16(vector: uint16x8_t, addition: uint16x8_t, shift: usize) -> uint16x8_t {
    return match shift {
        0x00 => vextq_u16::<0x00>(vector, addition),
        0x01 => vextq_u16::<0x01>(vector, addition),
        0x02 => vextq_u16::<0x02>(vector, addition),
        0x03 => vextq_u16::<0x03>(vector, addition),
        0x04 => vextq_u16::<0x04>(vector, addition),
        0x05 => vextq_u16::<0x05>(vector, addition),
        0x06 => vextq_u16::<0x06>(vector, addition),
        0x07 => vextq_u16::<0x07>(vector, addition),
        _ => vdupq_n_u16(0),
    };
}

#[target_feature(enable = "neon")]
pub unsafe fn arvextq_u16(vector: uint16x8_t, addition: uint16x8_t, shift: usize) -> uint16x8_t {
    return match shift {
        0x00 => vextq_u16::<0x00>(addition, vector),
        0x01 => vextq_u16::<0x01>(addition, vector),
        0x02 => vextq_u16::<0x02>(addition, vector),
        0x03 => vextq_u16::<0x03>(addition, vector),
        0x04 => vextq_u16::<0x04>(addition, vector),
        0x05 => vextq_u16::<0x05>(addition, vector),
        0x06 => vextq_u16::<0x06>(addition, vector),
        0x07 => vextq_u16::<0x07>(addition, vector),
        _ => vdupq_n_u16(0),
    };
}

#[target_feature(enable = "neon")]
pub unsafe fn alvext_u32(vector: uint32x2_t, addition: uint32x2_t, shift: usize) -> uint32x2_t {
    return match shift {
        0x00 => vext_u32::<0x00>(vector, addition),
        0x01 => vext_u32::<0x01>(vector, addition),
        _ => vdup_n_u32(0),
    };
}

#[target_feature(enable = "neon")]
pub unsafe fn arvext_u32(vector: uint32x2_t, addition: uint32x2_t, shift: usize) -> uint32x2_t {
    return match shift {
        0x00 => vext_u32::<0x00>(addition, vector),
        0x01 => vext_u32::<0x01>(addition, vector),
        _ => vdup_n_u32(0),
    };
}

#[target_feature(enable = "neon")]
pub unsafe fn alvextq_u32(vector: uint32x4_t, addition: uint32x4_t, shift: usize) -> uint32x4_t {
    return match shift {
        0x00 => vextq_u32::<0x00>(vector, addition),
        0x01 => vextq_u32::<0x01>(vector, addition),
        0x02 => vextq_u32::<0x02>(vector, addition),
        0x03 => vextq_u32::<0x03>(vector, addition),
        _ => vdupq_n_u32(0),
    };
}

#[target_feature(enable = "neon")]
pub unsafe fn arvextq_u32(vector: uint32x4_t, addition: uint32x4_t, shift: usize) -> uint32x4_t {
    return match shift {
        0x00 => vextq_u32::<0x00>(addition, vector),
        0x01 => vextq_u32::<0x01>(addition, vector),
        0x02 => vextq_u32::<0x02>(addition, vector),
        0x03 => vextq_u32::<0x03>(addition, vector),
        _ => vdupq_n_u32(0),
    };
}