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
            scalar::{
                _mm_movemask_epi16, _mm_movemask_epi32, _mm_movemask_epi64
            }
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
use Fasma::{
    eSIMD::{
        unrolled::{
            scalar::{
                _mm256_movemask_epi16, _mm256_movemask_epi32, _mm256_movemask_epi64
            }
        }
    }
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


#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[repr(align(16))]
struct int16x8([i16; 8_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[repr(align(16))]
struct int32x4([i32; 4_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[repr(align(16))]
struct int64x2([i64; 2_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[repr(align(32))]
struct int16x16([i16; 16_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[repr(align(32))]
struct int32x8([i32; 8_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[repr(align(32))]
struct int64x4([i64; 4_usize]);


#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_movemask_epi16_test() {
    let arr_v: [int16x8; 4] = [
        int16x8 {
            0: [
                -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF
            ]
        },
        int16x8 {
            0: [
                -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, -0x8000
            ]
        },
        int16x8 {
            0: [
                0x0000, 0x7FFF, 0x0000, 0x7FFF, 0x0000, 0x7FFF, 0x0000, 0x7FFF
            ]
        },
        int16x8 {
            0: [
                -0x8000, 0x0000, -0x0001, 0x7FFF, -0x8000, 0x0000, -0x0001, 0x7FFF
            ]
        }
    ];

    let v: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[3].0.as_ptr().cast::<__m128i>())
    ] };
    
    assert_eq!(unsafe { _mm_movemask_epi16(v[0]) }, 0x0000001F);

    assert_eq!(unsafe { _mm_movemask_epi16(v[1]) }, 0x000000FF);

    assert_eq!(unsafe { _mm_movemask_epi16(v[2]) }, 0x00000000);

    assert_eq!(unsafe { _mm_movemask_epi16(v[3]) }, 0x00000055);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_movemask_epi32_test() {
    let arr_v: [int32x4; 4] = [
        int32x4 {
            0: [
                -0x80000000, 0x7FFFFFFF, 0x00000000, -0x00000001
            ]
        },
        int32x4 {
            0: [
                -0x80000000, -0x80000000, -0x80000000, -0x80000000
            ]
        },
        int32x4 {
            0: [
                0x00000000, 0x7FFFFFFF, 0x00000000, 0x7FFFFFFF
            ]
        },
        int32x4 {
            0: [
                -0x80000000, 0x7FFFFFFF, -0x00000001, 0x00000000
            ]
        }
    ];

    let v: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[3].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { _mm_movemask_epi32(v[0]) }, 0x00000009);
    
    assert_eq!(unsafe { _mm_movemask_epi32(v[1]) }, 0x0000000F);

    assert_eq!(unsafe { _mm_movemask_epi32(v[2]) }, 0x00000000);

    assert_eq!(unsafe { _mm_movemask_epi32(v[3]) }, 0x00000005);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_movemask_epi64_test() {
    let arr_v: [int64x2; 4] = [
        int64x2 {
            0: [
                -0x8000000000000000, 0x7FFFFFFFFFFFFFFF
            ]
        },
        int64x2 {
            0: [
                -0x8000000000000000, -0x8000000000000000
            ]
        },
        int64x2 {
            0: [
                0x0000000000000000, 0x7FFFFFFFFFFFFFFF
            ]
        },
        int64x2 {
            0: [
                -0x8000000000000000, 0x7FFFFFFFFFFFFFFF
            ]
        }
    ];

    let v: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[3].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { _mm_movemask_epi64(v[0]) }, 0x00000001);
    
    assert_eq!(unsafe { _mm_movemask_epi64(v[1]) }, 0x00000003);

    assert_eq!(unsafe { _mm_movemask_epi64(v[2]) }, 0x00000000);

    assert_eq!(unsafe { _mm_movemask_epi64(v[3]) }, 0x00000001);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_movemask_epi16_test() {
    let arr_v: [int16x16; 4] = [
        int16x16 {
            0: [
                -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF,
                -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF
            ]
        },
        int16x16 {
            0: [
                -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, -0x8000,
                -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, -0x8000
            ]
        },
        int16x16 {
            0: [
                0x0000, 0x7FFF, 0x0000, 0x7FFF, 0x0000, 0x7FFF, 0x0000, 0x7FFF,
                0x0000, 0x7FFF, 0x0000, 0x7FFF, 0x0000, 0x7FFF, 0x0000, 0x7FFF
            ]
        },
        int16x16 {
            0: [
                -0x8000, 0x0000, -0x0001, 0x7FFF, -0x8000, 0x0000, -0x0001, 0x7FFF,
                -0x8000, 0x0000, -0x0001, 0x7FFF, -0x8000, 0x0000, -0x0001, 0x7FFF
            ]
        }
    ];

    let v: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[3].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { _mm256_movemask_epi16(v[0]) }, 0x00001F1F);

    assert_eq!(unsafe { _mm256_movemask_epi16(v[1]) }, 0x0000FFFF);

    assert_eq!(unsafe { _mm256_movemask_epi16(v[2]) }, 0x00000000);

    assert_eq!(unsafe { _mm256_movemask_epi16(v[3]) }, 0x00005555);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_movemask_epi32_test() {
    let arr_v: [int32x8; 4] = [
        int32x8 {
            0: [
                -0x80000000, 0x7FFFFFFF, 0x00000000, -0x00000001, -0x80000000, 0x7FFFFFFF, 0x00000000, -0x00000001
            ]
        },
        int32x8 {
            0: [
                -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000
            ]
        },
        int32x8 {
            0: [
                0x00000000, 0x7FFFFFFF, 0x00000000, 0x7FFFFFFF, 0x00000000, 0x7FFFFFFF, 0x00000000, 0x7FFFFFFF
            ]
        },
        int32x8 {
            0: [
                -0x80000000, 0x7FFFFFFF, -0x00000001, 0x00000000, -0x80000000, 0x7FFFFFFF, -0x00000001, 0x00000000
            ]
        }
    ];

    let v: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[3].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { _mm256_movemask_epi32(v[0]) }, 0x00000099);

    assert_eq!(unsafe { _mm256_movemask_epi32(v[1]) }, 0x000000FF);

    assert_eq!(unsafe { _mm256_movemask_epi32(v[2]) }, 0x00000000);

    assert_eq!(unsafe { _mm256_movemask_epi32(v[3]) }, 0x00000055);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_movemask_epi64_test() {
    let arr_v: [int64x4; 4] = [
        int64x4 {
            0: [
                -0x8000000000000000, 0x7FFFFFFFFFFFFFFF, -0x8000000000000000, 0x7FFFFFFFFFFFFFFF
            ]
        },
        int64x4 {
            0: [
                -0x8000000000000000, -0x8000000000000000, -0x8000000000000000, -0x8000000000000000
            ]
        },
        int64x4 {
            0: [
                0x0000000000000000, 0x7FFFFFFFFFFFFFFF, 0x0000000000000000, 0x7FFFFFFFFFFFFFFF
            ]
        },
        int64x4 {
            0: [
                -0x8000000000000000, -0x8000000000000000, -0x8000000000000000, 0x7FFFFFFFFFFFFFFF
            ]
        }
    ];

    let v: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[3].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { _mm256_movemask_epi64(v[0]) }, 0x00000005);

    assert_eq!(unsafe { _mm256_movemask_epi64(v[1]) }, 0x0000000F);

    assert_eq!(unsafe { _mm256_movemask_epi64(v[2]) }, 0x00000000);

    assert_eq!(unsafe { _mm256_movemask_epi64(v[3]) }, 0x00000007);
}