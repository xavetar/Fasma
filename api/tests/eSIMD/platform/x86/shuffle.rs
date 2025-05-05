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

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
use Fasma::{
    eSIMD::{
        unrolled::{
            shuffle::{
                _mm_shuffle_epi8, _mm_shuffle_epi16, _mm_shuffle_epi32
            }
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
use core::{
    mem::{
        transmute
    },
};

#[cfg(all(target_arch = "x86", target_feature = "sse2", not(target_feature = "ssse3")))]
use core::{
    arch::{
        x86::{
            __m128i,
            _mm_load_si128,
            _mm_set_epi8, _mm_set_epi16, _mm_set_epi32
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "sse2", not(target_feature = "ssse3")))]
use core::{
    arch::{
        x86_64::{
            __m128i,
            _mm_load_si128,
            _mm_set_epi8, _mm_set_epi16, _mm_set_epi32
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
#[repr(align(16))]
struct uint8x16([u8; 16_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
#[repr(align(16))]
struct uint16x8([u16; 8_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
#[repr(align(16))]
struct uint32x4([u32; 4_usize]);


#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
#[test]
fn _mm_shuffle_epi8_test() {
    let arr_v: [uint8x16; 2_usize] = [
        uint8x16 {
            0: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
            ]
        },
        uint8x16 {
            0: [
                0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00
            ],
        }
    ];

    let v: [__m128i; 2_usize] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };

    let indexes: __m128i = unsafe { _mm_set_epi8(0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F) };

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16_usize]>(_mm_shuffle_epi8(v[0], indexes)) }, [0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00]);
    assert_eq!(unsafe { transmute::<__m128i, [u8; 16_usize]>(_mm_shuffle_epi8(v[1], indexes)) }, [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
#[test]
fn _mm_shuffle_epi16_test() {
    let arr_v: [uint16x8; 2_usize] = [
        uint16x8 {
            0: [
                0x0001, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006, 0x0007, 0x0008
            ]
        },
        uint16x8 {
            0: [
                0x0008, 0x0007, 0x0006, 0x0005, 0x0004, 0x0003, 0x0002, 0x0001
            ]
        }
    ];

    let v: [__m128i; 2_usize] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };

    let indexes: __m128i = unsafe { _mm_set_epi16(0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006, 0x0007) };

    assert_eq!(unsafe { transmute::<__m128i, [u16; 8_usize]>(_mm_shuffle_epi16(v[0], indexes)) }, [0x0008, 0x0007, 0x0006, 0x0005, 0x0004, 0x0003, 0x0002, 0x0001]);
    assert_eq!(unsafe { transmute::<__m128i, [u16; 8_usize]>(_mm_shuffle_epi16(v[1], indexes)) }, [0x0001, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006, 0x0007, 0x0008]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
#[test]
fn _mm_shuffle_epi32_test() {
    let arr_v: [uint32x4; 2_usize] = [
        uint32x4 {
            0: [
                0x00000001, 0x00000002, 0x00000003, 0x00000004
            ]
        },
        uint32x4 {
            0: [
                0x00000004, 0x00000003, 0x00000002, 0x00000001
            ]
        }
    ];

    let v: [__m128i; 2_usize] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };

    let indexes: __m128i = unsafe { _mm_set_epi32(0x00000000, 0x00000001, 0x00000002, 0x00000003) };

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4_usize]>(_mm_shuffle_epi32(v[0], indexes)) }, [0x00000004, 0x00000003, 0x00000002, 0x00000001]);
    assert_eq!(unsafe { transmute::<__m128i, [u32; 4_usize]>(_mm_shuffle_epi32(v[1], indexes)) }, [0x00000001, 0x00000002, 0x00000003, 0x00000004]);
}