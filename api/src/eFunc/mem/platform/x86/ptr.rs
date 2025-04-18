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
use crate::{
    eSIMD::{
        rolled::{
            extract::{
                _mm_arvext_epi8,
                _mm_arvext_epi16,
                _mm_arvext_epi32,
                _mm_arvext_epi64
            }
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
use crate::{
    eSIMD::{
        rolled::{
            extract::{
                _mm256_arvext_epi8,
                _mm256_arvext_epi16,
                _mm256_arvext_epi32,
                _mm256_arvext_epi64,
                _mm256_arvext_epi128
            }
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
use crate::{
    eSIMD::{
        rolled::{
            extract::{
                _mm512_arvext_epi8,
                _mm512_arvext_epi16,
                _mm512_arvext_epi32,
                _mm512_arvext_epi64,
                _mm512_arvext_epi128,
                // _mm512_arvext_epi256 - For the future
            }
        }
    }
};

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
            _mm_load_si128,
            _mm_loadu_si128,
            _mm_store_si128
        }
    }
};

#[cfg(all(target_arch = "x86", target_feature = "avx", target_feature = "avx2"))]
use core::{
    arch::{
        x86::{
            __m256i,
            _mm256_load_si256,
            _mm256_loadu_si256,
            _mm256_store_si256
        }
    }
};

#[cfg(all(target_arch = "x86", target_feature = "avx512f", target_feature = "avx512bw"))]
use core::{
    arch::{
        x86::{
            __m512i,
            _mm512_load_si512,
            _mm512_loadu_si512,
            _mm512_store_si512
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use core::{
    arch::{
        x86_64::{
            __m128i,
            _mm_load_si128,
            _mm_loadu_si128,
            _mm_store_si128
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "avx", target_feature = "avx2"))]
use core::{
    arch::{
        x86_64::{
            __m256i,
            _mm256_load_si256,
            _mm256_loadu_si256,
            _mm256_store_si256
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f", target_feature = "avx512bw"))]
use core::{
    arch::{
        x86_64::{
            __m512i,
            _mm512_load_si512,
            _mm512_loadu_si512,
            _mm512_store_si512
        }
    }
};

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

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[repr(align(32))]
struct uint128x2([u128; 2_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[repr(align(64))]
struct uint8x64([u8; 64_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[repr(align(64))]
struct uint16x32([u16; 32_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[repr(align(64))]
struct uint32x16([u32; 16_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[repr(align(64))]
struct uint64x8([u64; 8_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
#[repr(align(64))]
struct uint128x4([u128; 4_usize]);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
macro_rules! align_remainder_sse2 {
    (
        $functor_name:ident,
        $pointer_type:ty,
        $register_pointer_type:ty,
        $substitution_type:ty,
        $aligned_buffer_type:ident,
        $load_function:ident,
        $loadu_function:ident,
        $store_function:ident,
        $align_function:ident,
        $total_register_items:expr
    ) => {
        pub unsafe fn $functor_name(data_ptr: *const $pointer_type, substitution_data: $substitution_type, length: usize) -> $substitution_type {
            let remainder: usize = length % $total_register_items;

            if remainder != 0_usize {
                let (mut index, mut buffer): (usize, $aligned_buffer_type) = (0_usize, $aligned_buffer_type { 0: substitution_data });

                if length >= $total_register_items {
                    $store_function(buffer.0.as_mut_ptr().cast::<$register_pointer_type>(), $align_function($loadu_function(data_ptr.add(length - $total_register_items).cast::<$register_pointer_type>()), $load_function(buffer.0.as_ptr().cast::<$register_pointer_type>()), $total_register_items - remainder));
                } else {
                    while index < length { buffer.0[index] = *data_ptr.add(index); index += 1_usize; }
                }

                return buffer.0;
            } else {
                return substitution_data;
            }
        }
    };
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
macro_rules! align_remainder_avx2 {
    (
        $functor_name:ident,
        $pointer_type:ty,
        $register_pointer_type:ty,
        $substitution_type:ty,
        $aligned_buffer_type:ident,
        $load_function:ident,
        $loadu_function:ident,
        $store_function:ident,
        $align_function:ident,
        $total_register_items:expr,
        $sse2_register_items:expr
    ) => {
        pub unsafe fn $functor_name(data_ptr: *const $pointer_type, substitution_data: $substitution_type, length: usize) -> $substitution_type {
            let remainder: usize = length % $total_register_items;

            if remainder != 0_usize {
                let (mut index, mut buffer): (usize, $aligned_buffer_type) = (0_usize, $aligned_buffer_type { 0: substitution_data });

                if length >= $total_register_items {
                    $store_function(buffer.0.as_mut_ptr().cast::<$register_pointer_type>(), $align_function($loadu_function(data_ptr.add(length - $total_register_items).cast::<$register_pointer_type>()), $load_function(buffer.0.as_ptr().cast::<$register_pointer_type>()), $total_register_items - remainder));
                } else {
                    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
                    if remainder >= $sse2_register_items {
                        _mm_store_si128(buffer.0.as_mut_ptr().cast::<__m128i>(), _mm_loadu_si128(data_ptr.cast::<__m128i>())); index += $sse2_register_items;
                    }

                    while index < length { buffer.0[index] = *data_ptr.add(index); index += 1_usize; }
                }

                return buffer.0;
            } else {
                return substitution_data;
            }
        }
    };
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
macro_rules! align_remainder_avx512 {
    (
        $functor_name:ident,
        $pointer_type:ty,
        $load_pointer_type:ty,
        $register_pointer_type:ty,
        $substitution_type:ty,
        $aligned_buffer_type:ident,
        $load_function:ident,
        $loadu_function:ident,
        $store_function:ident,
        $align_function:ident,
        $total_register_items:expr,
        $sse2_register_items:expr,
        $avx2_register_items:expr
    ) => {
        pub unsafe fn $functor_name(data_ptr: *const $pointer_type, substitution_data: $substitution_type, length: usize) -> $substitution_type {
            let remainder: usize = length % $total_register_items;

            if remainder != 0_usize {
                let (mut index, mut buffer): (usize, $aligned_buffer_type) = (0_usize, $aligned_buffer_type { 0: substitution_data });

                if length >= $total_register_items {
                    $store_function(buffer.0.as_mut_ptr().cast::<$register_pointer_type>(), $align_function($loadu_function(data_ptr.add(length - $total_register_items).cast::<$load_pointer_type>()), $load_function(buffer.0.as_ptr().cast::<$load_pointer_type>()), $total_register_items - remainder));
                } else {
                    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
                    if remainder >= $avx2_register_items {
                        _mm256_store_si256(buffer.0.as_mut_ptr().cast::<__m256i>(), _mm256_loadu_si256(data_ptr.cast::<__m256i>())); index += $avx2_register_items;
                    }

                    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
                    if remainder - index >= $sse2_register_items {
                        _mm_store_si128(buffer.0.as_mut_ptr().add(index).cast::<__m128i>(), _mm_loadu_si128(data_ptr.add(index).cast::<__m128i>())); index += $sse2_register_items;
                        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(all(target_feature = "avx", target_feature = "avx2"))))]
                        if remainder - index >= $sse2_register_items {
                            _mm_store_si128(buffer.0.as_mut_ptr().add(index).cast::<__m128i>(), _mm_loadu_si128(data_ptr.add(index).cast::<__m128i>())); index += $sse2_register_items;
                            if remainder - index >= $sse2_register_items {
                                _mm_store_si128(buffer.0.as_mut_ptr().add(index).cast::<__m128i>(), _mm_loadu_si128(data_ptr.add(index).cast::<__m128i>())); index += $sse2_register_items;
                            }
                        }
                    }

                    while index < length { buffer.0[index] = *data_ptr.add(index); index += 1_usize; }
                }

                return buffer.0;
            } else {
                return substitution_data;
            }
        }
    };
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
align_remainder_sse2!(align_remainder_8x16, u8, __m128i, [u8; 16_usize], uint8x16, _mm_load_si128, _mm_loadu_si128, _mm_store_si128, _mm_arvext_epi8, 16_usize);
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
align_remainder_sse2!(align_remainder_16x8, u16, __m128i, [u16; 8_usize], uint16x8, _mm_load_si128, _mm_loadu_si128, _mm_store_si128, _mm_arvext_epi16, 8_usize);
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
align_remainder_sse2!(align_remainder_32x4, u32, __m128i, [u32; 4_usize], uint32x4, _mm_load_si128, _mm_loadu_si128, _mm_store_si128, _mm_arvext_epi32, 4_usize);
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
align_remainder_sse2!(align_remainder_64x2, u64, __m128i, [u64; 2_usize], uint64x2, _mm_load_si128, _mm_loadu_si128, _mm_store_si128, _mm_arvext_epi64, 2_usize);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
align_remainder_avx2!(align_remainder_8x32, u8, __m256i, [u8; 32_usize], uint8x32, _mm256_load_si256, _mm256_loadu_si256, _mm256_store_si256, _mm256_arvext_epi8, 32_usize, 16_usize);
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
align_remainder_avx2!(align_remainder_16x16, u16, __m256i, [u16; 16_usize], uint16x16, _mm256_load_si256, _mm256_loadu_si256, _mm256_store_si256, _mm256_arvext_epi16, 16_usize, 8_usize);
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
align_remainder_avx2!(align_remainder_32x8, u32, __m256i, [u32; 8_usize], uint32x8, _mm256_load_si256, _mm256_loadu_si256, _mm256_store_si256, _mm256_arvext_epi32, 8_usize, 4_usize);
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
align_remainder_avx2!(align_remainder_64x4, u64, __m256i, [u64; 4_usize], uint64x4, _mm256_load_si256, _mm256_loadu_si256, _mm256_store_si256, _mm256_arvext_epi64, 4_usize, 2_usize);
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
align_remainder_avx2!(align_remainder_128x2, u128, __m256i, [u128; 2_usize], uint128x2, _mm256_load_si256, _mm256_loadu_si256, _mm256_store_si256, _mm256_arvext_epi128, 2_usize, 1_usize);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
align_remainder_avx512!(align_remainder_8x64, u8, i32, __m512i, [u8; 64_usize], uint8x64, _mm512_load_si512, _mm512_loadu_si512, _mm512_store_si512, _mm512_arvext_epi8, 64_usize, 32_usize, 16_usize);
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
align_remainder_avx512!(align_remainder_16x32, u16, i32, __m512i, [u16; 32_usize], uint16x32, _mm512_load_si512, _mm512_loadu_si512, _mm512_store_si512, _mm512_arvext_epi16, 32_usize, 16_usize, 8_usize);
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
align_remainder_avx512!(align_remainder_32x16, u32, i32, __m512i, [u32; 16_usize], uint32x16, _mm512_load_si512, _mm512_loadu_si512, _mm512_store_si512, _mm512_arvext_epi32, 16_usize, 8_usize, 4_usize);
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
align_remainder_avx512!(align_remainder_64x8, u64, i32, __m512i, [u64; 8_usize], uint64x8, _mm512_load_si512, _mm512_loadu_si512, _mm512_store_si512, _mm512_arvext_epi64, 8_usize, 4_usize, 2_usize);
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
align_remainder_avx512!(align_remainder_128x4, u128, i32, __m512i, [u128; 4_usize], uint128x4, _mm512_load_si512, _mm512_loadu_si512, _mm512_store_si512, _mm512_arvext_epi128, 4_usize, 2_usize, 1_usize);