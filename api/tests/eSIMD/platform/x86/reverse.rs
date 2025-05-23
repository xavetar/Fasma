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
    eSIMD::{
        unrolled::{
            reverse::{
                _mm_vrev16_epi8, _mm_vrev32_epi8,
                _mm_vrev32_epi16, _mm_vrev64_epi8,
                _mm_vrev64_epi16, _mm_vrev64_epi32
            }
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
use Fasma::{
    eSIMD::{
        unrolled::{
            reverse::{
                _mm256_vrev16_epi8, _mm256_vrev32_epi8,
                _mm256_vrev32_epi16, _mm256_vrev64_epi8,
                _mm256_vrev64_epi16, _mm256_vrev64_epi32,
                _mm256_vrev128_epi8, _mm256_vrev128_epi16,
                _mm256_vrev128_epi32, _mm256_vrev128_epi64
            }
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
use Fasma::{
    eSIMD::{
        unrolled::{
            reverse::{
                _mm512_vrev16_epi8, _mm512_vrev32_epi8,
                _mm512_vrev32_epi16, _mm512_vrev64_epi8,
                _mm512_vrev64_epi16, _mm512_vrev64_epi32,
                _mm512_vrev128_epi8, _mm512_vrev128_epi16,
                _mm512_vrev128_epi32, _mm512_vrev128_epi64,
                _mm512_vrev256_epi8, _mm512_vrev256_epi16,
                _mm512_vrev256_epi32, _mm512_vrev256_epi64,
                _mm512_vrev256_epi128
            }
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), any(target_feature = "sse2", all(target_feature = "avx", target_feature = "avx2"), all(target_feature = "avx512f", target_feature = "avx512bw"))))]
use core::{
    mem::{
        transmute
    },
};

#[cfg(all(target_arch = "x86", target_feature = "sse2"))]
use core::{
    arch::{
        x86::{
            __m128i,
            _mm_load_si128
        }
    }
};

#[cfg(all(target_arch = "x86", target_feature = "avx", target_feature = "avx2"))]
use core::{
    arch::{
        x86::{
            __m256i,
            _mm256_load_si256
        }
    }
};

#[cfg(all(target_arch = "x86", target_feature = "avx512f", target_feature = "avx512bw"))]
use core::{
    arch::{
        x86::{
            __m512i,
            _mm512_load_si512
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use core::{
    arch::{
        x86_64::{
            __m128i,
            _mm_load_si128
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "avx", target_feature = "avx2"))]
use core::{
    arch::{
        x86_64::{
            __m256i,
            _mm256_load_si256
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f", target_feature = "avx512bw"))]
use core::{
    arch::{
        x86_64::{
            __m512i,
            _mm512_load_si512
        }
    }
};


#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[repr(align(16))]
struct uint8x16([u8; 16_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[repr(align(32))]
struct uint8x32([u8; 32_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[repr(align(64))]
struct uint8x64([u8; 64_usize]);


#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_vrev16_epi8_test() {
    let arr_v: uint8x16 = uint8x16 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
        ]
    };

    let v: __m128i = unsafe { _mm_load_si128(arr_v.0.as_ptr().cast::<__m128i>()) };

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16_usize]>(_mm_vrev16_epi8(v)) }, [
        0x01, 0x00, 0x03, 0x02, 0x05, 0x04, 0x07, 0x06, 0x09, 0x08, 0x0B, 0x0A, 0x0D, 0x0C, 0x0F, 0x0E
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_vrev32_epi8_test() {
    let arr_v: uint8x16 = uint8x16 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
        ]
    };

    let v: __m128i = unsafe { _mm_load_si128(arr_v.0.as_ptr().cast::<__m128i>()) };

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16_usize]>(_mm_vrev32_epi8(v)) }, [
        0x03, 0x02, 0x01, 0x00, 0x07, 0x06, 0x05, 0x04, 0x0B, 0x0A, 0x09, 0x08, 0x0F, 0x0E, 0x0D, 0x0C
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_vrev32_epi16_test() {
    let arr_v: uint8x16 = uint8x16 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
        ]
    };

    let v: __m128i = unsafe { _mm_load_si128(arr_v.0.as_ptr().cast::<__m128i>()) };

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16_usize]>(_mm_vrev32_epi16(v)) }, [
        0x02, 0x03, 0x00, 0x01, 0x06, 0x07, 0x04, 0x05, 0x0A, 0x0B, 0x08, 0x09, 0x0E, 0x0F, 0x0C, 0x0D
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_vrev64_epi8_test() {
    let arr_v: uint8x16 = uint8x16 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
        ]
    };

    let v: __m128i = unsafe { _mm_load_si128(arr_v.0.as_ptr().cast::<__m128i>()) };

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16_usize]>(_mm_vrev64_epi8(v)) }, [
        0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09, 0x08
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_vrev64_epi16_test() {
    let arr_v: uint8x16 = uint8x16 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
        ]
    };

    let v: __m128i = unsafe { _mm_load_si128(arr_v.0.as_ptr().cast::<__m128i>()) };

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16_usize]>(_mm_vrev64_epi16(v)) }, [
        0x06, 0x07, 0x04, 0x05, 0x02, 0x03, 0x00, 0x01, 0x0E, 0x0F, 0x0C, 0x0D, 0x0A, 0x0B, 0x08, 0x09
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_vrev64_epi32_test() {
    let arr_v: uint8x16 = uint8x16 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
        ]
    };

    let v: __m128i = unsafe { _mm_load_si128(arr_v.0.as_ptr().cast::<__m128i>()) };

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16_usize]>(_mm_vrev64_epi32(v)) }, [
        0x04, 0x05, 0x06, 0x07, 0x00, 0x01, 0x02, 0x03, 0x0C, 0x0D, 0x0E, 0x0F, 0x08, 0x09, 0x0A, 0x0B
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_vrev16_epi8_test() {
    let arr_v: uint8x32 = uint8x32 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F
        ]
    };

    let v: __m256i = unsafe { _mm256_load_si256(arr_v.0.as_ptr().cast::<__m256i>()) };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32_usize]>(_mm256_vrev16_epi8(v)) }, [
        0x01, 0x00, 0x03, 0x02, 0x05, 0x04, 0x07, 0x06, 0x09, 0x08, 0x0B, 0x0A, 0x0D, 0x0C, 0x0F, 0x0E,
        0x11, 0x10, 0x13, 0x12, 0x15, 0x14, 0x17, 0x16, 0x19, 0x18, 0x1B, 0x1A, 0x1D, 0x1C, 0x1F, 0x1E
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_vrev32_epi8_test() {
    let arr_v: uint8x32 = uint8x32 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F
        ]
    };

    let v: __m256i = unsafe { _mm256_load_si256(arr_v.0.as_ptr().cast::<__m256i>()) };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32_usize]>(_mm256_vrev32_epi8(v)) }, [
        0x03, 0x02, 0x01, 0x00, 0x07, 0x06, 0x05, 0x04, 0x0B, 0x0A, 0x09, 0x08, 0x0F, 0x0E, 0x0D, 0x0C,
        0x13, 0x12, 0x11, 0x10, 0x17, 0x16, 0x15, 0x14, 0x1B, 0x1A, 0x19, 0x18, 0x1F, 0x1E, 0x1D, 0x1C
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_vrev32_epi16_test() {
    let arr_v: uint8x32 = uint8x32 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F
        ]
    };

    let v: __m256i = unsafe { _mm256_load_si256(arr_v.0.as_ptr().cast::<__m256i>()) };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32_usize]>(_mm256_vrev32_epi16(v)) }, [
        0x02, 0x03, 0x00, 0x01, 0x06, 0x07, 0x04, 0x05, 0x0A, 0x0B, 0x08, 0x09, 0x0E, 0x0F, 0x0C, 0x0D,
        0x12, 0x13, 0x10, 0x11, 0x16, 0x17, 0x14, 0x15, 0x1A, 0x1B, 0x18, 0x19, 0x1E, 0x1F, 0x1C, 0x1D
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_vrev64_epi8_test() {
    let arr_v: uint8x32 = uint8x32 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F
        ]
    };

    let v: __m256i = unsafe { _mm256_load_si256(arr_v.0.as_ptr().cast::<__m256i>()) };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32_usize]>(_mm256_vrev64_epi8(v)) }, [
        0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09, 0x08,
        0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10, 0x1F, 0x1E, 0x1D, 0x1C, 0x1B, 0x1A, 0x19, 0x18
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_vrev64_epi16_test() {
    let arr_v: uint8x32 = uint8x32 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F
        ]
    };

    let v: __m256i = unsafe { _mm256_load_si256(arr_v.0.as_ptr().cast::<__m256i>()) };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32_usize]>(_mm256_vrev64_epi16(v)) }, [
        0x06, 0x07, 0x04, 0x05, 0x02, 0x03, 0x00, 0x01, 0x0E, 0x0F, 0x0C, 0x0D, 0x0A, 0x0B, 0x08, 0x09,
        0x16, 0x17, 0x14, 0x15, 0x12, 0x13, 0x10, 0x11, 0x1E, 0x1F, 0x1C, 0x1D, 0x1A, 0x1B, 0x18, 0x19
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_vrev64_epi32_test() {
    let arr_v: uint8x32 = uint8x32 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F
        ]
    };

    let v: __m256i = unsafe { _mm256_load_si256(arr_v.0.as_ptr().cast::<__m256i>()) };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32_usize]>(_mm256_vrev64_epi32(v)) }, [
        0x04, 0x05, 0x06, 0x07, 0x00, 0x01, 0x02, 0x03, 0x0C, 0x0D, 0x0E, 0x0F, 0x08, 0x09, 0x0A, 0x0B,
        0x14, 0x15, 0x16, 0x17, 0x10, 0x11, 0x12, 0x13, 0x1C, 0x1D, 0x1E, 0x1F, 0x18, 0x19, 0x1A, 0x1B
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_vrev128_epi8_test() {
    let arr_v: uint8x32 = uint8x32 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F
        ]
    };

    let v: __m256i = unsafe { _mm256_load_si256(arr_v.0.as_ptr().cast::<__m256i>()) };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32_usize]>(_mm256_vrev128_epi8(v)) }, [
        0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00,
        0x1F, 0x1E, 0x1D, 0x1C, 0x1B, 0x1A, 0x19, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_vrev128_epi16_test() {
    let arr_v: uint8x32 = uint8x32 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F
        ]
    };

    let v: __m256i = unsafe { _mm256_load_si256(arr_v.0.as_ptr().cast::<__m256i>()) };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32_usize]>(_mm256_vrev128_epi16(v)) }, [
        0x0E, 0x0F, 0x0C, 0x0D, 0x0A, 0x0B, 0x08, 0x09, 0x06, 0x07, 0x04, 0x05, 0x02, 0x03, 0x00, 0x01,
        0x1E, 0x1F, 0x1C, 0x1D, 0x1A, 0x1B, 0x18, 0x19, 0x16, 0x17, 0x14, 0x15, 0x12, 0x13, 0x10, 0x11
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_vrev128_epi32_test() {
    let arr_v: uint8x32 = uint8x32 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F
        ]
    };

    let v: __m256i = unsafe { _mm256_load_si256(arr_v.0.as_ptr().cast::<__m256i>()) };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32_usize]>(_mm256_vrev128_epi32(v)) }, [
        0x0C, 0x0D, 0x0E, 0x0F, 0x08, 0x09, 0x0A, 0x0B, 0x04, 0x05, 0x06, 0x07, 0x00, 0x01, 0x02, 0x03,
        0x1C, 0x1D, 0x1E, 0x1F, 0x18, 0x19, 0x1A, 0x1B, 0x14, 0x15, 0x16, 0x17, 0x10, 0x11, 0x12, 0x13
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_vrev128_epi64_test() {
    let arr_v: uint8x32 = uint8x32 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F
        ]
    };

    let v: __m256i = unsafe { _mm256_load_si256(arr_v.0.as_ptr().cast::<__m256i>()) };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32_usize]>(_mm256_vrev128_epi64(v)) }, [
        0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[test]
fn _mm512_vrev16_epi8_test() {
    let arr_v: uint8x64 = uint8x64 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F
        ]
    };

    let v: __m512i = unsafe { _mm512_load_si512(arr_v.0.as_ptr().cast::<__m512i>()) };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64_usize]>(_mm512_vrev16_epi8(v)) }, [
        0x01, 0x00, 0x03, 0x02, 0x05, 0x04, 0x07, 0x06, 0x09, 0x08, 0x0B, 0x0A, 0x0D, 0x0C, 0x0F, 0x0E,
        0x11, 0x10, 0x13, 0x12, 0x15, 0x14, 0x17, 0x16, 0x19, 0x18, 0x1B, 0x1A, 0x1D, 0x1C, 0x1F, 0x1E,
        0x21, 0x20, 0x23, 0x22, 0x25, 0x24, 0x27, 0x26, 0x29, 0x28, 0x2B, 0x2A, 0x2D, 0x2C, 0x2F, 0x2E,
        0x31, 0x30, 0x33, 0x32, 0x35, 0x34, 0x37, 0x36, 0x39, 0x38, 0x3B, 0x3A, 0x3D, 0x3C, 0x3F, 0x3E
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[test]
fn _mm512_vrev32_epi8_test() {
    let arr_v: uint8x64 = uint8x64 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F
        ]
    };

    let v: __m512i = unsafe { _mm512_load_si512(arr_v.0.as_ptr().cast::<__m512i>()) };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64_usize]>(_mm512_vrev32_epi8(v)) }, [
        0x03, 0x02, 0x01, 0x00, 0x07, 0x06, 0x05, 0x04, 0x0B, 0x0A, 0x09, 0x08, 0x0F, 0x0E, 0x0D, 0x0C,
        0x13, 0x12, 0x11, 0x10, 0x17, 0x16, 0x15, 0x14, 0x1B, 0x1A, 0x19, 0x18, 0x1F, 0x1E, 0x1D, 0x1C,
        0x23, 0x22, 0x21, 0x20, 0x27, 0x26, 0x25, 0x24, 0x2B, 0x2A, 0x29, 0x28, 0x2F, 0x2E, 0x2D, 0x2C,
        0x33, 0x32, 0x31, 0x30, 0x37, 0x36, 0x35, 0x34, 0x3B, 0x3A, 0x39, 0x38, 0x3F, 0x3E, 0x3D, 0x3C
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[test]
fn _mm512_vrev32_epi16_test() {
    let arr_v: uint8x64 = uint8x64 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x05, 0x06, 0x07, 0x08, 0x09, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F
        ]
    };

    let v: __m512i = unsafe { _mm512_load_si512(arr_v.0.as_ptr().cast::<__m512i>()) };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64_usize]>(_mm512_vrev32_epi16(v)) }, [
        0x02, 0x03, 0x00, 0x01, 0x06, 0x07, 0x04, 0x05, 0x0A, 0x0B, 0x08, 0x09, 0x0E, 0x0F, 0x0C, 0x0D,
        0x12, 0x13, 0x10, 0x11, 0x16, 0x17, 0x14, 0x15, 0x1A, 0x1B, 0x18, 0x19, 0x1E, 0x1F, 0x1C, 0x1D,
        0x22, 0x23, 0x20, 0x21, 0x06, 0x07, 0x24, 0x05, 0x2A, 0x2B, 0x08, 0x09, 0x2E, 0x2F, 0x2C, 0x2D,
        0x32, 0x33, 0x30, 0x31, 0x36, 0x37, 0x34, 0x35, 0x3A, 0x3B, 0x38, 0x39, 0x3E, 0x3F, 0x3C, 0x3D
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[test]
fn _mm512_vrev64_epi8_test() {
    let arr_v: uint8x64 = uint8x64 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F
        ]
    };

    let v: __m512i = unsafe { _mm512_load_si512(arr_v.0.as_ptr().cast::<__m512i>()) };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64_usize]>(_mm512_vrev64_epi8(v)) }, [
        0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09, 0x08,
        0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10, 0x1F, 0x1E, 0x1D, 0x1C, 0x1B, 0x1A, 0x19, 0x18,
        0x27, 0x26, 0x25, 0x24, 0x23, 0x22, 0x21, 0x20, 0x2F, 0x2E, 0x2D, 0x2C, 0x2B, 0x2A, 0x29, 0x28,
        0x37, 0x36, 0x35, 0x34, 0x33, 0x32, 0x31, 0x30, 0x3F, 0x3E, 0x3D, 0x3C, 0x3B, 0x3A, 0x39, 0x38
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[test]
fn _mm512_vrev64_epi16_test() {
    let arr_v: uint8x64 = uint8x64 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F
        ]
    };

    let v: __m512i = unsafe { _mm512_load_si512(arr_v.0.as_ptr().cast::<__m512i>()) };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64_usize]>(_mm512_vrev64_epi16(v)) }, [
        0x06, 0x07, 0x04, 0x05, 0x02, 0x03, 0x00, 0x01, 0x0E, 0x0F, 0x0C, 0x0D, 0x0A, 0x0B, 0x08, 0x09,
        0x16, 0x17, 0x14, 0x15, 0x12, 0x13, 0x10, 0x11, 0x1E, 0x1F, 0x1C, 0x1D, 0x1A, 0x1B, 0x18, 0x19,
        0x26, 0x27, 0x24, 0x25, 0x22, 0x23, 0x20, 0x21, 0x2E, 0x2F, 0x2C, 0x2D, 0x2A, 0x2B, 0x28, 0x29,
        0x36, 0x37, 0x34, 0x35, 0x32, 0x33, 0x30, 0x31, 0x3E, 0x3F, 0x3C, 0x3D, 0x3A, 0x3B, 0x38, 0x39
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[test]
fn _mm512_vrev64_epi32_test() {
    let arr_v: uint8x64 = uint8x64 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F
        ]
    };

    let v: __m512i = unsafe { _mm512_load_si512(arr_v.0.as_ptr().cast::<__m512i>()) };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64_usize]>(_mm512_vrev64_epi32(v)) }, [
        0x04, 0x05, 0x06, 0x07, 0x00, 0x01, 0x02, 0x03, 0x0C, 0x0D, 0x0E, 0x0F, 0x08, 0x09, 0x0A, 0x0B,
        0x14, 0x15, 0x16, 0x17, 0x10, 0x11, 0x12, 0x13, 0x1C, 0x1D, 0x1E, 0x1F, 0x18, 0x19, 0x1A, 0x1B,
        0x24, 0x25, 0x26, 0x27, 0x20, 0x21, 0x22, 0x23, 0x2C, 0x2D, 0x2E, 0x2F, 0x28, 0x29, 0x2A, 0x2B,
        0x34, 0x35, 0x36, 0x37, 0x30, 0x31, 0x32, 0x33, 0x3C, 0x3D, 0x3E, 0x3F, 0x38, 0x39, 0x3A, 0x3B
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[test]
fn _mm512_vrev128_epi8_test() {
    let arr_v: uint8x64 = uint8x64 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F
        ]
    };

    let v: __m512i = unsafe { _mm512_load_si512(arr_v.0.as_ptr().cast::<__m512i>()) };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64_usize]>(_mm512_vrev128_epi8(v)) }, [
        0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00,
        0x1F, 0x1E, 0x1D, 0x1C, 0x1B, 0x1A, 0x19, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10,
        0x2F, 0x2E, 0x2D, 0x2C, 0x2B, 0x2A, 0x29, 0x28, 0x27, 0x26, 0x25, 0x24, 0x23, 0x22, 0x21, 0x20,
        0x3F, 0x3E, 0x3D, 0x3C, 0x3B, 0x3A, 0x39, 0x38, 0x37, 0x36, 0x35, 0x34, 0x33, 0x32, 0x31, 0x30
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[test]
fn _mm512_vrev128_epi16_test() {
    let arr_v: uint8x64 = uint8x64 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F
        ]
    };

    let v: __m512i = unsafe { _mm512_load_si512(arr_v.0.as_ptr().cast::<__m512i>()) };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64_usize]>(_mm512_vrev128_epi16(v)) }, [
        0x0E, 0x0F, 0x0C, 0x0D, 0x0A, 0x0B, 0x08, 0x09, 0x06, 0x07, 0x04, 0x05, 0x02, 0x03, 0x00, 0x01,
        0x1E, 0x1F, 0x1C, 0x1D, 0x1A, 0x1B, 0x18, 0x19, 0x16, 0x17, 0x14, 0x15, 0x12, 0x13, 0x10, 0x11,
        0x2E, 0x2F, 0x2C, 0x2D, 0x2A, 0x2B, 0x28, 0x29, 0x26, 0x27, 0x24, 0x25, 0x22, 0x23, 0x20, 0x21,
        0x3E, 0x3F, 0x3C, 0x3D, 0x3A, 0x3B, 0x38, 0x39, 0x36, 0x37, 0x34, 0x35, 0x32, 0x33, 0x30, 0x31
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[test]
fn _mm512_vrev128_epi32_test() {
    let arr_v: uint8x64 = uint8x64 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F
        ]
    };

    let v: __m512i = unsafe { _mm512_load_si512(arr_v.0.as_ptr().cast::<__m512i>()) };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64_usize]>(_mm512_vrev128_epi32(v)) }, [
        0x0C, 0x0D, 0x0E, 0x0F, 0x08, 0x09, 0x0A, 0x0B, 0x04, 0x05, 0x06, 0x07, 0x00, 0x01, 0x02, 0x03,
        0x1C, 0x1D, 0x1E, 0x1F, 0x18, 0x19, 0x1A, 0x1B, 0x14, 0x15, 0x16, 0x17, 0x10, 0x11, 0x12, 0x13,
        0x2C, 0x2D, 0x2E, 0x2F, 0x28, 0x29, 0x2A, 0x2B, 0x24, 0x25, 0x26, 0x27, 0x20, 0x21, 0x22, 0x23,
        0x3C, 0x3D, 0x3E, 0x3F, 0x38, 0x39, 0x3A, 0x3B, 0x34, 0x35, 0x36, 0x37, 0x30, 0x31, 0x32, 0x33
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[test]
fn _mm512_vrev128_epi64_test() {
    let arr_v: uint8x64 = uint8x64 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F
        ]
    };

    let v: __m512i = unsafe { _mm512_load_si512(arr_v.0.as_ptr().cast::<__m512i>()) };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64_usize]>(_mm512_vrev128_epi64(v)) }, [
        0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
        0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[test]
fn _mm512_vrev256_epi8_test() {
    let arr_v: uint8x64 = uint8x64 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F
        ]
    };

    let v: __m512i = unsafe { _mm512_load_si512(arr_v.0.as_ptr().cast::<__m512i>()) };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64_usize]>(_mm512_vrev256_epi8(v)) }, [
        0x1F, 0x1E, 0x1D, 0x1C, 0x1B, 0x1A, 0x19, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10,
        0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00,
        0x3F, 0x3E, 0x3D, 0x3C, 0x3B, 0x3A, 0x39, 0x38, 0x37, 0x36, 0x35, 0x34, 0x33, 0x32, 0x31, 0x30,
        0x2F, 0x2E, 0x2D, 0x2C, 0x2B, 0x2A, 0x29, 0x28, 0x27, 0x26, 0x25, 0x24, 0x23, 0x22, 0x21, 0x20
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[test]
fn _mm512_vrev256_epi16_test() {
    let arr_v: uint8x64 = uint8x64 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F
        ]
    };

    let v: __m512i = unsafe { _mm512_load_si512(arr_v.0.as_ptr().cast::<__m512i>()) };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64_usize]>(_mm512_vrev256_epi16(v)) }, [
        0x1E, 0x1F, 0x1C, 0x1D, 0x1A, 0x1B, 0x18, 0x19, 0x16, 0x17, 0x14, 0x15, 0x12, 0x13, 0x10, 0x11,
        0x0E, 0x0F, 0x0C, 0x0D, 0x0A, 0x0B, 0x08, 0x09, 0x06, 0x07, 0x04, 0x05, 0x02, 0x03, 0x00, 0x01,
        0x3E, 0x3F, 0x3C, 0x3D, 0x3A, 0x3B, 0x38, 0x39, 0x36, 0x37, 0x34, 0x35, 0x32, 0x33, 0x30, 0x31,
        0x2E, 0x2F, 0x2C, 0x2D, 0x2A, 0x2B, 0x28, 0x29, 0x26, 0x27, 0x24, 0x25, 0x22, 0x23, 0x20, 0x21
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[test]
fn _mm512_vrev256_epi32_test() {
    let arr_v: uint8x64 = uint8x64 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F
        ]
    };

    let v: __m512i = unsafe { _mm512_load_si512(arr_v.0.as_ptr().cast::<__m512i>()) };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64_usize]>(_mm512_vrev256_epi32(v)) }, [
        0x1C, 0x1D, 0x1E, 0x1F, 0x18, 0x19, 0x1A, 0x1B, 0x14, 0x15, 0x16, 0x17, 0x10, 0x11, 0x12, 0x13,
        0x0C, 0x0D, 0x0E, 0x0F, 0x08, 0x09, 0x0A, 0x0B, 0x04, 0x05, 0x06, 0x07, 0x00, 0x01, 0x02, 0x03,
        0x3C, 0x3D, 0x3E, 0x3F, 0x38, 0x39, 0x3A, 0x3B, 0x34, 0x35, 0x36, 0x37, 0x30, 0x31, 0x32, 0x33,
        0x2C, 0x2D, 0x2E, 0x2F, 0x28, 0x29, 0x2A, 0x2B, 0x24, 0x25, 0x26, 0x27, 0x20, 0x21, 0x22, 0x23
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[test]
fn _mm512_vrev256_epi64_test() {
    let arr_v: uint8x64 = uint8x64 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F
        ]
    };

    let v: __m512i = unsafe { _mm512_load_si512(arr_v.0.as_ptr().cast::<__m512i>()) };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64_usize]>(_mm512_vrev256_epi64(v)) }, [
        0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
        0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[test]
fn _mm512_vrev256_epi128_test() {
    let arr_v: uint8x64 = uint8x64 {
        0: [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F
        ]
    };

    let v: __m512i = unsafe { _mm512_load_si512(arr_v.0.as_ptr().cast::<__m512i>()) };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64_usize]>(_mm512_vrev256_epi128(v)) }, [
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F,
        0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F
    ]);
}
