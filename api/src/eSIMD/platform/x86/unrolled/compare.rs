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
            _mm_set1_epi8, _mm_set1_epi16, _mm_set1_epi32,
            _mm_xor_si128, _mm_or_si128,
            _mm_cmpgt_epi8, _mm_cmpeq_epi8, _mm_cmplt_epi8,
            _mm_cmpgt_epi16, _mm_cmpeq_epi16, _mm_cmplt_epi16,
            _mm_cmpgt_epi32, _mm_cmpeq_epi32, _mm_cmplt_epi32,
            _mm_subs_epu8, _mm_subs_epu16,
            _mm_setzero_si128
        }
    }
};

#[cfg(all(target_arch = "x86", target_feature = "sse2", not(target_feature = "sse4.1")))]
use core::{
    arch::{
        x86::{
            _mm_and_si128
        }
    }
};

#[cfg(all(target_arch = "x86", target_feature = "sse2", target_feature = "sse4.1"))]
use core::{
    arch::{
        x86::{
            _mm_cmpeq_epi64
        }
    }
};

#[cfg(all(target_arch = "x86", target_feature = "sse2", not(target_feature = "sse4.2")))]
use core::{
    arch::{
        x86::{
            _mm_andnot_si128,
            _mm_sub_epi64,
            _mm_srai_epi32
        }
    }
};

#[cfg(all(target_arch = "x86", target_feature = "sse2", any(not(target_feature = "sse4.1"), not(target_feature = "sse4.2"))))]
use core::{
    arch::{
        x86::{
            _mm_shuffle_epi32
        }
    }
};

#[cfg(all(target_arch = "x86", target_feature = "sse2", target_feature = "sse4.2"))]
use core::{
    arch::{
        x86::{
            _mm_set1_epi64x,
            _mm_cmpgt_epi64,
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use core::{
    arch::{
        x86_64::{
            __m128i,
            _mm_set1_epi8, _mm_set1_epi16, _mm_set1_epi32,
            _mm_xor_si128, _mm_or_si128,
            _mm_cmpgt_epi8, _mm_cmpeq_epi8, _mm_cmplt_epi8,
            _mm_cmpgt_epi16, _mm_cmpeq_epi16, _mm_cmplt_epi16,
            _mm_cmpgt_epi32, _mm_cmpeq_epi32, _mm_cmplt_epi32,
            _mm_subs_epu8, _mm_subs_epu16,
            _mm_setzero_si128
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "sse2", not(target_feature = "sse4.1")))]
use core::{
    arch::{
        x86_64::{
            _mm_and_si128
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "sse2", target_feature = "sse4.1"))]
use core::{
    arch::{
        x86_64::{
            _mm_cmpeq_epi64
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "sse2", not(target_feature = "sse4.2")))]
use core::{
    arch::{
        x86_64::{
            _mm_andnot_si128,
            _mm_sub_epi64,
            _mm_srai_epi32
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "sse2", any(not(target_feature = "sse4.1"), not(target_feature = "sse4.2"))))]
use core::{
    arch::{
        x86_64::{
            _mm_shuffle_epi32
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "sse2", target_feature = "sse4.2"))]
use core::{
    arch::{
        x86_64::{
            _mm_set1_epi64x,
            _mm_cmpgt_epi64,
        }
    }
};

#[cfg(all(target_arch = "x86", target_feature = "avx", target_feature = "avx2"))]
use core::{
    arch::{
        x86::{
            __m256i,
            _mm256_set1_epi8, _mm256_set1_epi16, _mm256_set1_epi32, _mm256_set1_epi64x,
            _mm256_xor_si256, _mm256_or_si256,
            _mm256_cmpgt_epi8, _mm256_cmpeq_epi8,
            _mm256_cmpgt_epi16, _mm256_cmpeq_epi16,
            _mm256_cmpgt_epi32, _mm256_cmpeq_epi32,
            _mm256_cmpgt_epi64, _mm256_cmpeq_epi64,
            _mm256_subs_epu8, _mm256_subs_epu16,
            _mm256_setzero_si256
        }
    }
};

#[cfg(all(target_arch = "x86_64", target_feature = "avx", target_feature = "avx2"))]
use core::{
    arch::{
        x86_64::{
            __m256i,
            _mm256_set1_epi8, _mm256_set1_epi16, _mm256_set1_epi32, _mm256_set1_epi64x,
            _mm256_xor_si256, _mm256_or_si256,
            _mm256_cmpgt_epi8, _mm256_cmpeq_epi8,
            _mm256_cmpgt_epi16, _mm256_cmpeq_epi16,
            _mm256_cmpgt_epi32, _mm256_cmpeq_epi32,
            _mm256_cmpgt_epi64, _mm256_cmpeq_epi64,
            _mm256_subs_epu8, _mm256_subs_epu16,
            _mm256_setzero_si256
        }
    }
};

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[inline]
pub unsafe fn _mm_cmpge_epi8(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmple_epi8(right, left);
    // 2. return _mm_cmpeq_epi8(_mm_min_epi8(left, right), right);
    // 3. return _mm_cmpeq_epi8(_mm_max_epi8(left, right), left);
    // 4. return _mm_cmpeq_epi8(_mm_subs_epu8(right, left), _mm_setzero_si128());
    // 4. return _mm_andnot_si128(_mm_cmpgt_epi8(right, left), _mm_set1_epi8(-0x01));
    // 5. return _mm_andnot_si128(_mm_cmplt_epi8(left, right), _mm_set1_epi8(-0x01));
    // 7. return _mm_or_si128(_mm_cmplt_epi8(right, left), _mm_cmpeq_epi8(left, right));
    return _mm_or_si128(_mm_cmpgt_epi8(left, right), _mm_cmpeq_epi8(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[inline]
pub unsafe fn _mm_cmple_epi8(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpge_epi8(right, left);
    // 2. return _mm_cmpeq_epi8(_mm_min_epi8(left, right), left);
    // 3. return _mm_cmpeq_epi8(_mm_max_epi8(left, right), right);
    // 4. return _mm_cmpeq_epi8(_mm_subs_epu8(left, right), _mm_setzero_si128());
    // 5. return _mm_andnot_si128(_mm_cmpgt_epi8(left, right), _mm_set1_epi8(-0x01));
    // 6. return _mm_andnot_si128(_mm_cmplt_epi8(right, left), _mm_set1_epi8(-0x01));
    // 7. return _mm_or_si128(_mm_cmpgt_epi8(right, left), _mm_cmpeq_epi8(left, right));
    return _mm_or_si128(_mm_cmplt_epi8(left, right), _mm_cmpeq_epi8(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[inline]
pub unsafe fn _mm_cmpge_epi16(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmple_epi16(right, left);
    // 2. return _mm_cmpeq_epi16(_mm_min_epi16(left, right), right);
    // 3. return _mm_cmpeq_epi16(_mm_max_epi16(left, right), left);
    // 4. return _mm_cmpeq_epi16(_mm_subs_epu16(right, left), _mm_setzero_si128());
    // 4. return _mm_andnot_si128(_mm_cmpgt_epi16(right, left), _mm_set1_epi8(-0x01));
    // 5. return _mm_andnot_si128(_mm_cmplt_epi16(left, right), _mm_set1_epi8(-0x01));
    // 7. return _mm_or_si128(_mm_cmplt_epi16(right, left), _mm_cmpeq_epi16(left, right));
    return _mm_or_si128(_mm_cmpgt_epi16(left, right), _mm_cmpeq_epi16(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[inline]
pub unsafe fn _mm_cmple_epi16(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpge_epi16(right, left);
    // 2. return _mm_cmpeq_epi16(_mm_min_epi16(left, right), left);
    // 3. return _mm_cmpeq_epi16(_mm_max_epi16(left, right), right);
    // 4. return _mm_cmpeq_epi16(_mm_subs_epu16(left, right), _mm_setzero_si128());
    // 5. return _mm_andnot_si128(_mm_cmpgt_epi16(left, right), _mm_set1_epi8(-0x01));
    // 6. return _mm_andnot_si128(_mm_cmplt_epi16(right, left), _mm_set1_epi8(-0x01));
    // 7. return _mm_or_si128(_mm_cmpgt_epi16(right, left), _mm_cmpeq_epi16(left, right));
    return _mm_or_si128(_mm_cmplt_epi16(left, right), _mm_cmpeq_epi16(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.1")))]
#[inline]
pub unsafe fn _mm_cmpge_epi32(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmple_epi32(right, left);
    // 2. return _mm_andnot_si128(_mm_cmpgt_epi32(right, left), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmplt_epi32(left, right), _mm_set1_epi8(-0x01));
    // 4. return _mm_or_si128(_mm_cmplt_epi32(right, left), _mm_cmpeq_epi32(left, right));
    return _mm_or_si128(_mm_cmpgt_epi32(left, right), _mm_cmpeq_epi32(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.1")))]
#[inline]
pub unsafe fn _mm_cmple_epi32(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpge_epi32(right, left);
    // 2. return _mm_andnot_si128(_mm_cmpgt_epi32(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmplt_epi32(right, left), _mm_set1_epi8(-0x01));
    // 4. return _mm_or_si128(_mm_cmpgt_epi32(right, left), _mm_cmpeq_epi32(left, right));
    return _mm_or_si128(_mm_cmplt_epi32(left, right), _mm_cmpeq_epi32(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.1"))]
#[inline]
pub unsafe fn _mm_cmpge_epi32(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmple_epi32(right, left);
    // 2. return _mm_cmpeq_epi32(_mm_min_epi32(left, right), right);
    // 3. return _mm_cmpeq_epi32(_mm_max_epi32(left, right), left);
    // 4. return _mm_andnot_si128(_mm_cmpgt_epi32(right, left), _mm_set1_epi8(-0x01));
    // 5. return _mm_andnot_si128(_mm_cmplt_epi32(left, right), _mm_set1_epi8(-0x01));
    // 6. return _mm_or_si128(_mm_cmplt_epi32(right, left), _mm_cmpeq_epi32(left, right));
    return _mm_or_si128(_mm_cmpgt_epi32(left, right), _mm_cmpeq_epi32(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.1"))]
#[inline]
pub unsafe fn _mm_cmple_epi32(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpge_epi32(right, left);
    // 2. return _mm_cmpeq_epi32(_mm_min_epi32(left, right), left);
    // 3. return _mm_cmpeq_epi32(_mm_max_epi32(left, right), right);
    // 4. return _mm_andnot_si128(_mm_cmpgt_epi32(left, right), _mm_set1_epi8(-0x01));
    // 5. return _mm_andnot_si128(_mm_cmplt_epi32(right, left), _mm_set1_epi8(-0x01));
    // 6. return _mm_or_si128(_mm_cmpgt_epi32(right, left), _mm_cmpeq_epi32(left, right));
    return _mm_or_si128(_mm_cmplt_epi32(left, right), _mm_cmpeq_epi32(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.1")))]
#[inline]
pub unsafe fn _mm_cmpeq_epi64(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_or_si128(_mm_and_si128(_mm_cmpeq_epi32(left, right), _mm_slli_epi64::<0x20>(_mm_cmpeq_epi32(left, right))), _mm_and_si128(_mm_cmpeq_epi32(left, right), _mm_srli_epi64::<0x20>(_mm_cmpeq_epi32(left, right))));
    return _mm_and_si128(_mm_cmpeq_epi32(left, right), _mm_shuffle_epi32::<0xB1>(_mm_cmpeq_epi32(left, right)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.2")))]
#[inline]
pub unsafe fn _mm_cmpgt_epi64(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmplt_epi64(right, left);
    // 2. return _mm_xor_si128(_mm_cmple_epi64(left, right), _mm_set1_epi8(-0x01));
    return _mm_shuffle_epi32::<0xF5>(_mm_srai_epi32::<0x1F>(_mm_or_si128(_mm_andnot_si128(_mm_xor_si128(right, left), _mm_sub_epi64(right, left)), _mm_andnot_si128(left, right))));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.2")))]
#[inline]
pub unsafe fn _mm_cmpge_epi64(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmple_epi64(right, left);
    // 2. return _mm_andnot_si128(_mm_cmpgt_epi64(right, left), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmplt_epi64(left, right), _mm_set1_epi8(-0x01));
    // 4. return _mm_or_si128(_mm_cmplt_epi64(right, left), _mm_cmpeq_epi64(left, right));
    return _mm_or_si128(_mm_cmpgt_epi64(left, right), _mm_cmpeq_epi64(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.2")))]
#[inline]
pub unsafe fn _mm_cmplt_epi64(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpgt_epi64(right, left);
    // 2. return _mm_xor_si128(_mm_cmpge_epi64(left, right), _mm_set1_epi8(-0x01));
    return _mm_shuffle_epi32::<0xF5>(_mm_srai_epi32::<0x1F>(_mm_or_si128(_mm_andnot_si128(_mm_xor_si128(left, right), _mm_sub_epi64(left, right)), _mm_andnot_si128(right, left))));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.2")))]
#[inline]
pub unsafe fn _mm_cmple_epi64(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpge_epi64(right, left);
    // 2. return _mm_andnot_si128(_mm_cmpgt_epi64(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmplt_epi64(right, left), _mm_set1_epi8(-0x01));
    // 4. return _mm_or_si128(_mm_cmpgt_epi64(right, left), _mm_cmpeq_epi64(left, right));
    return _mm_or_si128(_mm_cmplt_epi64(left, right), _mm_cmpeq_epi64(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.2"))]
#[inline]
pub unsafe fn _mm_cmpge_epi64(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmple_epi64(right, left);
    // 2. return _mm_andnot_si128(_mm_cmpgt_epi64(right, left), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmplt_epi64(left, right), _mm_set1_epi8(-0x01));
    // 4. return _mm_or_si128(_mm_cmplt_epi64(right, left), _mm_cmpeq_epi64(left, right));
    return _mm_or_si128(_mm_cmpgt_epi64(left, right), _mm_cmpeq_epi64(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.2"))]
#[inline]
pub unsafe fn _mm_cmplt_epi64(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_xor_si128(_mm_cmpge_epi64(left, right), _mm_set1_epi8(-0x01));
    // 2. return _mm_shuffle_epi32::<0xF5>(_mm_srai_epi32::<0x1F>(_mm_or_si128(_mm_andnot_si128(_mm_xor_si128(left, right), _mm_sub_epi64(left, right)), _mm_andnot_si128(right, left))));
    return _mm_cmpgt_epi64(right, left);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.2"))]
#[inline]
pub unsafe fn _mm_cmple_epi64(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpge_epi64(right, left);
    // 2. return _mm_andnot_si128(_mm_cmpgt_epi64(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmplt_epi64(right, left), _mm_set1_epi8(-0x01));
    // 4. return _mm_or_si128(_mm_cmpgt_epi64(right, left), _mm_cmpeq_epi64(left, right));
    return _mm_or_si128(_mm_cmplt_epi64(left, right), _mm_cmpeq_epi64(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmpge_epi8(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmple_epi8(right, left);
    // 2. return _mm256_cmpeq_epi8(_mm256_min_epi8(left, right), right);
    // 3. return _mm256_cmpeq_epi8(_mm256_max_epi8(left, right), left);
    // 4. return _mm256_cmpeq_epi8(_mm256_subs_epu8(right, left), _mm256_setzero_si256());
    // 4. return _mm256_andnot_si256(_mm256_cmpgt_epi8(right, left), _mm256_set1_epi8(-0x01));
    // 5. return _mm256_andnot_si256(_mm256_cmplt_epi8(left, right), _mm256_set1_epi8(-0x01));
    // 7. return _mm256_or_si256(_mm256_cmplt_epi8(right, left), _mm256_cmpeq_epi8(left, right));
    return _mm256_or_si256(_mm256_cmpgt_epi8(left, right), _mm256_cmpeq_epi8(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmplt_epi8(left: __m256i, right: __m256i) -> __m256i {
    return _mm256_cmpgt_epi8(right, left);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmple_epi8(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmpge_epi8(right, left);
    // 2. return _mm256_cmpeq_epi8(_mm256_min_epi8(left, right), left);
    // 3. return _mm256_cmpeq_epi8(_mm256_max_epi8(left, right), right);
    // 4. return _mm256_cmpeq_epi8(_mm256_subs_epu8(left, right), _mm256_setzero_si256());
    // 5. return _mm256_andnot_si256(_mm256_cmpgt_epi8(left, right), _mm256_set1_epi8(-0x01));
    // 6. return _mm256_andnot_si256(_mm256_cmplt_epi8(right, left), _mm256_set1_epi8(-0x01));
    // 7. return _mm256_or_si256(_mm256_cmpgt_epi8(right, left), _mm256_cmpeq_epi8(left, right));
    return _mm256_or_si256(_mm256_cmplt_epi8(left, right), _mm256_cmpeq_epi8(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmpge_epi16(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmple_epi16(right, left);
    // 2. return _mm256_cmpeq_epi16(_mm256_min_epi16(left, right), right);
    // 3. return _mm256_cmpeq_epi16(_mm256_max_epi16(left, right), left);
    // 4. return _mm256_cmpeq_epi16(_mm256_subs_epu16(right, left), _mm256_setzero_si256());
    // 4. return _mm256_andnot_si256(_mm256_cmpgt_epi16(right, left), _mm256_set1_epi8(-0x01));
    // 5. return _mm256_andnot_si256(_mm256_cmplt_epi16(left, right), _mm256_set1_epi8(-0x01));
    // 7. return _mm256_or_si256(_mm256_cmplt_epi16(right, left), _mm256_cmpeq_epi16(left, right));
    return _mm256_or_si256(_mm256_cmpgt_epi16(left, right), _mm256_cmpeq_epi16(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmplt_epi16(left: __m256i, right: __m256i) -> __m256i {
    return _mm256_cmpgt_epi16(right, left);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmple_epi16(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmpge_epi16(right, left);
    // 2. return _mm256_cmpeq_epi16(_mm256_min_epi16(left, right), left);
    // 3. return _mm256_cmpeq_epi16(_mm256_max_epi16(left, right), right);
    // 4. return _mm256_cmpeq_epi16(_mm256_subs_epu16(left, right), _mm256_setzero_si256());
    // 5. return _mm256_andnot_si256(_mm256_cmpgt_epi16(left, right), _mm256_set1_epi8(-0x01));
    // 6. return _mm256_andnot_si256(_mm256_cmplt_epi16(right, left), _mm256_set1_epi8(-0x01));
    // 7. return _mm256_or_si256(_mm256_cmpgt_epi16(right, left), _mm256_cmpeq_epi16(left, right));
    return _mm256_or_si256(_mm256_cmplt_epi16(left, right), _mm256_cmpeq_epi16(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmpge_epi32(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmple_epi32(right, left);
    // 2. return _mm256_cmpeq_epi32(_mm256_min_epi32(left, right), right);
    // 3. return _mm256_cmpeq_epi32(_mm256_max_epi32(left, right), left);
    // 4. return _mm256_andnot_si256(_mm256_cmpgt_epi32(right, left), _mm256_set1_epi8(-0x01));
    // 5. return _mm256_andnot_si256(_mm256_cmplt_epi32(left, right), _mm256_set1_epi8(-0x01));
    // 6. return _mm256_or_si256(_mm256_cmplt_epi32(right, left), _mm256_cmpeq_epi32(left, right));
    return _mm256_or_si256(_mm256_cmpgt_epi32(left, right), _mm256_cmpeq_epi32(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmplt_epi32(left: __m256i, right: __m256i) -> __m256i {
    return _mm256_cmpgt_epi32(right, left);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmple_epi32(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmpge_epi32(right, left);
    // 2. return _mm256_cmpeq_epi32(_mm256_min_epi32(left, right), left);
    // 3. return _mm256_cmpeq_epi32(_mm256_max_epi32(left, right), right);
    // 4. return _mm256_andnot_si256(_mm256_cmpgt_epi32(left, right), _mm256_set1_epi8(-0x01));
    // 5. return _mm256_andnot_si256(_mm256_cmplt_epi32(right, left), _mm256_set1_epi8(-0x01));
    // 6. return _mm256_or_si256(_mm256_cmpgt_epi32(right, left), _mm256_cmpeq_epi32(left, right));
    return _mm256_or_si256(_mm256_cmplt_epi32(left, right), _mm256_cmpeq_epi32(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmpge_epi64(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmple_epi64(right, left);
    // 2. return _mm256_andnot_si256(_mm256_cmpgt_epi64(right, left), _mm256_set1_epi8(-0x01));
    // 3. return _mm256_andnot_si256(_mm256_cmplt_epi64(left, right), _mm256_set1_epi8(-0x01));
    // 4. return _mm256_or_si256(_mm256_cmplt_epi64(right, left), _mm256_cmpeq_epi64(left, right));
    return _mm256_or_si256(_mm256_cmpgt_epi64(left, right), _mm256_cmpeq_epi64(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmplt_epi64(left: __m256i, right: __m256i) -> __m256i {
    return _mm256_cmpgt_epi64(right, left);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmple_epi64(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmpge_epi64(right, left);
    // 2. return _mm256_andnot_si256(_mm256_cmpgt_epi64(left, right), _mm256_set1_epi8(-0x01));
    // 3. return _mm256_andnot_si256(_mm256_cmplt_epi64(right, left), _mm256_set1_epi8(-0x01));
    // 4. return _mm256_or_si256(_mm256_cmpgt_epi64(right, left), _mm256_cmpeq_epi64(left, right));
    return _mm256_or_si256(_mm256_cmplt_epi64(left, right), _mm256_cmpeq_epi64(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[inline]
pub unsafe fn _mm_cmpgt_epu8(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmplt_epu8(right, left);
    // 2. return _mm_xor_si128(_mm_cmple_epu8(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_xor_si128(_mm_cmpeq_epi8(_mm_min_epu8(left, right), left), _mm_set1_epi8(-0x01));
    // 4. return _mm_xor_si128(_mm_cmpeq_epi8(_mm_max_epu8(left, right), right), _mm_set1_epi8(-0x01));
    // 5. return _mm_xor_si128(_mm_cmpeq_epi8(_mm_subs_epu8(left, right), _mm_setzero_si128()), _mm_set1_epi8(-0x01));
    // 6. return _mm_andnot_si128(_mm_cmpeq_epi8(left, right), _mm_cmpeq_epi8(_mm_max_epu8(left, right), left));
    // 7. return _mm_andnot_si128(_mm_cmpeq_epi8(left, right), _mm_cmple_epu8(right, left));
    // 8. return _mm_cmplt_epi8(_mm_xor_si128(right, _mm_set1_epi8(-0x80)), _mm_xor_si128(left, _mm_set1_epi8(-0x80)));
    return _mm_cmpgt_epi8(_mm_xor_si128(left, _mm_set1_epi8(-0x80)), _mm_xor_si128(right, _mm_set1_epi8(-0x80)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[inline]
pub unsafe fn _mm_cmpge_epu8(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmple_epu8(right, left);
    // 2. return _mm_cmpeq_epi8(_mm_min_epu8(left, right), right);
    // 3. return _mm_cmpeq_epi8(_mm_max_epu8(left, right), left);
    // 4. return _mm_andnot_si128(_mm_cmpgt_epu8(right, left), _mm_set1_epi8(-0x01));
    // 5. return _mm_andnot_si128(_mm_cmplt_epu8(left, right), _mm_set1_epi8(-0x01));
    // 6. return _mm_or_si128(_mm_cmpgt_epu8(left, right), _mm_cmpeq_epi8(left, right));
    // 7. return _mm_or_si128(_mm_cmplt_epu8(right, left), _mm_cmpeq_epi8(left, right));
    return _mm_cmpeq_epi8(_mm_subs_epu8(right, left), _mm_setzero_si128());
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[inline]
pub unsafe fn _mm_cmplt_epu8(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpgt_epu8(right, left);
    // 2. return _mm_xor_si128(_mm_cmpge_epu8(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_xor_si128(_mm_cmpeq_epi8(_mm_min_epu8(left, right), right), _mm_set1_epi8(-0x01));
    // 4. return _mm_xor_si128(_mm_cmpeq_epi8(_mm_max_epu8(left, right), left), _mm_set1_epi8(-0x01));
    // 5. return _mm_xor_si128(_mm_cmpeq_epi8(_mm_subs_epu8(right, left), _mm_setzero_si128()), _mm_set1_epi8(-0x01));
    // 6. return _mm_andnot_si128(_mm_cmpeq_epi8(left, right), _mm_cmpeq_epi8(_mm_max_epu8(left, right), right));
    // 7. return _mm_andnot_si128(_mm_cmpeq_epi8(left, right), _mm_cmple_epu8(left, right));
    // 8. return _mm_cmpgt_epi8(_mm_xor_si128(right, _mm_set1_epi8(-0x80)), _mm_xor_si128(left, _mm_set1_epi8(-0x80)));
    return _mm_cmplt_epi8(_mm_xor_si128(left, _mm_set1_epi8(-0x80)), _mm_xor_si128(right, _mm_set1_epi8(-0x80)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[inline]
pub unsafe fn _mm_cmple_epu8(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpge_epu8(right, left);
    // 2. return _mm_cmpeq_epi8(_mm_min_epu8(left, right), left);
    // 3. return _mm_cmpeq_epi8(_mm_max_epu8(left, right), right);
    // 4. return _mm_andnot_si128(_mm_cmpgt_epu8(left, right), _mm_set1_epi8(-0x01));
    // 5. return _mm_andnot_si128(_mm_cmplt_epu8(right, left), _mm_set1_epi8(-0x01));
    // 6. return _mm_or_si128(_mm_cmpgt_epu8(right, left), _mm_cmpeq_epi8(left, right));
    // 7. return _mm_or_si128(_mm_cmplt_epu8(left, right), _mm_cmpeq_epi8(left, right));
    return _mm_cmpeq_epi8(_mm_subs_epu8(left, right), _mm_setzero_si128());
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.1")))]
#[inline]
pub unsafe fn _mm_cmpgt_epu16(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmplt_epu16(right, left);
    // 2. return _mm_xor_si128(_mm_cmple_epu16(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_xor_si128(_mm_cmpeq_epi16(_mm_subs_epu16(left, right), _mm_setzero_si128()), _mm_set1_epi8(-0x01));
    // 4. return _mm_andnot_si128(_mm_cmpeq_epi16(left, right), _mm_cmple_epu16(right, left));
    // 5. return _mm_cmplt_epi16(_mm_xor_si128(right, _mm_set1_epi16(-0x8000)), _mm_xor_si128(left, _mm_set1_epi8(-0x8000)));
    return _mm_cmpgt_epi16(_mm_xor_si128(left, _mm_set1_epi16(-0x8000)), _mm_xor_si128(right, _mm_set1_epi16(-0x8000)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.1")))]
#[inline]
pub unsafe fn _mm_cmpge_epu16(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmple_epu16(right, left);
    // 2. return _mm_andnot_si128(_mm_cmpgt_epu16(right, left), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmplt_epu16(left, right), _mm_set1_epi8(-0x01));
    // 4. return _mm_or_si128(_mm_cmpgt_epu16(left, right), _mm_cmpeq_epi16(left, right));
    // 5. return _mm_or_si128(_mm_cmplt_epu16(right, left), _mm_cmpeq_epi16(left, right));
    return _mm_cmpeq_epi16(_mm_subs_epu16(right, left), _mm_setzero_si128());
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.1")))]
#[inline]
pub unsafe fn _mm_cmplt_epu16(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpgt_epu16(right, left);
    // 2. return _mm_xor_si128(_mm_cmpge_epu16(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_xor_si128(_mm_cmpeq_epi16(_mm_subs_epu16(right, left), _mm_setzero_si128()), _mm_set1_epi8(-0x01));
    // 4. return _mm_andnot_si128(_mm_cmpeq_epi16(left, right), _mm_cmple_epu16(left, right));
    // 5. return _mm_cmpgt_epi16(_mm_xor_si128(right, _mm_set1_epi16(-0x8000)), _mm_xor_si128(left, _mm_set1_epi16(-0x8000)));
    return _mm_cmplt_epi16(_mm_xor_si128(left, _mm_set1_epi16(-0x8000)), _mm_xor_si128(right, _mm_set1_epi16(-0x8000)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.1")))]
#[inline]
pub unsafe fn _mm_cmple_epu16(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpge_epu16(right, left);
    // 2. return _mm_andnot_si128(_mm_cmpgt_epu16(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmplt_epu16(right, left), _mm_set1_epi8(-0x01));
    // 4. return _mm_or_si128(_mm_cmpgt_epu16(right, left), _mm_cmpeq_epi16(left, right));
    // 5. return _mm_or_si128(_mm_cmplt_epu16(left, right), _mm_cmpeq_epi16(left, right));
    return _mm_cmpeq_epi16(_mm_subs_epu16(left, right), _mm_setzero_si128());
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.1"))]
#[inline]
pub unsafe fn _mm_cmpgt_epu16(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmplt_epu16(right, left);
    // 2. return _mm_xor_si128(_mm_cmple_epu16(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_xor_si128(_mm_cmpeq_epi16(_mm_min_epu16(left, right), right), _mm_set1_epi8(-0x01));
    // 4. return _mm_xor_si128(_mm_cmpeq_epi16(_mm_max_epu16(left, right), right), _mm_set1_epi8(-0x01));
    // 5. return _mm_xor_si128(_mm_cmpeq_epi16(_mm_subs_epu16(left, right), _mm_setzero_si128()), _mm_set1_epi8(-0x01));
    // 6. return _mm_andnot_si128(_mm_cmpeq_epi16(left, right), _mm_cmpeq_epi16(_mm_max_epu16(left, right), left));
    // 7. return _mm_andnot_si128(_mm_cmpeq_epi16(left, right), _mm_cmple_epu16(right, left));
    // 8. return _mm_cmplt_epi16(_mm_xor_si128(right, _mm_set1_epi16(-0x8000)), _mm_xor_si128(left, _mm_set1_epi16(-0x8000)));
    return _mm_cmpgt_epi16(_mm_xor_si128(left, _mm_set1_epi16(-0x8000)), _mm_xor_si128(right, _mm_set1_epi16(-0x8000)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.1"))]
#[inline]
pub unsafe fn _mm_cmpge_epu16(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmple_epu16(right, left);
    // 2. return _mm_cmpeq_epi16(_mm_min_epu16(left, right), right);
    // 3. return _mm_cmpeq_epi16(_mm_max_epu16(left, right), left);
    // 4. return _mm_andnot_si128(_mm_cmpgt_epu16(right, left), _mm_set1_epi8(-0x01));
    // 5. return _mm_andnot_si128(_mm_cmplt_epu16(left, right), _mm_set1_epi8(-0x01));
    // 6. return _mm_or_si128(_mm_cmpgt_epu16(left, right), _mm_cmpeq_epi16(left, right));
    // 7. return _mm_or_si128(_mm_cmplt_epu16(right, left), _mm_cmpeq_epi16(left, right));
    return _mm_cmpeq_epi16(_mm_subs_epu16(right, left), _mm_setzero_si128());
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.1"))]
#[inline]
pub unsafe fn _mm_cmplt_epu16(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpgt_epu16(right, left);
    // 2. return _mm_xor_si128(_mm_cmpge_epu16(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_xor_si128(_mm_cmpeq_epi16(_mm_min_epu16(left, right), right), _mm_set1_epi8(-0x01));
    // 4. return _mm_xor_si128(_mm_cmpeq_epi16(_mm_max_epu16(left, right), left), _mm_set1_epi8(-0x01));
    // 5. return _mm_xor_si128(_mm_cmpeq_epi16(_mm_subs_epu16(right, left), _mm_setzero_si128()), _mm_set1_epi8(-0x01));
    // 6. return _mm_andnot_si128(_mm_cmpeq_epi16(left, right), _mm_cmpeq_epi16(_mm_max_epu16(left, right), right));
    // 7. return _mm_andnot_si128(_mm_cmpeq_epi16(left, right), _mm_cmple_epu16(left, right));
    // 8. return _mm_cmpgt_epi16(_mm_xor_si128(right, _mm_set1_epi16(-0x8000)), _mm_xor_si128(left, _mm_set1_epi16(-0x8000)));
    return _mm_cmplt_epi16(_mm_xor_si128(left, _mm_set1_epi16(-0x8000)), _mm_xor_si128(right, _mm_set1_epi16(-0x8000)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.1"))]
#[inline]
pub unsafe fn _mm_cmple_epu16(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpge_epu16(right, left);
    // 2. return _mm_cmpeq_epi16(_mm_min_epu16(left, right), left);
    // 3. return _mm_cmpeq_epi16(_mm_max_epu16(left, right), right);
    // 4. return _mm_andnot_si128(_mm_cmpgt_epu16(left, right), _mm_set1_epi8(-0x01));
    // 5. return _mm_andnot_si128(_mm_cmplt_epu16(right, left), _mm_set1_epi8(-0x01));
    // 6. return _mm_or_si128(_mm_cmpgt_epu16(right, left), _mm_cmpeq_epi16(left, right));
    // 7. return _mm_or_si128(_mm_cmplt_epu16(left, right), _mm_cmpeq_epi16(left, right));
    return _mm_cmpeq_epi16(_mm_subs_epu16(left, right), _mm_setzero_si128());
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.1")))]
#[inline]
pub unsafe fn _mm_cmpgt_epu32(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmplt_epu32(right, left);
    // 2. return _mm_xor_si128(_mm_cmple_epu32(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmpeq_epi32(left, right), _mm_cmple_epu32(right, left));
    // 4. return _mm_cmplt_epi32(_mm_xor_si128(right, _mm_set1_epi32(-0x80000000)), _mm_xor_si128(left, _mm_set1_epi32(-0x80000000)));
    return _mm_cmpgt_epi32(_mm_xor_si128(left, _mm_set1_epi32(-0x80000000)), _mm_xor_si128(right, _mm_set1_epi32(-0x80000000)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.1")))]
#[inline]
pub unsafe fn _mm_cmpge_epu32(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmple_epu32(right, left);
    // 2. return _mm_andnot_si128(_mm_cmpgt_epu32(right, left), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmplt_epu32(left, right), _mm_set1_epi8(-0x01));
    // 4. return _mm_or_si128(_mm_cmplt_epu32(right, left), _mm_cmpeq_epi32(left, right));
    return _mm_or_si128(_mm_cmpgt_epu32(left, right), _mm_cmpeq_epi32(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.1")))]
#[inline]
pub unsafe fn _mm_cmplt_epu32(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpgt_epu32(right, left);
    // 2. return _mm_xor_si128(_mm_cmpge_epu32(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmpeq_epi32(right, left), _mm_cmple_epu32(left, right));
    // 4. return _mm_cmpgt_epi32(_mm_xor_si128(right, _mm_set1_epi32(-0x80000000)), _mm_xor_si128(left, _mm_set1_epi32(-0x80000000)));
    return _mm_cmplt_epi32(_mm_xor_si128(left, _mm_set1_epi32(-0x80000000)), _mm_xor_si128(right, _mm_set1_epi32(-0x80000000)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.1")))]
#[inline]
pub unsafe fn _mm_cmple_epu32(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpge_epu32(right, left);
    // 2. return _mm_andnot_si128(_mm_cmpgt_epu32(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmplt_epu32(right, left), _mm_set1_epi8(-0x01));
    // 4. return _mm_or_si128(_mm_cmpgt_epu32(right, left), _mm_cmpeq_epi32(left, right));
    return _mm_or_si128(_mm_cmplt_epu32(left, right), _mm_cmpeq_epi32(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.1"))]
#[inline]
pub unsafe fn _mm_cmpgt_epu32(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmplt_epu32(right, left);
    // 2. return _mm_xor_si128(_mm_cmple_epu32(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_xor_si128(_mm_cmpeq_epi32(_mm_min_epu32(left, right), right), _mm_set1_epi8(-0x01));
    // 4. return _mm_xor_si128(_mm_cmpeq_epi32(_mm_max_epu32(left, right), right), _mm_set1_epi8(-0x01));
    // 5. return _mm_andnot_si128(_mm_cmpeq_epi32(left, right), _mm_cmpeq_epi32(_mm_max_epu32(left, right), left));
    // 6. return _mm_andnot_si128(_mm_cmpeq_epi32(left, right), _mm_cmple_epu32(right, left));
    // 7. return _mm_cmplt_epi32(_mm_xor_si128(right, _mm_set1_epi32(-0x80000000)), _mm_xor_si128(left, _mm_set1_epi32(-0x80000000)));
    return _mm_cmpgt_epi32(_mm_xor_si128(left, _mm_set1_epi32(-0x80000000)), _mm_xor_si128(right, _mm_set1_epi32(-0x80000000)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.1"))]
#[inline]
pub unsafe fn _mm_cmpge_epu32(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmple_epu32(right, left);
    // 2. return _mm_cmpeq_epi32(_mm_min_epu32(left, right), right);
    // 3. return _mm_cmpeq_epi32(_mm_max_epu32(left, right), left);
    // 4. return _mm_andnot_si128(_mm_cmpgt_epu32(right, left), _mm_set1_epi8(-0x01));
    // 5. return _mm_andnot_si128(_mm_cmplt_epu32(left, right), _mm_set1_epi8(-0x01));
    // 6. return _mm_or_si128(_mm_cmplt_epu32(right, left), _mm_cmpeq_epi32(left, right));
    return _mm_or_si128(_mm_cmpgt_epu32(left, right), _mm_cmpeq_epi32(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.1"))]
#[inline]
pub unsafe fn _mm_cmplt_epu32(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpgt_epu32(right, left);
    // 2. return _mm_xor_si128(_mm_cmpge_epu32(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_xor_si128(_mm_cmpeq_epi32(_mm_min_epu32(left, right), right), _mm_set1_epi8(-0x01));
    // 4. return _mm_xor_si128(_mm_cmpeq_epi32(_mm_max_epu32(left, right), left), _mm_set1_epi8(-0x01));
    // 5. return _mm_andnot_si128(_mm_cmpeq_epi32(left, right), _mm_cmpeq_epi32(_mm_max_epu16(left, right), right));
    // 6. return _mm_andnot_si128(_mm_cmpeq_epi32(left, right), _mm_cmple_epu32(left, right));
    // 7. return _mm_cmpgt_epi32(_mm_xor_si128(right, _mm_set1_epi32(-0x80000000)), _mm_xor_si128(left, _mm_set1_epi32(-0x80000000)));
    return _mm_cmplt_epi32(_mm_xor_si128(left, _mm_set1_epi32(-0x80000000)), _mm_xor_si128(right, _mm_set1_epi32(-0x80000000)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.1"))]
#[inline]
pub unsafe fn _mm_cmple_epu32(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpge_epu32(right, left);
    // 2. return _mm_cmpeq_epi32(_mm_min_epu32(left, right), left);
    // 3. return _mm_cmpeq_epi32(_mm_max_epu32(left, right), right);
    // 4. return _mm_andnot_si128(_mm_cmpgt_epu32(left, right), _mm_set1_epi8(-0x01));
    // 5. return _mm_andnot_si128(_mm_cmplt_epu32(right, left), _mm_set1_epi8(-0x01));
    // 6. return _mm_or_si128(_mm_cmpgt_epu32(right, left), _mm_cmpeq_epi32(left, right));
    return _mm_or_si128(_mm_cmplt_epu32(left, right), _mm_cmpeq_epi32(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.2")))]
#[inline]
pub unsafe fn _mm_cmpgt_epu64(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmplt_epu64(right, left);
    // 2. return _mm_xor_si128(_mm_cmple_epu64(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmpeq_epi64(left, right), _mm_cmple_epu64(right, left));
    // 4. return _mm_cmplt_epi64(_mm_xor_si128(left, _mm_set1_epi64x(-0x8000000000000000)), _mm_xor_si128(left, _mm_set1_epi64x(-0x8000000000000000)));
    // 5. return _mm_cmpgt_epi64(_mm_xor_si128(left, _mm_set1_epi64x(-0x8000000000000000)), _mm_xor_si128(right, _mm_set1_epi64x(-0x8000000000000000)));
    return _mm_shuffle_epi32::<0xF5>(_mm_srai_epi32::<0x1F>(_mm_or_si128(_mm_andnot_si128(_mm_xor_si128(right, left), _mm_sub_epi64(right, left)), _mm_andnot_si128(right, left))));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.2")))]
#[inline]
pub unsafe fn _mm_cmpge_epu64(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmple_epu64(right, left);
    // 2. return _mm_andnot_si128(_mm_cmpgt_epu64(right, left), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmplt_epu64(left, right), _mm_set1_epi8(-0x01));
    // 4. return _mm_or_si128(_mm_cmplt_epu64(right, left), _mm_cmpeq_epi64(left, right));
    return _mm_or_si128(_mm_cmpgt_epu64(left, right), _mm_cmpeq_epi64(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.2")))]
#[inline]
pub unsafe fn _mm_cmplt_epu64(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpgt_epu64(right, left);
    // 2. return _mm_xor_si128(_mm_cmpge_epu64(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmpeq_epi64(left, right), _mm_cmple_epu64(left, right));
    // 4. return _mm_cmpgt_epi64(_mm_xor_si128(right, _mm_set1_epi64x(-0x8000000000000000)), _mm_xor_si128(left, _mm_set1_epi64x(-0x8000000000000000)));
    // 5. return _mm_cmplt_epi64(_mm_xor_si128(left, _mm_set1_epi64x(-0x8000000000000000), _mm_xor_si128(right, _mm_set1_epi64x(-0x8000000000000000))));
    return _mm_shuffle_epi32::<0xF5>(_mm_srai_epi32::<0x1F>(_mm_or_si128(_mm_andnot_si128(_mm_xor_si128(left, right), _mm_sub_epi64(left, right)), _mm_andnot_si128(left, right))));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", not(target_feature = "sse4.2")))]
#[inline]
pub unsafe fn _mm_cmple_epu64(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpge_epu64(right, left);
    // 2. return _mm_andnot_si128(_mm_cmpgt_epu64(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmplt_epu64(right, left), _mm_set1_epi8(-0x01));
    // 4. return _mm_or_si128(_mm_cmpgt_epu64(right, left), _mm_cmpeq_epi64(left, right));
    return _mm_or_si128(_mm_cmplt_epu64(left, right), _mm_cmpeq_epi64(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.2"))]
#[inline]
pub unsafe fn _mm_cmpgt_epu64(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmplt_epu64(right, left);
    // 2. return _mm_xor_si128(_mm_cmple_epu64(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmpeq_epi64(left, right), _mm_cmple_epu64(right, left));
    // 4. return _mm_cmplt_epi64(_mm_xor_si128(left, _mm_set1_epi64x(-0x8000000000000000)), _mm_xor_si128(left, _mm_set1_epi64x(-0x8000000000000000)));
    // 5. return _mm_cmpgt_epi64(_mm_xor_si128(left, _mm_set1_epi64x(-0x8000000000000000)), _mm_xor_si128(right, _mm_set1_epi64x(-0x8000000000000000)));
    // 6. return _mm_shuffle_epi32::<0xF5>(_mm_srai_epi32::<0x1F>(_mm_or_si128(_mm_andnot_si128(_mm_xor_si128(right, left), _mm_sub_epi64(right, left)), _mm_andnot_si128(right, left))));
    return _mm_cmpgt_epi64(_mm_xor_si128(left, _mm_set1_epi64x(-0x8000000000000000)), _mm_xor_si128(right, _mm_set1_epi64x(-0x8000000000000000)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.2"))]
#[inline]
pub unsafe fn _mm_cmpge_epu64(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmple_epu64(right, left);
    // 2. return _mm_andnot_si128(_mm_cmpgt_epu64(right, left), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmplt_epu64(left, right), _mm_set1_epi8(-0x01));
    // 4. return _mm_or_si128(_mm_cmplt_epu64(right, left), _mm_cmpeq_epi64(left, right));
    return _mm_or_si128(_mm_cmpgt_epu64(left, right), _mm_cmpeq_epi64(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.2"))]
#[inline]
pub unsafe fn _mm_cmplt_epu64(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpgt_epu64(right, left);
    // 2. return _mm_xor_si128(_mm_cmpge_epu64(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmpeq_epi64(left, right), _mm_cmple_epu64(left, right));
    // 4. return _mm_cmpgt_epi64(_mm_xor_si128(right, _mm_set1_epi64x(-0x8000000000000000)), _mm_xor_si128(left, _mm_set1_epi64x(-0x8000000000000000)));
    // 5. return _mm_cmplt_epi64(_mm_xor_si128(left, _mm_set1_epi64x(-0x8000000000000000), _mm_xor_si128(right, _mm_set1_epi64x(-0x8000000000000000))));
    // 6. return _mm_shuffle_epi32::<0xF5>(_mm_srai_epi32::<0x1F>(_mm_or_si128(_mm_andnot_si128(_mm_xor_si128(left, right), _mm_sub_epi64(left, right)), _mm_andnot_si128(left, right))));
    return _mm_cmplt_epi64(_mm_xor_si128(left, _mm_set1_epi64x(-0x8000000000000000)), _mm_xor_si128(right, _mm_set1_epi64x(-0x8000000000000000)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2", target_feature = "sse4.2"))]
#[inline]
pub unsafe fn _mm_cmple_epu64(left: __m128i, right: __m128i) -> __m128i {
    // 1. return _mm_cmpge_epu64(right, left);
    // 2. return _mm_andnot_si128(_mm_cmpgt_epu64(left, right), _mm_set1_epi8(-0x01));
    // 3. return _mm_andnot_si128(_mm_cmplt_epu64(right, left), _mm_set1_epi8(-0x01));
    // 4. return _mm_or_si128(_mm_cmpgt_epu64(right, left), _mm_cmpeq_epi64(left, right));
    return _mm_or_si128(_mm_cmplt_epu64(left, right), _mm_cmpeq_epi64(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmpgt_epu8(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmplt_epu8(right, left);
    // 2. return _mm256_xor_si256(_mm256_cmple_epu8(left, right), _mm256_set1_epi8(-0x01));
    // 3. return _mm256_xor_si256(_mm256_cmpeq_epi8(_mm256_min_epu8(left, right), left), _mm256_set1_epi8(-0x01));
    // 4. return _mm256_xor_si256(_mm256_cmpeq_epi8(_mm256_max_epu8(left, right), right), _mm256_set1_epi8(-0x01));
    // 5. return _mm256_xor_si256(_mm256_cmpeq_epi8(_mm256_subs_epu8(left, right), _mm256_setzero_si256()), _mm256_set1_epi8(-0x01));
    // 6. return _mm256_andnot_si256(_mm256_cmpeq_epi8(left, right), _mm256_cmpeq_epi8(_mm256_max_epu8(left, right), left));
    // 7. return _mm256_andnot_si256(_mm256_cmpeq_epi8(left, right), _mm256_cmple_epu8(right, left));
    // 8. return _mm256_cmplt_epi8(_mm256_xor_si256(right, _mm256_set1_epi8(-0x80)), _mm256_xor_si256(left, _mm256_set1_epi8(-0x80)));
    return _mm256_cmpgt_epi8(_mm256_xor_si256(left, _mm256_set1_epi8(-0x80)), _mm256_xor_si256(right, _mm256_set1_epi8(-0x80)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmpge_epu8(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmple_epu8(right, left);
    // 2. return _mm256_cmpeq_epi8(_mm256_min_epu8(left, right), right);
    // 3. return _mm256_cmpeq_epi8(_mm256_max_epu8(left, right), left);
    // 4. return _mm256_andnot_si256(_mm256_cmpgt_epu8(right, left), _mm256_set1_epi8(-0x01));
    // 5. return _mm256_andnot_si256(_mm256_cmplt_epu8(left, right), _mm256_set1_epi8(-0x01));
    // 6. return _mm256_or_si256(_mm256_cmpgt_epu8(left, right), _mm256_cmpeq_epi8(left, right));
    // 7. return _mm256_or_si256(_mm256_cmplt_epu8(right, left), _mm256_cmpeq_epi8(left, right));
    return _mm256_cmpeq_epi8(_mm256_subs_epu8(right, left), _mm256_setzero_si256());
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmplt_epu8(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmpgt_epu8(right, left);
    // 2. return _mm256_xor_si256(_mm256_cmpge_epu8(left, right), _mm256_set1_epi8(-0x01));
    // 3. return _mm256_xor_si256(_mm256_cmpeq_epi8(_mm256_min_epu8(left, right), right), _mm256_set1_epi8(-0x01));
    // 4. return _mm256_xor_si256(_mm256_cmpeq_epi8(_mm256_max_epu8(left, right), left), _mm256_set1_epi8(-0x01));
    // 5. return _mm256_xor_si256(_mm256_cmpeq_epi8(_mm256_subs_epu8(right, left), _mm256_setzero_si256()), _mm256_set1_epi8(-0x01));
    // 6. return _mm256_andnot_si256(_mm256_cmpeq_epi8(left, right), _mm256_cmpeq_epi8(_mm256_max_epu8(left, right), right));
    // 7. return _mm256_andnot_si256(_mm256_cmpeq_epi8(left, right), _mm256_cmple_epu8(left, right));
    // 8. return _mm256_cmpgt_epi8(_mm256_xor_si256(right, _mm256_set1_epi8(-0x80)), _mm256_xor_si256(left, _mm256_set1_epi8(-0x80)));
    return _mm256_cmplt_epi8(_mm256_xor_si256(left, _mm256_set1_epi8(-0x80)), _mm256_xor_si256(right, _mm256_set1_epi8(-0x80)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmple_epu8(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmpge_epu8(right, left);
    // 2. return _mm256_cmpeq_epi8(_mm256_min_epu8(left, right), left);
    // 3. return _mm256_cmpeq_epi8(_mm256_max_epu8(left, right), right);
    // 4. return _mm256_andnot_si256(_mm256_cmpgt_epu8(left, right), _mm256_set1_epi8(-0x01));
    // 5. return _mm256_andnot_si256(_mm256_cmplt_epu8(right, left), _mm256_set1_epi8(-0x01));
    // 6. return _mm256_or_si256(_mm256_cmpgt_epu8(right, left), _mm256_cmpeq_epi8(left, right));
    // 7. return _mm256_or_si256(_mm256_cmplt_epu8(left, right), _mm256_cmpeq_epi8(left, right));
    return _mm256_cmpeq_epi8(_mm256_subs_epu8(left, right), _mm256_setzero_si256());
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmpgt_epu16(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmplt_epu16(right, left);
    // 2. return _mm256_xor_si256(_mm256_cmple_epu16(left, right), _mm256_set1_epi8(-0x01));
    // 3. return _mm256_xor_si256(_mm256_cmpeq_epi16(_mm256_min_epu16(left, right), left), _mm256_set1_epi8(-0x01));
    // 4. return _mm256_xor_si256(_mm256_cmpeq_epi16(_mm256_max_epu16(left, right), right), _mm256_set1_epi8(-0x01));
    // 5. return _mm256_xor_si256(_mm256_cmpeq_epi16(_mm256_subs_epu16(left, right), _mm256_setzero_si256()), _mm256_set1_epi8(-0x01));
    // 6. return _mm256_andnot_si256(_mm256_cmpeq_epi16(left, right), _mm256_cmpeq_epi16(_mm256_max_epu16(left, right), left));
    // 7. return _mm256_andnot_si256(_mm256_cmpeq_epi16(left, right), _mm256_cmple_epu16(right, left));
    // 8. return _mm256_cmplt_epi16(_mm256_xor_si256(right, _mm256_set1_epi16(-0x8000)), _mm256_xor_si256(left, _mm256_set1_epi16(-0x8000)));
    return _mm256_cmpgt_epi16(_mm256_xor_si256(left, _mm256_set1_epi16(-0x8000)), _mm256_xor_si256(right, _mm256_set1_epi16(-0x8000)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmpge_epu16(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmple_epu16(right, left);
    // 2. return _mm256_cmpeq_epi16(_mm256_min_epu16(left, right), right);
    // 3. return _mm256_cmpeq_epi16(_mm256_max_epu16(left, right), left);
    // 4. return _mm256_andnot_si256(_mm256_cmpgt_epu16(right, left), _mm256_set1_epi8(-0x01));
    // 5. return _mm256_andnot_si256(_mm256_cmplt_epu16(left, right), _mm256_set1_epi8(-0x01));
    // 6. return _mm256_or_si256(_mm256_cmpgt_epu16(left, right), _mm256_cmpeq_epi16(left, right));
    // 7. return _mm256_or_si256(_mm256_cmplt_epu16(right, left), _mm256_cmpeq_epi16(left, right));
    return _mm256_cmpeq_epi16(_mm256_subs_epu16(right, left), _mm256_setzero_si256());
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmplt_epu16(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmpgt_epu16(right, left);
    // 2. return _mm256_xor_si256(_mm256_cmpge_epu16(left, right), _mm256_set1_epi8(-0x01));
    // 3. return _mm256_xor_si256(_mm256_cmpeq_epi16(_mm256_min_epu16(left, right), right), _mm256_set1_epi8(-0x01));
    // 4. return _mm256_xor_si256(_mm256_cmpeq_epi16(_mm256_max_epu16(left, right), left), _mm256_set1_epi8(-0x01));
    // 5. return _mm256_xor_si256(_mm256_cmpeq_epi16(_mm256_subs_epu16(right, left), _mm256_setzero_si256()), _mm256_set1_epi8(-0x01));
    // 6. return _mm256_andnot_si256(_mm256_cmpeq_epi16(left, right), _mm256_cmpeq_epi16(_mm256_max_epu16(left, right), right));
    // 7. return _mm256_andnot_si256(_mm256_cmpeq_epi16(left, right), _mm256_cmple_epu16(left, right));
    // 8. return _mm256_cmpgt_epi16(_mm256_xor_si256(right, _mm256_set1_epi16(-0x8000)), _mm256_xor_si256(left, _mm256_set1_epi16(-0x8000)));
    return _mm256_cmplt_epi16(_mm256_xor_si256(left, _mm256_set1_epi16(-0x8000)), _mm256_xor_si256(right, _mm256_set1_epi16(-0x8000)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmple_epu16(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmpge_epu16(right, left);
    // 2. return _mm256_cmpeq_epi16(_mm256_min_epu16(left, right), left);
    // 3. return _mm256_cmpeq_epi16(_mm256_max_epu16(left, right), right);
    // 4. return _mm256_andnot_si256(_mm256_cmpgt_epu16(left, right), _mm256_set1_epi8(-0x01));
    // 5. return _mm256_andnot_si256(_mm256_cmplt_epu16(right, left), _mm256_set1_epi8(-0x01));
    // 6. return _mm256_or_si256(_mm256_cmpgt_epu16(right, left), _mm256_cmpeq_epi16(left, right));
    // 7. return _mm256_or_si256(_mm256_cmplt_epu16(left, right), _mm256_cmpeq_epi16(left, right));
    return _mm256_cmpeq_epi16(_mm256_subs_epu16(left, right), _mm256_setzero_si256());
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmpgt_epu32(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmplt_epu32(right, left);
    // 2. return _mm256_xor_si256(_mm256_cmple_epu32(left, right), _mm256_set1_epi8(-0x01));
    // 3. return _mm256_xor_si256(_mm256_cmpeq_epi32(_mm256_min_epu32(left, right), left), _mm256_set1_epi8(-0x01));
    // 4. return _mm256_xor_si256(_mm256_cmpeq_epi32(_mm256_max_epu32(left, right), right), _mm256_set1_epi8(-0x01));
    // 5. return _mm256_andnot_si256(_mm256_cmpeq_epi32(left, right), _mm256_cmpeq_epi32(_mm256_max_epu32(left, right), left));
    // 6. return _mm256_andnot_si256(_mm256_cmpeq_epi32(left, right), _mm256_cmple_epu32(right, left));
    // 7. return _mm256_cmplt_epi32(_mm256_xor_si256(right, _mm256_set1_epi32(-0x80000000)), _mm256_xor_si256(left, _mm256_set1_epi32(-0x80000000)));
    return _mm256_cmpgt_epi32(_mm256_xor_si256(left, _mm256_set1_epi32(-0x80000000)), _mm256_xor_si256(right, _mm256_set1_epi32(-0x80000000)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmpge_epu32(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmple_epu32(right, left);
    // 2. return _mm256_cmpeq_epi32(_mm256_min_epu32(left, right), right);
    // 3. return _mm256_cmpeq_epi32(_mm256_max_epu32(left, right), left);
    // 4. return _mm256_andnot_si256(_mm256_cmpgt_epu32(right, left), _mm256_set1_epi8(-0x01));
    // 5. return _mm256_andnot_si256(_mm256_cmplt_epu32(left, right), _mm256_set1_epi8(-0x01));
    // 6. return _mm256_or_si256(_mm256_cmplt_epu32(right, left), _mm256_cmpeq_epi32(left, right));
    return _mm256_or_si256(_mm256_cmpgt_epu32(left, right), _mm256_cmpeq_epi32(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmplt_epu32(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmpgt_epu32(right, left);
    // 2. return _mm256_xor_si256(_mm256_cmpge_epu32(left, right), _mm256_set1_epi8(-0x01));
    // 3. return _mm256_xor_si256(_mm256_cmpeq_epi32(_mm256_min_epu32(left, right), right), _mm256_set1_epi8(-0x01));
    // 4. return _mm256_xor_si256(_mm256_cmpeq_epi32(_mm256_max_epu32(left, right), left), _mm256_set1_epi8(-0x01));
    // 5. return _mm256_andnot_si256(_mm256_cmpeq_epi32(left, right), _mm256_cmpeq_epi32(_mm256_max_epu32(left, right), right));
    // 6. return _mm256_andnot_si256(_mm256_cmpeq_epi32(left, right), _mm256_cmple_epu32(left, right));
    // 7. return _mm256_cmpgt_epi32(_mm256_xor_si256(right, _mm256_set1_epi32(-0x80000000)), _mm256_xor_si256(left, _mm256_set1_epi32(-0x80000000)));
    return _mm256_cmplt_epi32(_mm256_xor_si256(left, _mm256_set1_epi32(-0x80000000)), _mm256_xor_si256(right, _mm256_set1_epi32(-0x80000000)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmple_epu32(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmpge_epu32(right, left);
    // 2. return _mm256_cmpeq_epi32(_mm256_min_epu32(left, right), left);
    // 3. return _mm256_cmpeq_epi32(_mm256_max_epu32(left, right), right);
    // 4. return _mm256_andnot_si256(_mm256_cmpgt_epu32(left, right), _mm256_set1_epi8(-0x01));
    // 5. return _mm256_andnot_si256(_mm256_cmplt_epu32(right, left), _mm256_set1_epi8(-0x01));
    // 6. return _mm256_or_si256(_mm256_cmpgt_epu32(right, left), _mm256_cmpeq_epi32(left, right));
    return _mm256_or_si256(_mm256_cmplt_epu32(left, right), _mm256_cmpeq_epi32(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmpgt_epu64(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmplt_epu64(right, left);
    // 2. return _mm256_xor_si256(_mm256_cmple_epu64(left, right), _mm256_set1_epi8(-0x01));
    // 3. return _mm256_andnot_si256(_mm256_cmpeq_epi64(left, right), _mm256_cmple_epu64(right, left));
    // 4. return _mm256_cmplt_epi64(_mm256_xor_si256(right, _mm256_set1_epi64x(-0x8000000000000000)), _mm256_xor_si256(left, _mm256_set1_epi64x(-0x8000000000000000)));
    return _mm256_cmpgt_epi64(_mm256_xor_si256(left, _mm256_set1_epi64x(-0x8000000000000000)), _mm256_xor_si256(right, _mm256_set1_epi64x(-0x8000000000000000)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmpge_epu64(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmple_epu64(right, left);
    // 2. return _mm256_andnot_si256(_mm256_cmpgt_epu64(right, left), _mm256_set1_epi8(-0x01));
    // 3. return _mm256_andnot_si256(_mm256_cmplt_epu64(left, right), _mm256_set1_epi8(-0x01));
    // 4. return _mm256_or_si256(_mm256_cmplt_epu64(right, left), _mm256_cmpeq_epi64(left, right));
    return _mm256_or_si256(_mm256_cmpgt_epu64(left, right), _mm256_cmpeq_epi64(left, right));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmplt_epu64(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmpgt_epu64(right, left);
    // 2. return _mm256_xor_si256(_mm256_cmpge_epu64(left, right), _mm256_set1_epi8(-0x01));
    // 3. return _mm256_andnot_si256(_mm256_cmpeq_epi64(left, right), _mm256_cmple_epu64(left, right));
    // 4. return _mm256_cmpgt_epi64(_mm256_xor_si256(right, _mm256_set1_epi64x(-0x8000000000000000)), _mm256_xor_si256(left, _mm256_set1_epi64x(-0x8000000000000000)));
    return _mm256_cmplt_epi64(_mm256_xor_si256(left, _mm256_set1_epi64x(-0x8000000000000000)), _mm256_xor_si256(right, _mm256_set1_epi64x(-0x8000000000000000)));
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx", target_feature = "avx2"))]
#[inline]
pub unsafe fn _mm256_cmple_epu64(left: __m256i, right: __m256i) -> __m256i {
    // 1. return _mm256_cmpge_epu64(right, left);
    // 2. return _mm256_andnot_si256(_mm256_cmpgt_epu64(left, right), _mm256_set1_epi8(-0x01));
    // 3. return _mm256_andnot_si256(_mm256_cmplt_epu64(right, left), _mm256_set1_epi8(-0x01));
    // 4. return _mm256_or_si256(_mm256_cmpgt_epu64(right, left), _mm256_cmpeq_epi64(left, right));
    return _mm256_or_si256(_mm256_cmplt_epu64(left, right), _mm256_cmpeq_epi64(left, right));
}