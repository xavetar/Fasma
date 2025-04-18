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
    eFunc::{
        mem::{
            ptr::{
                align_remainder_8x8, align_remainder_8x16,
                align_remainder_16x4, align_remainder_16x8,
                align_remainder_32x2, align_remainder_32x4,
                align_remainder_64x1, align_remainder_64x2
            }
        }
    }
};

#[test]
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
fn align_remainder_8x8_test() {
    let array: [u8; 10_usize] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x09, 0x0A
    ];
    let length: usize = array.len();
    let substitution_data: [u8; 8_usize] = [0xFF; 8_usize];

    // Remainder: 10 % 8 = 2, takes last 2 bytes [9, 10], pads with 6 substitution bytes [0xFF...]
    assert_eq!(unsafe { align_remainder_8x8(array.as_ptr(), substitution_data, length) }, [
        0x09, 0x0A, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
    ]);
}

#[test]
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
fn align_remainder_8x16_test() {
    let array: [u8; 20_usize] = [
        0x0001, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006, 0x0007, 0x0008,
        0x0009, 0x000A, 0x000B, 0x000C, 0x000D, 0x000E, 0x000F, 0x0010,
        0x0011, 0x0012, 0x0013, 0x0014
    ];
    let length: usize = array.len();
    let substitution_data: [u8; 16_usize] = [
        0x00D8, 0x0000, 0x00DC, 0x0000, 0x00D8, 0x0000, 0x00DC, 0x0000,
        0x00D8, 0x0000, 0x00DC, 0x0000, 0x00D8, 0x0000, 0x00DC, 0x0000,
    ];

    // Remainder: 20 % 16 = 4, takes last 4 bytes [17, 18, 19, 20], pads with 12 substitution bytes
    assert_eq!(unsafe { align_remainder_8x16(array.as_ptr(), substitution_data, length) }, [
        0x0011, 0x0012, 0x0013, 0x0014, 0x00D8, 0x0000, 0x00DC, 0x0000,
        0x00D8, 0x0000, 0x00DC, 0x0000, 0x00D8, 0x0000, 0x00DC, 0x0000
    ]);
}

#[test]
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
fn align_remainder_16x4_test() {
    let array: [u16; 6_usize] = [
        0x0001, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006
    ];
    let length: usize = array.len();
    let substitution_data: [u16; 4_usize] = [
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF
    ];

    // Remainder: 6 % 4 = 2, takes last 2 elements [5, 6], pads with 2 substitution values [0xFFFF, 0xFFFF]
    assert_eq!(unsafe { align_remainder_16x4(array.as_ptr(), substitution_data, length) }, [
        0x0005, 0x0006, 0xFFFF, 0xFFFF
    ]);
}

#[test]
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
fn align_remainder_16x8_test() {
    let array: [u16; 10_usize] = [
        0x0001, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006, 0x0007, 0x0008,
        0x0009, 0x000A
    ];
    let length: usize = array.len();
    let substitution_data: [u16; 8_usize] = [
        0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD
    ];

    // Remainder: 10 % 8 = 2, takes last 2 elements [9, 10], pads with 6 substitution values [0xABCD...]
    assert_eq!(unsafe { align_remainder_16x8(array.as_ptr(), substitution_data, length) }, [
        0x0009, 0x000A, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD
    ]);
}

#[test]
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
fn align_remainder_32x2_test() {
    let array: [u32; 4_usize] = [
        0x00000001, 0x00000002, 0x00000003, 0x00000004
    ];
    let length: usize = array.len();
    let substitution_data: [u32; 2_usize] = [
        0xFFFFFFFF, 0xFFFFFFFF
    ];

    // Remainder: 4 % 2 = 0, no elements to take (fully aligned), returns substitution data [0xFFFFFFFF, 0xFFFFFFFF]
    assert_eq!(unsafe { align_remainder_32x2(array.as_ptr(), substitution_data, length) }, [
        0xFFFFFFFF, 0xFFFFFFFF
    ]);
}

#[test]
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
fn align_remainder_32x4_test() {
    let array: [u32; 6_usize] = [
        0x00000001, 0x00000002, 0x00000003, 0x00000004,
        0x00000005, 0x00000006
    ];
    let length: usize = array.len();
    let substitution_data: [u32; 4_usize] = [
        0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF
    ];

    // Remainder: 6 % 4 = 2, takes last 2 elements [5, 6], pads with 2 substitution values [0xDEADBEEF, 0xDEADBEEF]
    assert_eq!(unsafe { align_remainder_32x4(array.as_ptr(), substitution_data, length) }, [
        0x00000005, 0x00000006, 0xDEADBEEF, 0xDEADBEEF
    ]);
}

#[test]
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
fn align_remainder_64x1_test() {
    let array: [u64; 2_usize] = [
        0x0000000000000001, 0x0000000000000002
    ];
    let length: usize = array.len();
    let substitution_data: [u64; 1_usize] = [
        0xFFFFFFFFFFFFFFFF
    ];

    // Remainder: 2 % 1 = 0, no elements to take (fully aligned), returns substitution data [0xFFFFFFFFFFFFFFFF]
    assert_eq!(unsafe { align_remainder_64x1(array.as_ptr(), substitution_data, length) }, [0xFFFFFFFFFFFFFFFF]);
}

#[test]
#[cfg(all(any(all(target_arch = "arm", target_feature = "v7"), target_arch = "aarch64"), target_feature = "neon"))]
fn align_remainder_64x2_test() {
    let array: [u64; 5_usize] = [
        0x0000000000000001, 0x0000000000000002, 0x0000000000000003, 0x0000000000000004,
        0x0000000000000005
    ];
    let length: usize = array.len();
    let substitution_data: [u64; 2_usize] = [
        0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA
    ];

    // Remainder: 5 % 2 = 1, takes last 1 element [5], pads with 1 substitution value [0xAAAAAAAAAAAAAAAA]
    assert_eq!(unsafe { align_remainder_64x2(array.as_ptr(), substitution_data, length) }, [
        0x0000000000000005, 0xAAAAAAAAAAAAAAAA
    ]);
}