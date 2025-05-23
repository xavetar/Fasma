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

#[cfg(all(target_arch = "x86", target_feature = "sse2"))]
use core::{
    arch::{
        x86::{
            __m128i,
            _mm_set1_epi8,
            _mm_and_si128,
            _mm_slli_epi16, _mm_srli_epi16,
            _mm_setzero_si128
        }
    }
};

#[cfg(all(target_arch = "x86", target_feature = "avx", target_feature = "avx2"))]
use core::{
    arch::{
        x86::{
            __m256i,
            _mm256_set1_epi8,
            _mm256_setr_epi8,
            _mm256_and_si256,
            _mm256_slli_epi16, _mm256_srli_epi16,
            _mm256_alignr_epi8,
            _mm256_permute2x128_si256,
            _mm256_setzero_si256
        }
    }
};

#[cfg(all(target_arch = "x86", target_feature = "avx512f", target_feature = "avx512bw"))]
use core::{
    arch::{
        x86::{
            __m512i,
            _mm512_set1_epi8,
            _mm512_set_epi64,
            _mm512_or_si512, _mm512_and_si512,
            _mm512_slli_epi16, _mm512_srli_epi16,
            _mm512_bslli_epi128, _mm512_bsrli_epi128,
            _mm512_permutexvar_epi64,
            _mm512_setzero_si512
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use core::{
    arch::{
        x86_64::{
            __m128i,
            _mm_set1_epi8,
            _mm_and_si128,
            _mm_slli_epi16, _mm_srli_epi16,
            _mm_setzero_si128
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "avx", target_feature = "avx2"))]
use core::{
    arch::{
        x86_64::{
            __m256i,
            _mm256_set1_epi8,
            _mm256_setr_epi8,
            _mm256_and_si256,
            _mm256_slli_epi16, _mm256_srli_epi16,
            _mm256_alignr_epi8,
            _mm256_permute2x128_si256,
            _mm256_setzero_si256
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f", target_feature = "avx512bw"))]
use core::{
    arch::{
        x86_64::{
            __m512i,
            _mm512_set1_epi8,
            _mm512_set_epi64,
            _mm512_or_si512, _mm512_and_si512,
            _mm512_slli_epi16, _mm512_srli_epi16,
            _mm512_bslli_epi128, _mm512_bsrli_epi128,
            _mm512_permutexvar_epi64,
            _mm512_setzero_si512
        }
    }
};


/// # Example (_mm_slli_epi8):
///
/// - Source: ```[00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f]```
/// - Shifted left by 1 bit: ```[00, 02, 04, 06, 08, 0a, 0c, 0e, 10, 12, 14, 16, 18, 1a, 1c, 1e]```
/// - Shifted left by 2 bits: ```[00, 04, 08, 0c, 10, 14, 18, 1c, 20, 24, 28, 2c, 30, 34, 38, 3c]```
/// - Shifted left by 3 bits: ```[00, 08, 10, 18, 20, 28, 30, 38, 40, 48, 50, 58, 60, 68, 70, 78]```
/// - Shifted left by 4 bits: ```[00, 10, 20, 30, 40, 50, 60, 70, 80, 90, a0, b0, c0, d0, e0, f0]```
/// - Shifted left by 5 bits: ```[00, 20, 40, 60, 80, a0, c0, e0, 00, 20, 40, 60, 80, a0, c0, e0]```
/// - Shifted left by 6 bits: ```[00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0]```
/// - Shifted left by 7 bits: ```[00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80]```
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
pub unsafe fn _mm_slli_epi8(vector: __m128i, shift: usize) -> __m128i {
    return match shift {
        0x00 => vector,
        0x01 => _mm_slli_epi16::<0x01>(_mm_and_si128(vector, _mm_set1_epi8(0x7F))),
        0x02 => _mm_slli_epi16::<0x02>(_mm_and_si128(vector, _mm_set1_epi8(0x3F))),
        0x03 => _mm_slli_epi16::<0x03>(_mm_and_si128(vector, _mm_set1_epi8(0x1F))),
        0x04 => _mm_slli_epi16::<0x04>(_mm_and_si128(vector, _mm_set1_epi8(0x0F))),
        0x05 => _mm_slli_epi16::<0x05>(_mm_and_si128(vector, _mm_set1_epi8(0x07))),
        0x06 => _mm_slli_epi16::<0x06>(_mm_and_si128(vector, _mm_set1_epi8(0x03))),
        0x07 => _mm_slli_epi16::<0x07>(_mm_and_si128(vector, _mm_set1_epi8(0x01))),
        _ => _mm_setzero_si128()
    };
}

/// # Example (_mm_srli_epi8):
///
/// - Source: ```[00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f]```
/// - Shifted right by 1 bit: ```[00, 00, 01, 01, 02, 02, 03, 03, 04, 04, 05, 05, 06, 06, 07, 07]```
/// - Shifted right by 2 bits: ```[00, 00, 00, 00, 01, 01, 01, 01, 02, 02, 02, 02, 03, 03, 03, 03]```
/// - Shifted right by 3 bits: ```[00, 00, 00, 00, 00, 00, 00, 00, 01, 01, 01, 01, 01, 01, 01, 01]```
/// - Shifted right by 4 bits: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Shifted right by 5 bits: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Shifted right by 6 bits: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Shifted right by 7 bits: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
pub unsafe fn _mm_srli_epi8(vector: __m128i, shift: usize) -> __m128i {
    return match shift {
        0x00 => vector,
        0x01 => _mm_srli_epi16::<0x01>(_mm_and_si128(vector, _mm_set1_epi8(-0x02))),
        0x02 => _mm_srli_epi16::<0x02>(_mm_and_si128(vector, _mm_set1_epi8(-0x04))),
        0x03 => _mm_srli_epi16::<0x03>(_mm_and_si128(vector, _mm_set1_epi8(-0x08))),
        0x04 => _mm_srli_epi16::<0x04>(_mm_and_si128(vector, _mm_set1_epi8(-0x10))),
        0x05 => _mm_srli_epi16::<0x05>(_mm_and_si128(vector, _mm_set1_epi8(-0x20))),
        0x06 => _mm_srli_epi16::<0x06>(_mm_and_si128(vector, _mm_set1_epi8(-0x40))),
        0x07 => _mm_srli_epi16::<0x07>(_mm_and_si128(vector, _mm_set1_epi8(-0x80))),
        _ => _mm_setzero_si128()
    };
}

/// # Example (_mm256_slli_epi8):
///
/// - Source: ```[00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f]```
/// - Shifted left by 1 bit: ```[00, 02, 04, 06, 08, 0a, 0c, 0e, 10, 12, 14, 16, 18, 1a, 1c, 1e, 20, 22, 24, 26, 28, 2a, 2c, 2e, 30, 32, 34, 36, 38, 3a, 3c, 3e]```
/// - Shifted left by 2 bits: ```[00, 04, 08, 0c, 10, 14, 18, 1c, 20, 24, 28, 2c, 30, 34, 38, 3c, 40, 44, 48, 4c, 50, 54, 58, 5c, 60, 64, 68, 6c, 70, 74, 78, 7c]```
/// - Shifted left by 3 bits: ```[00, 08, 10, 18, 20, 28, 30, 38, 40, 48, 50, 58, 60, 68, 70, 78, 80, 88, 90, 98, a0, a8, b0, b8, c0, c8, d0, d8, e0, e8, f0, f8]```
/// - Shifted left by 4 bits: ```[00, 10, 20, 30, 40, 50, 60, 70, 80, 90, a0, b0, c0, d0, e0, f0, 00, 10, 20, 30, 40, 50, 60, 70, 80, 90, a0, b0, c0, d0, e0, f0]```
/// - Shifted left by 5 bits: ```[00, 20, 40, 60, 80, a0, c0, e0, 00, 20, 40, 60, 80, a0, c0, e0, 00, 20, 40, 60, 80, a0, c0, e0, 00, 20, 40, 60, 80, a0, c0, e0]```
/// - Shifted left by 6 bits: ```[00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0]```
/// - Shifted left by 7 bits: ```[00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80]```
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
pub unsafe fn _mm256_slli_epi8(vector: __m256i, shift: usize) -> __m256i {
    return match shift {
        0x00 => vector,
        0x01 => _mm256_slli_epi16::<0x01>(_mm256_and_si256(vector, _mm256_set1_epi8(0x7F))),
        0x02 => _mm256_slli_epi16::<0x02>(_mm256_and_si256(vector, _mm256_set1_epi8(0x3F))),
        0x03 => _mm256_slli_epi16::<0x03>(_mm256_and_si256(vector, _mm256_set1_epi8(0x1F))),
        0x04 => _mm256_slli_epi16::<0x04>(_mm256_and_si256(vector, _mm256_set1_epi8(0x0F))),
        0x05 => _mm256_slli_epi16::<0x05>(_mm256_and_si256(vector, _mm256_set1_epi8(0x07))),
        0x06 => _mm256_slli_epi16::<0x06>(_mm256_and_si256(vector, _mm256_set1_epi8(0x03))),
        0x07 => _mm256_slli_epi16::<0x07>(_mm256_and_si256(vector, _mm256_set1_epi8(0x01))),
        _ => _mm256_setzero_si256()
    };
}

/// # Example (_mm256_srli_epi8):
///
/// - Source: ```[00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f]```
/// - Shifted right by 1 bit: ```[00, 00, 01, 01, 02, 02, 03, 03, 04, 04, 05, 05, 06, 06, 07, 07, 08, 08, 09, 09, 0a, 0a, 0b, 0b, 0c, 0c, 0d, 0d, 0e, 0e, 0f, 0f]```
/// - Shifted right by 2 bits: ```[00, 00, 00, 00, 01, 01, 01, 01, 02, 02, 02, 02, 03, 03, 03, 03, 04, 04, 04, 04, 05, 05, 05, 05, 06, 06, 06, 06, 07, 07, 07, 07]```
/// - Shifted right by 3 bits: ```[00, 00, 00, 00, 00, 00, 00, 00, 01, 01, 01, 01, 01, 01, 01, 01, 02, 02, 02, 02, 02, 02, 02, 02, 03, 03, 03, 03, 03, 03, 03, 03]```
/// - Shifted right by 4 bits: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01]```
/// - Shifted right by 5 bits: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Shifted right by 6 bits: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Shifted right by 7 bits: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
pub unsafe fn _mm256_srli_epi8(vector: __m256i, shift: usize) -> __m256i {
    return match shift {
        0x00 => vector,
        0x01 => _mm256_srli_epi16::<0x01>(_mm256_and_si256(vector, _mm256_set1_epi8(-0x02))),
        0x02 => _mm256_srli_epi16::<0x02>(_mm256_and_si256(vector, _mm256_set1_epi8(-0x04))),
        0x03 => _mm256_srli_epi16::<0x03>(_mm256_and_si256(vector, _mm256_set1_epi8(-0x08))),
        0x04 => _mm256_srli_epi16::<0x04>(_mm256_and_si256(vector, _mm256_set1_epi8(-0x10))),
        0x05 => _mm256_srli_epi16::<0x05>(_mm256_and_si256(vector, _mm256_set1_epi8(-0x20))),
        0x06 => _mm256_srli_epi16::<0x06>(_mm256_and_si256(vector, _mm256_set1_epi8(-0x40))),
        0x07 => _mm256_srli_epi16::<0x07>(_mm256_and_si256(vector, _mm256_set1_epi8(-0x80))),
        _ => _mm256_setzero_si256()
    };
}

/// # Example (_mm256_slli_si256):
///
/// - Source: ```[00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f]```
/// - Shifted left by 1 byte: ```[00, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e]```
/// - Shifted left by 2 bytes: ```[00, 00, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d]```
/// - Shifted left by 15 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Shifted left by 16 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f]```
/// - Shifted left by 20 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b]```
/// - Shifted left by 31 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
pub unsafe fn _mm256_slli_si256(vector: __m256i, shift: usize) -> __m256i {
    return match shift {
        0x00 => vector,
        0x01 => _mm256_and_si256(_mm256_alignr_epi8::<0x0F>(vector, _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x02 => _mm256_and_si256(_mm256_alignr_epi8::<0x0E>(vector, _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x03 => _mm256_and_si256(_mm256_alignr_epi8::<0x0D>(vector, _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x04 => _mm256_and_si256(_mm256_alignr_epi8::<0x0C>(vector, _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x05 => _mm256_and_si256(_mm256_alignr_epi8::<0x0B>(vector, _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x06 => _mm256_and_si256(_mm256_alignr_epi8::<0x0A>(vector, _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x07 => _mm256_and_si256(_mm256_alignr_epi8::<0x09>(vector, _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x08 => _mm256_and_si256(_mm256_alignr_epi8::<0x08>(vector, _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x09 => _mm256_and_si256(_mm256_alignr_epi8::<0x07>(vector, _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x0A => _mm256_and_si256(_mm256_alignr_epi8::<0x06>(vector, _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x0B => _mm256_and_si256(_mm256_alignr_epi8::<0x05>(vector, _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x0C => _mm256_and_si256(_mm256_alignr_epi8::<0x04>(vector, _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x0D => _mm256_and_si256(_mm256_alignr_epi8::<0x03>(vector, _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x0E => _mm256_and_si256(_mm256_alignr_epi8::<0x02>(vector, _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x0F => _mm256_and_si256(_mm256_alignr_epi8::<0x01>(vector, _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x10 => _mm256_and_si256(_mm256_alignr_epi8::<0x10>(_mm256_permute2x128_si256::<0x01>(vector, vector), _mm256_setzero_si256()), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x11 => _mm256_and_si256(_mm256_alignr_epi8::<0x0F>(_mm256_permute2x128_si256::<0x01>(vector, vector), _mm256_setzero_si256()), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x12 => _mm256_and_si256(_mm256_alignr_epi8::<0x0E>(_mm256_permute2x128_si256::<0x01>(vector, vector), _mm256_setzero_si256()), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x13 => _mm256_and_si256(_mm256_alignr_epi8::<0x0D>(_mm256_permute2x128_si256::<0x01>(vector, vector), _mm256_setzero_si256()), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x14 => _mm256_and_si256(_mm256_alignr_epi8::<0x0C>(_mm256_permute2x128_si256::<0x01>(vector, vector), _mm256_setzero_si256()), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x15 => _mm256_and_si256(_mm256_alignr_epi8::<0x0B>(_mm256_permute2x128_si256::<0x01>(vector, vector), _mm256_setzero_si256()), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x16 => _mm256_and_si256(_mm256_alignr_epi8::<0x0A>(_mm256_permute2x128_si256::<0x01>(vector, vector), _mm256_setzero_si256()), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x17 => _mm256_and_si256(_mm256_alignr_epi8::<0x09>(_mm256_permute2x128_si256::<0x01>(vector, vector), _mm256_setzero_si256()), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x18 => _mm256_and_si256(_mm256_alignr_epi8::<0x08>(_mm256_permute2x128_si256::<0x01>(vector, vector), _mm256_setzero_si256()), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1)),
        0x19 => _mm256_and_si256(_mm256_alignr_epi8::<0x07>(_mm256_permute2x128_si256::<0x01>(vector, vector), _mm256_setzero_si256()), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1)),
        0x1A => _mm256_and_si256(_mm256_alignr_epi8::<0x06>(_mm256_permute2x128_si256::<0x01>(vector, vector), _mm256_setzero_si256()), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1)),
        0x1B => _mm256_and_si256(_mm256_alignr_epi8::<0x05>(_mm256_permute2x128_si256::<0x01>(vector, vector), _mm256_setzero_si256()), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1)),
        0x1C => _mm256_and_si256(_mm256_alignr_epi8::<0x04>(_mm256_permute2x128_si256::<0x01>(vector, vector), _mm256_setzero_si256()), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1)),
        0x1D => _mm256_and_si256(_mm256_alignr_epi8::<0x03>(_mm256_permute2x128_si256::<0x01>(vector, vector), _mm256_setzero_si256()), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1)),
        0x1E => _mm256_and_si256(_mm256_alignr_epi8::<0x02>(_mm256_permute2x128_si256::<0x01>(vector, vector), _mm256_setzero_si256()), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1)),
        0x1F => _mm256_and_si256(_mm256_alignr_epi8::<0x01>(_mm256_permute2x128_si256::<0x01>(vector, vector), _mm256_setzero_si256()), _mm256_setr_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1)),
        _ => _mm256_setzero_si256()
    };
}

/// # Example (_mm256_srli_si256):
///
/// - Source: ```[00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f]```
/// - Shifted right by 1 byte: ```[01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 00]```
/// - Shifted right by 2 bytes: ```[02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 00, 00]```
/// - Shifted right by 15 bytes: ```[0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Shifted right by 16 bytes: ```[10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Shifted right by 20 bytes: ```[14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Shifted right by 31 bytes: ```[1f, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
pub unsafe fn _mm256_srli_si256(vector: __m256i, shift: usize) -> __m256i {
    return match shift {
        0x00 => vector,
        0x01 => _mm256_and_si256(_mm256_alignr_epi8::<0x01>(_mm256_permute2x128_si256::<0x01>(vector, vector), vector), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0)),
        0x02 => _mm256_and_si256(_mm256_alignr_epi8::<0x02>(_mm256_permute2x128_si256::<0x01>(vector, vector), vector), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0)),
        0x03 => _mm256_and_si256(_mm256_alignr_epi8::<0x03>(_mm256_permute2x128_si256::<0x01>(vector, vector), vector), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0)),
        0x04 => _mm256_and_si256(_mm256_alignr_epi8::<0x04>(_mm256_permute2x128_si256::<0x01>(vector, vector), vector), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0)),
        0x05 => _mm256_and_si256(_mm256_alignr_epi8::<0x05>(_mm256_permute2x128_si256::<0x01>(vector, vector), vector), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0)),
        0x06 => _mm256_and_si256(_mm256_alignr_epi8::<0x06>(_mm256_permute2x128_si256::<0x01>(vector, vector), vector), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0)),
        0x07 => _mm256_and_si256(_mm256_alignr_epi8::<0x07>(_mm256_permute2x128_si256::<0x01>(vector, vector), vector), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0)),
        0x08 => _mm256_and_si256(_mm256_alignr_epi8::<0x08>(_mm256_permute2x128_si256::<0x01>(vector, vector), vector), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x09 => _mm256_and_si256(_mm256_alignr_epi8::<0x09>(_mm256_permute2x128_si256::<0x01>(vector, vector), vector), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x0A => _mm256_and_si256(_mm256_alignr_epi8::<0x0A>(_mm256_permute2x128_si256::<0x01>(vector, vector), vector), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x0B => _mm256_and_si256(_mm256_alignr_epi8::<0x0B>(_mm256_permute2x128_si256::<0x01>(vector, vector), vector), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x0C => _mm256_and_si256(_mm256_alignr_epi8::<0x0C>(_mm256_permute2x128_si256::<0x01>(vector, vector), vector), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x0D => _mm256_and_si256(_mm256_alignr_epi8::<0x0D>(_mm256_permute2x128_si256::<0x01>(vector, vector), vector), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x0E => _mm256_and_si256(_mm256_alignr_epi8::<0x0E>(_mm256_permute2x128_si256::<0x01>(vector, vector), vector), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x0F => _mm256_and_si256(_mm256_alignr_epi8::<0x0F>(_mm256_permute2x128_si256::<0x01>(vector, vector), vector), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x10 => _mm256_and_si256(_mm256_alignr_epi8::<0x10>(_mm256_permute2x128_si256::<0x01>(vector, vector), vector), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x11 => _mm256_and_si256(_mm256_alignr_epi8::<0x01>(_mm256_setzero_si256(), _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x12 => _mm256_and_si256(_mm256_alignr_epi8::<0x02>(_mm256_setzero_si256(), _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x13 => _mm256_and_si256(_mm256_alignr_epi8::<0x03>(_mm256_setzero_si256(), _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x14 => _mm256_and_si256(_mm256_alignr_epi8::<0x04>(_mm256_setzero_si256(), _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x15 => _mm256_and_si256(_mm256_alignr_epi8::<0x05>(_mm256_setzero_si256(), _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x16 => _mm256_and_si256(_mm256_alignr_epi8::<0x06>(_mm256_setzero_si256(), _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x17 => _mm256_and_si256(_mm256_alignr_epi8::<0x07>(_mm256_setzero_si256(), _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x18 => _mm256_and_si256(_mm256_alignr_epi8::<0x08>(_mm256_setzero_si256(), _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x19 => _mm256_and_si256(_mm256_alignr_epi8::<0x09>(_mm256_setzero_si256(), _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x1A => _mm256_and_si256(_mm256_alignr_epi8::<0x0A>(_mm256_setzero_si256(), _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(-1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x1B => _mm256_and_si256(_mm256_alignr_epi8::<0x0B>(_mm256_setzero_si256(), _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(-1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x1C => _mm256_and_si256(_mm256_alignr_epi8::<0x0C>(_mm256_setzero_si256(), _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(-1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x1D => _mm256_and_si256(_mm256_alignr_epi8::<0x0D>(_mm256_setzero_si256(), _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(-1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x1E => _mm256_and_si256(_mm256_alignr_epi8::<0x0E>(_mm256_setzero_si256(), _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        0x1F => _mm256_and_si256(_mm256_alignr_epi8::<0x0F>(_mm256_setzero_si256(), _mm256_permute2x128_si256::<0x01>(vector, vector)), _mm256_setr_epi8(-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
        _ => _mm256_setzero_si256()
    };
}

/// # Example (_mm512_slli_epi8):
///
/// - Source: ```[00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f]```
/// - Shifted left by 1 bit: ```[00, 02, 04, 06, 08, 0a, 0c, 0e, 10, 12, 14, 16, 18, 1a, 1c, 1e, 20, 22, 24, 26, 28, 2a, 2c, 2e, 30, 32, 34, 36, 38, 3a, 3c, 3e, 40, 42, 44, 46, 48, 4a, 4c, 4e, 50, 52, 54, 56, 58, 5a, 5c, 5e, 60, 62, 64, 66, 68, 6a, 6c, 6e, 70, 72, 74, 76, 78, 7a, 7c, 7e]```
/// - Shifted left by 2 bits: ```[00, 04, 08, 0c, 10, 14, 18, 1c, 20, 24, 28, 2c, 30, 34, 38, 3c, 40, 44, 48, 4c, 50, 54, 58, 5c, 60, 64, 68, 6c, 70, 74, 78, 7c, 80, 84, 88, 8c, 90, 94, 98, 9c, a0, a4, a8, ac, b0, b4, b8, bc, c0, c4, c8, cc, d0, d4, d8, dc, e0, e4, e8, ec, f0, f4, f8, fc]```
/// - Shifted left by 3 bits: ```[00, 08, 10, 18, 20, 28, 30, 38, 40, 48, 50, 58, 60, 68, 70, 78, 80, 88, 90, 98, a0, a8, b0, b8, c0, c8, d0, d8, e0, e8, f0, f8, 00, 08, 10, 18, 20, 28, 30, 38, 40, 48, 50, 58, 60, 68, 70, 78, 80, 88, 90, 98, a0, a8, b0, b8, c0, c8, d0, d8, e0, e8, f0, f8]```
/// - Shifted left by 4 bits: ```[00, 10, 20, 30, 40, 50, 60, 70, 80, 90, a0, b0, c0, d0, e0, f0, 00, 10, 20, 30, 40, 50, 60, 70, 80, 90, a0, b0, c0, d0, e0, f0, 00, 10, 20, 30, 40, 50, 60, 70, 80, 90, a0, b0, c0, d0, e0, f0, 00, 10, 20, 30, 40, 50, 60, 70, 80, 90, a0, b0, c0, d0, e0, f0]```
/// - Shifted left by 5 bits: ```[00, 20, 40, 60, 80, a0, c0, e0, 00, 20, 40, 60, 80, a0, c0, e0, 00, 20, 40, 60, 80, a0, c0, e0, 00, 20, 40, 60, 80, a0, c0, e0, 00, 20, 40, 60, 80, a0, c0, e0, 00, 20, 40, 60, 80, a0, c0, e0, 00, 20, 40, 60, 80, a0, c0, e0, 00, 20, 40, 60, 80, a0, c0, e0]```
/// - Shifted left by 6 bits: ```[00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0, 00, 40, 80, c0]```
/// - Shifted left by 7 bits: ```[00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80, 00, 80]```
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
pub unsafe fn _mm512_slli_epi8(vector: __m512i, shift: usize) -> __m512i {
    return match shift {
        0x00 => vector,
        0x01 => _mm512_slli_epi16::<0x01>(_mm512_and_si512(vector, _mm512_set1_epi8(0x7F))),
        0x02 => _mm512_slli_epi16::<0x02>(_mm512_and_si512(vector, _mm512_set1_epi8(0x3F))),
        0x03 => _mm512_slli_epi16::<0x03>(_mm512_and_si512(vector, _mm512_set1_epi8(0x1F))),
        0x04 => _mm512_slli_epi16::<0x04>(_mm512_and_si512(vector, _mm512_set1_epi8(0x0F))),
        0x05 => _mm512_slli_epi16::<0x05>(_mm512_and_si512(vector, _mm512_set1_epi8(0x07))),
        0x06 => _mm512_slli_epi16::<0x06>(_mm512_and_si512(vector, _mm512_set1_epi8(0x03))),
        0x07 => _mm512_slli_epi16::<0x07>(_mm512_and_si512(vector, _mm512_set1_epi8(0x01))),
        _ => _mm512_setzero_si512()
    };
}

/// # Example (_mm512_srli_epi8):
///
/// - Source: ```[00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f]```
/// - Shifted right by 1 bit: ```[00, 00, 01, 01, 02, 02, 03, 03, 04, 04, 05, 05, 06, 06, 07, 07, 08, 08, 09, 09, 0a, 0a, 0b, 0b, 0c, 0c, 0d, 0d, 0e, 0e, 0f, 0f, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 1a, 1a, 1b, 1b, 1c, 1c, 1d, 1d, 1e, 1e, 1f, 1f]```
/// - Shifted right by 2 bits: ```[00, 00, 00, 00, 01, 01, 01, 01, 02, 02, 02, 02, 03, 03, 03, 03, 04, 04, 04, 04, 05, 05, 05, 05, 06, 06, 06, 06, 07, 07, 07, 07, 08, 08, 08, 08, 09, 09, 09, 09, 0a, 0a, 0a, 0a, 0b, 0b, 0b, 0b, 0c, 0c, 0c, 0c, 0d, 0d, 0d, 0d, 0e, 0e, 0e, 0e, 0f, 0f, 0f, 0f]```
/// - Shifted right by 3 bits: ```[00, 00, 00, 00, 00, 00, 00, 00, 01, 01, 01, 01, 01, 01, 01, 01, 02, 02, 02, 02, 02, 02, 02, 02, 03, 03, 03, 03, 03, 03, 03, 03, 04, 04, 04, 04, 04, 04, 04, 04, 05, 05, 05, 05, 05, 05, 05, 05, 06, 06, 06, 06, 06, 06, 06, 06, 07, 07, 07, 07, 07, 07, 07, 07]```
/// - Shifted right by 4 bits: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 02, 02, 02, 02, 02, 02, 02, 02, 02, 02, 02, 02, 02, 02, 02, 02, 03, 03, 03, 03, 03, 03, 03, 03, 03, 03, 03, 03, 03, 03, 03, 03]```
/// - Shifted right by 5 bits: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01, 01]```
/// - Shifted right by 6 bits: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Shifted right by 7 bits: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
pub unsafe fn _mm512_srli_epi8(vector: __m512i, shift: usize) -> __m512i {
    return match shift {
        0x00 => vector,
        0x01 => _mm512_srli_epi16::<0x01>(_mm512_and_si512(vector, _mm512_set1_epi8(-0x02))),
        0x02 => _mm512_srli_epi16::<0x02>(_mm512_and_si512(vector, _mm512_set1_epi8(-0x04))),
        0x03 => _mm512_srli_epi16::<0x03>(_mm512_and_si512(vector, _mm512_set1_epi8(-0x08))),
        0x04 => _mm512_srli_epi16::<0x04>(_mm512_and_si512(vector, _mm512_set1_epi8(-0x10))),
        0x05 => _mm512_srli_epi16::<0x05>(_mm512_and_si512(vector, _mm512_set1_epi8(-0x20))),
        0x06 => _mm512_srli_epi16::<0x06>(_mm512_and_si512(vector, _mm512_set1_epi8(-0x40))),
        0x07 => _mm512_srli_epi16::<0x07>(_mm512_and_si512(vector, _mm512_set1_epi8(-0x80))),
        _ => _mm512_setzero_si512()
    };
}

/// # Example (_mm512_slli_si512):
///
/// - Source: ```[00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f]```
/// - Shifted left by 1 byte: ```[00, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e]```
/// - Shifted left by 2 bytes: ```[00, 00, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d]```
/// - Shifted left by 15 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30]```
/// - Shifted left by 16 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f]```
/// - Shifted left by 20 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b]```
/// - Shifted left by 34 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d]```
/// - Shifted left by 35 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c]```
/// - Shifted left by 42 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15]```
/// - Shifted left by 63 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
pub unsafe fn _mm512_slli_si512(vector: __m512i, shift: usize) -> __m512i {
    return match shift {
        0x00 => vector,
        0x01 => _mm512_or_si512(_mm512_bslli_epi128::<0x01>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x0F>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0))),
        0x02 => _mm512_or_si512(_mm512_bslli_epi128::<0x02>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x0E>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0))),
        0x03 => _mm512_or_si512(_mm512_bslli_epi128::<0x03>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x0D>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0))),
        0x04 => _mm512_or_si512(_mm512_bslli_epi128::<0x04>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x0C>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0))),
        0x05 => _mm512_or_si512(_mm512_bslli_epi128::<0x05>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x0B>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0))),
        0x06 => _mm512_or_si512(_mm512_bslli_epi128::<0x06>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x0A>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0))),
        0x07 => _mm512_or_si512(_mm512_bslli_epi128::<0x07>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x09>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0))),
        0x08 => _mm512_or_si512(_mm512_bslli_epi128::<0x08>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x08>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0))),
        0x09 => _mm512_or_si512(_mm512_bslli_epi128::<0x09>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x07>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0))),
        0x0A => _mm512_or_si512(_mm512_bslli_epi128::<0x0A>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x06>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0))),
        0x0B => _mm512_or_si512(_mm512_bslli_epi128::<0x0B>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x05>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0))),
        0x0C => _mm512_or_si512(_mm512_bslli_epi128::<0x0C>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x04>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0))),
        0x0D => _mm512_or_si512(_mm512_bslli_epi128::<0x0D>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x03>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0))),
        0x0E => _mm512_or_si512(_mm512_bslli_epi128::<0x0E>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x02>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0))),
        0x0F => _mm512_or_si512(_mm512_bslli_epi128::<0x0F>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x01>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0))),
        0x10 => _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), vector), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0)),
        0x11 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x01>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x0F>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0))),
        0x12 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x02>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x0E>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0))),
        0x13 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x03>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x0D>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0))),
        0x14 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x04>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x0C>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0))),
        0x15 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x05>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x0B>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0))),
        0x16 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x06>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x0A>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0))),
        0x17 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x07>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x09>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0))),
        0x18 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x08>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x08>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0))),
        0x19 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x09>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x07>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0))),
        0x1A => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x0A>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x06>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0))),
        0x1B => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x0B>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x05>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0))),
        0x1C => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x0C>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x04>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0))),
        0x1D => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x0D>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x03>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0))),
        0x1E => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x0E>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x02>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0))),
        0x1F => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x0F>(vector)), _mm512_set_epi64(-1, -1, -1, -1, -1, -1, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x01>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0))),
        0x20 => _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), vector), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0)),
        0x21 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x01>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x0F>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0))),
        0x22 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x02>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x0E>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0))),
        0x23 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x03>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x0D>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0))),
        0x24 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x04>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x0C>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0))),
        0x25 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x05>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x0B>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0))),
        0x26 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x06>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x0A>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0))),
        0x27 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x07>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x09>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0))),
        0x28 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x08>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x08>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0))),
        0x29 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x09>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x07>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0))),
        0x2A => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x0A>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x06>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0))),
        0x2B => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x0B>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x05>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0))),
        0x2C => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x0C>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x04>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0))),
        0x2D => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x0D>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x03>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0))),
        0x2E => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x0E>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x02>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0))),
        0x2F => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x0F>(vector)), _mm512_set_epi64(-1, -1, -1, -1, 0, 0, 0, 0)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x01>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0))),
        0x30 => _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), vector), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0)),
        0x31 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x01>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0)), _mm512_setzero_si512()),
        0x32 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x02>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0)), _mm512_setzero_si512()),
        0x33 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x03>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0)), _mm512_setzero_si512()),
        0x34 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x04>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0)), _mm512_setzero_si512()),
        0x35 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x05>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0)), _mm512_setzero_si512()),
        0x36 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x06>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0)), _mm512_setzero_si512()),
        0x37 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x07>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0)), _mm512_setzero_si512()),
        0x38 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x08>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0)), _mm512_setzero_si512()),
        0x39 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x09>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0)), _mm512_setzero_si512()),
        0x3A => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x0A>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0)), _mm512_setzero_si512()),
        0x3B => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x0B>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0)), _mm512_setzero_si512()),
        0x3C => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x0C>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0)), _mm512_setzero_si512()),
        0x3D => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x0D>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0)), _mm512_setzero_si512()),
        0x3E => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x0E>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0)), _mm512_setzero_si512()),
        0x3F => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x0F>(vector)), _mm512_set_epi64(-1, -1, 0, 0, 0, 0, 0, 0)), _mm512_setzero_si512()),
        _ => _mm512_setzero_si512()
    };
}

/// # Example (_mm512_srli_si512):
///
/// - Source: ```[00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f]```
/// - Shifted right by 1 byte: ```[01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 00]```
/// - Shifted right by 2 bytes: ```[02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 00, 00]```
/// - Shifted right by 15 bytes: ```[0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Shifted right by 16 bytes: ```[10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Shifted right by 20 bytes: ```[14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Shifted right by 34 bytes: ```[22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Shifted right by 35 bytes: ```[23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Shifted right by 42 bytes: ```[2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Shifted right by 63 bytes: ```[3f, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f", target_feature = "avx512bw"))]
pub unsafe fn _mm512_srli_si512(vector: __m512i, shift: usize) -> __m512i {
    return match shift {
        0x00 => vector,
        0x01 => _mm512_or_si512(_mm512_bsrli_epi128::<0x01>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x0F>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1))),
        0x02 => _mm512_or_si512(_mm512_bsrli_epi128::<0x02>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x0E>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1))),
        0x03 => _mm512_or_si512(_mm512_bsrli_epi128::<0x03>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x0D>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1))),
        0x04 => _mm512_or_si512(_mm512_bsrli_epi128::<0x04>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x0C>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1))),
        0x05 => _mm512_or_si512(_mm512_bsrli_epi128::<0x05>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x0B>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1))),
        0x06 => _mm512_or_si512(_mm512_bsrli_epi128::<0x06>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x0A>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1))),
        0x07 => _mm512_or_si512(_mm512_bsrli_epi128::<0x07>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x09>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1))),
        0x08 => _mm512_or_si512(_mm512_bsrli_epi128::<0x08>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x08>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1))),
        0x09 => _mm512_or_si512(_mm512_bsrli_epi128::<0x09>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x07>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1))),
        0x0A => _mm512_or_si512(_mm512_bsrli_epi128::<0x0A>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x06>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1))),
        0x0B => _mm512_or_si512(_mm512_bsrli_epi128::<0x0B>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x05>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1))),
        0x0C => _mm512_or_si512(_mm512_bsrli_epi128::<0x0C>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x04>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1))),
        0x0D => _mm512_or_si512(_mm512_bsrli_epi128::<0x0D>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x03>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1))),
        0x0E => _mm512_or_si512(_mm512_bsrli_epi128::<0x0E>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x02>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1))),
        0x0F => _mm512_or_si512(_mm512_bsrli_epi128::<0x0F>(vector), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bslli_epi128::<0x01>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1))),
        0x10 => _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), vector), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1)),
        0x11 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x01>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x0F>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1))),
        0x12 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x02>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x0E>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1))),
        0x13 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x03>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x0D>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1))),
        0x14 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x04>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x0C>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1))),
        0x15 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x05>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x0B>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1))),
        0x16 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x06>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x0A>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1))),
        0x17 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x07>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x09>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1))),
        0x18 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x08>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x08>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1))),
        0x19 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x09>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x07>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1))),
        0x1A => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x0A>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x06>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1))),
        0x1B => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x0B>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x05>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1))),
        0x1C => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x0C>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x04>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1))),
        0x1D => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x0D>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x03>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1))),
        0x1E => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x0E>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x02>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1))),
        0x1F => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(1, 0, 7, 6, 5, 4, 3, 2), _mm512_bsrli_epi128::<0x0F>(vector)), _mm512_set_epi64(0, 0, -1, -1, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bslli_epi128::<0x01>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1))),
        0x20 => _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), vector), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1)),
        0x21 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x01>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x0F>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1))),
        0x22 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x02>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x0E>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1))),
        0x23 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x03>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x0D>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1))),
        0x24 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x04>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x0C>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1))),
        0x25 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x05>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x0B>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1))),
        0x26 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x06>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x0A>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1))),
        0x27 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x07>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x09>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1))),
        0x28 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x08>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x08>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1))),
        0x29 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x09>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x07>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1))),
        0x2A => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x0A>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x06>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1))),
        0x2B => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x0B>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x05>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1))),
        0x2C => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x0C>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x04>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1))),
        0x2D => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x0D>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x03>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1))),
        0x2E => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x0E>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x02>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1))),
        0x2F => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4), _mm512_bsrli_epi128::<0x0F>(vector)), _mm512_set_epi64(0, 0, 0, 0, -1, -1, -1, -1)), _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bslli_epi128::<0x01>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1))),
        0x30 => _mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), vector), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1)),
        0x31 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x01>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1)), _mm512_setzero_si512()),
        0x32 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x02>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1)), _mm512_setzero_si512()),
        0x33 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x03>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1)), _mm512_setzero_si512()),
        0x34 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x04>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1)), _mm512_setzero_si512()),
        0x35 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x05>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1)), _mm512_setzero_si512()),
        0x36 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x06>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1)), _mm512_setzero_si512()),
        0x37 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x07>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1)), _mm512_setzero_si512()),
        0x38 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x08>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1)), _mm512_setzero_si512()),
        0x39 => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x09>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1)), _mm512_setzero_si512()),
        0x3A => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x0A>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1)), _mm512_setzero_si512()),
        0x3B => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x0B>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1)), _mm512_setzero_si512()),
        0x3C => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x0C>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1)), _mm512_setzero_si512()),
        0x3D => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x0D>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1)), _mm512_setzero_si512()),
        0x3E => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x0E>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1)), _mm512_setzero_si512()),
        0x3F => _mm512_or_si512(_mm512_and_si512(_mm512_permutexvar_epi64(_mm512_set_epi64(5, 4, 3, 2, 1, 0, 7, 6), _mm512_bsrli_epi128::<0x0F>(vector)), _mm512_set_epi64(0, 0, 0, 0, 0, 0, -1, -1)), _mm512_setzero_si512()),
        _ => _mm512_setzero_si512()
    };
}
