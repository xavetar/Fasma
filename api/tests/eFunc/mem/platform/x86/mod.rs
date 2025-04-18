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

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
use Fasma::{
    eFunc::{
        mem::{
            ptr::{
                align_remainder_8x16,
                align_remainder_16x8,
                align_remainder_32x4,
                align_remainder_64x2
            }
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
use Fasma::{
    eFunc::{
        mem::{
            ptr::{
                align_remainder_8x32,
                align_remainder_16x16,
                align_remainder_32x8,
                align_remainder_64x4,
                align_remainder_128x2
            }
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
use Fasma::{
    eFunc::{
        mem::{
            ptr::{
                align_remainder_8x64,
                align_remainder_16x32,
                align_remainder_32x16,
                align_remainder_64x8,
                align_remainder_128x4,
                // align_remainder_256x2 - no built-in type, will break the logic
            }
        }
    }
};

#[test]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
fn align_remainder_8x16_test() {
    let array: [u8; 20_usize] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
        0x11, 0x12, 0x13, 0x14
    ];
    let substitution_data: [u8; 16_usize] = [
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00
    ];

    // Remainder: 20 % 16 = 4, takes last 4 bytes [17, 18, 19, 20], pads with 12 substitution bytes
    assert_eq!(unsafe { align_remainder_8x16(array.as_ptr(), substitution_data, array.len()) }, [
        0x11, 0x12, 0x13, 0x14, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00
    ]);
}

#[test]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
fn align_remainder_16x8_test() {
    let array: [u16; 10_usize] = [
        0x0001, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006, 0x0007, 0x0008,
        0x0009, 0x000A
    ];
    let substitution_data: [u16; 8_usize] = [0xABCD; 8_usize];

    // Remainder: 10 % 8 = 2, takes last 2 elements [9, 10], pads with 6 substitution values [0xABCD...]
    assert_eq!(unsafe { align_remainder_16x8(array.as_ptr(), substitution_data, array.len()) }, [
        0x0009, 0x000A, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD
    ]);
}

#[test]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
fn align_remainder_32x4_test() {
    let array: [u32; 6_usize] = [
        0x00000001, 0x00000002, 0x00000003, 0x00000004,
        0x00000005, 0x00000006
    ];
    let substitution_data: [u32; 4_usize] = [
        0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF
    ];

    // Remainder: 6 % 4 = 2, takes last 2 elements [5, 6], pads with 2 substitution values [0xDEADBEEF, 0xDEADBEEF]
    assert_eq!(unsafe { align_remainder_32x4(array.as_ptr(), substitution_data, array.len()) }, [
        0x00000005, 0x00000006, 0xDEADBEEF, 0xDEADBEEF
    ]);
}

#[test]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
fn align_remainder_64x2_test() {
    let array: [u64; 5_usize] = [
        0x01, 0x02, 0x03, 0x04, 0x05
    ];
    let substitution_data: [u64; 2_usize] = [
        0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA
    ];

    // Remainder: 5 % 2 = 1, takes last 1 element [5], pads with 1 substitution value [0xAAAAAAAAAAAAAAAA]
    assert_eq!(unsafe { align_remainder_64x2(array.as_ptr(), substitution_data, array.len()) }, [
        0x0000000000000005, 0xAAAAAAAAAAAAAAAA
    ]);
}

#[test]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
fn align_remainder_8x32_test() {
    let array: [u8; 40_usize] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
        0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
        0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20,
        0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28
    ];
    let substitution_data: [u8; 32_usize] = [
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00
    ];

    // Remainder: 40 % 32 = 8, takes last 8 bytes [33..40], pads with 24 substitution bytes
    assert_eq!(unsafe { align_remainder_8x32(array.as_ptr(), substitution_data, array.len()) }, [
        0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00
    ]);
}

#[test]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
fn align_remainder_16x16_test() {
    let array: [u16; 20_usize] = [
        0x0001, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006, 0x0007, 0x0008,
        0x0009, 0x000A, 0x000B, 0x000C, 0x000D, 0x000E, 0x000F, 0x0010,
        0x0011, 0x0012, 0x0013, 0x0014
    ];
    let substitution_data: [u16; 16_usize] = [
        0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD,
        0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD
    ];

    // Remainder: 20 % 16 = 4, takes last 4 elements [17..20], pads with 12 substitution values
    assert_eq!(unsafe { align_remainder_16x16(array.as_ptr(), substitution_data, array.len()) }, [
        0x0011, 0x0012, 0x0013, 0x0014, 0xABCD, 0xABCD, 0xABCD, 0xABCD,
        0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD
    ]);
}

#[test]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
fn align_remainder_32x8_test() {
    let array: [u32; 12_usize] = [
        0x00000001, 0x00000002, 0x00000003, 0x00000004,
        0x00000005, 0x00000006, 0x00000007, 0x00000008,
        0x00000009, 0x0000000A, 0x0000000B, 0x0000000C
    ];
    let substitution_data: [u32; 8_usize] = [
        0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF,
        0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF
    ];

    // Remainder: 12 % 8 = 4, takes last 4 elements [9..12], pads with 4 substitution values
    assert_eq!(unsafe { align_remainder_32x8(array.as_ptr(), substitution_data, array.len()) }, [
        0x00000009, 0x0000000A, 0x0000000B, 0x0000000C,
        0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF
    ]);
}

#[test]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
fn align_remainder_64x4_test() {
    let array: [u64; 7_usize] = [
        0x0000000000000001, 0x0000000000000002, 0x0000000000000003, 0x0000000000000004,
        0x0000000000000005, 0x0000000000000006, 0x0000000000000007
    ];
    let substitution_data: [u64; 4_usize] = [
        0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA
    ];

    // Remainder: 7 % 4 = 3, takes last 3 elements [5..7], pads with 1 substitution value
    assert_eq!(unsafe { align_remainder_64x4(array.as_ptr(), substitution_data, array.len()) }, [
        0x0000000000000005, 0x0000000000000006, 0x0000000000000007, 0xAAAAAAAAAAAAAAAA
    ]);
}

#[test]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
fn align_remainder_128x2_test() {
    let array: [u128; 5_usize] = [
        0x0000000000000001, 0x0000000000000002, 0x0000000000000003, 0x0000000000000004,
        0x0000000000000005
    ];
    let substitution_data: [u128; 2_usize] = [
        0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    ];

    // Remainder: 5 % 2 = 1, takes last 1 element [5], pads with 1 substitution value
    assert_eq!(unsafe { align_remainder_128x2(array.as_ptr(), substitution_data, array.len()) }, [
        0x00000000000000000000000000000005, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    ]);
}

#[test]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
fn align_remainder_8x64_test() {
    let array: [u8; 80_usize] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
        0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
        0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20,
        0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
        0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38,
        0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x40,
        0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48,
        0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50
    ];
    let substitution_data: [u8; 64_usize] = [
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00
    ];

    // Remainder: 80 % 64 = 16, takes last 16 bytes [65..80], pads with 48 substitution bytes
    assert_eq!(unsafe { align_remainder_8x64(array.as_ptr(), substitution_data, array.len()) }, [
        0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48,
        0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00,
        0xD8, 0x00, 0xDC, 0x00, 0xD8, 0x00, 0xDC, 0x00
    ]);
}

#[test]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
fn align_remainder_16x32_test() {
    let array: [u16; 40_usize] = [
        0x0001, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006, 0x0007, 0x0008,
        0x0009, 0x000A, 0x000B, 0x000C, 0x000D, 0x000E, 0x000F, 0x0010,
        0x0011, 0x0012, 0x0013, 0x0014, 0x0015, 0x0016, 0x0017, 0x0018,
        0x0019, 0x001A, 0x001B, 0x001C, 0x001D, 0x001E, 0x001F, 0x0020,
        0x0021, 0x0022, 0x0023, 0x0024, 0x0025, 0x0026, 0x0027, 0x0028
    ];
    let substitution_data: [u16; 32_usize] = [
        0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD,
        0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD,
        0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD,
        0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD
    ];

    // Remainder: 40 % 32 = 8, takes last 8 elements [33..40], pads with 24 substitution values
    assert_eq!(unsafe { align_remainder_16x32(array.as_ptr(), substitution_data, array.len()) }, [
        0x0021, 0x0022, 0x0023, 0x0024, 0x0025, 0x0026, 0x0027, 0x0028,
        0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD,
        0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD,
        0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD, 0xABCD
    ]);
}

#[test]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
fn align_remainder_32x16_test() {
    let array: [u32; 20_usize] = [
        0x00000001, 0x00000002, 0x00000003, 0x00000004,
        0x00000005, 0x00000006, 0x00000007, 0x00000008,
        0x00000009, 0x0000000A, 0x0000000B, 0x0000000C,
        0x0000000D, 0x0000000E, 0x0000000F, 0x00000010,
        0x00000011, 0x00000012, 0x00000013, 0x00000014
    ];
    let substitution_data: [u32; 16_usize] = [
        0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF,
        0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF,
        0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF,
        0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF
    ];

    // Remainder: 20 % 16 = 4, takes last 4 elements [17..20], pads with 12 substitution values
    assert_eq!(unsafe { align_remainder_32x16(array.as_ptr(), substitution_data, array.len()) }, [
        0x00000011, 0x00000012, 0x00000013, 0x00000014,
        0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF,
        0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF,
        0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF, 0xDEADBEEF
    ]);
}

#[test]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
fn align_remainder_64x8_test() {
    let array: [u64; 12_usize] = [
        0x0000000000000001, 0x0000000000000002, 0x0000000000000003, 0x0000000000000004,
        0x0000000000000005, 0x0000000000000006, 0x0000000000000007, 0x0000000000000008,
        0x0000000000000009, 0x000000000000000A, 0x000000000000000B, 0x000000000000000C
    ];
    let substitution_data: [u64; 8_usize] = [
        0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA,
        0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA
    ];

    // Remainder: 12 % 8 = 4, takes last 4 elements [9..12], pads with 4 substitution values
    assert_eq!(unsafe { align_remainder_64x8(array.as_ptr(), substitution_data, array.len()) }, [
        0x0000000000000009, 0x000000000000000A, 0x000000000000000B, 0x000000000000000C,
        0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA
    ]);
}

#[test]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
fn align_remainder_128x4_test() {
    let array: [u128; 7_usize] = [
        0x00000000000000000000000000000001, 0x00000000000000000000000000000002,
        0x00000000000000000000000000000003, 0x00000000000000000000000000000004,
        0x00000000000000000000000000000005, 0x00000000000000000000000000000006,
        0x00000000000000000000000000000007
    ];
    let substitution_data: [u128; 4_usize] = [
        0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    ];

    // Remainder: 7 % 4 = 3, takes last 3 elements [5..7], pads with 1 substitution value
    assert_eq!(unsafe { align_remainder_128x4(array.as_ptr(), substitution_data, array.len()) }, [
        0x00000000000000000000000000000005, 0x00000000000000000000000000000006,
        0x00000000000000000000000000000007, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    ]);
}