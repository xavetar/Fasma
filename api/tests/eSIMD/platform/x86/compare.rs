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
            compare::{
                _mm_cmpge_epi8, _mm_cmple_epi8,
                _mm_cmpge_epi16, _mm_cmple_epi16,
                _mm_cmpge_epi32, _mm_cmple_epi32,
                _mm_cmpge_epi64, _mm_cmplt_epi64, _mm_cmple_epi64,
                _mm_cmpgt_epu8, _mm_cmpge_epu8, _mm_cmplt_epu8, _mm_cmple_epu8,
                _mm_cmpgt_epu16, _mm_cmpge_epu16, _mm_cmplt_epu16, _mm_cmple_epu16,
                _mm_cmpgt_epu32, _mm_cmpge_epu32, _mm_cmplt_epu32, _mm_cmple_epu32,
                _mm_cmpgt_epu64, _mm_cmpge_epu64, _mm_cmplt_epu64, _mm_cmple_epu64
            }
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.1")))]
use Fasma::{
    eSIMD::{
        unrolled::{
            compare::{
                _mm_cmpeq_epi64
            }
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.2")))]
use Fasma::{
    eSIMD::{
        unrolled::{
            compare::{
                _mm_cmpgt_epi64
            }
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
use Fasma::{
    eSIMD::{
        unrolled::{
            compare::{
                _mm256_cmpge_epi8, _mm256_cmplt_epi8, _mm256_cmple_epi8,
                _mm256_cmpge_epi16, _mm256_cmplt_epi16, _mm256_cmple_epi16,
                _mm256_cmpge_epi32, _mm256_cmplt_epi32, _mm256_cmple_epi32,
                _mm256_cmpge_epi64, _mm256_cmplt_epi64, _mm256_cmple_epi64,
                _mm256_cmpgt_epu8, _mm256_cmpge_epu8, _mm256_cmplt_epu8, _mm256_cmple_epu8,
                _mm256_cmpgt_epu16, _mm256_cmpge_epu16, _mm256_cmplt_epu16, _mm256_cmple_epu16,
                _mm256_cmpgt_epu32, _mm256_cmpge_epu32, _mm256_cmplt_epu32, _mm256_cmple_epu32,
                _mm256_cmpgt_epu64, _mm256_cmpge_epu64, _mm256_cmplt_epu64, _mm256_cmple_epu64
            }
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
use Fasma::{
    eSIMD::{
        unrolled::{
            compare::{
                _mm512_cmpgt_epi32, _mm512_cmpge_epi32, _mm512_cmpeq_epi32, _mm512_cmplt_epi32, _mm512_cmple_epi32,
                _mm512_cmpgt_epi64, _mm512_cmpge_epi64, _mm512_cmpeq_epi64, _mm512_cmplt_epi64, _mm512_cmple_epi64,
                _mm512_cmpgt_epu32, _mm512_cmpge_epu32, _mm512_cmpeq_epu32, _mm512_cmplt_epu32, _mm512_cmple_epu32,
                _mm512_cmpgt_epu64, _mm512_cmpge_epu64, _mm512_cmpeq_epu64, _mm512_cmplt_epu64, _mm512_cmple_epu64
            }
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
use Fasma::{
    eSIMD::{
        unrolled::{
            compare::{
                _mm512_cmpgt_epi8, _mm512_cmpge_epi8, _mm512_cmpeq_epi8, _mm512_cmplt_epi8, _mm512_cmple_epi8,
                _mm512_cmpgt_epi16, _mm512_cmpge_epi16, _mm512_cmpeq_epi16, _mm512_cmplt_epi16, _mm512_cmple_epi16,
                _mm512_cmpgt_epu8, _mm512_cmpge_epu8, _mm512_cmpeq_epu8, _mm512_cmplt_epu8, _mm512_cmple_epu8,
                _mm512_cmpgt_epu16, _mm512_cmpge_epu16, _mm512_cmpeq_epu16, _mm512_cmplt_epu16, _mm512_cmple_epu16
            }
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), any(target_feature = "sse2", all(target_feature = "avx", target_feature = "avx2"), any(target_feature = "avx512f", target_feature = "avx512bw"))))]
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

#[cfg(all(target_arch = "x86", any(target_feature = "avx512f", target_feature = "avx512bw")))]
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

#[cfg(all(target_arch = "x86_64", any(target_feature = "avx512f", target_feature = "avx512bw")))]
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
struct int8x16([i8; 16_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[repr(align(16))]
struct int16x8([i16; 8_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[repr(align(16))]
struct int32x4([i32; 4_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[repr(align(16))]
struct int64x2([i64; 2_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[repr(align(16))]
struct uint8x16([u8; 16_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[repr(align(16))]
struct uint16x8([u16; 8_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[repr(align(16))]
struct uint32x4([u32; 4_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[repr(align(16))]
struct uint64x2([u64; 2_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[repr(align(32))]
struct int8x32([i8; 32_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[repr(align(32))]
struct int16x16([i16; 16_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[repr(align(32))]
struct int32x8([i32; 8_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[repr(align(32))]
struct int64x4([i64; 4_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[repr(align(32))]
struct uint8x32([u8; 32_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[repr(align(32))]
struct uint16x16([u16; 16_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[repr(align(32))]
struct uint32x8([u32; 8_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[repr(align(32))]
struct uint64x4([u64; 4_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), any(target_feature = "avx512f", target_feature = "avx512bw")))]
#[repr(align(64))]
struct int8x64([i8; 64_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), any(target_feature = "avx512f", target_feature = "avx512bw")))]
#[repr(align(64))]
struct int16x32([i16; 32_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), any(target_feature = "avx512f", target_feature = "avx512bw")))]
#[repr(align(64))]
struct int32x16([i32; 16_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), any(target_feature = "avx512f", target_feature = "avx512bw")))]
#[repr(align(64))]
struct int64x8([i64; 8_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), any(target_feature = "avx512f", target_feature = "avx512bw")))]
#[repr(align(64))]
struct uint8x64([u8; 64_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), any(target_feature = "avx512f", target_feature = "avx512bw")))]
#[repr(align(64))]
struct uint16x32([u16; 32_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), any(target_feature = "avx512f", target_feature = "avx512bw")))]
#[repr(align(64))]
struct uint32x16([u32; 16_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), any(target_feature = "avx512f", target_feature = "avx512bw")))]
#[repr(align(64))]
struct uint64x8([u64; 8_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmpge_epi8_test() {
    let arr_v: [int8x16; 2] = [
        int8x16 {
            0: [
                -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80
            ]
        },
        int8x16 {
            0: [
                -0x80, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, -0x01
            ]
        }
    ];

    let left: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16]>(_mm_cmpge_epi8(left[0], right[0])) }, [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0xFF, 0x00
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16]>(_mm_cmpge_epi8(left[1], right[1])) }, [
        0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0x00, 0xFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmple_epi8_test() {
    let arr_v: [int8x16; 2] = [
        int8x16 {
            0: [
                -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80
            ]
        },
        int8x16 {
            0: [
                -0x80, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, -0x01
            ]
        }
    ];

    let left: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16]>(_mm_cmple_epi8(left[0], right[0])) }, [
        0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0x00, 0xFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16]>(_mm_cmple_epi8(left[1], right[1])) }, [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0xFF, 0x00
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmpge_epi16_test() {
    let arr_v: [int16x8; 2] = [
        int16x8 {
            0: [
                -0x8000, 0x7FFF, 0x0000, -0x0001, 0x0001, -0x8000, 0x7FFF, 0x0000
            ]
        },
        int16x8 {
            0: [
                -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF
            ]
        }
    ];

    let left: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u16; 8]>(_mm_cmpge_epi16(left[0], right[0])) }, [
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u16; 8]>(_mm_cmpge_epi16(left[1], right[1])) }, [
        0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmple_epi16_test() {
    let arr_v: [int16x8; 2] = [
        int16x8 {
            0: [
                -0x8000, 0x7FFF, 0x0000, -0x0001, 0x0001, -0x8000, 0x7FFF, 0x0000
            ]
        },
        int16x8 {
            0: [
                -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF
            ]
        }
    ];

    let left: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u16; 8]>(_mm_cmple_epi16(left[0], right[0])) }, [
        0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u16; 8]>(_mm_cmple_epi16(left[1], right[1])) }, [
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmpge_epi32_test() {
    let arr_v: [int32x4; 3] = [
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
                0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF
            ]
        }
    ];

    let left: [__m128i; 3] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 3] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmpge_epi32(left[0], right[0])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmpge_epi32(left[1], right[1])) }, [
        0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmpge_epi32(left[2], right[2])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmple_epi32_test() {
    let arr_v: [int32x4; 3] = [
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
                0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF
            ]
        }
    ];

    let left: [__m128i; 3] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 3] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmple_epi32(left[0], right[0])) }, [
        0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmple_epi32(left[1], right[1])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmple_epi32(left[2], right[2])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmpge_epi64_test() {
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
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF
            ]
        },
        int64x2 {
            0: [
                0x0000000000000000, -0x0000000000000001
            ]
        }
    ];

    let left: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[3].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpge_epi64(left[0], right[0])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpge_epi64(left[1], right[1])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpge_epi64(left[2], right[2])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpge_epi64(left[3], right[3])) }, [
        0x0000000000000000, 0x0000000000000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmplt_epi64_test() {
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
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF
            ]
        },
        int64x2 {
            0: [
                0x0000000000000000, -0x0000000000000001
            ]
        }
    ];

    let left: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[3].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmplt_epi64(left[0], right[0])) }, [
        0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmplt_epi64(left[1], right[1])) }, [
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmplt_epi64(left[2], right[2])) }, [
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmplt_epi64(left[3], right[3])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmple_epi64_test() {
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
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF
            ]
        },
        int64x2 {
            0: [
                0x0000000000000000, -0x0000000000000001
            ]
        }
    ];

    let left: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[3].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmple_epi64(left[0], right[0])) }, [
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmple_epi64(left[1], right[1])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmple_epi64(left[2], right[2])) }, [
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmple_epi64(left[3], right[3])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.1")))]
#[test]
fn _mm_cmpeq_epi64_test() {
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
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF
            ]
        },
        int64x2 {
            0: [
                0x0000000000000000, -0x0000000000000001
            ]
        }
    ];

    let left: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[3].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpeq_epi64(left[0], right[0])) }, [
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpeq_epi64(left[1], right[1])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpeq_epi64(left[2], right[2])) }, [
        0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpeq_epi64(left[3], right[3])) }, [
        0x0000000000000000, 0x0000000000000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.2")))]
#[test]
fn _mm_cmpgt_epi64_test() {
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
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF
            ]
        },
        int64x2 {
            0: [
                0x0000000000000000, -0x0000000000000001
            ]
        }
    ];

    let left: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[3].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpgt_epi64(left[0], right[0])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpgt_epi64(left[1], right[1])) }, [
        0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpgt_epi64(left[2], right[2])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpgt_epi64(left[3], right[3])) }, [
        0x0000000000000000, 0x0000000000000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmpgt_epu8_test() {
    let arr_v: [uint8x16; 2] = [
        uint8x16 {
            0: [
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01
            ]
        },
        uint8x16 {
            0: [
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01
            ]
        }
    ];

    let left: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16]>(_mm_cmpgt_epu8(left[0], right[0])) }, [
        0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16]>(_mm_cmpgt_epu8(left[1], right[1])) }, [
        0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmpge_epu8_test() {
    let arr_v: [uint8x16; 2] = [
        uint8x16 {
            0: [
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01
            ]
        },
        uint8x16 {
            0: [
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01
            ]
        }
    ];

    let left: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16]>(_mm_cmpge_epu8(left[0], right[0])) }, [
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16]>(_mm_cmpge_epu8(left[1], right[1])) }, [
        0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmplt_epu8_test() {
    let arr_v: [uint8x16; 2] = [
        uint8x16 {
            0: [
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01
            ]
        },
        uint8x16 {
            0: [
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01
            ]
        }
    ];

    let left: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16]>(_mm_cmplt_epu8(left[0], right[0])) }, [
        0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16]>(_mm_cmplt_epu8(left[1], right[1])) }, [
        0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmple_epu8_test() {
    let arr_v: [uint8x16; 2] = [
        uint8x16 {
            0: [
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01
            ]
        },
        uint8x16 {
            0: [
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01
            ]
        }
    ];

    let left: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16]>(_mm_cmple_epu8(left[0], right[0])) }, [
        0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u8; 16]>(_mm_cmple_epu8(left[1], right[1])) }, [
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmpgt_epu16_test() {
    let arr_v: [uint16x8; 2] = [
        uint16x8 {
            0: [
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001
            ]
        },
        uint16x8 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF
            ]
        }
    ];

    let left: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u16; 8]>(_mm_cmpgt_epu16(left[0], right[0])) }, [
        0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u16; 8]>(_mm_cmpgt_epu16(left[1], right[1])) }, [
        0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0xFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmpge_epu16_test() {
    let arr_v: [uint16x8; 2] = [
        uint16x8 {
            0: [
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001
            ]
        },
        uint16x8 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF
            ]
        }
    ];

    let left: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u16; 8]>(_mm_cmpge_epu16(left[0], right[0])) }, [
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0x0000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u16; 8]>(_mm_cmpge_epu16(left[1], right[1])) }, [
        0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmplt_epu16_test() {
    let arr_v: [uint16x8; 2] = [
        uint16x8 {
            0: [
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001
            ]
        },
        uint16x8 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF
            ]
        }
    ];

    let left: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u16; 8]>(_mm_cmplt_epu16(left[0], right[0])) }, [
        0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0xFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u16; 8]>(_mm_cmplt_epu16(left[1], right[1])) }, [
        0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmple_epu16_test() {
    let arr_v: [uint16x8; 2] = [
        uint16x8 {
            0: [
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001
            ]
        },
        uint16x8 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF
            ]
        }
    ];

    let left: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 2] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u16; 8]>(_mm_cmple_epu16(left[0], right[0])) }, [
        0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u16; 8]>(_mm_cmple_epu16(left[1], right[1])) }, [
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0x0000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmpgt_epu32_test() {
    let arr_v: [uint32x4; 3] = [
        uint32x4 {
            0: [
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001
            ]
        },
        uint32x4 {
            0: [
                0x00000000, 0x00000000, 0x00000000, 0x00000000
            ]
        },
        uint32x4 {
            0: [
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
            ]
        }
    ];

    let left: [__m128i; 3] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 3] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmpgt_epu32(left[0], right[0])) }, [
        0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmpgt_epu32(left[1], right[1])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmpgt_epu32(left[2], right[2])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmpge_epu32_test() {
    let arr_v: [uint32x4; 3] = [
        uint32x4 {
            0: [
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001
            ]
        },
        uint32x4 {
            0: [
                0x00000000, 0x00000000, 0x00000000, 0x00000000
            ]
        },
        uint32x4 {
            0: [
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
            ]
        }
    ];

    let left: [__m128i; 3] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 3] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmpge_epu32(left[0], right[0])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmpge_epu32(left[1], right[1])) }, [
        0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmpge_epu32(left[2], right[2])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmplt_epu32_test() {
    let arr_v: [uint32x4; 3] = [
        uint32x4 {
            0: [
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001
            ]
        },
        uint32x4 {
            0: [
                0x00000000, 0x00000000, 0x00000000, 0x00000000
            ]
        },
        uint32x4 {
            0: [
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
            ]
        }
    ];

    let left: [__m128i; 3] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 3] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmplt_epu32(left[0], right[0])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmplt_epu32(left[1], right[1])) }, [
        0xFFFFFFFF, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmplt_epu32(left[2], right[2])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmple_epu32_test() {
    let arr_v: [uint32x4; 3] = [
        uint32x4 {
            0: [
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001
            ]
        },
        uint32x4 {
            0: [
                0x00000000, 0x00000000, 0x00000000, 0x00000000
            ]
        },
        uint32x4 {
            0: [
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
            ]
        }
    ];

    let left: [__m128i; 3] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 3] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmple_epu32(left[0], right[0])) }, [
        0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmple_epu32(left[1], right[1])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u32; 4]>(_mm_cmple_epu32(left[2], right[2])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmpgt_epu64_test() {
    let arr_v: [uint64x2; 4] = [
        uint64x2 {
            0: [
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF
            ]
        },
        uint64x2 {
            0: [
                0x0000000000000000, 0x0000000000000000
            ]
        },
        uint64x2 {
            0: [
                0x8000000000000000, 0x0000000000000001
            ]
        },
        uint64x2 {
            0: [
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[3].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[3].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpgt_epu64(left[0], right[0])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpgt_epu64(left[1], right[1])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpgt_epu64(left[2], right[2])) }, [
        0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpgt_epu64(left[3], right[3])) }, [
        0x0000000000000000, 0x0000000000000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmpge_epu64_test() {
    let arr_v: [uint64x2; 4] = [
        uint64x2 {
            0: [
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF
            ]
        },
        uint64x2 {
            0: [
                0x0000000000000000, 0x0000000000000000
            ]
        },
        uint64x2 {
            0: [
                0x8000000000000000, 0x0000000000000001
            ]
        },
        uint64x2 {
            0: [
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[3].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[3].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpge_epu64(left[0], right[0])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpge_epu64(left[1], right[1])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpge_epu64(left[2], right[2])) }, [
        0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmpge_epu64(left[3], right[3])) }, [
        0x0000000000000000, 0x0000000000000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmplt_epu64_test() {
    let arr_v: [uint64x2; 4] = [
        uint64x2 {
            0: [
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF
            ]
        },
        uint64x2 {
            0: [
                0x0000000000000000, 0x0000000000000000
            ]
        },
        uint64x2 {
            0: [
                0x8000000000000000, 0x0000000000000001
            ]
        },
        uint64x2 {
            0: [
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[3].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[3].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmplt_epu64(left[0], right[0])) }, [
        0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmplt_epu64(left[1], right[1])) }, [
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmplt_epu64(left[2], right[2])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmplt_epu64(left[3], right[3])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn _mm_cmple_epu64_test() {
    let arr_v: [uint64x2; 4] = [
        uint64x2 {
            0: [
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF
            ]
        },
        uint64x2 {
            0: [
                0x0000000000000000, 0x0000000000000000
            ]
        },
        uint64x2 {
            0: [
                0x8000000000000000, 0x0000000000000001
            ]
        },
        uint64x2 {
            0: [
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[0].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i>())
    ] };
    let right: [__m128i; 4] = unsafe { [
        _mm_load_si128(arr_v[1].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[2].0.as_ptr().cast::<__m128i> ()),
        _mm_load_si128(arr_v[3].0.as_ptr().cast::<__m128i>()),
        _mm_load_si128(arr_v[3].0.as_ptr().cast::<__m128i>())
    ] };

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmple_epu64(left[0], right[0])) }, [
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmple_epu64(left[1], right[1])) }, [
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmple_epu64(left[2], right[2])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m128i, [u64; 2]>(_mm_cmple_epu64(left[3], right[3])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmpge_epi8_test() {
    let arr_v: [int8x32; 2] = [
        int8x32 {
            0: [
                -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80,
                -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80
            ]
        },
        int8x32 {
            0: [
                -0x80, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, -0x01,
                -0x80, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, -0x01
            ]
        }
    ];

    let left: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32]>(_mm256_cmpge_epi8(left[0], right[0])) }, [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0xFF, 0x00,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0xFF, 0x00
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32]>(_mm256_cmpge_epi8(left[1], right[1])) }, [
        0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0x00, 0xFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmplt_epi8_test() {
    let arr_v: [int8x32; 2] = [
        int8x32 {
            0: [
                -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80,
                -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80
            ]
        },
        int8x32 {
            0: [
                -0x80, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, -0x01,
                -0x80, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, -0x01
            ]
        }
    ];

    let left: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32]>(_mm256_cmplt_epi8(left[0], right[0])) }, [
        0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0xFF,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0xFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32]>(_mm256_cmplt_epi8(left[1], right[1])) }, [
        0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00,
        0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmple_epi8_test() {
    let arr_v: [int8x32; 2] = [
        int8x32 {
            0: [
                -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80,
                -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80, 0x7F, 0x00, -0x01, 0x01, -0x80
            ]
        },
        int8x32 {
            0: [
                -0x80, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, -0x01,
                -0x80, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, -0x01
            ]
        }
    ];

    let left: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32]>(_mm256_cmple_epi8(left[0], right[0])) }, [
        0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0x00, 0xFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32]>(_mm256_cmple_epi8(left[1], right[1])) }, [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0xFF, 0x00,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0xFF, 0x00
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmpge_epi16_test() {
    let arr_v: [int16x16; 2] = [
        int16x16 {
            0: [
                -0x8000, 0x7FFF, 0x0000, -0x0001, 0x0001, -0x8000, 0x7FFF, 0x0000,
                -0x0001, 0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, 0x0001, -0x8000
            ]
        },
        int16x16 {
            0: [
                -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF,
                0x7FFF, 0x7FFF, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, -0x0001
            ]
        }
    ];

    let left: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u16; 16]>(_mm256_cmpge_epi16(left[0], right[0])) }, [
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000,
        0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u16; 16]>(_mm256_cmpge_epi16(left[1], right[1])) }, [
        0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmplt_epi16_test() {
    let arr_v: [int16x16; 2] = [
        int16x16 {
            0: [
                -0x8000, 0x7FFF, 0x0000, -0x0001, 0x0001, -0x8000, 0x7FFF, 0x0000,
                -0x0001, 0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, 0x0001, -0x8000
            ]
        },
        int16x16 {
            0: [
                -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF,
                0x7FFF, 0x7FFF, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, -0x0001
            ]
        }
    ];

    let left: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u16; 16]>(_mm256_cmplt_epi16(left[0], right[0])) }, [
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF, 0x0000, 0xFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u16; 16]>(_mm256_cmplt_epi16(left[1], right[1])) }, [
        0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0x0000, 0xFFFF, 0x0000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmple_epi16_test() {
    let arr_v: [int16x16; 2] = [
        int16x16 {
            0: [
                -0x8000, 0x7FFF, 0x0000, -0x0001, 0x0001, -0x8000, 0x7FFF, 0x0000,
                -0x0001, 0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, 0x0001, -0x8000
            ]
        },
        int16x16 {
            0: [
                -0x8000, -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF,
                0x7FFF, 0x7FFF, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, -0x0001
            ]
        }
    ];

    let left: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u16; 16]>(_mm256_cmple_epi16(left[0], right[0])) }, [
        0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u16; 16]>(_mm256_cmple_epi16(left[1], right[1])) }, [
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000,
        0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmpge_epi32_test() {
    let arr_v: [int32x8; 3] = [
        int32x8 {
            0: [
                -0x80000000, 0x7FFFFFFF, 0x00000000, -0x00000001, 0x00000001, -0x80000000, 0x7FFFFFFF, 0x00000000
            ]
        },
        int32x8 {
            0: [
                -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000
            ]
        },
        int32x8 {
            0: [
                0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF
            ]
        }
    ];

    let left: [__m256i; 3] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 3] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmpge_epi32(left[0], right[0])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmpge_epi32(left[1], right[1])) }, [
        0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmpge_epi32(left[2], right[2])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmplt_epi32_test() {
    let arr_v: [int32x8; 3] = [
        int32x8 {
            0: [
                -0x80000000, 0x7FFFFFFF, 0x00000000, -0x00000001, 0x00000001, -0x80000000, 0x7FFFFFFF, 0x00000000
            ]
        },
        int32x8 {
            0: [
                -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000
            ]
        },
        int32x8 {
            0: [
                0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF
            ]
        }
    ];

    let left: [__m256i; 3] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 3] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmplt_epi32(left[0], right[0])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmplt_epi32(left[1], right[1])) }, [
        0xFFFFFFFF, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmplt_epi32(left[2], right[2])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmple_epi32_test() {
    let arr_v: [int32x8; 3] = [
        int32x8 {
            0: [
                -0x80000000, 0x7FFFFFFF, 0x00000000, -0x00000001, 0x00000001, -0x80000000, 0x7FFFFFFF, 0x00000000
            ]
        },
        int32x8 {
            0: [
                -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000
            ]
        },
        int32x8 {
            0: [
                0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF
            ]
        }
    ];

    let left: [__m256i; 3] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 3] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmple_epi32(left[0], right[0])) }, [
        0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmple_epi32(left[1], right[1])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmple_epi32(left[2], right[2])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmpge_epi64_test() {
    let arr_v: [int64x4; 4] = [
        int64x4 {
            0: [
                -0x8000000000000000, 0x7FFFFFFFFFFFFFFF, 0x0000000000000000, -0x0000000000000001
            ]
        },
        int64x4 {
            0: [
                -0x8000000000000000, -0x8000000000000000, -0x8000000000000000, -0x8000000000000000
            ]
        },
        int64x4 {
            0: [
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF
            ]
        },
        int64x4 {
            0: [
                0x0000000000000000, 0x0000000000000001, -0x0000000000000001, 0x0000000000000000
            ]
        }
    ];

    let left: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[3].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmpge_epi64(left[0], right[0])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmpge_epi64(left[1], right[1])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmpge_epi64(left[2], right[2])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmpge_epi64(left[3], right[3])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmplt_epi64_test() {
    let arr_v: [int64x4; 4] = [
        int64x4 {
            0: [
                -0x8000000000000000, 0x7FFFFFFFFFFFFFFF, 0x0000000000000000, -0x0000000000000001
            ]
        },
        int64x4 {
            0: [
                -0x8000000000000000, -0x8000000000000000, -0x8000000000000000, -0x8000000000000000
            ]
        },
        int64x4 {
            0: [
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF
            ]
        },
        int64x4 {
            0: [
                0x0000000000000000, 0x0000000000000001, -0x0000000000000001, 0x0000000000000000
            ]
        }
    ];

    let left: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[3].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmplt_epi64(left[0], right[0])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmplt_epi64(left[1], right[1])) }, [
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmplt_epi64(left[2], right[2])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmplt_epi64(left[3], right[3])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmple_epi64_test() {
    let arr_v: [int64x4; 4] = [
        int64x4 {
            0: [
                -0x8000000000000000, 0x7FFFFFFFFFFFFFFF, 0x0000000000000000, -0x0000000000000001
            ]
        },
        int64x4 {
            0: [
                -0x8000000000000000, -0x8000000000000000, -0x8000000000000000, -0x8000000000000000
            ]
        },
        int64x4 {
            0: [
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF
            ]
        },
        int64x4 {
            0: [
                0x0000000000000000, 0x0000000000000001, -0x0000000000000001, 0x0000000000000000
            ]
        }
    ];

    let left: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[3].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmple_epi64(left[0], right[0])) }, [
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmple_epi64(left[1], right[1])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmple_epi64(left[2], right[2])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmple_epi64(left[3], right[3])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmpgt_epu8_test() {
    let arr_v: [uint8x32; 2] = [
        uint8x32 {
            0: [
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01
            ]
        },
        uint8x32 {
            0: [
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01
            ]
        }
    ];

    let left: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32]>(_mm256_cmpgt_epu8(left[0], right[0])) }, [
        0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32]>(_mm256_cmpgt_epu8(left[1], right[1])) }, [
        0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmpge_epu8_test() {
    let arr_v: [uint8x32; 2] = [
        uint8x32 {
            0: [
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01
            ]
        },
        uint8x32 {
            0: [
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01
            ]
        }
    ];

    let left: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32]>(_mm256_cmpge_epu8(left[0], right[0])) }, [
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32]>(_mm256_cmpge_epu8(left[1], right[1])) }, [
        0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmplt_epu8_test() {
    let arr_v: [uint8x32; 2] = [
        uint8x32 {
            0: [
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01
            ]
        },
        uint8x32 {
            0: [
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01
            ]
        }
    ];

    let left: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32]>(_mm256_cmplt_epu8(left[0], right[0])) }, [
        0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32]>(_mm256_cmplt_epu8(left[1], right[1])) }, [
        0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmple_epu8_test() {
    let arr_v: [uint8x32; 2] = [
        uint8x32 {
            0: [
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01
            ]
        },
        uint8x32 {
            0: [
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01
            ]
        }
    ];

    let left: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32]>(_mm256_cmple_epu8(left[0], right[0])) }, [
        0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u8; 32]>(_mm256_cmple_epu8(left[1], right[1])) }, [
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmpgt_epu16_test() {
    let arr_v: [uint16x16; 2] = [
        uint16x16 {
            0: [
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001
            ]
        },
        uint16x16 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                0x8000, 0x8000, 0x8000, 0x8000, 0x0001, 0x0001, 0x0001, 0x0001
            ]
        }
    ];

    let left: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u16; 16]>(_mm256_cmpgt_epu16(left[0], right[0])) }, [
        0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u16; 16]>(_mm256_cmpgt_epu16(left[1], right[1])) }, [
        0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0xFFFF,
        0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmpge_epu16_test() {
    let arr_v: [uint16x16; 2] = [
        uint16x16 {
            0: [
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001
            ]
        },
        uint16x16 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                0x8000, 0x8000, 0x8000, 0x8000, 0x0001, 0x0001, 0x0001, 0x0001
            ]
        }
    ];

    let left: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u16; 16]>(_mm256_cmpge_epu16(left[0], right[0])) }, [
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0x0000,
        0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u16; 16]>(_mm256_cmpge_epu16(left[1], right[1])) }, [
        0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmplt_epu16_test() {
    let arr_v: [uint16x16; 2] = [
        uint16x16 {
            0: [
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001
            ]
        },
        uint16x16 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                0x8000, 0x8000, 0x8000, 0x8000, 0x0001, 0x0001, 0x0001, 0x0001
            ]
        }
    ];

    let left: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u16; 16]>(_mm256_cmplt_epu16(left[0], right[0])) }, [
        0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0xFFFF,
        0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u16; 16]>(_mm256_cmplt_epu16(left[1], right[1])) }, [
        0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmple_epu16_test() {
    let arr_v: [uint16x16; 2] = [
        uint16x16 {
            0: [
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001
            ]
        },
        uint16x16 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                0x8000, 0x8000, 0x8000, 0x8000, 0x0001, 0x0001, 0x0001, 0x0001
            ]
        }
    ];

    let left: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 2] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u16; 16]>(_mm256_cmple_epu16(left[0], right[0])) }, [
        0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u16; 16]>(_mm256_cmple_epu16(left[1], right[1])) }, [
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0x0000,
        0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmpgt_epu32_test() {
    let arr_v: [uint32x8; 3] = [
        uint32x8 {
            0: [
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001, 0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001
            ]
        },
        uint32x8 {
            0: [
                0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
            ]
        },
        uint32x8 {
            0: [
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
            ]
        }
    ];

    let left: [__m256i; 3] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 3] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmpgt_epu32(left[0], right[0])) }, [
        0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmpgt_epu32(left[1], right[1])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmpgt_epu32(left[2], right[2])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmpge_epu32_test() {
    let arr_v: [uint32x8; 3] = [
        uint32x8 {
            0: [
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001, 0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001
            ]
        },
        uint32x8 {
            0: [
                0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
            ]
        },
        uint32x8 {
            0: [
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
            ]
        }
    ];

    let left: [__m256i; 3] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 3] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmpge_epu32(left[0], right[0])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmpge_epu32(left[1], right[1])) }, [
        0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmpge_epu32(left[2], right[2])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmplt_epu32_test() {
    let arr_v: [uint32x8; 3] = [
        uint32x8 {
            0: [
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001, 0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001
            ]
        },
        uint32x8 {
            0: [
                0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
            ]
        },
        uint32x8 {
            0: [
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
            ]
        }
    ];

    let left: [__m256i; 3] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 3] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmplt_epu32(left[0], right[0])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmplt_epu32(left[1], right[1])) }, [
        0xFFFFFFFF, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmplt_epu32(left[2], right[2])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmple_epu32_test() {
    let arr_v: [uint32x8; 3] = [
        uint32x8 {
            0: [
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001, 0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001
            ]
        },
        uint32x8 {
            0: [
                0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
            ]
        },
        uint32x8 {
            0: [
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
            ]
        }
    ];

    let left: [__m256i; 3] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 3] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmple_epu32(left[0], right[0])) }, [
        0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmple_epu32(left[1], right[1])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u32; 8]>(_mm256_cmple_epu32(left[2], right[2])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmpgt_epu64_test() {
    let arr_v: [uint64x4; 4] = [
        uint64x4 {
            0: [
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x8000000000000000, 0x0000000000000001
            ]
        },
        uint64x4 {
            0: [
                0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
            ]
        },
        uint64x4 {
            0: [
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
            ]
        },
        uint64x4 {
            0: [
                0x8000000000000000, 0x0000000000000001, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[3].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmpgt_epu64(left[0], right[0])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmpgt_epu64(left[1], right[1])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmpgt_epu64(left[2], right[2])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmpgt_epu64(left[3], right[3])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0x0000000000000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmpge_epu64_test() {
    let arr_v: [uint64x4; 4] = [
        uint64x4 {
            0: [
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x8000000000000000, 0x0000000000000001
            ]
        },
        uint64x4 {
            0: [
                0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
            ]
        },
        uint64x4 {
            0: [
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
            ]
        },
        uint64x4 {
            0: [
                0x8000000000000000, 0x0000000000000001, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[3].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmpge_epu64(left[0], right[0])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmpge_epu64(left[1], right[1])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmpge_epu64(left[2], right[2])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmpge_epu64(left[3], right[3])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmplt_epu64_test() {
    let arr_v: [uint64x4; 4] = [
        uint64x4 {
            0: [
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x8000000000000000, 0x0000000000000001
            ]
        },
        uint64x4 {
            0: [
                0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
            ]
        },
        uint64x4 {
            0: [
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
            ]
        },
        uint64x4 {
            0: [
                0x8000000000000000, 0x0000000000000001, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[3].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmplt_epu64(left[0], right[0])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmplt_epu64(left[1], right[1])) }, [
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmplt_epu64(left[2], right[2])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmplt_epu64(left[3], right[3])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[test]
fn _mm256_cmple_epu64_test() {
    let arr_v: [uint64x4; 4] = [
        uint64x4 {
            0: [
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x8000000000000000, 0x0000000000000001
            ]
        },
        uint64x4 {
            0: [
                0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
            ]
        },
        uint64x4 {
            0: [
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
            ]
        },
        uint64x4 {
            0: [
                0x8000000000000000, 0x0000000000000001, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[0].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>())
    ] };
    let right: [__m256i; 4] = unsafe { [
        _mm256_load_si256(arr_v[1].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[2].0.as_ptr().cast::<__m256i>()),
        _mm256_load_si256(arr_v[3].0.as_ptr().cast::<__m256i>())
    ] };

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmple_epu64(left[0], right[0])) }, [
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmple_epu64(left[1], right[1])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmple_epu64(left[2], right[2])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m256i, [u64; 4]>(_mm256_cmple_epu64(left[3], right[3])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmpgt_epi8_test() {
    let arr_v: [int8x64; 2] = [
        int8x64 {
            0: [
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F,
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F,
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F,
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F
            ]
        },
        int8x64 {
            0: [
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F,
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F,
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F,
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmpgt_epi8(left[0], right[0])) }, [
        0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmpgt_epi8(left[1], right[1])) }, [
        0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00,
        0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00,
        0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00,
        0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmpge_epi8_test() {
    let arr_v: [int8x64; 2] = [
        int8x64 {
            0: [
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F,
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F,
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F,
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F
            ]
        },
        int8x64 {
            0: [
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F,
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F,
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F,
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmpge_epi8(left[0], right[0])) }, [
        0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmpge_epi8(left[1], right[1])) }, [
        0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmpeq_epi8_test() {
    let arr_v: [int8x64; 2] = [
        int8x64 {
            0: [
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F,
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F,
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F,
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F
            ]
        },
        int8x64 {
            0: [
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F,
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F,
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F,
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmpeq_epi8(left[0], right[0])) }, [
        0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmpeq_epi8(left[1], right[1])) }, [
        0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmplt_epi8_test() {
    let arr_v: [int8x64; 2] = [
        int8x64 {
            0: [
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F,
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F,
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F,
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F
            ]
        },
        int8x64 {
            0: [
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F,
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F,
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F,
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmplt_epi8(left[0], right[0])) }, [
        0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00,
        0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00,
        0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00,
        0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmplt_epi8(left[1], right[1])) }, [
        0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmple_epi8_test() {
    let arr_v: [int8x64; 2] = [
        int8x64 {
            0: [
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F,
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F,
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F,
                0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F, 0x00, -0x01, -0x80, 0x7F
            ]
        },
        int8x64 {
            0: [
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F,
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F,
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F,
                0x00, 0x00, 0x00, 0x00, -0x01, -0x01, -0x01, -0x01, -0x80, -0x80, -0x80, -0x80, 0x7F, 0x7F, 0x7F, 0x7F
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmple_epi8(left[0], right[0])) }, [
        0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmple_epi8(left[1], right[1])) }, [
        0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmpgt_epi16_test() {
    let arr_v: [int16x32; 2] = [
        int16x32 {
            0: [
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF,
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF,
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF,
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF
            ]
        },
        int16x32 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, -0x0001, -0x0001, -0x0001, -0x0001,
                -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF,
                0x0000, 0x0000, 0x0000, 0x0000, -0x0001, -0x0001, -0x0001, -0x0001,
                -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmpgt_epi16(left[0], right[0])) }, [
        0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF,
        0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF,
        0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmpgt_epi16(left[1], right[1])) }, [
        0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000,
        0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmpge_epi16_test() {
    let arr_v: [int16x32; 2] = [
        int16x32 {
            0: [
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF,
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF,
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF,
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF
            ]
        },
        int16x32 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, -0x0001, -0x0001, -0x0001, -0x0001,
                -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF,
                0x0000, 0x0000, 0x0000, 0x0000, -0x0001, -0x0001, -0x0001, -0x0001,
                -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmpge_epi16(left[0], right[0])) }, [
        0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF,
        0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmpge_epi16(left[1], right[1])) }, [
        0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000,
        0x0000, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000,
        0x0000, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmpeq_epi16_test() {
    let arr_v: [int16x32; 2] = [
        int16x32 {
            0: [
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF,
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF,
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF,
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF
            ]
        },
        int16x32 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, -0x0001, -0x0001, -0x0001, -0x0001,
                -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF,
                0x0000, 0x0000, 0x0000, 0x0000, -0x0001, -0x0001, -0x0001, -0x0001,
                -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmpeq_epi16(left[0], right[0])) }, [
        0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0x0000,
        0x0000, 0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF,
        0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0x0000,
        0x0000, 0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmpeq_epi16(left[1], right[1])) }, [
        0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0x0000,
        0x0000, 0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF,
        0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0x0000,
        0x0000, 0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmplt_epi16_test() {
    let arr_v: [int16x32; 2] = [
        int16x32 {
            0: [
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF,
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF,
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF,
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF
            ]
        },
        int16x32 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, -0x0001, -0x0001, -0x0001, -0x0001,
                -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF,
                0x0000, 0x0000, 0x0000, 0x0000, -0x0001, -0x0001, -0x0001, -0x0001,
                -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmplt_epi16(left[0], right[0])) }, [
        0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000,
        0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmplt_epi16(left[1], right[1])) }, [
        0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF,
        0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF,
        0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmple_epi16_test() {
    let arr_v: [int16x32; 2] = [
        int16x32 {
            0: [
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF,
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF,
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF,
                0x0000, -0x0001, -0x8000, 0x7FFF, 0x0000, -0x0001, -0x8000, 0x7FFF
            ]
        },
        int16x32 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, -0x0001, -0x0001, -0x0001, -0x0001,
                -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF,
                0x0000, 0x0000, 0x0000, 0x0000, -0x0001, -0x0001, -0x0001, -0x0001,
                -0x8000, -0x8000, -0x8000, -0x8000, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmple_epi16(left[0], right[0])) }, [
        0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000,
        0x0000, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000,
        0x0000, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmple_epi16(left[1], right[1])) }, [
        0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF,
        0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmpgt_epu8_test() {
    let arr_v: [uint8x64; 2] = [
        uint8x64 {
            0: [
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01
            ]
        },
        uint8x64 {
            0: [
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmpgt_epu8(left[0], right[0])) }, [
        0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmpgt_epu8(left[1], right[1])) }, [
        0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmpge_epu8_test() {
    let arr_v: [uint8x64; 2] = [
        uint8x64 {
            0: [
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01
            ]
        },
        uint8x64 {
            0: [
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmpge_epu8(left[0], right[0])) }, [
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmpge_epu8(left[1], right[1])) }, [
        0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmpeq_epu8_test() {
    let arr_v: [uint8x64; 3] = [
        uint8x64 {
            0: [
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01
            ]
        },
        uint8x64 {
            0: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
            ]
        },
        uint8x64 {
            0: [
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmpeq_epu8(left[0], right[0])) }, [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmpeq_epu8(left[1], right[1])) }, [
        0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00,
        0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00,
        0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00,
        0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmpeq_epu8(left[2], right[2])) }, [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmplt_epu8_test() {
    let arr_v: [uint8x64; 2] = [
        uint8x64 {
            0: [
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01
            ]
        },
        uint8x64 {
            0: [
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmplt_epu8(left[0], right[0])) }, [
        0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmplt_epu8(left[1], right[1])) }, [
        0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmple_epu8_test() {
    let arr_v: [uint8x64; 2] = [
        uint8x64 {
            0: [
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01,
                0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01, 0x00, 0xFF, 0x80, 0x01
            ]
        },
        uint8x64 {
            0: [
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x80, 0x80, 0x80, 0x01, 0x01, 0x01, 0x01
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmple_epu8(left[0], right[0])) }, [
        0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
        0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u8; 64]>(_mm512_cmple_epu8(left[1], right[1])) }, [
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmpgt_epu16_test() {
    let arr_v: [uint16x32; 2] = [
        uint16x32 {
            0: [
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001
            ]
        },
        uint16x32 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                0x8000, 0x8000, 0x8000, 0x8000, 0x0001, 0x0001, 0x0001, 0x0001,
                0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                0x8000, 0x8000, 0x8000, 0x8000, 0x0001, 0x0001, 0x0001, 0x0001
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmpgt_epu16(left[0], right[0])) }, [
        0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000,
        0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmpgt_epu16(left[1], right[1])) }, [
        0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0xFFFF,
        0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0xFFFF,
        0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmpge_epu16_test() {
    let arr_v: [uint16x32; 2] = [
        uint16x32 {
            0: [
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001
            ]
        },
        uint16x32 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                0x8000, 0x8000, 0x8000, 0x8000, 0x0001, 0x0001, 0x0001, 0x0001,
                0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                0x8000, 0x8000, 0x8000, 0x8000, 0x0001, 0x0001, 0x0001, 0x0001
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmpge_epu16(left[0], right[0])) }, [
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0x0000,
        0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0x0000,
        0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmpge_epu16(left[1], right[1])) }, [
        0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF,
        0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmpeq_epu16_test() {
    let arr_v: [uint16x32; 3] = [
        uint16x32 {
            0: [
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001
            ]
        },
        uint16x32 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000
            ]
        },
        uint16x32 {
            0: [
                0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmpeq_epu16(left[0], right[0])) }, [
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmpeq_epu16(left[1], right[1])) }, [
        0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0x0000,
        0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0x0000,
        0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0x0000,
        0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0x0000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmpeq_epu16(left[2], right[2])) }, [
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmplt_epu16_test() {
    let arr_v: [uint16x32; 2] = [
        uint16x32 {
            0: [
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001
            ]
        },
        uint16x32 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                0x8000, 0x8000, 0x8000, 0x8000, 0x0001, 0x0001, 0x0001, 0x0001,
                0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                0x8000, 0x8000, 0x8000, 0x8000, 0x0001, 0x0001, 0x0001, 0x0001
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmplt_epu16(left[0], right[0])) }, [
        0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0xFFFF,
        0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0xFFFF,
        0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmplt_epu16(left[1], right[1])) }, [
        0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000,
        0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512bw"))]
#[test]
fn _mm512_cmple_epu16_test() {
    let arr_v: [uint16x32; 2] = [
        uint16x32 {
            0: [
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001,
                0x0000, 0xFFFF, 0x8000, 0x0001, 0x0000, 0xFFFF, 0x8000, 0x0001
            ]
        },
        uint16x32 {
            0: [
                0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                0x8000, 0x8000, 0x8000, 0x8000, 0x0001, 0x0001, 0x0001, 0x0001,
                0x0000, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                0x8000, 0x8000, 0x8000, 0x8000, 0x0001, 0x0001, 0x0001, 0x0001
            ]
        }
    ];

    let left: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 2] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmple_epu16(left[0], right[0])) }, [
        0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF,
        0xFFFF, 0x0000, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u16; 32]>(_mm512_cmple_epu16(left[1], right[1])) }, [
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0x0000,
        0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF,
        0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0x0000,
        0x0000, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmpgt_epi32_test() {
    let arr_v: [int32x16; 3] = [
        int32x16 {
            0: [
                -0x80000000, 0x7FFFFFFF, 0x00000000, -0x00000001, 0x00000001, -0x80000000, 0x7FFFFFFF, 0x00000000,
                -0x80000000, 0x7FFFFFFF, 0x00000000, -0x00000001, 0x00000001, -0x80000000, 0x7FFFFFFF, 0x00000000
            ]
        },
        int32x16 {
            0: [
                -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000,
                -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000
            ]
        },
        int32x16 {
            0: [
                0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF,
                0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpgt_epi32(left[0], right[0])) }, [
        0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF,
        0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpgt_epi32(left[1], right[1])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpgt_epi32(left[2], right[2])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmpge_epi32_test() {
    let arr_v: [int32x16; 3] = [
        int32x16 {
            0: [
                -0x80000000, 0x7FFFFFFF, 0x00000000, -0x00000001, 0x00000001, -0x80000000, 0x7FFFFFFF, 0x00000000,
                -0x80000000, 0x7FFFFFFF, 0x00000000, -0x00000001, 0x00000001, -0x80000000, 0x7FFFFFFF, 0x00000000
            ]
        },
        int32x16 {
            0: [
                -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000,
                -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000
            ]
        },
        int32x16 {
            0: [
                0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF,
                0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpge_epi32(left[0], right[0])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpge_epi32(left[1], right[1])) }, [
        0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000,
        0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpge_epi32(left[2], right[2])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmpeq_epi32_test() {
    let arr_v: [int32x16; 3] = [
        int32x16 {
            0: [
                -0x80000000, 0x7FFFFFFF, 0x00000000, -0x00000001, 0x00000001, -0x80000000, 0x7FFFFFFF, 0x00000000,
                -0x80000000, 0x7FFFFFFF, 0x00000000, -0x00000001, 0x00000001, -0x80000000, 0x7FFFFFFF, 0x00000000
            ]
        },
        int32x16 {
            0: [
                -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000,
                -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000
            ]
        },
        int32x16 {
            0: [
                0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF,
                0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpeq_epi32(left[0], right[0])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpeq_epi32(left[1], right[1])) }, [
        0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000,
        0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpeq_epi32(left[2], right[2])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmplt_epi32_test() {
    let arr_v: [int32x16; 3] = [
        int32x16 {
            0: [
                -0x80000000, 0x7FFFFFFF, 0x00000000, -0x00000001, 0x00000001, -0x80000000, 0x7FFFFFFF, 0x00000000,
                -0x80000000, 0x7FFFFFFF, 0x00000000, -0x00000001, 0x00000001, -0x80000000, 0x7FFFFFFF, 0x00000000
            ]
        },
        int32x16 {
            0: [
                -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000,
                -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000
            ]
        },
        int32x16 {
            0: [
                0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF,
                0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmplt_epi32(left[0], right[0])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmplt_epi32(left[1], right[1])) }, [
        0xFFFFFFFF, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0xFFFFFFFF,
        0xFFFFFFFF, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmplt_epi32(left[2], right[2])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmple_epi32_test() {
    let arr_v: [int32x16; 3] = [
        int32x16 {
            0: [
                -0x80000000, 0x7FFFFFFF, 0x00000000, -0x00000001, 0x00000001, -0x80000000, 0x7FFFFFFF, 0x00000000,
                -0x80000000, 0x7FFFFFFF, 0x00000000, -0x00000001, 0x00000001, -0x80000000, 0x7FFFFFFF, 0x00000000
            ]
        },
        int32x16 {
            0: [
                -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000,
                -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000, -0x80000000
            ]
        },
        int32x16 {
            0: [
                0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF,
                0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmple_epi32(left[0], right[0])) }, [
        0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000,
        0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmple_epi32(left[1], right[1])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmple_epi32(left[2], right[2])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmpgt_epi64_test() {
    let arr_v: [int64x8; 3] = [
        int64x8 {
            0: [
                -0x8000000000000000, 0x7FFFFFFFFFFFFFFF, 0x0000000000000000, -0x0000000000000001,
                0x0000000000000001, -0x8000000000000000, 0x7FFFFFFFFFFFFFFF, 0x0000000000000000
            ]
        },
        int64x8 {
            0: [
                -0x8000000000000000, -0x8000000000000000, -0x8000000000000000, -0x8000000000000000,
                -0x8000000000000000, -0x8000000000000000, -0x8000000000000000, -0x8000000000000000
            ]
        },
        int64x8 {
            0: [
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF,
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpgt_epi64(left[0], right[0])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpgt_epi64(left[1], right[1])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpgt_epi64(left[2], right[2])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmpge_epi64_test() {
    let arr_v: [int64x8; 3] = [
        int64x8 {
            0: [
                -0x8000000000000000, 0x7FFFFFFFFFFFFFFF, 0x0000000000000000, -0x0000000000000001,
                0x0000000000000001, -0x8000000000000000, 0x7FFFFFFFFFFFFFFF, 0x0000000000000000
            ]
        },
        int64x8 {
            0: [
                -0x8000000000000000, -0x8000000000000000, -0x8000000000000000, -0x8000000000000000,
                -0x8000000000000000, -0x8000000000000000, -0x8000000000000000, -0x8000000000000000
            ]
        },
        int64x8 {
            0: [
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF,
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpge_epi64(left[0], right[0])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpge_epi64(left[1], right[1])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpge_epi64(left[2], right[2])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmpeq_epi64_test() {
    let arr_v: [int64x8; 3] = [
        int64x8 {
            0: [
                -0x8000000000000000, 0x7FFFFFFFFFFFFFFF, 0x0000000000000000, -0x0000000000000001,
                -0x8000000000000000, 0x7FFFFFFFFFFFFFFF, 0x0000000000000000, -0x0000000000000001
            ]
        },
        int64x8 {
            0: [
                -0x8000000000000000, -0x8000000000000000, -0x8000000000000000, -0x8000000000000000,
                -0x8000000000000000, -0x8000000000000000, -0x8000000000000000, -0x8000000000000000
            ]
        },
        int64x8 {
            0: [
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF,
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpeq_epi64(left[0], right[0])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpeq_epi64(left[1], right[1])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpeq_epi64(left[2], right[2])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmplt_epi64_test() {
    let arr_v: [int64x8; 3] = [
        int64x8 {
            0: [
                -0x8000000000000000, 0x7FFFFFFFFFFFFFFF, 0x0000000000000000, -0x0000000000000001,
                0x0000000000000001, -0x8000000000000000, 0x7FFFFFFFFFFFFFFF, 0x0000000000000000
            ]
        },
        int64x8 {
            0: [
                -0x8000000000000000, -0x8000000000000000, -0x8000000000000000, -0x8000000000000000,
                -0x8000000000000000, -0x8000000000000000, -0x8000000000000000, -0x8000000000000000
            ]
        },
        int64x8 {
            0: [
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF,
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmplt_epi64(left[0], right[0])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmplt_epi64(left[1], right[1])) }, [
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmplt_epi64(left[2], right[2])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmple_epi64_test() {
    let arr_v: [int64x8; 3] = [
        int64x8 {
            0: [
                -0x8000000000000000, 0x7FFFFFFFFFFFFFFF, 0x0000000000000000, -0x0000000000000001,
                0x0000000000000001, -0x8000000000000000, 0x7FFFFFFFFFFFFFFF, 0x0000000000000000
            ]
        },
        int64x8 {
            0: [
                -0x8000000000000000, -0x8000000000000000, -0x8000000000000000, -0x8000000000000000,
                -0x8000000000000000, -0x8000000000000000, -0x8000000000000000, -0x8000000000000000
            ]
        },
        int64x8 {
            0: [
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF,
                0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF, 0x7FFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmple_epi64(left[0], right[0])) }, [
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmple_epi64(left[1], right[1])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmple_epi64(left[2], right[2])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmpgt_epu32_test() {
    let arr_v: [uint32x16; 3] = [
        uint32x16 {
            0: [
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001, 0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001,
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001, 0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001
            ]
        },
        uint32x16 {
            0: [
                0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
                0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
            ]
        },
        uint32x16 {
            0: [
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpgt_epu32(left[0], right[0])) }, [
        0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
        0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpgt_epu32(left[1], right[1])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpgt_epu32(left[2], right[2])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmpge_epu32_test() {
    let arr_v: [uint32x16; 3] = [
        uint32x16 {
            0: [
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001, 0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001,
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001, 0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001
            ]
        },
        uint32x16 {
            0: [
                0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
                0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
            ]
        },
        uint32x16 {
            0: [
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpge_epu32(left[0], right[0])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpge_epu32(left[1], right[1])) }, [
        0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000,
        0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpge_epu32(left[2], right[2])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmpeq_epu32_test() {
    let arr_v: [uint32x16; 3] = [
        uint32x16 {
            0: [
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001, 0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001,
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001, 0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001
            ]
        },
        uint32x16 {
            0: [
                0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
                0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
            ]
        },
        uint32x16 {
            0: [
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpeq_epu32(left[0], right[0])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpeq_epu32(left[1], right[1])) }, [
        0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000,
        0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmpeq_epu32(left[2], right[2])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmplt_epu32_test() {
    let arr_v: [uint32x16; 3] = [
        uint32x16 {
            0: [
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001, 0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001,
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001, 0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001
            ]
        },
        uint32x16 {
            0: [
                0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
                0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
            ]
        },
        uint32x16 {
            0: [
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmplt_epu32(left[0], right[0])) }, [
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmplt_epu32(left[1], right[1])) }, [
        0xFFFFFFFF, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF,
        0xFFFFFFFF, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmplt_epu32(left[2], right[2])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmple_epu32_test() {
    let arr_v: [uint32x16; 3] = [
        uint32x16 {
            0: [
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001, 0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001,
                0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001, 0x00000000, 0xFFFFFFFF, 0x80000000, 0x00000001
            ]
        },
        uint32x16 {
            0: [
                0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
                0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000
            ]
        },
        uint32x16 {
            0: [
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
                0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmple_epu32(left[0], right[0])) }, [
        0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000,
        0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmple_epu32(left[1], right[1])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u32; 16]>(_mm512_cmple_epu32(left[2], right[2])) }, [
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
        0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmpgt_epu64_test() {
    let arr_v: [uint64x8; 4] = [
        uint64x8 {
            0: [
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x8000000000000000, 0x0000000000000001,
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x8000000000000000, 0x0000000000000001
            ]
        },
        uint64x8 {
            0: [
                0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
                0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
            ]
        },
        uint64x8 {
            0: [
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
            ]
        },
        uint64x8 {
            0: [
                0x8000000000000000, 0x0000000000000001, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF,
                0x8000000000000000, 0x0000000000000001, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m512i; 4] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 4] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[3].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpgt_epu64(left[0], right[0])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpgt_epu64(left[1], right[1])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpgt_epu64(left[2], right[2])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpgt_epu64(left[3], right[3])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0x0000000000000000,
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0x0000000000000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmpge_epu64_test() {
    let arr_v: [uint64x8; 4] = [
        uint64x8 {
            0: [
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x8000000000000000, 0x0000000000000001,
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x8000000000000000, 0x0000000000000001
            ]
        },
        uint64x8 {
            0: [
                0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
                0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
            ]
        },
        uint64x8 {
            0: [
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
            ]
        },
        uint64x8 {
            0: [
                0x8000000000000000, 0x0000000000000001, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF,
                0x8000000000000000, 0x0000000000000001, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m512i; 4] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 4] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[3].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpge_epu64(left[0], right[0])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpge_epu64(left[1], right[1])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpge_epu64(left[2], right[2])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpge_epu64(left[3], right[3])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmpeq_epu64_test() {
    let arr_v: [uint64x8; 3] = [
        uint64x8 {
            0: [
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x8000000000000000, 0x0000000000000001,
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x8000000000000000, 0x0000000000000001
            ]
        },
        uint64x8 {
            0: [
                0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
                0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
            ]
        },
        uint64x8 {
            0: [
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 3] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpeq_epu64(left[0], right[0])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpeq_epu64(left[1], right[1])) }, [
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmpeq_epu64(left[2], right[2])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmplt_epu64_test() {
    let arr_v: [uint64x8; 4] = [
        uint64x8 {
            0: [
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x8000000000000000, 0x0000000000000001,
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x8000000000000000, 0x0000000000000001
            ]
        },
        uint64x8 {
            0: [
                0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
                0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
            ]
        },
        uint64x8 {
            0: [
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
            ]
        },
        uint64x8 {
            0: [
                0x8000000000000000, 0x0000000000000001, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF,
                0x8000000000000000, 0x0000000000000001, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m512i; 4] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 4] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[3].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmplt_epu64(left[0], right[0])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmplt_epu64(left[1], right[1])) }, [
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmplt_epu64(left[2], right[2])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmplt_epu64(left[3], right[3])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[test]
fn _mm512_cmple_epu64_test() {
    let arr_v: [uint64x8; 4] = [
        uint64x8 {
            0: [
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x8000000000000000, 0x0000000000000001,
                0x0000000000000000, 0xFFFFFFFFFFFFFFFF, 0x8000000000000000, 0x0000000000000001
            ]
        },
        uint64x8 {
            0: [
                0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
                0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
            ]
        },
        uint64x8 {
            0: [
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
                0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
            ]
        },
        uint64x8 {
            0: [
                0x8000000000000000, 0x0000000000000001, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF,
                0x8000000000000000, 0x0000000000000001, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF
            ]
        }
    ];

    let left: [__m512i; 4] = unsafe { [
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[0].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>())
    ] };
    let right: [__m512i; 4] = unsafe { [
        _mm512_load_si512(arr_v[1].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[2].0.as_ptr().cast::<__m512i>()),
        _mm512_load_si512(arr_v[3].0.as_ptr().cast::<__m512i>())
    ] };

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmple_epu64(left[0], right[0])) }, [
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000,
        0xFFFFFFFFFFFFFFFF, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmple_epu64(left[1], right[1])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmple_epu64(left[2], right[2])) }, [
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF
    ]);

    assert_eq!(unsafe { transmute::<__m512i, [u64; 8]>(_mm512_cmple_epu64(left[3], right[3])) }, [
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF,
        0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0xFFFFFFFFFFFFFFFF
    ]);
}
