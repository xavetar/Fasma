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
            _mm_set1_epi16,
            _mm_and_si128,
            _mm_srli_epi16,
            _mm_cmplt_epi8,
            _mm_extract_epi16,
            _mm_packus_epi16,
            _mm_unpackhi_epi16, _mm_unpacklo_epi16,
            _mm_andnot_si128,
            _mm_setzero_si128
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "sse2", not(target_feature = "ssse3")))]
use core::{
    arch::{
        x86_64::{
            __m128i,
            _mm_set1_epi16,
            _mm_and_si128,
            _mm_srli_epi16,
            _mm_cmplt_epi8,
            _mm_extract_epi16,
            _mm_packus_epi16,
            _mm_unpackhi_epi16, _mm_unpacklo_epi16,
            _mm_andnot_si128,
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
    (union $name:ident, $vector_ty:ty, $source_map_ty:ty, $external_map_ty:ty, $source_map_size:expr, $external_map_size:expr) => {
        #[repr(C)]
        union $name {
            vector: $vector_ty,
            bytes: [$source_map_ty; $source_map_size],
            bytes_u64: [$external_map_ty; $external_map_size],
        }
    };
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
define_shuffle!(struct U256, align(32), __m128i, 2_usize);

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
define_shuffle!(union ShuffleEPI8, U256, u16, u64, 16_usize, 4_usize);

#[inline]
#[allow(unsafe_op_in_unsafe_fn)]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
pub unsafe fn _mm_shuffle_epi8(vector: __m128i, indexes: __m128i) -> __m128i {
    let (vec, idx, mut result): (ShuffleEPI8, ShuffleEPI8, ShuffleEPI8) = (
        ShuffleEPI8 {
            vector: U256 {
                0: [
                    _mm_unpacklo_epi16(
                        _mm_and_si128(vector, _mm_set1_epi16(0x00FF)),
                        _mm_srli_epi16::<0x08>(vector)
                    ),
                    _mm_unpackhi_epi16(
                        _mm_and_si128(vector, _mm_set1_epi16(0x00FF)),
                        _mm_srli_epi16::<0x08>(vector)
                    )
                ]
            }
        },
        ShuffleEPI8 {
            vector: U256 {
                0: [
                    _mm_unpacklo_epi16(
                        _mm_and_si128(indexes, _mm_set1_epi16(0x000F)),
                        _mm_and_si128(_mm_srli_epi16::<0x08>(indexes), _mm_set1_epi16(0x000F))
                    ),
                    _mm_unpackhi_epi16(
                        _mm_and_si128(indexes, _mm_set1_epi16(0x000F)),
                        _mm_and_si128(_mm_srli_epi16::<0x08>(indexes), _mm_set1_epi16(0x000F))
                    )
                ]
            }
        },
        ShuffleEPI8 {
            vector: U256 {
                0: [
                    _mm_setzero_si128(),
                    _mm_setzero_si128()
                ]
            }
        }
    );

    result.bytes[0x00] = vec.bytes[_mm_extract_epi16::<0x00>(idx.vector.0[0]) as usize];
    result.bytes[0x01] = vec.bytes[_mm_extract_epi16::<0x01>(idx.vector.0[0]) as usize];
    result.bytes[0x02] = vec.bytes[_mm_extract_epi16::<0x02>(idx.vector.0[0]) as usize];
    result.bytes[0x03] = vec.bytes[_mm_extract_epi16::<0x03>(idx.vector.0[0]) as usize];
    result.bytes[0x04] = vec.bytes[_mm_extract_epi16::<0x04>(idx.vector.0[0]) as usize];
    result.bytes[0x05] = vec.bytes[_mm_extract_epi16::<0x05>(idx.vector.0[0]) as usize];
    result.bytes[0x06] = vec.bytes[_mm_extract_epi16::<0x06>(idx.vector.0[0]) as usize];
    result.bytes[0x07] = vec.bytes[_mm_extract_epi16::<0x07>(idx.vector.0[0]) as usize];
    result.bytes[0x08] = vec.bytes[_mm_extract_epi16::<0x00>(idx.vector.0[1]) as usize];
    result.bytes[0x09] = vec.bytes[_mm_extract_epi16::<0x01>(idx.vector.0[1]) as usize];
    result.bytes[0x0A] = vec.bytes[_mm_extract_epi16::<0x02>(idx.vector.0[1]) as usize];
    result.bytes[0x0B] = vec.bytes[_mm_extract_epi16::<0x03>(idx.vector.0[1]) as usize];
    result.bytes[0x0C] = vec.bytes[_mm_extract_epi16::<0x04>(idx.vector.0[1]) as usize];
    result.bytes[0x0D] = vec.bytes[_mm_extract_epi16::<0x05>(idx.vector.0[1]) as usize];
    result.bytes[0x0E] = vec.bytes[_mm_extract_epi16::<0x06>(idx.vector.0[1]) as usize];
    result.bytes[0x0F] = vec.bytes[_mm_extract_epi16::<0x07>(idx.vector.0[1]) as usize];

    result.vector.0[0] = _mm_packus_epi16(result.vector.0[0x00], _mm_setzero_si128());
    result.vector.0[1] = _mm_packus_epi16(result.vector.0[0x01], _mm_setzero_si128());

    result.bytes_u64[1] = result.bytes_u64[2];

    return _mm_andnot_si128(_mm_cmplt_epi8(indexes, _mm_setzero_si128()), result.vector.0[0]);
}