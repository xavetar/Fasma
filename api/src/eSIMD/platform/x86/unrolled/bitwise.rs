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
            _mm_xor_si128
        }
    }
};

#[cfg(all(target_arch = "x86", target_feature = "avx", target_feature = "avx2"))]
use core::{
    arch::{
        x86::{
            __m256i,
            _mm256_set1_epi8,
            _mm256_xor_si256
        }
    }
};

#[cfg(all(target_arch = "x86", target_feature = "avx512f"))]
use core::{
    arch::{
        x86::{
            __m512i,
            _mm512_set1_epi8,
            _mm512_xor_si512
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use core::{
    arch::{
        x86_64::{
            __m128i,
            _mm_set1_epi8,
            _mm_xor_si128
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "avx", target_feature = "avx2"))]
use core::{
    arch::{
        x86_64::{
            __m256i,
            _mm256_set1_epi8,
            _mm256_xor_si256
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
use core::{
    arch::{
        x86_64::{
            __m512i,
            _mm512_set1_epi8,
            _mm512_xor_si512
        }
    }
};


#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[inline]
pub unsafe fn _mm_not_si128(vector: __m128i) -> __m128i {
    return _mm_xor_si128(vector, _mm_set1_epi8(-0x01));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_not_si256(vector: __m256i) -> __m256i {
    return _mm256_xor_si256(vector, _mm256_set1_epi8(-0x01));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx512f"))]
#[inline]
pub unsafe fn _mm512_not_si512(vector: __m512i) -> __m512i {
    return _mm512_xor_si512(vector, _mm512_set1_epi8(-0x01));
}
