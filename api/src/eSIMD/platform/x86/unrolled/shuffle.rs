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

#[cfg(all(target_arch = "x86", target_feature = "sse2", not(target_feature = "ssse3")))]
use core::{
    arch::{
        x86::{
            __m128i,
            _mm_set1_epi8, _mm_set1_epi16, _mm_set1_epi32,
            _mm_slli_epi16, _mm_srli_epi16,
            _mm_cmplt_epi8, _mm_cmplt_epi16, _mm_cmplt_epi32,
            _mm_and_si128, _mm_andnot_si128,
            _mm_unpacklo_epi8, _mm_packus_epi16,
            _mm_setzero_si128
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "sse2", not(target_feature = "ssse3")))]
use core::{
    arch::{
        x86_64::{
            __m128i,
            _mm_set1_epi8, _mm_set1_epi16, _mm_set1_epi32,
            _mm_slli_epi16, _mm_srli_epi16,
            _mm_cmplt_epi8, _mm_cmplt_epi16, _mm_cmplt_epi32,
            _mm_and_si128, _mm_andnot_si128,
            _mm_unpacklo_epi8, _mm_packus_epi16,
            _mm_setzero_si128
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
macro_rules! define_shuffle {
    (struct $name:ident, align($align:expr), $inner_ty:ty, $size:expr) => {
        #[repr(align($align))]
        #[derive(Copy, Clone)]
        struct $name([$inner_ty; $size]);
    };
    (union $name:ident, $vector_ty:ty, $byte_ty:ty, $size:expr) => {
        #[repr(C)]
        union $name {
            vector: $vector_ty,
            bytes: [$byte_ty; $size],
        }
    };
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
define_shuffle!(struct U256, align(32), __m128i, 2_usize);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
define_shuffle!(union ShuffleEPI8, __m128i, u8, 16_usize);
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
define_shuffle!(union ShuffleEPI16, __m128i, u16, 8_usize);
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
define_shuffle!(union ShuffleEPI32, __m128i, u32, 4_usize);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn _mm_shuffle_epi8(vector: __m128i, indexes: __m128i) -> __m128i {
    let (vec, idx, mut result): (ShuffleEPI8, ShuffleEPI8, ShuffleEPI8) = (
        ShuffleEPI8 { vector: vector },
        ShuffleEPI8 { vector: _mm_and_si128(indexes, _mm_set1_epi8(0x0F)) },
        ShuffleEPI8 { vector: _mm_setzero_si128() }
    );

    result.bytes[0x00] = vec.bytes[idx.bytes[0x00] as usize];
    result.bytes[0x01] = vec.bytes[idx.bytes[0x01] as usize];
    result.bytes[0x02] = vec.bytes[idx.bytes[0x02] as usize];
    result.bytes[0x03] = vec.bytes[idx.bytes[0x03] as usize];
    result.bytes[0x04] = vec.bytes[idx.bytes[0x04] as usize];
    result.bytes[0x05] = vec.bytes[idx.bytes[0x05] as usize];
    result.bytes[0x06] = vec.bytes[idx.bytes[0x06] as usize];
    result.bytes[0x07] = vec.bytes[idx.bytes[0x07] as usize];
    result.bytes[0x08] = vec.bytes[idx.bytes[0x08] as usize];
    result.bytes[0x09] = vec.bytes[idx.bytes[0x09] as usize];
    result.bytes[0x0A] = vec.bytes[idx.bytes[0x0A] as usize];
    result.bytes[0x0B] = vec.bytes[idx.bytes[0x0B] as usize];
    result.bytes[0x0C] = vec.bytes[idx.bytes[0x0C] as usize];
    result.bytes[0x0D] = vec.bytes[idx.bytes[0x0D] as usize];
    result.bytes[0x0E] = vec.bytes[idx.bytes[0x0E] as usize];
    result.bytes[0x0F] = vec.bytes[idx.bytes[0x0F] as usize];

    return _mm_andnot_si128(_mm_cmplt_epi8(indexes, _mm_setzero_si128()), result.vector);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn _mm_shuffle_epi16(vector: __m128i, indexes: __m128i) -> __m128i {
    let (vec, idx, mut result): (ShuffleEPI16, ShuffleEPI16, ShuffleEPI16) = (
        ShuffleEPI16 { vector: vector },
        ShuffleEPI16 { vector: _mm_and_si128(indexes, _mm_set1_epi16(0x0007)) },
        ShuffleEPI16 { vector: _mm_setzero_si128() }
    );

    result.bytes[0x00] = vec.bytes[idx.bytes[0x00] as usize];
    result.bytes[0x01] = vec.bytes[idx.bytes[0x01] as usize];
    result.bytes[0x02] = vec.bytes[idx.bytes[0x02] as usize];
    result.bytes[0x03] = vec.bytes[idx.bytes[0x03] as usize];
    result.bytes[0x04] = vec.bytes[idx.bytes[0x04] as usize];
    result.bytes[0x05] = vec.bytes[idx.bytes[0x05] as usize];
    result.bytes[0x06] = vec.bytes[idx.bytes[0x06] as usize];
    result.bytes[0x07] = vec.bytes[idx.bytes[0x07] as usize];

    return _mm_andnot_si128(_mm_cmplt_epi16(indexes, _mm_setzero_si128()), result.vector);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn _mm_shuffle_epi32(vector: __m128i, indexes: __m128i) -> __m128i {
    let (vec, idx, mut result): (ShuffleEPI32, ShuffleEPI32, ShuffleEPI32) = (
        ShuffleEPI32 { vector: vector },
        ShuffleEPI32 { vector: _mm_and_si128(indexes, _mm_set1_epi32(0x0003)) },
        ShuffleEPI32 { vector: _mm_setzero_si128() }
    );

    result.bytes[0x00] = vec.bytes[idx.bytes[0x00] as usize];
    result.bytes[0x01] = vec.bytes[idx.bytes[0x01] as usize];
    result.bytes[0x02] = vec.bytes[idx.bytes[0x02] as usize];
    result.bytes[0x03] = vec.bytes[idx.bytes[0x03] as usize];

    return _mm_andnot_si128(_mm_cmplt_epi32(indexes, _mm_setzero_si128()), result.vector);
}