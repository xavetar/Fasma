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

#[cfg(all(target_arch = "x86", all(target_feature = "sse2", not(target_feature = "ssse3"))))]
use core::{
    arch::{
        x86::{
            __m128i,
            _mm_set_epi8,
            _mm_and_si128,
            _mm_srli_epi16, _mm_srli_epi32, _mm_srli_epi64,
            _mm_packus_epi16,
            _mm_shuffle_epi32,
            _mm_movemask_epi8,
            _mm_setzero_si128
        }
    }
};

#[cfg(all(target_arch = "x86", all(target_feature = "sse2", target_feature = "ssse3")))]
use core::{
    arch::{
        x86::{
            __m128i,
            _mm_set_epi8,
            _mm_shuffle_epi8,
            _mm_movemask_epi8
        }
    }
};

#[cfg(all(target_arch = "x86", target_feature = "avx", target_feature = "avx2"))]
use core::{
    arch::{
        x86::{
            __m256i,
            _mm256_set_epi8,
            _mm256_permute4x64_epi64,
            _mm256_shuffle_epi8,
            _mm256_movemask_epi8
        }
    }
};

#[cfg(all(target_arch = "x86_64", all(target_feature = "sse2", not(target_feature = "ssse3"))))]
use core::{
    arch::{
        x86_64::{
            __m128i,
            _mm_set_epi8,
            _mm_and_si128,
            _mm_srli_epi16, _mm_srli_epi32, _mm_srli_epi64,
            _mm_packus_epi16,
            _mm_shuffle_epi32,
            _mm_movemask_epi8,
            _mm_setzero_si128
        }
    }
};

#[cfg(all(target_arch = "x86_64", all(target_feature = "sse2", target_feature = "ssse3")))]
use core::{
    arch::{
        x86_64::{
            __m128i,
            _mm_set_epi8,
            _mm_shuffle_epi8,
            _mm_movemask_epi8
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "avx", target_feature = "avx2"))]
use core::{
    arch::{
        x86_64::{
            __m256i,
            _mm256_set_epi8,
            _mm256_permute4x64_epi64,
            _mm256_shuffle_epi8,
            _mm256_movemask_epi8
        }
    }
};


#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
#[inline]
pub unsafe fn _mm_movemask_epi16(vector: __m128i) -> i32 {
    return _mm_movemask_epi8(
        _mm_and_si128(
            _mm_packus_epi16(
                _mm_srli_epi16::<0x08>(vector),
                _mm_srli_epi16::<0x08>(vector)
            ),
            _mm_set_epi8(
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                -0x01, -0x01, -0x01, -0x01, -0x01, -0x01, -0x01, -0x01
            )
        )
    );
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
#[inline]
pub unsafe fn _mm_movemask_epi32(vector: __m128i) -> i32 {
    return _mm_movemask_epi8(
        _mm_packus_epi16(
            _mm_packus_epi16(
                _mm_srli_epi32::<0x18>(vector),
                _mm_setzero_si128()
            ),
            _mm_setzero_si128()
        )
    )
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "ssse3")))]
#[inline]
pub unsafe fn _mm_movemask_epi64(vector: __m128i) -> i32 {
    return _mm_movemask_epi8(
        _mm_packus_epi16(
            _mm_packus_epi16(
                _mm_shuffle_epi32::<0xD8>(
                    _mm_srli_epi64::<0x38>(vector)
                ),
                _mm_setzero_si128()
            ),
            _mm_setzero_si128()
        ),
    );
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "ssse3"))]
#[inline]
pub unsafe fn _mm_movemask_epi16(vector: __m128i) -> i32 {
    return _mm_movemask_epi8(
        _mm_shuffle_epi8(
            vector,
            _mm_set_epi8(
                -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, 0x0F, 0x0D, 0x0B, 0x09, 0x07, 0x05, 0x03, 0x01
            )
        )
    );
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "ssse3"))]
#[inline]
pub unsafe fn _mm_movemask_epi32(vector: __m128i) -> i32 {
    return _mm_movemask_epi8(
        _mm_shuffle_epi8(
            vector,
            _mm_set_epi8(
                -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, 0x0F, 0x0B, 0x07, 0x03
            )
        )
    );
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "ssse3"))]
#[inline]
pub unsafe fn _mm_movemask_epi64(vector: __m128i) -> i32 {
    return _mm_movemask_epi8(
        _mm_shuffle_epi8(
            vector,
            _mm_set_epi8(
                -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, 0x0F, 0x07
            )
        )
    );
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_movemask_epi16(vector: __m256i) -> i32 {
    return _mm256_movemask_epi8(
        _mm256_permute4x64_epi64::<0xD8>(
            _mm256_shuffle_epi8(vector,
                _mm256_set_epi8(
                    -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, 0x1F, 0x1D, 0x1B, 0x19, 0x17, 0x15, 0x13, 0x11,
                    -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, 0x0F, 0x0D, 0x0B, 0x09, 0x07, 0x05, 0x03, 0x01
                )
            )
        )
    );
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_movemask_epi32(vector: __m256i) -> i32 {
    return _mm256_movemask_epi8(
        _mm256_shuffle_epi8(
            _mm256_permute4x64_epi64::<0xD8>(
                _mm256_shuffle_epi8(
                    vector,
                    _mm256_set_epi8(
                        -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, 0x1F, 0x1B, 0x17, 0x13,
                        -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, 0x0F, 0x0B, 0x07, 0x03
                    )
                )
            ),
            _mm256_set_epi8(
                -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80,
                -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, 0x0B, 0x0A, 0x09, 0x08, 0x03, 0x02, 0x01, 0x00
            )
        )
    );
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_movemask_epi64(vector: __m256i) -> i32 {
    return _mm256_movemask_epi8(
        _mm256_shuffle_epi8(
            _mm256_permute4x64_epi64::<0xD8>(
                _mm256_shuffle_epi8(
                    vector,
                    _mm256_set_epi8(
                        -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, 0x1F, 0x17,
                        -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, 0x0F, 0x07
                    )
                )
            ),
            _mm256_set_epi8(
                -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80,
                -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, -0x80, 0x09, 0x08, 0x01, 0x00
            )
        )
    );
}
