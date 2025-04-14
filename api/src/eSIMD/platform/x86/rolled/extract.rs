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

#[cfg(target_feature = "sse2")]
use core::{
    arch::{
        x86::{
            __m128i,
            _mm_or_si128,
            _mm_slli_si128, _mm_srli_si128,
            _mm_setzero_si128
        }
    }
};

#[cfg(target_feature = "avx2")]
use core::{
    arch::{
        x86::{
            __m256i,
            _mm256_or_si256,
            _mm256_setzero_si256
        }
    }
};

#[cfg(all(target_feature = "avx512f", target_feature = "avx512bw"))]
use core::{
    arch::{
        x86::{
            __m512i,
            _mm512_or_si512,
            _mm512_setzero_si512
        }
    }
};

#[cfg(target_feature = "avx2")]
use crate::{
    eSIMD::{
        platform::{
            x86::{
                unrolled::{
                    shift::{
                        _mm256_slli_si256, _mm256_srli_si256
                    }
                }
            }
        }
    }
};

#[cfg(all(target_feature = "avx512f", target_feature = "avx512bw"))]
use crate::{
    eSIMD::{
        platform::{
            x86::{
                unrolled::{
                    shift::{
                        _mm512_slli_si512, _mm512_srli_si512
                    }
                }
            }
        }
    }
};

/// # Example (_mm_alvext_epi8):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Addition: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
///
/// - Vext'ed left by 1 byte: ```[20, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f]```
/// - Vext'ed left by 2 bytes: ```[1f, 20, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e]```
/// - Vext'ed left by 6 bytes: ```[1b, 1c, 1d, 1e, 1f, 20, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a]```
/// - Vext'ed left by 10 bytes: ```[17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 01, 02, 03, 04, 05, 06]```
/// - Vext'ed left by 16 bytes: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
/// - Vext'ed left by 20 bytes: ```[00, 00, 00, 00, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c]```
/// - Vext'ed left by 21 bytes: ```[00, 00, 00, 00, 00, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b]```
/// - Vext'ed left by 22 bytes: ```[00, 00, 00, 00, 00, 00, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a]```
/// - Vext'ed left by 31 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 11]```
#[cfg(target_feature = "sse2")]
pub unsafe fn _mm_alvext_epi8(vector: __m128i, addition: __m128i, shift: usize) -> __m128i {
    return match shift {
        0x00 => vector,
        0x01 => _mm_or_si128(_mm_slli_si128::<0x01>(vector), _mm_srli_si128::<0x0F>(addition)),
        0x02 => _mm_or_si128(_mm_slli_si128::<0x02>(vector), _mm_srli_si128::<0x0E>(addition)),
        0x03 => _mm_or_si128(_mm_slli_si128::<0x03>(vector), _mm_srli_si128::<0x0D>(addition)),
        0x04 => _mm_or_si128(_mm_slli_si128::<0x04>(vector), _mm_srli_si128::<0x0C>(addition)),
        0x05 => _mm_or_si128(_mm_slli_si128::<0x05>(vector), _mm_srli_si128::<0x0B>(addition)),
        0x06 => _mm_or_si128(_mm_slli_si128::<0x06>(vector), _mm_srli_si128::<0x0A>(addition)),
        0x07 => _mm_or_si128(_mm_slli_si128::<0x07>(vector), _mm_srli_si128::<0x09>(addition)),
        0x08 => _mm_or_si128(_mm_slli_si128::<0x08>(vector), _mm_srli_si128::<0x08>(addition)),
        0x09 => _mm_or_si128(_mm_slli_si128::<0x09>(vector), _mm_srli_si128::<0x07>(addition)),
        0x0A => _mm_or_si128(_mm_slli_si128::<0x0A>(vector), _mm_srli_si128::<0x06>(addition)),
        0x0B => _mm_or_si128(_mm_slli_si128::<0x0B>(vector), _mm_srli_si128::<0x05>(addition)),
        0x0C => _mm_or_si128(_mm_slli_si128::<0x0C>(vector), _mm_srli_si128::<0x04>(addition)),
        0x0D => _mm_or_si128(_mm_slli_si128::<0x0D>(vector), _mm_srli_si128::<0x03>(addition)),
        0x0E => _mm_or_si128(_mm_slli_si128::<0x0E>(vector), _mm_srli_si128::<0x02>(addition)),
        0x0F => _mm_or_si128(_mm_slli_si128::<0x0F>(vector), _mm_srli_si128::<0x01>(addition)),
        0x10 => addition,
        0x11 => _mm_slli_si128::<0x01>(addition),
        0x12 => _mm_slli_si128::<0x02>(addition),
        0x13 => _mm_slli_si128::<0x03>(addition),
        0x14 => _mm_slli_si128::<0x04>(addition),
        0x15 => _mm_slli_si128::<0x05>(addition),
        0x16 => _mm_slli_si128::<0x06>(addition),
        0x17 => _mm_slli_si128::<0x07>(addition),
        0x18 => _mm_slli_si128::<0x08>(addition),
        0x19 => _mm_slli_si128::<0x09>(addition),
        0x1A => _mm_slli_si128::<0x0A>(addition),
        0x1B => _mm_slli_si128::<0x0B>(addition),
        0x1C => _mm_slli_si128::<0x0C>(addition),
        0x1D => _mm_slli_si128::<0x0D>(addition),
        0x1E => _mm_slli_si128::<0x0E>(addition),
        0x1F => _mm_slli_si128::<0x0F>(addition),
        _ => _mm_setzero_si128()
    };
}

/// # Example (_mm_arvext_epi8):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Addition: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
///
/// - Vext'ed right by 1 byte: ```[02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11]```
/// - Vext'ed right by 2 bytes: ```[03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12]```
/// - Vext'ed right by 6 bytes: ```[07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16]```
/// - Vext'ed right by 10 bytes: ```[0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a]```
/// - Vext'ed right by 16 bytes: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
/// - Vext'ed right by 20 bytes: ```[15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 00, 00, 00, 00]```
/// - Vext'ed right by 21 bytes: ```[16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 22 bytes: ```[17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 31 bytes: ```[20, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(target_feature = "sse2")]
pub unsafe fn _mm_arvext_epi8(vector: __m128i, addition: __m128i, shift: usize) -> __m128i {
    return match shift {
        0x00 => vector,
        0x01 => _mm_or_si128(_mm_srli_si128::<0x01>(vector), _mm_slli_si128::<0x0F>(addition)),
        0x02 => _mm_or_si128(_mm_srli_si128::<0x02>(vector), _mm_slli_si128::<0x0E>(addition)),
        0x03 => _mm_or_si128(_mm_srli_si128::<0x03>(vector), _mm_slli_si128::<0x0D>(addition)),
        0x04 => _mm_or_si128(_mm_srli_si128::<0x04>(vector), _mm_slli_si128::<0x0C>(addition)),
        0x05 => _mm_or_si128(_mm_srli_si128::<0x05>(vector), _mm_slli_si128::<0x0B>(addition)),
        0x06 => _mm_or_si128(_mm_srli_si128::<0x06>(vector), _mm_slli_si128::<0x0A>(addition)),
        0x07 => _mm_or_si128(_mm_srli_si128::<0x07>(vector), _mm_slli_si128::<0x09>(addition)),
        0x08 => _mm_or_si128(_mm_srli_si128::<0x08>(vector), _mm_slli_si128::<0x08>(addition)),
        0x09 => _mm_or_si128(_mm_srli_si128::<0x09>(vector), _mm_slli_si128::<0x07>(addition)),
        0x0A => _mm_or_si128(_mm_srli_si128::<0x0A>(vector), _mm_slli_si128::<0x06>(addition)),
        0x0B => _mm_or_si128(_mm_srli_si128::<0x0B>(vector), _mm_slli_si128::<0x05>(addition)),
        0x0C => _mm_or_si128(_mm_srli_si128::<0x0C>(vector), _mm_slli_si128::<0x04>(addition)),
        0x0D => _mm_or_si128(_mm_srli_si128::<0x0D>(vector), _mm_slli_si128::<0x03>(addition)),
        0x0E => _mm_or_si128(_mm_srli_si128::<0x0E>(vector), _mm_slli_si128::<0x02>(addition)),
        0x0F => _mm_or_si128(_mm_srli_si128::<0x0F>(vector), _mm_slli_si128::<0x01>(addition)),
        0x10 => addition,
        0x11 => _mm_srli_si128::<0x01>(addition),
        0x12 => _mm_srli_si128::<0x02>(addition),
        0x13 => _mm_srli_si128::<0x03>(addition),
        0x14 => _mm_srli_si128::<0x04>(addition),
        0x15 => _mm_srli_si128::<0x05>(addition),
        0x16 => _mm_srli_si128::<0x06>(addition),
        0x17 => _mm_srli_si128::<0x07>(addition),
        0x18 => _mm_srli_si128::<0x08>(addition),
        0x19 => _mm_srli_si128::<0x09>(addition),
        0x1A => _mm_srli_si128::<0x0A>(addition),
        0x1B => _mm_srli_si128::<0x0B>(addition),
        0x1C => _mm_srli_si128::<0x0C>(addition),
        0x1D => _mm_srli_si128::<0x0D>(addition),
        0x1E => _mm_srli_si128::<0x0E>(addition),
        0x1F => _mm_srli_si128::<0x0F>(addition),
        _ => _mm_setzero_si128()
    };
}

/// # Example (_mm_alvext_epi16):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08]```
/// - Addition: ```[09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
///
/// - Vext'ed left by 1 pair: ```[10, 01, 02, 03, 04, 05, 06, 07]```
/// - Vext'ed left by 2 pairs: ```[0f, 10, 01, 02, 03, 04, 05, 06]```
/// - Vext'ed left by 3 pairs: ```[0e, 0f, 10, 01, 02, 03, 04, 05]```
/// - Vext'ed left by 4 pairs: ```[0d, 0e, 0f, 10, 01, 02, 03, 04]```
/// - Vext'ed left by 5 pairs: ```[0c, 0d, 0e, 0f, 10, 01, 02, 03]```
/// - Vext'ed left by 6 pairs: ```[0b, 0c, 0d, 0e, 0f, 10, 01, 02]```
/// - Vext'ed left by 7 pairs: ```[0a, 0b, 0c, 0d, 0e, 0f, 10, 01]```
/// - Vext'ed left by 8 pairs: ```[09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Vext'ed left by 9 pairs: ```[00, 09, 0a, 0b, 0c, 0d, 0e, 0f]```
/// - Vext'ed left by 10 pairs: ```[00, 00, 09, 0a, 0b, 0c, 0d, 0e]```
/// - Vext'ed left by 11 pairs: ```[00, 00, 00, 09, 0a, 0b, 0c, 0d]```
/// - Vext'ed left by 12 pairs: ```[00, 00, 00, 00, 09, 0a, 0b, 0c]```
/// - Vext'ed left by 13 pairs: ```[00, 00, 00, 00, 00, 09, 0a, 0b]```
/// - Vext'ed left by 14 pairs: ```[00, 00, 00, 00, 00, 00, 09, 0a]```
/// - Vext'ed left by 15 pairs: ```[00, 00, 00, 00, 00, 00, 00, 09]```
#[cfg(target_feature = "sse2")]
pub unsafe fn _mm_alvext_epi16(vector: __m128i, addition: __m128i, shift: usize) -> __m128i {
    return match shift {
        0x00 => vector,
        0x01 => _mm_or_si128(_mm_slli_si128::<0x02>(vector), _mm_srli_si128::<0x0E>(addition)),
        0x02 => _mm_or_si128(_mm_slli_si128::<0x04>(vector), _mm_srli_si128::<0x0C>(addition)),
        0x03 => _mm_or_si128(_mm_slli_si128::<0x06>(vector), _mm_srli_si128::<0x0A>(addition)),
        0x04 => _mm_or_si128(_mm_slli_si128::<0x08>(vector), _mm_srli_si128::<0x08>(addition)),
        0x05 => _mm_or_si128(_mm_slli_si128::<0x0A>(vector), _mm_srli_si128::<0x06>(addition)),
        0x06 => _mm_or_si128(_mm_slli_si128::<0x0C>(vector), _mm_srli_si128::<0x04>(addition)),
        0x07 => _mm_or_si128(_mm_slli_si128::<0x0E>(vector), _mm_srli_si128::<0x02>(addition)),
        0x08 => addition,
        0x09 => _mm_slli_si128::<0x02>(addition),
        0x0A => _mm_slli_si128::<0x04>(addition),
        0x0B => _mm_slli_si128::<0x06>(addition),
        0x0C => _mm_slli_si128::<0x08>(addition),
        0x0D => _mm_slli_si128::<0x0A>(addition),
        0x0E => _mm_slli_si128::<0x0C>(addition),
        0x0F => _mm_slli_si128::<0x0E>(addition),
        _ => _mm_setzero_si128()
    };
}

/// # Example (_mm_arvext_epi16):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08]```
/// - Addition: ```[09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
///
/// - Vext'ed right by 1 pair: ```[02, 03, 04, 05, 06, 07, 08, 09]```
/// - Vext'ed right by 2 pairs: ```[03, 04, 05, 06, 07, 08, 09, 0a]```
/// - Vext'ed right by 3 pairs: ```[04, 05, 06, 07, 08, 09, 0a, 0b]```
/// - Vext'ed right by 4 pairs: ```[05, 06, 07, 08, 09, 0a, 0b, 0c]```
/// - Vext'ed right by 5 pairs: ```[06, 07, 08, 09, 0a, 0b, 0c, 0d]```
/// - Vext'ed right by 6 pairs: ```[07, 08, 09, 0a, 0b, 0c, 0d, 0e]```
/// - Vext'ed right by 7 pairs: ```[08, 09, 0a, 0b, 0c, 0d, 0e, 0f]```
/// - Vext'ed right by 8 pairs: ```[09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Vext'ed right by 9 pairs: ```[0a, 0b, 0c, 0d, 0e, 0f, 10, 00]```
/// - Vext'ed right by 10 pairs: ```[0b, 0c, 0d, 0e, 0f, 10, 00, 00]```
/// - Vext'ed right by 11 pairs: ```[0c, 0d, 0e, 0f, 10, 00, 00, 00]```
/// - Vext'ed right by 12 pairs: ```[0d, 0e, 0f, 10, 00, 00, 00, 00]```
/// - Vext'ed right by 13 pairs: ```[0e, 0f, 10, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 14 pairs: ```[0f, 10, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 15 pairs: ```[10, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(target_feature = "sse2")]
pub unsafe fn _mm_arvext_epi16(vector: __m128i, addition: __m128i, shift: usize) -> __m128i {
    return match shift {
        0x00 => vector,
        0x01 => _mm_or_si128(_mm_srli_si128::<0x02>(vector), _mm_slli_si128::<0x0E>(addition)),
        0x02 => _mm_or_si128(_mm_srli_si128::<0x04>(vector), _mm_slli_si128::<0x0C>(addition)),
        0x03 => _mm_or_si128(_mm_srli_si128::<0x06>(vector), _mm_slli_si128::<0x0A>(addition)),
        0x04 => _mm_or_si128(_mm_srli_si128::<0x08>(vector), _mm_slli_si128::<0x08>(addition)),
        0x05 => _mm_or_si128(_mm_srli_si128::<0x0A>(vector), _mm_slli_si128::<0x06>(addition)),
        0x06 => _mm_or_si128(_mm_srli_si128::<0x0C>(vector), _mm_slli_si128::<0x04>(addition)),
        0x07 => _mm_or_si128(_mm_srli_si128::<0x0E>(vector), _mm_slli_si128::<0x02>(addition)),
        0x08 => addition,
        0x09 => _mm_srli_si128::<0x02>(addition),
        0x0A => _mm_srli_si128::<0x04>(addition),
        0x0B => _mm_srli_si128::<0x06>(addition),
        0x0C => _mm_srli_si128::<0x08>(addition),
        0x0D => _mm_srli_si128::<0x0A>(addition),
        0x0E => _mm_srli_si128::<0x0C>(addition),
        0x0F => _mm_srli_si128::<0x0E>(addition),
        _ => _mm_setzero_si128()
    };
}

/// # Example (_mm_alvext_epi32):
///
/// - Source: ```[01, 02, 03, 04]```
/// - Addition: ```[05, 06, 07, 08]```
///
/// - Vext'ed left by 1 pair: ```[08, 01, 02, 03]```
/// - Vext'ed left by 2 pairs: ```[07, 08, 01, 02]```
/// - Vext'ed left by 3 pairs: ```[06, 07, 08, 01]```
/// - Vext'ed left by 4 pairs: ```[05, 06, 07, 08]```
/// - Vext'ed left by 5 pairs: ```[00, 05, 06, 07]```
/// - Vext'ed left by 6 pairs: ```[00, 00, 05, 06]```
/// - Vext'ed left by 7 pairs: ```[00, 00, 00, 05]```
#[cfg(target_feature = "sse2")]
pub unsafe fn _mm_alvext_epi32(vector: __m128i, addition: __m128i, shift: usize) -> __m128i {
    return match shift {
        0x00 => vector,
        0x01 => _mm_or_si128(_mm_slli_si128::<0x04>(vector), _mm_srli_si128::<0x0C>(addition)),
        0x02 => _mm_or_si128(_mm_slli_si128::<0x08>(vector), _mm_srli_si128::<0x08>(addition)),
        0x03 => _mm_or_si128(_mm_slli_si128::<0x0C>(vector), _mm_srli_si128::<0x04>(addition)),
        0x04 => addition,
        0x05 => _mm_slli_si128::<0x04>(addition),
        0x06 => _mm_slli_si128::<0x08>(addition),
        0x07 => _mm_slli_si128::<0x0C>(addition),
        _ => _mm_setzero_si128()
    };
}

/// # Example (_mm_arvext_epi32):
///
/// - Source: ```[01, 02, 03, 04]```
/// - Addition: ```[05, 06, 07, 08]```
///
/// - Vext'ed right by 1 pair: ```[02, 03, 04, 05]```
/// - Vext'ed right by 2 pairs: ```[03, 04, 05, 06]```
/// - Vext'ed right by 3 pairs: ```[04, 05, 06, 07]```
/// - Vext'ed right by 4 pairs: ```[05, 06, 07, 08]```
/// - Vext'ed right by 5 pairs: ```[06, 07, 08, 00]```
/// - Vext'ed right by 6 pairs: ```[07, 08, 00, 00]```
/// - Vext'ed right by 7 pairs: ```[08, 00, 00, 00]```
#[cfg(target_feature = "sse2")]
pub unsafe fn _mm_arvext_epi32(vector: __m128i, addition: __m128i, shift: usize) -> __m128i {
    return match shift {
        0x00 => vector,
        0x01 => _mm_or_si128(_mm_srli_si128::<0x04>(vector), _mm_slli_si128::<0x0C>(addition)),
        0x02 => _mm_or_si128(_mm_srli_si128::<0x08>(vector), _mm_slli_si128::<0x08>(addition)),
        0x03 => _mm_or_si128(_mm_srli_si128::<0x0C>(vector), _mm_slli_si128::<0x04>(addition)),
        0x04 => addition,
        0x05 => _mm_srli_si128::<0x04>(addition),
        0x06 => _mm_srli_si128::<0x08>(addition),
        0x07 => _mm_srli_si128::<0x0C>(addition),
        _ => _mm_setzero_si128()
    };
}

/// # Example (_mm_alvext_epi64):
///
/// - Source: ```[01, 02]```
/// - Addition: ```[03, 04]```
///
/// - Vext'ed left by 1 pair: ```[04, 01]```
/// - Vext'ed left by 2 pairs: ```[03, 04]```
/// - Vext'ed left by 3 pairs: ```[00, 03]```
#[cfg(target_feature = "sse2")]
pub unsafe fn _mm_alvext_epi64(vector: __m128i, addition: __m128i, shift: usize) -> __m128i {
    return match shift {
        0x00 => vector,
        0x01 => _mm_or_si128(_mm_slli_si128::<0x08>(vector), _mm_srli_si128::<0x08>(addition)),
        0x02 => addition,
        0x03 => _mm_slli_si128::<0x08>(addition),
        _ => _mm_setzero_si128()
    };
}

/// # Example (_mm_arvext_epi64):
///
/// - Source: ```[01, 02]```
/// - Addition: ```[03, 04]```
///
/// - Vext'ed right by 1 pair: ```[02, 03]```
/// - Vext'ed right by 2 pairs: ```[03, 04]```
/// - Vext'ed right by 3 pairs: ```[04, 00]```
#[cfg(target_feature = "sse2")]
pub unsafe fn _mm_arvext_epi64(vector: __m128i, addition: __m128i, shift: usize) -> __m128i {
    return match shift {
        0x00 => vector,
        0x01 => _mm_or_si128(_mm_srli_si128::<0x08>(vector), _mm_slli_si128::<0x08>(addition)),
        0x02 => addition,
        0x03 => _mm_srli_si128::<0x08>(addition),
        _ => _mm_setzero_si128()
    };
}

/// # Example (_mm256_alvext_epi8):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
/// - Addition: ```[21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40]```
///
/// - Vext'ed left by 1 byte: ```[40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f]```
/// - Vext'ed left by 2 bytes: ```[3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e]```
/// - Vext'ed left by 6 bytes: ```[3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a]```
/// - Vext'ed left by 10 bytes: ```[37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16]```
/// - Vext'ed left by 16 bytes: ```[31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Vext'ed left by 20 bytes: ```[2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c]```
/// - Vext'ed left by 21 bytes: ```[2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b]```
/// - Vext'ed left by 22 bytes: ```[2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a]```
/// - Vext'ed left by 31 bytes: ```[22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01]```
/// - Vext'ed left by 32 bytes: ```[21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40]```
/// - Vext'ed left by 36 bytes: ```[00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c]```
/// - Vext'ed left by 40 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38]```
/// - Vext'ed left by 46 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32]```
/// - Vext'ed left by 50 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e]```
/// - Vext'ed left by 51 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d]```
/// - Vext'ed left by 52 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c]```
/// - Vext'ed left by 56 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28]```
/// - Vext'ed left by 60 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24]```
/// - Vext'ed left by 61 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23]```
/// - Vext'ed left by 62 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22]```
/// - Vext'ed left by 63 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21]```
#[cfg(target_feature = "avx2")]
pub unsafe fn _mm256_alvext_epi8(vector: __m256i, addition: __m256i, shift: usize) -> __m256i {
    return match shift {
        0x00 => vector,
        0x01 => _mm256_or_si256(_mm256_slli_si256::<0x01>(vector), _mm256_srli_si256::<0x1F>(addition)),
        0x02 => _mm256_or_si256(_mm256_slli_si256::<0x02>(vector), _mm256_srli_si256::<0x1E>(addition)),
        0x03 => _mm256_or_si256(_mm256_slli_si256::<0x03>(vector), _mm256_srli_si256::<0x1D>(addition)),
        0x04 => _mm256_or_si256(_mm256_slli_si256::<0x04>(vector), _mm256_srli_si256::<0x1C>(addition)),
        0x05 => _mm256_or_si256(_mm256_slli_si256::<0x05>(vector), _mm256_srli_si256::<0x1B>(addition)),
        0x06 => _mm256_or_si256(_mm256_slli_si256::<0x06>(vector), _mm256_srli_si256::<0x1A>(addition)),
        0x07 => _mm256_or_si256(_mm256_slli_si256::<0x07>(vector), _mm256_srli_si256::<0x19>(addition)),
        0x08 => _mm256_or_si256(_mm256_slli_si256::<0x08>(vector), _mm256_srli_si256::<0x18>(addition)),
        0x09 => _mm256_or_si256(_mm256_slli_si256::<0x09>(vector), _mm256_srli_si256::<0x17>(addition)),
        0x0A => _mm256_or_si256(_mm256_slli_si256::<0x0A>(vector), _mm256_srli_si256::<0x16>(addition)),
        0x0B => _mm256_or_si256(_mm256_slli_si256::<0x0B>(vector), _mm256_srli_si256::<0x15>(addition)),
        0x0C => _mm256_or_si256(_mm256_slli_si256::<0x0C>(vector), _mm256_srli_si256::<0x14>(addition)),
        0x0D => _mm256_or_si256(_mm256_slli_si256::<0x0D>(vector), _mm256_srli_si256::<0x13>(addition)),
        0x0E => _mm256_or_si256(_mm256_slli_si256::<0x0E>(vector), _mm256_srli_si256::<0x12>(addition)),
        0x0F => _mm256_or_si256(_mm256_slli_si256::<0x0F>(vector), _mm256_srli_si256::<0x11>(addition)),
        0x10 => _mm256_or_si256(_mm256_slli_si256::<0x10>(vector), _mm256_srli_si256::<0x10>(addition)),
        0x11 => _mm256_or_si256(_mm256_slli_si256::<0x11>(vector), _mm256_srli_si256::<0x0F>(addition)),
        0x12 => _mm256_or_si256(_mm256_slli_si256::<0x12>(vector), _mm256_srli_si256::<0x0E>(addition)),
        0x13 => _mm256_or_si256(_mm256_slli_si256::<0x13>(vector), _mm256_srli_si256::<0x0D>(addition)),
        0x14 => _mm256_or_si256(_mm256_slli_si256::<0x14>(vector), _mm256_srli_si256::<0x0C>(addition)),
        0x15 => _mm256_or_si256(_mm256_slli_si256::<0x15>(vector), _mm256_srli_si256::<0x0B>(addition)),
        0x16 => _mm256_or_si256(_mm256_slli_si256::<0x16>(vector), _mm256_srli_si256::<0x0A>(addition)),
        0x17 => _mm256_or_si256(_mm256_slli_si256::<0x17>(vector), _mm256_srli_si256::<0x09>(addition)),
        0x18 => _mm256_or_si256(_mm256_slli_si256::<0x18>(vector), _mm256_srli_si256::<0x08>(addition)),
        0x19 => _mm256_or_si256(_mm256_slli_si256::<0x19>(vector), _mm256_srli_si256::<0x07>(addition)),
        0x1A => _mm256_or_si256(_mm256_slli_si256::<0x1A>(vector), _mm256_srli_si256::<0x06>(addition)),
        0x1B => _mm256_or_si256(_mm256_slli_si256::<0x1B>(vector), _mm256_srli_si256::<0x05>(addition)),
        0x1C => _mm256_or_si256(_mm256_slli_si256::<0x1C>(vector), _mm256_srli_si256::<0x04>(addition)),
        0x1D => _mm256_or_si256(_mm256_slli_si256::<0x1D>(vector), _mm256_srli_si256::<0x03>(addition)),
        0x1E => _mm256_or_si256(_mm256_slli_si256::<0x1E>(vector), _mm256_srli_si256::<0x02>(addition)),
        0x1F => _mm256_or_si256(_mm256_slli_si256::<0x1F>(vector), _mm256_srli_si256::<0x01>(addition)),
        0x20 => addition,
        0x21 => _mm256_slli_si256::<0x01>(addition),
        0x22 => _mm256_slli_si256::<0x02>(addition),
        0x23 => _mm256_slli_si256::<0x03>(addition),
        0x24 => _mm256_slli_si256::<0x04>(addition),
        0x25 => _mm256_slli_si256::<0x05>(addition),
        0x26 => _mm256_slli_si256::<0x06>(addition),
        0x27 => _mm256_slli_si256::<0x07>(addition),
        0x28 => _mm256_slli_si256::<0x08>(addition),
        0x29 => _mm256_slli_si256::<0x09>(addition),
        0x2A => _mm256_slli_si256::<0x0A>(addition),
        0x2B => _mm256_slli_si256::<0x0B>(addition),
        0x2C => _mm256_slli_si256::<0x0C>(addition),
        0x2D => _mm256_slli_si256::<0x0D>(addition),
        0x2E => _mm256_slli_si256::<0x0E>(addition),
        0x2F => _mm256_slli_si256::<0x0F>(addition),
        0x30 => _mm256_slli_si256::<0x10>(addition),
        0x31 => _mm256_slli_si256::<0x11>(addition),
        0x32 => _mm256_slli_si256::<0x12>(addition),
        0x33 => _mm256_slli_si256::<0x13>(addition),
        0x34 => _mm256_slli_si256::<0x14>(addition),
        0x35 => _mm256_slli_si256::<0x15>(addition),
        0x36 => _mm256_slli_si256::<0x16>(addition),
        0x37 => _mm256_slli_si256::<0x17>(addition),
        0x38 => _mm256_slli_si256::<0x18>(addition),
        0x39 => _mm256_slli_si256::<0x19>(addition),
        0x3A => _mm256_slli_si256::<0x1A>(addition),
        0x3B => _mm256_slli_si256::<0x1B>(addition),
        0x3C => _mm256_slli_si256::<0x1C>(addition),
        0x3D => _mm256_slli_si256::<0x1D>(addition),
        0x3E => _mm256_slli_si256::<0x1E>(addition),
        0x3F => _mm256_slli_si256::<0x1F>(addition),
        _ => _mm256_setzero_si256()
    };
}

/// # Example (_mm256_arvext_epi8):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
/// - Addition: ```[21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40]```
///
/// - Vext'ed right by 1 byte: ```[02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21]```
/// - Vext'ed right by 2 bytes: ```[03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22]```
/// - Vext'ed right by 6 bytes: ```[07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26]```
/// - Vext'ed right by 10 bytes: ```[0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a]```
/// - Vext'ed right by 16 bytes: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30]```
/// - Vext'ed right by 20 bytes: ```[15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34]```
/// - Vext'ed right by 21 bytes: ```[16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35]```
/// - Vext'ed right by 22 bytes: ```[17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36]```
/// - Vext'ed right by 31 bytes: ```[20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f]```
/// - Vext'ed right by 32 bytes: ```[21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40]```
/// - Vext'ed right by 36 bytes: ```[25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00]```
/// - Vext'ed right by 40 bytes: ```[29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 46 bytes: ```[2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 50 bytes: ```[33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 51 bytes: ```[34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 52 bytes: ```[35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 56 bytes: ```[39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 60 bytes: ```[3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 61 bytes: ```[3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 62 bytes: ```[3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 63 bytes: ```[40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(target_feature = "avx2")]
pub unsafe fn _mm256_arvext_epi8(vector: __m256i, addition: __m256i, shift: usize) -> __m256i {
    return match shift {
        0x00 => vector,
        0x01 => _mm256_or_si256(_mm256_srli_si256::<0x01>(vector), _mm256_slli_si256::<0x1F>(addition)),
        0x02 => _mm256_or_si256(_mm256_srli_si256::<0x02>(vector), _mm256_slli_si256::<0x1E>(addition)),
        0x03 => _mm256_or_si256(_mm256_srli_si256::<0x03>(vector), _mm256_slli_si256::<0x1D>(addition)),
        0x04 => _mm256_or_si256(_mm256_srli_si256::<0x04>(vector), _mm256_slli_si256::<0x1C>(addition)),
        0x05 => _mm256_or_si256(_mm256_srli_si256::<0x05>(vector), _mm256_slli_si256::<0x1B>(addition)),
        0x06 => _mm256_or_si256(_mm256_srli_si256::<0x06>(vector), _mm256_slli_si256::<0x1A>(addition)),
        0x07 => _mm256_or_si256(_mm256_srli_si256::<0x07>(vector), _mm256_slli_si256::<0x19>(addition)),
        0x08 => _mm256_or_si256(_mm256_srli_si256::<0x08>(vector), _mm256_slli_si256::<0x18>(addition)),
        0x09 => _mm256_or_si256(_mm256_srli_si256::<0x09>(vector), _mm256_slli_si256::<0x17>(addition)),
        0x0A => _mm256_or_si256(_mm256_srli_si256::<0x0A>(vector), _mm256_slli_si256::<0x16>(addition)),
        0x0B => _mm256_or_si256(_mm256_srli_si256::<0x0B>(vector), _mm256_slli_si256::<0x15>(addition)),
        0x0C => _mm256_or_si256(_mm256_srli_si256::<0x0C>(vector), _mm256_slli_si256::<0x14>(addition)),
        0x0D => _mm256_or_si256(_mm256_srli_si256::<0x0D>(vector), _mm256_slli_si256::<0x13>(addition)),
        0x0E => _mm256_or_si256(_mm256_srli_si256::<0x0E>(vector), _mm256_slli_si256::<0x12>(addition)),
        0x0F => _mm256_or_si256(_mm256_srli_si256::<0x0F>(vector), _mm256_slli_si256::<0x11>(addition)),
        0x10 => _mm256_or_si256(_mm256_srli_si256::<0x10>(vector), _mm256_slli_si256::<0x10>(addition)),
        0x11 => _mm256_or_si256(_mm256_srli_si256::<0x11>(vector), _mm256_slli_si256::<0x0F>(addition)),
        0x12 => _mm256_or_si256(_mm256_srli_si256::<0x12>(vector), _mm256_slli_si256::<0x0E>(addition)),
        0x13 => _mm256_or_si256(_mm256_srli_si256::<0x13>(vector), _mm256_slli_si256::<0x0D>(addition)),
        0x14 => _mm256_or_si256(_mm256_srli_si256::<0x14>(vector), _mm256_slli_si256::<0x0C>(addition)),
        0x15 => _mm256_or_si256(_mm256_srli_si256::<0x15>(vector), _mm256_slli_si256::<0x0B>(addition)),
        0x16 => _mm256_or_si256(_mm256_srli_si256::<0x16>(vector), _mm256_slli_si256::<0x0A>(addition)),
        0x17 => _mm256_or_si256(_mm256_srli_si256::<0x17>(vector), _mm256_slli_si256::<0x09>(addition)),
        0x18 => _mm256_or_si256(_mm256_srli_si256::<0x18>(vector), _mm256_slli_si256::<0x08>(addition)),
        0x19 => _mm256_or_si256(_mm256_srli_si256::<0x19>(vector), _mm256_slli_si256::<0x07>(addition)),
        0x1A => _mm256_or_si256(_mm256_srli_si256::<0x1A>(vector), _mm256_slli_si256::<0x06>(addition)),
        0x1B => _mm256_or_si256(_mm256_srli_si256::<0x1B>(vector), _mm256_slli_si256::<0x05>(addition)),
        0x1C => _mm256_or_si256(_mm256_srli_si256::<0x1C>(vector), _mm256_slli_si256::<0x04>(addition)),
        0x1D => _mm256_or_si256(_mm256_srli_si256::<0x1D>(vector), _mm256_slli_si256::<0x03>(addition)),
        0x1E => _mm256_or_si256(_mm256_srli_si256::<0x1E>(vector), _mm256_slli_si256::<0x02>(addition)),
        0x1F => _mm256_or_si256(_mm256_srli_si256::<0x1F>(vector), _mm256_slli_si256::<0x01>(addition)),
        0x20 => addition,
        0x21 => _mm256_srli_si256::<0x01>(addition),
        0x22 => _mm256_srli_si256::<0x02>(addition),
        0x23 => _mm256_srli_si256::<0x03>(addition),
        0x24 => _mm256_srli_si256::<0x04>(addition),
        0x25 => _mm256_srli_si256::<0x05>(addition),
        0x26 => _mm256_srli_si256::<0x06>(addition),
        0x27 => _mm256_srli_si256::<0x07>(addition),
        0x28 => _mm256_srli_si256::<0x08>(addition),
        0x29 => _mm256_srli_si256::<0x09>(addition),
        0x2A => _mm256_srli_si256::<0x0A>(addition),
        0x2B => _mm256_srli_si256::<0x0B>(addition),
        0x2C => _mm256_srli_si256::<0x0C>(addition),
        0x2D => _mm256_srli_si256::<0x0D>(addition),
        0x2E => _mm256_srli_si256::<0x0E>(addition),
        0x2F => _mm256_srli_si256::<0x0F>(addition),
        0x30 => _mm256_srli_si256::<0x10>(addition),
        0x31 => _mm256_srli_si256::<0x11>(addition),
        0x32 => _mm256_srli_si256::<0x12>(addition),
        0x33 => _mm256_srli_si256::<0x13>(addition),
        0x34 => _mm256_srli_si256::<0x14>(addition),
        0x35 => _mm256_srli_si256::<0x15>(addition),
        0x36 => _mm256_srli_si256::<0x16>(addition),
        0x37 => _mm256_srli_si256::<0x17>(addition),
        0x38 => _mm256_srli_si256::<0x18>(addition),
        0x39 => _mm256_srli_si256::<0x19>(addition),
        0x3A => _mm256_srli_si256::<0x1A>(addition),
        0x3B => _mm256_srli_si256::<0x1B>(addition),
        0x3C => _mm256_srli_si256::<0x1C>(addition),
        0x3D => _mm256_srli_si256::<0x1D>(addition),
        0x3E => _mm256_srli_si256::<0x1E>(addition),
        0x3F => _mm256_srli_si256::<0x1F>(addition),
        _ => _mm256_setzero_si256()
    };
}

/// # Example (_mm256_alvext_epi16):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Addition: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
///
/// - Vext'ed left by 1 pair: ```[20, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f]```
/// - Vext'ed left by 2 pairs: ```[1f, 20, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e]```
/// - Vext'ed left by 6 pairs: ```[1b, 1c, 1d, 1e, 1f, 20, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a]```
/// - Vext'ed left by 10 pairs: ```[17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 01, 02, 03, 04, 05, 06]```
/// - Vext'ed left by 16 pairs: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
/// - Vext'ed left by 20 pairs: ```[00, 00, 00, 00, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c]```
/// - Vext'ed left by 31 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 11]```
#[cfg(target_feature = "avx2")]
pub unsafe fn _mm256_alvext_epi16(vector: __m256i, addition: __m256i, shift: usize) -> __m256i {
    return match shift {
        0x00 => vector,
        0x01 => _mm256_or_si256(_mm256_slli_si256::<0x02>(vector), _mm256_srli_si256::<0x1E>(addition)),
        0x02 => _mm256_or_si256(_mm256_slli_si256::<0x04>(vector), _mm256_srli_si256::<0x1C>(addition)),
        0x03 => _mm256_or_si256(_mm256_slli_si256::<0x06>(vector), _mm256_srli_si256::<0x1A>(addition)),
        0x04 => _mm256_or_si256(_mm256_slli_si256::<0x08>(vector), _mm256_srli_si256::<0x18>(addition)),
        0x05 => _mm256_or_si256(_mm256_slli_si256::<0x0A>(vector), _mm256_srli_si256::<0x16>(addition)),
        0x06 => _mm256_or_si256(_mm256_slli_si256::<0x0C>(vector), _mm256_srli_si256::<0x14>(addition)),
        0x07 => _mm256_or_si256(_mm256_slli_si256::<0x0E>(vector), _mm256_srli_si256::<0x12>(addition)),
        0x08 => _mm256_or_si256(_mm256_slli_si256::<0x10>(vector), _mm256_srli_si256::<0x10>(addition)),
        0x09 => _mm256_or_si256(_mm256_slli_si256::<0x12>(vector), _mm256_srli_si256::<0x0E>(addition)),
        0x0A => _mm256_or_si256(_mm256_slli_si256::<0x14>(vector), _mm256_srli_si256::<0x0C>(addition)),
        0x0B => _mm256_or_si256(_mm256_slli_si256::<0x16>(vector), _mm256_srli_si256::<0x0A>(addition)),
        0x0C => _mm256_or_si256(_mm256_slli_si256::<0x18>(vector), _mm256_srli_si256::<0x08>(addition)),
        0x0D => _mm256_or_si256(_mm256_slli_si256::<0x1A>(vector), _mm256_srli_si256::<0x06>(addition)),
        0x0E => _mm256_or_si256(_mm256_slli_si256::<0x1C>(vector), _mm256_srli_si256::<0x04>(addition)),
        0x0F => _mm256_or_si256(_mm256_slli_si256::<0x1E>(vector), _mm256_srli_si256::<0x02>(addition)),
        0x10 => addition,
        0x11 => _mm256_slli_si256::<0x02>(addition),
        0x12 => _mm256_slli_si256::<0x04>(addition),
        0x13 => _mm256_slli_si256::<0x06>(addition),
        0x14 => _mm256_slli_si256::<0x08>(addition),
        0x15 => _mm256_slli_si256::<0x0A>(addition),
        0x16 => _mm256_slli_si256::<0x0C>(addition),
        0x17 => _mm256_slli_si256::<0x0E>(addition),
        0x18 => _mm256_slli_si256::<0x10>(addition),
        0x19 => _mm256_slli_si256::<0x12>(addition),
        0x1A => _mm256_slli_si256::<0x14>(addition),
        0x1B => _mm256_slli_si256::<0x16>(addition),
        0x1C => _mm256_slli_si256::<0x18>(addition),
        0x1D => _mm256_slli_si256::<0x1A>(addition),
        0x1E => _mm256_slli_si256::<0x1C>(addition),
        0x1F => _mm256_slli_si256::<0x1E>(addition),
        _ => _mm256_setzero_si256()
    };
}

/// # Example (_mm256_arvext_epi16):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Addition: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
///
/// - Vext'ed right by 1 pair: ```[02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11]```
/// - Vext'ed right by 2 pairs: ```[03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12]```
/// - Vext'ed right by 6 pairs: ```[07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16]```
/// - Vext'ed right by 10 pairs: ```[0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a]```
/// - Vext'ed right by 16 pairs: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
/// - Vext'ed right by 20 pairs: ```[15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 00, 00, 00, 00]```
/// - Vext'ed right by 31 pairs: ```[20, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(target_feature = "avx2")]
pub unsafe fn _mm256_arvext_epi16(vector: __m256i, addition: __m256i, shift: usize) -> __m256i {
    return match shift {
        0x00 => vector,
        0x01 => _mm256_or_si256(_mm256_srli_si256::<0x02>(vector), _mm256_slli_si256::<0x1E>(addition)),
        0x02 => _mm256_or_si256(_mm256_srli_si256::<0x04>(vector), _mm256_slli_si256::<0x1C>(addition)),
        0x03 => _mm256_or_si256(_mm256_srli_si256::<0x06>(vector), _mm256_slli_si256::<0x1A>(addition)),
        0x04 => _mm256_or_si256(_mm256_srli_si256::<0x08>(vector), _mm256_slli_si256::<0x18>(addition)),
        0x05 => _mm256_or_si256(_mm256_srli_si256::<0x0A>(vector), _mm256_slli_si256::<0x16>(addition)),
        0x06 => _mm256_or_si256(_mm256_srli_si256::<0x0C>(vector), _mm256_slli_si256::<0x14>(addition)),
        0x07 => _mm256_or_si256(_mm256_srli_si256::<0x0E>(vector), _mm256_slli_si256::<0x12>(addition)),
        0x08 => _mm256_or_si256(_mm256_srli_si256::<0x10>(vector), _mm256_slli_si256::<0x10>(addition)),
        0x09 => _mm256_or_si256(_mm256_srli_si256::<0x12>(vector), _mm256_slli_si256::<0x0E>(addition)),
        0x0A => _mm256_or_si256(_mm256_srli_si256::<0x14>(vector), _mm256_slli_si256::<0x0C>(addition)),
        0x0B => _mm256_or_si256(_mm256_srli_si256::<0x16>(vector), _mm256_slli_si256::<0x0A>(addition)),
        0x0C => _mm256_or_si256(_mm256_srli_si256::<0x18>(vector), _mm256_slli_si256::<0x08>(addition)),
        0x0D => _mm256_or_si256(_mm256_srli_si256::<0x1A>(vector), _mm256_slli_si256::<0x06>(addition)),
        0x0E => _mm256_or_si256(_mm256_srli_si256::<0x1C>(vector), _mm256_slli_si256::<0x04>(addition)),
        0x0F => _mm256_or_si256(_mm256_srli_si256::<0x1E>(vector), _mm256_slli_si256::<0x02>(addition)),
        0x10 => addition,
        0x11 => _mm256_srli_si256::<0x02>(addition),
        0x12 => _mm256_srli_si256::<0x04>(addition),
        0x13 => _mm256_srli_si256::<0x06>(addition),
        0x14 => _mm256_srli_si256::<0x08>(addition),
        0x15 => _mm256_srli_si256::<0x0A>(addition),
        0x16 => _mm256_srli_si256::<0x0C>(addition),
        0x17 => _mm256_srli_si256::<0x0E>(addition),
        0x18 => _mm256_srli_si256::<0x10>(addition),
        0x19 => _mm256_srli_si256::<0x12>(addition),
        0x1A => _mm256_srli_si256::<0x14>(addition),
        0x1B => _mm256_srli_si256::<0x16>(addition),
        0x1C => _mm256_srli_si256::<0x18>(addition),
        0x1D => _mm256_srli_si256::<0x1A>(addition),
        0x1E => _mm256_srli_si256::<0x1C>(addition),
        0x1F => _mm256_srli_si256::<0x1E>(addition),
        _ => _mm256_setzero_si256()
    };
}

/// # Example (_mm256_alvext_epi32):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08]```
/// - Addition: ```[11, 12, 13, 14, 15, 16, 17, 18]```
///
/// - Vext'ed left by 1 pair: ```[18, 01, 02, 03, 04, 05, 06, 07]```
/// - Vext'ed left by 2 pairs: ```[17, 18, 01, 02, 03, 04, 05, 06]```
/// - Vext'ed left by 3 pairs: ```[16, 17, 18, 01, 02, 03, 04, 05]```
/// - Vext'ed left by 4 pairs: ```[15, 16, 17, 18, 01, 02, 03, 04]```
/// - Vext'ed left by 5 pairs: ```[14, 15, 16, 17, 18, 01, 02, 03]```
/// - Vext'ed left by 6 pairs: ```[13, 14, 15, 16, 17, 18, 01, 02]```
/// - Vext'ed left by 7 pairs: ```[12, 13, 14, 15, 16, 17, 18, 01]```
/// - Vext'ed left by 8 pairs: ```[11, 12, 13, 14, 15, 16, 17, 18]```
/// - Vext'ed left by 9 pairs: ```[00, 11, 12, 13, 14, 15, 16, 17]```
/// - Vext'ed left by 10 pairs: ```[00, 00, 11, 12, 13, 14, 15, 16]```
/// - Vext'ed left by 11 pairs: ```[00, 00, 00, 11, 12, 13, 14, 15]```
/// - Vext'ed left by 12 pairs: ```[00, 00, 00, 00, 11, 12, 13, 14]```
/// - Vext'ed left by 13 pairs: ```[00, 00, 00, 00, 00, 11, 12, 13]```
/// - Vext'ed left by 14 pairs: ```[00, 00, 00, 00, 00, 00, 11, 12]```
/// - Vext'ed left by 15 pairs: ```[00, 00, 00, 00, 00, 00, 00, 11]```
#[cfg(target_feature = "avx2")]
pub unsafe fn _mm256_alvext_epi32(vector: __m256i, addition: __m256i, shift: usize) -> __m256i {
    return match shift {
        0x00 => vector,
        0x01 => _mm256_or_si256(_mm256_slli_si256::<0x04>(vector), _mm256_srli_si256::<0x1C>(addition)),
        0x02 => _mm256_or_si256(_mm256_slli_si256::<0x08>(vector), _mm256_srli_si256::<0x18>(addition)),
        0x03 => _mm256_or_si256(_mm256_slli_si256::<0x0C>(vector), _mm256_srli_si256::<0x14>(addition)),
        0x04 => _mm256_or_si256(_mm256_slli_si256::<0x10>(vector), _mm256_srli_si256::<0x10>(addition)),
        0x05 => _mm256_or_si256(_mm256_slli_si256::<0x14>(vector), _mm256_srli_si256::<0x0C>(addition)),
        0x06 => _mm256_or_si256(_mm256_slli_si256::<0x18>(vector), _mm256_srli_si256::<0x08>(addition)),
        0x07 => _mm256_or_si256(_mm256_slli_si256::<0x1C>(vector), _mm256_srli_si256::<0x04>(addition)),
        0x08 => addition,
        0x09 => _mm256_slli_si256::<0x04>(addition),
        0x0A => _mm256_slli_si256::<0x08>(addition),
        0x0B => _mm256_slli_si256::<0x0C>(addition),
        0x0C => _mm256_slli_si256::<0x10>(addition),
        0x0D => _mm256_slli_si256::<0x14>(addition),
        0x0E => _mm256_slli_si256::<0x18>(addition),
        0x0F => _mm256_slli_si256::<0x1C>(addition),
        _ => _mm256_setzero_si256()
    };
}

/// # Example (_mm256_arvext_epi32):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08]```
/// - Addition: ```[11, 12, 13, 14, 15, 16, 17, 18]```
///
/// - Vext'ed right by 1 byte:  [02, 03, 04, 05, 06, 07, 08, 11]```
/// - Vext'ed right by 2 bytes:  [03, 04, 05, 06, 07, 08, 11, 12]```
/// - Vext'ed right by 3 bytes:  [04, 05, 06, 07, 08, 11, 12, 13]```
/// - Vext'ed right by 4 bytes:  [05, 06, 07, 08, 11, 12, 13, 14]```
/// - Vext'ed right by 5 bytes:  [06, 07, 08, 11, 12, 13, 14, 15]```
/// - Vext'ed right by 6 bytes:  [07, 08, 11, 12, 13, 14, 15, 16]```
/// - Vext'ed right by 7 bytes:  [08, 11, 12, 13, 14, 15, 16, 17]```
/// - Vext'ed right by 8 bytes:  [11, 12, 13, 14, 15, 16, 17, 18]```
/// - Vext'ed right by 9 bytes:  [12, 13, 14, 15, 16, 17, 18, 00]```
/// - Vext'ed right by 10 bytes:  [13, 14, 15, 16, 17, 18, 00, 00]```
/// - Vext'ed right by 11 bytes:  [14, 15, 16, 17, 18, 00, 00, 00]```
/// - Vext'ed right by 12 bytes:  [15, 16, 17, 18, 00, 00, 00, 00]```
/// - Vext'ed right by 13 bytes:  [16, 17, 18, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 14 bytes:  [17, 18, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 15 bytes:  [18, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(target_feature = "avx2")]
pub unsafe fn _mm256_arvext_epi32(vector: __m256i, addition: __m256i, shift: usize) -> __m256i {
    return match shift {
        0x00 => vector,
        0x01 => _mm256_or_si256(_mm256_srli_si256::<0x04>(vector), _mm256_slli_si256::<0x1C>(addition)),
        0x02 => _mm256_or_si256(_mm256_srli_si256::<0x08>(vector), _mm256_slli_si256::<0x18>(addition)),
        0x03 => _mm256_or_si256(_mm256_srli_si256::<0x0C>(vector), _mm256_slli_si256::<0x14>(addition)),
        0x04 => _mm256_or_si256(_mm256_srli_si256::<0x10>(vector), _mm256_slli_si256::<0x10>(addition)),
        0x05 => _mm256_or_si256(_mm256_srli_si256::<0x14>(vector), _mm256_slli_si256::<0x0C>(addition)),
        0x06 => _mm256_or_si256(_mm256_srli_si256::<0x18>(vector), _mm256_slli_si256::<0x08>(addition)),
        0x07 => _mm256_or_si256(_mm256_srli_si256::<0x1C>(vector), _mm256_slli_si256::<0x04>(addition)),
        0x08 => addition,
        0x09 => _mm256_srli_si256::<0x04>(addition),
        0x0A => _mm256_srli_si256::<0x08>(addition),
        0x0B => _mm256_srli_si256::<0x0C>(addition),
        0x0C => _mm256_srli_si256::<0x10>(addition),
        0x0D => _mm256_srli_si256::<0x14>(addition),
        0x0E => _mm256_srli_si256::<0x18>(addition),
        0x0F => _mm256_srli_si256::<0x1C>(addition),
        _ => _mm256_setzero_si256()
    };
}

/// # Example (_mm256_alvext_epi64):
///
/// - Source: ```[01, 02, 03, 04]```
/// - Addition: ```[05, 06, 07, 08]```
///
/// - Vext'ed left by 1 pair: ```[08, 01, 02, 03]```
/// - Vext'ed left by 2 pairs: ```[07, 08, 01, 02]```
/// - Vext'ed left by 3 pairs: ```[06, 07, 08, 01]```
/// - Vext'ed left by 4 pairs: ```[05, 06, 07, 08]```
/// - Vext'ed left by 5 pairs: ```[00, 05, 06, 07]```
/// - Vext'ed left by 6 pairs: ```[00, 00, 05, 06]```
/// - Vext'ed left by 7 pairs: ```[00, 00, 00, 05]```
/// - Vext'ed left by 8 pairs: ```[00, 00, 00, 00]```
#[cfg(target_feature = "avx2")]
pub unsafe fn _mm256_alvext_epi64(vector: __m256i, addition: __m256i, shift: usize) -> __m256i {
    return match shift {
        0x00 => vector,
        0x01 => _mm256_or_si256(_mm256_slli_si256::<0x08>(vector), _mm256_srli_si256::<0x18>(addition)),
        0x02 => _mm256_or_si256(_mm256_slli_si256::<0x10>(vector), _mm256_srli_si256::<0x10>(addition)),
        0x03 => _mm256_or_si256(_mm256_slli_si256::<0x18>(vector), _mm256_srli_si256::<0x08>(addition)),
        0x04 => addition,
        0x05 => _mm256_slli_si256::<0x08>(addition),
        0x06 => _mm256_slli_si256::<0x10>(addition),
        0x07 => _mm256_slli_si256::<0x18>(addition),
        _ => _mm256_setzero_si256()
    };
}

/// # Example (_mm256_arvext_epi64):
///
/// - Source: ```[01, 02, 03, 04]```
/// - Addition: ```[05, 06, 07, 08]```
///
/// - Vext'ed right by 1 pair: ```[02, 03, 04, 05]```
/// - Vext'ed right by 2 pairs: ```[03, 04, 05, 06]```
/// - Vext'ed right by 3 pairs: ```[04, 05, 06, 07]```
/// - Vext'ed right by 4 pairs: ```[05, 06, 07, 08]```
/// - Vext'ed right by 5 pairs: ```[06, 07, 08, 00]```
/// - Vext'ed right by 6 pairs: ```[07, 08, 00, 00]```
/// - Vext'ed right by 7 pairs: ```[08, 00, 00, 00]```
/// - Vext'ed right by 8 pairs: ```[00, 00, 00, 00]```
#[cfg(target_feature = "avx2")]
pub unsafe fn _mm256_arvext_epi64(vector: __m256i, addition: __m256i, shift: usize) -> __m256i {
    return match shift {
        0x00 => vector,
        0x01 => _mm256_or_si256(_mm256_srli_si256::<0x08>(vector), _mm256_slli_si256::<0x18>(addition)),
        0x02 => _mm256_or_si256(_mm256_srli_si256::<0x10>(vector), _mm256_slli_si256::<0x10>(addition)),
        0x03 => _mm256_or_si256(_mm256_srli_si256::<0x18>(vector), _mm256_slli_si256::<0x08>(addition)),
        0x04 => addition,
        0x05 => _mm256_srli_si256::<0x08>(addition),
        0x06 => _mm256_srli_si256::<0x10>(addition),
        0x07 => _mm256_srli_si256::<0x18>(addition),
        _ => _mm256_setzero_si256()
    };
}

/// # Example (_mm256_alvext_epi128):
///
/// - Source: ```[01, 02]```
/// - Addition: ```[03, 04]```
///
/// - Vext'ed left by 1 pair: ```[04, 01]```
/// - Vext'ed left by 2 pairs: ```[03, 04]```
/// - Vext'ed left by 3 pairs: ```[00, 03]```
/// - Vext'ed left by 4 pairs: ```[00, 00]```
#[cfg(target_feature = "avx2")]
pub unsafe fn _mm256_alvext_epi128(vector: __m256i, addition: __m256i, shift: usize) -> __m256i {
    return match shift {
        0x00 => vector,
        0x01 => _mm256_or_si256(_mm256_slli_si256::<0x10>(vector), _mm256_srli_si256::<0x10>(addition)),
        0x02 => addition,
        0x03 => _mm256_slli_si256::<0x10>(addition),
        _ => _mm256_setzero_si256()
    };
}

/// # Example (_mm256_arvext_epi128):
///
/// - Source: ```[01, 02]```
/// - Addition: ```[03, 04]```
///
/// - Vext'ed right by 1 pair: ```[02, 03]```
/// - Vext'ed right by 2 pairs: ```[03, 04]```
/// - Vext'ed right by 3 pairs: ```[04, 00]```
/// - Vext'ed right by 4 pairs: ```[00, 00]```
#[cfg(target_feature = "avx2")]
pub unsafe fn _mm256_arvext_epi128(vector: __m256i, addition: __m256i, shift: usize) -> __m256i {
    return match shift {
        0x00 => vector,
        0x01 => _mm256_or_si256(_mm256_srli_si256::<0x10>(vector), _mm256_slli_si256::<0x10>(addition)),
        0x02 => addition,
        0x03 => _mm256_srli_si256::<0x10>(addition),
        _ => _mm256_setzero_si256()
    };
}

/// # Example (_mm512_alvext_epi8):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40]```
/// - Addition: ```[41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80]```
///
/// - Vext'ed left by 1 byte: ```[80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f]```
/// - Vext'ed left by 2 bytes: ```[7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e]```
/// - Vext'ed left by 3 bytes: ```[7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d]```
/// - Vext'ed left by 4 bytes: ```[7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c]```
/// - Vext'ed left by 5 bytes: ```[7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b]```
/// - Vext'ed left by 6 bytes: ```[7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a]```
/// - Vext'ed left by 7 bytes: ```[7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39]```
/// - Vext'ed left by 8 bytes: ```[79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38]```
/// - Vext'ed left by 9 bytes: ```[78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37]```
/// - Vext'ed left by 10 bytes: ```[77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36]```
/// - Vext'ed left by 11 bytes: ```[76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35]```
/// - Vext'ed left by 12 bytes: ```[75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34]```
/// - Vext'ed left by 13 bytes: ```[74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33]```
/// - Vext'ed left by 14 bytes: ```[73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32]```
/// - Vext'ed left by 15 bytes: ```[72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31]```
/// - Vext'ed left by 16 bytes: ```[71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30]```
/// - Vext'ed left by 17 bytes: ```[70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f]```
/// - Vext'ed left by 18 bytes: ```[6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e]```
/// - Vext'ed left by 19 bytes: ```[6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d]```
/// - Vext'ed left by 20 bytes: ```[6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c]```
/// - Vext'ed left by 21 bytes: ```[6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b]```
/// - Vext'ed left by 22 bytes: ```[6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a]```
/// - Vext'ed left by 23 bytes: ```[6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]```
/// - Vext'ed left by 24 bytes: ```[69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28]```
/// - Vext'ed left by 25 bytes: ```[68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27]```
/// - Vext'ed left by 26 bytes: ```[67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26]```
/// - Vext'ed left by 27 bytes: ```[66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25]```
/// - Vext'ed left by 28 bytes: ```[65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24]```
/// - Vext'ed left by 29 bytes: ```[64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23]```
/// - Vext'ed left by 30 bytes: ```[63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22]```
/// - Vext'ed left by 31 bytes: ```[62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21]```
/// - Vext'ed left by 32 bytes: ```[61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
/// - Vext'ed left by 33 bytes: ```[60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f]```
/// - Vext'ed left by 34 bytes: ```[5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e]```
/// - Vext'ed left by 35 bytes: ```[5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d]```
/// - Vext'ed left by 36 bytes: ```[5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c]```
/// - Vext'ed left by 37 bytes: ```[5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b]```
/// - Vext'ed left by 38 bytes: ```[5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a]```
/// - Vext'ed left by 39 bytes: ```[5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]```
/// - Vext'ed left by 40 bytes: ```[59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18]```
/// - Vext'ed left by 41 bytes: ```[58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17]```
/// - Vext'ed left by 42 bytes: ```[57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16]```
/// - Vext'ed left by 43 bytes: ```[56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15]```
/// - Vext'ed left by 44 bytes: ```[55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14]```
/// - Vext'ed left by 45 bytes: ```[54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13]```
/// - Vext'ed left by 46 bytes: ```[53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12]```
/// - Vext'ed left by 47 bytes: ```[52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11]```
/// - Vext'ed left by 48 bytes: ```[51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Vext'ed left by 49 bytes: ```[50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f]```
/// - Vext'ed left by 50 bytes: ```[4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e]```
/// - Vext'ed left by 51 bytes: ```[4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d]```
/// - Vext'ed left by 52 bytes: ```[4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c]```
/// - Vext'ed left by 53 bytes: ```[4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b]```
/// - Vext'ed left by 54 bytes: ```[4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a]```
/// - Vext'ed left by 55 bytes: ```[4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08, 09]```
/// - Vext'ed left by 56 bytes: ```[49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07, 08]```
/// - Vext'ed left by 57 bytes: ```[48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06, 07]```
/// - Vext'ed left by 58 bytes: ```[47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05, 06]```
/// - Vext'ed left by 59 bytes: ```[46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04, 05]```
/// - Vext'ed left by 60 bytes: ```[45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03, 04]```
/// - Vext'ed left by 61 bytes: ```[44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02, 03]```
/// - Vext'ed left by 62 bytes: ```[43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01, 02]```
/// - Vext'ed left by 63 bytes: ```[42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 01]```
/// - Vext'ed left by 64 bytes: ```[41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80]```
/// - Vext'ed left by 65 bytes: ```[00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f]```
/// - Vext'ed left by 66 bytes: ```[00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e]```
/// - Vext'ed left by 67 bytes: ```[00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d]```
/// - Vext'ed left by 68 bytes: ```[00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c]```
/// - Vext'ed left by 69 bytes: ```[00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b]```
/// - Vext'ed left by 70 bytes: ```[00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a]```
/// - Vext'ed left by 71 bytes: ```[00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79]```
/// - Vext'ed left by 72 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78]```
/// - Vext'ed left by 73 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77]```
/// - Vext'ed left by 74 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76]```
/// - Vext'ed left by 75 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75]```
/// - Vext'ed left by 76 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74]```
/// - Vext'ed left by 77 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73]```
/// - Vext'ed left by 78 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72]```
/// - Vext'ed left by 79 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71]```
/// - Vext'ed left by 80 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70]```
/// - Vext'ed left by 81 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f]```
/// - Vext'ed left by 82 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e]```
/// - Vext'ed left by 83 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d]```
/// - Vext'ed left by 84 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c]```
/// - Vext'ed left by 85 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b]```
/// - Vext'ed left by 86 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a]```
/// - Vext'ed left by 87 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69]```
/// - Vext'ed left by 88 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68]```
/// - Vext'ed left by 89 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67]```
/// - Vext'ed left by 90 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66]```
/// - Vext'ed left by 91 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65]```
/// - Vext'ed left by 92 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64]```
/// - Vext'ed left by 93 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63]```
/// - Vext'ed left by 94 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62]```
/// - Vext'ed left by 95 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61]```
/// - Vext'ed left by 96 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60]```
/// - Vext'ed left by 97 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f]```
/// - Vext'ed left by 98 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e]```
/// - Vext'ed left by 99 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d]```
/// - Vext'ed left by 100 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c]```
/// - Vext'ed left by 101 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b]```
/// - Vext'ed left by 102 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a]```
/// - Vext'ed left by 103 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59]```
/// - Vext'ed left by 104 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58]```
/// - Vext'ed left by 105 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57]```
/// - Vext'ed left by 106 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56]```
/// - Vext'ed left by 107 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55]```
/// - Vext'ed left by 108 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54]```
/// - Vext'ed left by 109 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53]```
/// - Vext'ed left by 110 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52]```
/// - Vext'ed left by 111 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51]```
/// - Vext'ed left by 112 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50]```
/// - Vext'ed left by 113 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f]```
/// - Vext'ed left by 114 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e]```
/// - Vext'ed left by 115 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d]```
/// - Vext'ed left by 116 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c]```
/// - Vext'ed left by 117 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b]```
/// - Vext'ed left by 118 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a]```
/// - Vext'ed left by 119 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48, 49]```
/// - Vext'ed left by 120 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47, 48]```
/// - Vext'ed left by 121 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46, 47]```
/// - Vext'ed left by 122 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45, 46]```
/// - Vext'ed left by 123 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44, 45]```
/// - Vext'ed left by 124 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43, 44]```
/// - Vext'ed left by 125 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42, 43]```
/// - Vext'ed left by 126 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 42]```
/// - Vext'ed left by 127 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41]```
/// - Vext'ed left by 128 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(target_feature = "avx512f", target_feature = "avx512bw"))]
pub unsafe fn _mm512_alvext_epi8(vector: __m512i, addition: __m512i, shift: usize) -> __m512i {
    return match shift {
        0x00 => vector,
        0x01 => _mm512_or_si512(_mm512_slli_si512::<0x01>(vector), _mm512_srli_si512::<0x3F>(addition)),
        0x02 => _mm512_or_si512(_mm512_slli_si512::<0x02>(vector), _mm512_srli_si512::<0x3E>(addition)),
        0x03 => _mm512_or_si512(_mm512_slli_si512::<0x03>(vector), _mm512_srli_si512::<0x3D>(addition)),
        0x04 => _mm512_or_si512(_mm512_slli_si512::<0x04>(vector), _mm512_srli_si512::<0x3C>(addition)),
        0x05 => _mm512_or_si512(_mm512_slli_si512::<0x05>(vector), _mm512_srli_si512::<0x3B>(addition)),
        0x06 => _mm512_or_si512(_mm512_slli_si512::<0x06>(vector), _mm512_srli_si512::<0x3A>(addition)),
        0x07 => _mm512_or_si512(_mm512_slli_si512::<0x07>(vector), _mm512_srli_si512::<0x39>(addition)),
        0x08 => _mm512_or_si512(_mm512_slli_si512::<0x08>(vector), _mm512_srli_si512::<0x38>(addition)),
        0x09 => _mm512_or_si512(_mm512_slli_si512::<0x09>(vector), _mm512_srli_si512::<0x37>(addition)),
        0x0A => _mm512_or_si512(_mm512_slli_si512::<0x0A>(vector), _mm512_srli_si512::<0x36>(addition)),
        0x0B => _mm512_or_si512(_mm512_slli_si512::<0x0B>(vector), _mm512_srli_si512::<0x35>(addition)),
        0x0C => _mm512_or_si512(_mm512_slli_si512::<0x0C>(vector), _mm512_srli_si512::<0x34>(addition)),
        0x0D => _mm512_or_si512(_mm512_slli_si512::<0x0D>(vector), _mm512_srli_si512::<0x33>(addition)),
        0x0E => _mm512_or_si512(_mm512_slli_si512::<0x0E>(vector), _mm512_srli_si512::<0x32>(addition)),
        0x0F => _mm512_or_si512(_mm512_slli_si512::<0x0F>(vector), _mm512_srli_si512::<0x31>(addition)),
        0x10 => _mm512_or_si512(_mm512_slli_si512::<0x10>(vector), _mm512_srli_si512::<0x30>(addition)),
        0x11 => _mm512_or_si512(_mm512_slli_si512::<0x11>(vector), _mm512_srli_si512::<0x2F>(addition)),
        0x12 => _mm512_or_si512(_mm512_slli_si512::<0x12>(vector), _mm512_srli_si512::<0x2E>(addition)),
        0x13 => _mm512_or_si512(_mm512_slli_si512::<0x13>(vector), _mm512_srli_si512::<0x2D>(addition)),
        0x14 => _mm512_or_si512(_mm512_slli_si512::<0x14>(vector), _mm512_srli_si512::<0x2C>(addition)),
        0x15 => _mm512_or_si512(_mm512_slli_si512::<0x15>(vector), _mm512_srli_si512::<0x2B>(addition)),
        0x16 => _mm512_or_si512(_mm512_slli_si512::<0x16>(vector), _mm512_srli_si512::<0x2A>(addition)),
        0x17 => _mm512_or_si512(_mm512_slli_si512::<0x17>(vector), _mm512_srli_si512::<0x29>(addition)),
        0x18 => _mm512_or_si512(_mm512_slli_si512::<0x18>(vector), _mm512_srli_si512::<0x28>(addition)),
        0x19 => _mm512_or_si512(_mm512_slli_si512::<0x19>(vector), _mm512_srli_si512::<0x27>(addition)),
        0x1A => _mm512_or_si512(_mm512_slli_si512::<0x1A>(vector), _mm512_srli_si512::<0x26>(addition)),
        0x1B => _mm512_or_si512(_mm512_slli_si512::<0x1B>(vector), _mm512_srli_si512::<0x25>(addition)),
        0x1C => _mm512_or_si512(_mm512_slli_si512::<0x1C>(vector), _mm512_srli_si512::<0x24>(addition)),
        0x1D => _mm512_or_si512(_mm512_slli_si512::<0x1D>(vector), _mm512_srli_si512::<0x23>(addition)),
        0x1E => _mm512_or_si512(_mm512_slli_si512::<0x1E>(vector), _mm512_srli_si512::<0x22>(addition)),
        0x1F => _mm512_or_si512(_mm512_slli_si512::<0x1F>(vector), _mm512_srli_si512::<0x21>(addition)),
        0x20 => _mm512_or_si512(_mm512_slli_si512::<0x20>(vector), _mm512_srli_si512::<0x20>(addition)),
        0x21 => _mm512_or_si512(_mm512_slli_si512::<0x21>(vector), _mm512_srli_si512::<0x1F>(addition)),
        0x22 => _mm512_or_si512(_mm512_slli_si512::<0x22>(vector), _mm512_srli_si512::<0x1E>(addition)),
        0x23 => _mm512_or_si512(_mm512_slli_si512::<0x23>(vector), _mm512_srli_si512::<0x1D>(addition)),
        0x24 => _mm512_or_si512(_mm512_slli_si512::<0x24>(vector), _mm512_srli_si512::<0x1C>(addition)),
        0x25 => _mm512_or_si512(_mm512_slli_si512::<0x25>(vector), _mm512_srli_si512::<0x1B>(addition)),
        0x26 => _mm512_or_si512(_mm512_slli_si512::<0x26>(vector), _mm512_srli_si512::<0x1A>(addition)),
        0x27 => _mm512_or_si512(_mm512_slli_si512::<0x27>(vector), _mm512_srli_si512::<0x19>(addition)),
        0x28 => _mm512_or_si512(_mm512_slli_si512::<0x28>(vector), _mm512_srli_si512::<0x18>(addition)),
        0x29 => _mm512_or_si512(_mm512_slli_si512::<0x29>(vector), _mm512_srli_si512::<0x17>(addition)),
        0x2A => _mm512_or_si512(_mm512_slli_si512::<0x2A>(vector), _mm512_srli_si512::<0x16>(addition)),
        0x2B => _mm512_or_si512(_mm512_slli_si512::<0x2B>(vector), _mm512_srli_si512::<0x15>(addition)),
        0x2C => _mm512_or_si512(_mm512_slli_si512::<0x2C>(vector), _mm512_srli_si512::<0x14>(addition)),
        0x2D => _mm512_or_si512(_mm512_slli_si512::<0x2D>(vector), _mm512_srli_si512::<0x13>(addition)),
        0x2E => _mm512_or_si512(_mm512_slli_si512::<0x2E>(vector), _mm512_srli_si512::<0x12>(addition)),
        0x2F => _mm512_or_si512(_mm512_slli_si512::<0x2F>(vector), _mm512_srli_si512::<0x11>(addition)),
        0x30 => _mm512_or_si512(_mm512_slli_si512::<0x30>(vector), _mm512_srli_si512::<0x10>(addition)),
        0x31 => _mm512_or_si512(_mm512_slli_si512::<0x31>(vector), _mm512_srli_si512::<0x0F>(addition)),
        0x32 => _mm512_or_si512(_mm512_slli_si512::<0x32>(vector), _mm512_srli_si512::<0x0E>(addition)),
        0x33 => _mm512_or_si512(_mm512_slli_si512::<0x33>(vector), _mm512_srli_si512::<0x0D>(addition)),
        0x34 => _mm512_or_si512(_mm512_slli_si512::<0x34>(vector), _mm512_srli_si512::<0x0C>(addition)),
        0x35 => _mm512_or_si512(_mm512_slli_si512::<0x35>(vector), _mm512_srli_si512::<0x0B>(addition)),
        0x36 => _mm512_or_si512(_mm512_slli_si512::<0x36>(vector), _mm512_srli_si512::<0x0A>(addition)),
        0x37 => _mm512_or_si512(_mm512_slli_si512::<0x37>(vector), _mm512_srli_si512::<0x09>(addition)),
        0x38 => _mm512_or_si512(_mm512_slli_si512::<0x38>(vector), _mm512_srli_si512::<0x08>(addition)),
        0x39 => _mm512_or_si512(_mm512_slli_si512::<0x39>(vector), _mm512_srli_si512::<0x07>(addition)),
        0x3A => _mm512_or_si512(_mm512_slli_si512::<0x3A>(vector), _mm512_srli_si512::<0x06>(addition)),
        0x3B => _mm512_or_si512(_mm512_slli_si512::<0x3B>(vector), _mm512_srli_si512::<0x05>(addition)),
        0x3C => _mm512_or_si512(_mm512_slli_si512::<0x3C>(vector), _mm512_srli_si512::<0x04>(addition)),
        0x3D => _mm512_or_si512(_mm512_slli_si512::<0x3D>(vector), _mm512_srli_si512::<0x03>(addition)),
        0x3E => _mm512_or_si512(_mm512_slli_si512::<0x3E>(vector), _mm512_srli_si512::<0x02>(addition)),
        0x3F => _mm512_or_si512(_mm512_slli_si512::<0x3F>(vector), _mm512_srli_si512::<0x01>(addition)),
        0x40 => addition,
        0x41 => _mm512_slli_si512::<0x01>(addition),
        0x42 => _mm512_slli_si512::<0x02>(addition),
        0x43 => _mm512_slli_si512::<0x03>(addition),
        0x44 => _mm512_slli_si512::<0x04>(addition),
        0x45 => _mm512_slli_si512::<0x05>(addition),
        0x46 => _mm512_slli_si512::<0x06>(addition),
        0x47 => _mm512_slli_si512::<0x07>(addition),
        0x48 => _mm512_slli_si512::<0x08>(addition),
        0x49 => _mm512_slli_si512::<0x09>(addition),
        0x4A => _mm512_slli_si512::<0x0A>(addition),
        0x4B => _mm512_slli_si512::<0x0B>(addition),
        0x4C => _mm512_slli_si512::<0x0C>(addition),
        0x4D => _mm512_slli_si512::<0x0D>(addition),
        0x4E => _mm512_slli_si512::<0x0E>(addition),
        0x4F => _mm512_slli_si512::<0x0F>(addition),
        0x50 => _mm512_slli_si512::<0x10>(addition),
        0x51 => _mm512_slli_si512::<0x11>(addition),
        0x52 => _mm512_slli_si512::<0x12>(addition),
        0x53 => _mm512_slli_si512::<0x13>(addition),
        0x54 => _mm512_slli_si512::<0x14>(addition),
        0x55 => _mm512_slli_si512::<0x15>(addition),
        0x56 => _mm512_slli_si512::<0x16>(addition),
        0x57 => _mm512_slli_si512::<0x17>(addition),
        0x58 => _mm512_slli_si512::<0x18>(addition),
        0x59 => _mm512_slli_si512::<0x19>(addition),
        0x5A => _mm512_slli_si512::<0x1A>(addition),
        0x5B => _mm512_slli_si512::<0x1B>(addition),
        0x5C => _mm512_slli_si512::<0x1C>(addition),
        0x5D => _mm512_slli_si512::<0x1D>(addition),
        0x5E => _mm512_slli_si512::<0x1E>(addition),
        0x5F => _mm512_slli_si512::<0x1F>(addition),
        0x60 => _mm512_slli_si512::<0x20>(addition),
        0x61 => _mm512_slli_si512::<0x21>(addition),
        0x62 => _mm512_slli_si512::<0x22>(addition),
        0x63 => _mm512_slli_si512::<0x23>(addition),
        0x64 => _mm512_slli_si512::<0x24>(addition),
        0x65 => _mm512_slli_si512::<0x25>(addition),
        0x66 => _mm512_slli_si512::<0x26>(addition),
        0x67 => _mm512_slli_si512::<0x27>(addition),
        0x68 => _mm512_slli_si512::<0x28>(addition),
        0x69 => _mm512_slli_si512::<0x29>(addition),
        0x6A => _mm512_slli_si512::<0x2A>(addition),
        0x6B => _mm512_slli_si512::<0x2B>(addition),
        0x6C => _mm512_slli_si512::<0x2C>(addition),
        0x6D => _mm512_slli_si512::<0x2D>(addition),
        0x6E => _mm512_slli_si512::<0x2E>(addition),
        0x6F => _mm512_slli_si512::<0x2F>(addition),
        0x70 => _mm512_slli_si512::<0x30>(addition),
        0x71 => _mm512_slli_si512::<0x31>(addition),
        0x72 => _mm512_slli_si512::<0x32>(addition),
        0x73 => _mm512_slli_si512::<0x33>(addition),
        0x74 => _mm512_slli_si512::<0x34>(addition),
        0x75 => _mm512_slli_si512::<0x35>(addition),
        0x76 => _mm512_slli_si512::<0x36>(addition),
        0x77 => _mm512_slli_si512::<0x37>(addition),
        0x78 => _mm512_slli_si512::<0x38>(addition),
        0x79 => _mm512_slli_si512::<0x39>(addition),
        0x7A => _mm512_slli_si512::<0x3A>(addition),
        0x7B => _mm512_slli_si512::<0x3B>(addition),
        0x7C => _mm512_slli_si512::<0x3C>(addition),
        0x7D => _mm512_slli_si512::<0x3D>(addition),
        0x7E => _mm512_slli_si512::<0x3E>(addition),
        0x7F => _mm512_slli_si512::<0x3F>(addition),
        _ => _mm512_setzero_si512()
    };
}

/// # Example (_mm512_arvext_epi8):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40]```
/// - Addition: ```[41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80]```
///
/// - Vext'ed right by 1 byte: ```[02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41]```
/// - Vext'ed right by 2 bytes: ```[03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42]```
/// - Vext'ed right by 3 bytes: ```[04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43]```
/// - Vext'ed right by 4 bytes: ```[05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44]```
/// - Vext'ed right by 5 bytes: ```[06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45]```
/// - Vext'ed right by 6 bytes: ```[07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46]```
/// - Vext'ed right by 7 bytes: ```[08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47]```
/// - Vext'ed right by 8 bytes: ```[09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48]```
/// - Vext'ed right by 9 bytes: ```[0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49]```
/// - Vext'ed right by 10 bytes: ```[0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a]```
/// - Vext'ed right by 11 bytes: ```[0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b]```
/// - Vext'ed right by 12 bytes: ```[0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c]```
/// - Vext'ed right by 13 bytes: ```[0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d]```
/// - Vext'ed right by 14 bytes: ```[0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e]```
/// - Vext'ed right by 15 bytes: ```[10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f]```
/// - Vext'ed right by 16 bytes: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50]```
/// - Vext'ed right by 17 bytes: ```[12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51]```
/// - Vext'ed right by 18 bytes: ```[13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52]```
/// - Vext'ed right by 19 bytes: ```[14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53]```
/// - Vext'ed right by 20 bytes: ```[15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54]```
/// - Vext'ed right by 21 bytes: ```[16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55]```
/// - Vext'ed right by 22 bytes: ```[17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56]```
/// - Vext'ed right by 23 bytes: ```[18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57]```
/// - Vext'ed right by 24 bytes: ```[19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58]```
/// - Vext'ed right by 25 bytes: ```[1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59]```
/// - Vext'ed right by 26 bytes: ```[1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a]```
/// - Vext'ed right by 27 bytes: ```[1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b]```
/// - Vext'ed right by 28 bytes: ```[1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c]```
/// - Vext'ed right by 29 bytes: ```[1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d]```
/// - Vext'ed right by 30 bytes: ```[1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e]```
/// - Vext'ed right by 31 bytes: ```[20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f]```
/// - Vext'ed right by 32 bytes: ```[21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60]```
/// - Vext'ed right by 33 bytes: ```[22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61]```
/// - Vext'ed right by 34 bytes: ```[23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62]```
/// - Vext'ed right by 35 bytes: ```[24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63]```
/// - Vext'ed right by 36 bytes: ```[25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64]```
/// - Vext'ed right by 37 bytes: ```[26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65]```
/// - Vext'ed right by 38 bytes: ```[27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66]```
/// - Vext'ed right by 39 bytes: ```[28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67]```
/// - Vext'ed right by 40 bytes: ```[29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68]```
/// - Vext'ed right by 41 bytes: ```[2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69]```
/// - Vext'ed right by 42 bytes: ```[2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a]```
/// - Vext'ed right by 43 bytes: ```[2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b]```
/// - Vext'ed right by 44 bytes: ```[2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c]```
/// - Vext'ed right by 45 bytes: ```[2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d]```
/// - Vext'ed right by 46 bytes: ```[2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e]```
/// - Vext'ed right by 47 bytes: ```[30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f]```
/// - Vext'ed right by 48 bytes: ```[31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70]```
/// - Vext'ed right by 49 bytes: ```[32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71]```
/// - Vext'ed right by 50 bytes: ```[33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72]```
/// - Vext'ed right by 51 bytes: ```[34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73]```
/// - Vext'ed right by 52 bytes: ```[35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74]```
/// - Vext'ed right by 53 bytes: ```[36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75]```
/// - Vext'ed right by 54 bytes: ```[37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76]```
/// - Vext'ed right by 55 bytes: ```[38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77]```
/// - Vext'ed right by 56 bytes: ```[39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78]```
/// - Vext'ed right by 57 bytes: ```[3a, 3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79]```
/// - Vext'ed right by 58 bytes: ```[3b, 3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a]```
/// - Vext'ed right by 59 bytes: ```[3c, 3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b]```
/// - Vext'ed right by 60 bytes: ```[3d, 3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c]```
/// - Vext'ed right by 61 bytes: ```[3e, 3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d]```
/// - Vext'ed right by 62 bytes: ```[3f, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e]```
/// - Vext'ed right by 63 bytes: ```[40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f]```
/// - Vext'ed right by 64 bytes: ```[41, 42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80]```
/// - Vext'ed right by 65 bytes: ```[42, 43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00]```
/// - Vext'ed right by 66 bytes: ```[43, 44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00]```
/// - Vext'ed right by 67 bytes: ```[44, 45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00]```
/// - Vext'ed right by 68 bytes: ```[45, 46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00]```
/// - Vext'ed right by 69 bytes: ```[46, 47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 70 bytes: ```[47, 48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 71 bytes: ```[48, 49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 72 bytes: ```[49, 4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 73 bytes: ```[4a, 4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 74 bytes: ```[4b, 4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 75 bytes: ```[4c, 4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 76 bytes: ```[4d, 4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 77 bytes: ```[4e, 4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 78 bytes: ```[4f, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 79 bytes: ```[50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 80 bytes: ```[51, 52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 81 bytes: ```[52, 53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 82 bytes: ```[53, 54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 83 bytes: ```[54, 55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 84 bytes: ```[55, 56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 85 bytes: ```[56, 57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 86 bytes: ```[57, 58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 87 bytes: ```[58, 59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 88 bytes: ```[59, 5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 89 bytes: ```[5a, 5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 90 bytes: ```[5b, 5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 91 bytes: ```[5c, 5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 92 bytes: ```[5d, 5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 93 bytes: ```[5e, 5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 94 bytes: ```[5f, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 95 bytes: ```[60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 96 bytes: ```[61, 62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 97 bytes: ```[62, 63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 98 bytes: ```[63, 64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 99 bytes: ```[64, 65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 100 bytes: ```[65, 66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 101 bytes: ```[66, 67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 102 bytes: ```[67, 68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 103 bytes: ```[68, 69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 104 bytes: ```[69, 6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 105 bytes: ```[6a, 6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 106 bytes: ```[6b, 6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 107 bytes: ```[6c, 6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 108 bytes: ```[6d, 6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 109 bytes: ```[6e, 6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 110 bytes: ```[6f, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 111 bytes: ```[70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 112 bytes: ```[71, 72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 113 bytes: ```[72, 73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 114 bytes: ```[73, 74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 115 bytes: ```[74, 75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 116 bytes: ```[75, 76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 117 bytes: ```[76, 77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 118 bytes: ```[77, 78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 119 bytes: ```[78, 79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 120 bytes: ```[79, 7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 121 bytes: ```[7a, 7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 122 bytes: ```[7b, 7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 123 bytes: ```[7c, 7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 124 bytes: ```[7d, 7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 125 bytes: ```[7e, 7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 126 bytes: ```[7f, 80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 127 bytes: ```[80, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 128 bytes: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(target_feature = "avx512f", target_feature = "avx512bw"))]
pub unsafe fn _mm512_arvext_epi8(vector: __m512i, addition: __m512i, shift: usize) -> __m512i {
    return match shift {
        0x00 => vector,
        0x01 => _mm512_or_si512(_mm512_srli_si512::<0x01>(vector), _mm512_slli_si512::<0x3F>(addition)),
        0x02 => _mm512_or_si512(_mm512_srli_si512::<0x02>(vector), _mm512_slli_si512::<0x3E>(addition)),
        0x03 => _mm512_or_si512(_mm512_srli_si512::<0x03>(vector), _mm512_slli_si512::<0x3D>(addition)),
        0x04 => _mm512_or_si512(_mm512_srli_si512::<0x04>(vector), _mm512_slli_si512::<0x3C>(addition)),
        0x05 => _mm512_or_si512(_mm512_srli_si512::<0x05>(vector), _mm512_slli_si512::<0x3B>(addition)),
        0x06 => _mm512_or_si512(_mm512_srli_si512::<0x06>(vector), _mm512_slli_si512::<0x3A>(addition)),
        0x07 => _mm512_or_si512(_mm512_srli_si512::<0x07>(vector), _mm512_slli_si512::<0x39>(addition)),
        0x08 => _mm512_or_si512(_mm512_srli_si512::<0x08>(vector), _mm512_slli_si512::<0x38>(addition)),
        0x09 => _mm512_or_si512(_mm512_srli_si512::<0x09>(vector), _mm512_slli_si512::<0x37>(addition)),
        0x0A => _mm512_or_si512(_mm512_srli_si512::<0x0A>(vector), _mm512_slli_si512::<0x36>(addition)),
        0x0B => _mm512_or_si512(_mm512_srli_si512::<0x0B>(vector), _mm512_slli_si512::<0x35>(addition)),
        0x0C => _mm512_or_si512(_mm512_srli_si512::<0x0C>(vector), _mm512_slli_si512::<0x34>(addition)),
        0x0D => _mm512_or_si512(_mm512_srli_si512::<0x0D>(vector), _mm512_slli_si512::<0x33>(addition)),
        0x0E => _mm512_or_si512(_mm512_srli_si512::<0x0E>(vector), _mm512_slli_si512::<0x32>(addition)),
        0x0F => _mm512_or_si512(_mm512_srli_si512::<0x0F>(vector), _mm512_slli_si512::<0x31>(addition)),
        0x10 => _mm512_or_si512(_mm512_srli_si512::<0x10>(vector), _mm512_slli_si512::<0x30>(addition)),
        0x11 => _mm512_or_si512(_mm512_srli_si512::<0x11>(vector), _mm512_slli_si512::<0x2F>(addition)),
        0x12 => _mm512_or_si512(_mm512_srli_si512::<0x12>(vector), _mm512_slli_si512::<0x2E>(addition)),
        0x13 => _mm512_or_si512(_mm512_srli_si512::<0x13>(vector), _mm512_slli_si512::<0x2D>(addition)),
        0x14 => _mm512_or_si512(_mm512_srli_si512::<0x14>(vector), _mm512_slli_si512::<0x2C>(addition)),
        0x15 => _mm512_or_si512(_mm512_srli_si512::<0x15>(vector), _mm512_slli_si512::<0x2B>(addition)),
        0x16 => _mm512_or_si512(_mm512_srli_si512::<0x16>(vector), _mm512_slli_si512::<0x2A>(addition)),
        0x17 => _mm512_or_si512(_mm512_srli_si512::<0x17>(vector), _mm512_slli_si512::<0x29>(addition)),
        0x18 => _mm512_or_si512(_mm512_srli_si512::<0x18>(vector), _mm512_slli_si512::<0x28>(addition)),
        0x19 => _mm512_or_si512(_mm512_srli_si512::<0x19>(vector), _mm512_slli_si512::<0x27>(addition)),
        0x1A => _mm512_or_si512(_mm512_srli_si512::<0x1A>(vector), _mm512_slli_si512::<0x26>(addition)),
        0x1B => _mm512_or_si512(_mm512_srli_si512::<0x1B>(vector), _mm512_slli_si512::<0x25>(addition)),
        0x1C => _mm512_or_si512(_mm512_srli_si512::<0x1C>(vector), _mm512_slli_si512::<0x24>(addition)),
        0x1D => _mm512_or_si512(_mm512_srli_si512::<0x1D>(vector), _mm512_slli_si512::<0x23>(addition)),
        0x1E => _mm512_or_si512(_mm512_srli_si512::<0x1E>(vector), _mm512_slli_si512::<0x22>(addition)),
        0x1F => _mm512_or_si512(_mm512_srli_si512::<0x1F>(vector), _mm512_slli_si512::<0x21>(addition)),
        0x20 => _mm512_or_si512(_mm512_srli_si512::<0x20>(vector), _mm512_slli_si512::<0x20>(addition)),
        0x21 => _mm512_or_si512(_mm512_srli_si512::<0x21>(vector), _mm512_slli_si512::<0x1F>(addition)),
        0x22 => _mm512_or_si512(_mm512_srli_si512::<0x22>(vector), _mm512_slli_si512::<0x1E>(addition)),
        0x23 => _mm512_or_si512(_mm512_srli_si512::<0x23>(vector), _mm512_slli_si512::<0x1D>(addition)),
        0x24 => _mm512_or_si512(_mm512_srli_si512::<0x24>(vector), _mm512_slli_si512::<0x1C>(addition)),
        0x25 => _mm512_or_si512(_mm512_srli_si512::<0x25>(vector), _mm512_slli_si512::<0x1B>(addition)),
        0x26 => _mm512_or_si512(_mm512_srli_si512::<0x26>(vector), _mm512_slli_si512::<0x1A>(addition)),
        0x27 => _mm512_or_si512(_mm512_srli_si512::<0x27>(vector), _mm512_slli_si512::<0x19>(addition)),
        0x28 => _mm512_or_si512(_mm512_srli_si512::<0x28>(vector), _mm512_slli_si512::<0x18>(addition)),
        0x29 => _mm512_or_si512(_mm512_srli_si512::<0x29>(vector), _mm512_slli_si512::<0x17>(addition)),
        0x2A => _mm512_or_si512(_mm512_srli_si512::<0x2A>(vector), _mm512_slli_si512::<0x16>(addition)),
        0x2B => _mm512_or_si512(_mm512_srli_si512::<0x2B>(vector), _mm512_slli_si512::<0x15>(addition)),
        0x2C => _mm512_or_si512(_mm512_srli_si512::<0x2C>(vector), _mm512_slli_si512::<0x14>(addition)),
        0x2D => _mm512_or_si512(_mm512_srli_si512::<0x2D>(vector), _mm512_slli_si512::<0x13>(addition)),
        0x2E => _mm512_or_si512(_mm512_srli_si512::<0x2E>(vector), _mm512_slli_si512::<0x12>(addition)),
        0x2F => _mm512_or_si512(_mm512_srli_si512::<0x2F>(vector), _mm512_slli_si512::<0x11>(addition)),
        0x30 => _mm512_or_si512(_mm512_srli_si512::<0x30>(vector), _mm512_slli_si512::<0x10>(addition)),
        0x31 => _mm512_or_si512(_mm512_srli_si512::<0x31>(vector), _mm512_slli_si512::<0x0F>(addition)),
        0x32 => _mm512_or_si512(_mm512_srli_si512::<0x32>(vector), _mm512_slli_si512::<0x0E>(addition)),
        0x33 => _mm512_or_si512(_mm512_srli_si512::<0x33>(vector), _mm512_slli_si512::<0x0D>(addition)),
        0x34 => _mm512_or_si512(_mm512_srli_si512::<0x34>(vector), _mm512_slli_si512::<0x0C>(addition)),
        0x35 => _mm512_or_si512(_mm512_srli_si512::<0x35>(vector), _mm512_slli_si512::<0x0B>(addition)),
        0x36 => _mm512_or_si512(_mm512_srli_si512::<0x36>(vector), _mm512_slli_si512::<0x0A>(addition)),
        0x37 => _mm512_or_si512(_mm512_srli_si512::<0x37>(vector), _mm512_slli_si512::<0x09>(addition)),
        0x38 => _mm512_or_si512(_mm512_srli_si512::<0x38>(vector), _mm512_slli_si512::<0x08>(addition)),
        0x39 => _mm512_or_si512(_mm512_srli_si512::<0x39>(vector), _mm512_slli_si512::<0x07>(addition)),
        0x3A => _mm512_or_si512(_mm512_srli_si512::<0x3A>(vector), _mm512_slli_si512::<0x06>(addition)),
        0x3B => _mm512_or_si512(_mm512_srli_si512::<0x3B>(vector), _mm512_slli_si512::<0x05>(addition)),
        0x3C => _mm512_or_si512(_mm512_srli_si512::<0x3C>(vector), _mm512_slli_si512::<0x04>(addition)),
        0x3D => _mm512_or_si512(_mm512_srli_si512::<0x3D>(vector), _mm512_slli_si512::<0x03>(addition)),
        0x3E => _mm512_or_si512(_mm512_srli_si512::<0x3E>(vector), _mm512_slli_si512::<0x02>(addition)),
        0x3F => _mm512_or_si512(_mm512_srli_si512::<0x3F>(vector), _mm512_slli_si512::<0x01>(addition)),
        0x40 => addition,
        0x41 => _mm512_srli_si512::<0x01>(addition),
        0x42 => _mm512_srli_si512::<0x02>(addition),
        0x43 => _mm512_srli_si512::<0x03>(addition),
        0x44 => _mm512_srli_si512::<0x04>(addition),
        0x45 => _mm512_srli_si512::<0x05>(addition),
        0x46 => _mm512_srli_si512::<0x06>(addition),
        0x47 => _mm512_srli_si512::<0x07>(addition),
        0x48 => _mm512_srli_si512::<0x08>(addition),
        0x49 => _mm512_srli_si512::<0x09>(addition),
        0x4A => _mm512_srli_si512::<0x0A>(addition),
        0x4B => _mm512_srli_si512::<0x0B>(addition),
        0x4C => _mm512_srli_si512::<0x0C>(addition),
        0x4D => _mm512_srli_si512::<0x0D>(addition),
        0x4E => _mm512_srli_si512::<0x0E>(addition),
        0x4F => _mm512_srli_si512::<0x0F>(addition),
        0x50 => _mm512_srli_si512::<0x10>(addition),
        0x51 => _mm512_srli_si512::<0x11>(addition),
        0x52 => _mm512_srli_si512::<0x12>(addition),
        0x53 => _mm512_srli_si512::<0x13>(addition),
        0x54 => _mm512_srli_si512::<0x14>(addition),
        0x55 => _mm512_srli_si512::<0x15>(addition),
        0x56 => _mm512_srli_si512::<0x16>(addition),
        0x57 => _mm512_srli_si512::<0x17>(addition),
        0x58 => _mm512_srli_si512::<0x18>(addition),
        0x59 => _mm512_srli_si512::<0x19>(addition),
        0x5A => _mm512_srli_si512::<0x1A>(addition),
        0x5B => _mm512_srli_si512::<0x1B>(addition),
        0x5C => _mm512_srli_si512::<0x1C>(addition),
        0x5D => _mm512_srli_si512::<0x1D>(addition),
        0x5E => _mm512_srli_si512::<0x1E>(addition),
        0x5F => _mm512_srli_si512::<0x1F>(addition),
        0x60 => _mm512_srli_si512::<0x20>(addition),
        0x61 => _mm512_srli_si512::<0x21>(addition),
        0x62 => _mm512_srli_si512::<0x22>(addition),
        0x63 => _mm512_srli_si512::<0x23>(addition),
        0x64 => _mm512_srli_si512::<0x24>(addition),
        0x65 => _mm512_srli_si512::<0x25>(addition),
        0x66 => _mm512_srli_si512::<0x26>(addition),
        0x67 => _mm512_srli_si512::<0x27>(addition),
        0x68 => _mm512_srli_si512::<0x28>(addition),
        0x69 => _mm512_srli_si512::<0x29>(addition),
        0x6A => _mm512_srli_si512::<0x2A>(addition),
        0x6B => _mm512_srli_si512::<0x2B>(addition),
        0x6C => _mm512_srli_si512::<0x2C>(addition),
        0x6D => _mm512_srli_si512::<0x2D>(addition),
        0x6E => _mm512_srli_si512::<0x2E>(addition),
        0x6F => _mm512_srli_si512::<0x2F>(addition),
        0x70 => _mm512_srli_si512::<0x30>(addition),
        0x71 => _mm512_srli_si512::<0x31>(addition),
        0x72 => _mm512_srli_si512::<0x32>(addition),
        0x73 => _mm512_srli_si512::<0x33>(addition),
        0x74 => _mm512_srli_si512::<0x34>(addition),
        0x75 => _mm512_srli_si512::<0x35>(addition),
        0x76 => _mm512_srli_si512::<0x36>(addition),
        0x77 => _mm512_srli_si512::<0x37>(addition),
        0x78 => _mm512_srli_si512::<0x38>(addition),
        0x79 => _mm512_srli_si512::<0x39>(addition),
        0x7A => _mm512_srli_si512::<0x3A>(addition),
        0x7B => _mm512_srli_si512::<0x3B>(addition),
        0x7C => _mm512_srli_si512::<0x3C>(addition),
        0x7D => _mm512_srli_si512::<0x3D>(addition),
        0x7E => _mm512_srli_si512::<0x3E>(addition),
        0x7F => _mm512_srli_si512::<0x3F>(addition),
        _ => _mm512_setzero_si512()
    };
}

/// # Example (_mm512_alvext_epi16):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
/// - Addition: ```[21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40]```
///
/// - Vext'ed left by 1 pair: ```[40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f]```
/// - Vext'ed left by 2 pairs: ```[3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e]```
/// - Vext'ed left by 3 pairs: ```[3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d]```
/// - Vext'ed left by 4 pairs: ```[3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c]```
/// - Vext'ed left by 5 pairs: ```[3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b]```
/// - Vext'ed left by 6 pairs: ```[3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a]```
/// - Vext'ed left by 7 pairs: ```[3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]```
/// - Vext'ed left by 8 pairs: ```[39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18]```
/// - Vext'ed left by 9 pairs: ```[38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17]```
/// - Vext'ed left by 10 pairs: ```[37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16]```
/// - Vext'ed left by 11 pairs: ```[36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15]```
/// - Vext'ed left by 12 pairs: ```[35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14]```
/// - Vext'ed left by 13 pairs: ```[34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13]```
/// - Vext'ed left by 14 pairs: ```[33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12]```
/// - Vext'ed left by 15 pairs: ```[32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11]```
/// - Vext'ed left by 16 pairs: ```[31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Vext'ed left by 17 pairs: ```[30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f]```
/// - Vext'ed left by 18 pairs: ```[2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e]```
/// - Vext'ed left by 19 pairs: ```[2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d]```
/// - Vext'ed left by 20 pairs: ```[2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c]```
/// - Vext'ed left by 21 pairs: ```[2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b]```
/// - Vext'ed left by 22 pairs: ```[2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a]```
/// - Vext'ed left by 23 pairs: ```[2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08, 09]```
/// - Vext'ed left by 24 pairs: ```[29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07, 08]```
/// - Vext'ed left by 25 pairs: ```[28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06, 07]```
/// - Vext'ed left by 26 pairs: ```[27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05, 06]```
/// - Vext'ed left by 27 pairs: ```[26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04, 05]```
/// - Vext'ed left by 28 pairs: ```[25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03, 04]```
/// - Vext'ed left by 29 pairs: ```[24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02, 03]```
/// - Vext'ed left by 30 pairs: ```[23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01, 02]```
/// - Vext'ed left by 31 pairs: ```[22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 01]```
/// - Vext'ed left by 32 pairs: ```[21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40]```
/// - Vext'ed left by 33 pairs: ```[00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f]```
/// - Vext'ed left by 34 pairs: ```[00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e]```
/// - Vext'ed left by 35 pairs: ```[00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d]```
/// - Vext'ed left by 36 pairs: ```[00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c]```
/// - Vext'ed left by 37 pairs: ```[00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b]```
/// - Vext'ed left by 38 pairs: ```[00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a]```
/// - Vext'ed left by 39 pairs: ```[00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39]```
/// - Vext'ed left by 40 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38]```
/// - Vext'ed left by 41 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37]```
/// - Vext'ed left by 42 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36]```
/// - Vext'ed left by 43 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35]```
/// - Vext'ed left by 44 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34]```
/// - Vext'ed left by 45 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33]```
/// - Vext'ed left by 46 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32]```
/// - Vext'ed left by 47 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31]```
/// - Vext'ed left by 48 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30]```
/// - Vext'ed left by 49 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f]```
/// - Vext'ed left by 50 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e]```
/// - Vext'ed left by 51 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d]```
/// - Vext'ed left by 52 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c]```
/// - Vext'ed left by 53 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b]```
/// - Vext'ed left by 54 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a]```
/// - Vext'ed left by 55 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28, 29]```
/// - Vext'ed left by 56 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27, 28]```
/// - Vext'ed left by 57 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26, 27]```
/// - Vext'ed left by 58 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25, 26]```
/// - Vext'ed left by 59 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24, 25]```
/// - Vext'ed left by 60 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23, 24]```
/// - Vext'ed left by 61 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22, 23]```
/// - Vext'ed left by 62 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21, 22]```
/// - Vext'ed left by 63 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 21]```
/// - Vext'ed left by 64 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(target_feature = "avx512f", target_feature = "avx512bw"))]
pub unsafe fn _mm512_alvext_epi16(vector: __m512i, addition: __m512i, shift: usize) -> __m512i {
    return match shift {
        0x00 => vector,
        0x01 => _mm512_or_si512(_mm512_slli_si512::<0x02>(vector), _mm512_srli_si512::<0x3E>(addition)),
        0x02 => _mm512_or_si512(_mm512_slli_si512::<0x04>(vector), _mm512_srli_si512::<0x3C>(addition)),
        0x03 => _mm512_or_si512(_mm512_slli_si512::<0x06>(vector), _mm512_srli_si512::<0x3A>(addition)),
        0x04 => _mm512_or_si512(_mm512_slli_si512::<0x08>(vector), _mm512_srli_si512::<0x38>(addition)),
        0x05 => _mm512_or_si512(_mm512_slli_si512::<0x0A>(vector), _mm512_srli_si512::<0x36>(addition)),
        0x06 => _mm512_or_si512(_mm512_slli_si512::<0x0C>(vector), _mm512_srli_si512::<0x34>(addition)),
        0x07 => _mm512_or_si512(_mm512_slli_si512::<0x0E>(vector), _mm512_srli_si512::<0x32>(addition)),
        0x08 => _mm512_or_si512(_mm512_slli_si512::<0x10>(vector), _mm512_srli_si512::<0x30>(addition)),
        0x09 => _mm512_or_si512(_mm512_slli_si512::<0x12>(vector), _mm512_srli_si512::<0x2E>(addition)),
        0x0A => _mm512_or_si512(_mm512_slli_si512::<0x14>(vector), _mm512_srli_si512::<0x2C>(addition)),
        0x0B => _mm512_or_si512(_mm512_slli_si512::<0x16>(vector), _mm512_srli_si512::<0x2A>(addition)),
        0x0C => _mm512_or_si512(_mm512_slli_si512::<0x18>(vector), _mm512_srli_si512::<0x28>(addition)),
        0x0D => _mm512_or_si512(_mm512_slli_si512::<0x1A>(vector), _mm512_srli_si512::<0x26>(addition)),
        0x0E => _mm512_or_si512(_mm512_slli_si512::<0x1C>(vector), _mm512_srli_si512::<0x24>(addition)),
        0x0F => _mm512_or_si512(_mm512_slli_si512::<0x1E>(vector), _mm512_srli_si512::<0x22>(addition)),
        0x10 => _mm512_or_si512(_mm512_slli_si512::<0x20>(vector), _mm512_srli_si512::<0x20>(addition)),
        0x11 => _mm512_or_si512(_mm512_slli_si512::<0x22>(vector), _mm512_srli_si512::<0x1E>(addition)),
        0x12 => _mm512_or_si512(_mm512_slli_si512::<0x24>(vector), _mm512_srli_si512::<0x1C>(addition)),
        0x13 => _mm512_or_si512(_mm512_slli_si512::<0x26>(vector), _mm512_srli_si512::<0x1A>(addition)),
        0x14 => _mm512_or_si512(_mm512_slli_si512::<0x28>(vector), _mm512_srli_si512::<0x18>(addition)),
        0x15 => _mm512_or_si512(_mm512_slli_si512::<0x2A>(vector), _mm512_srli_si512::<0x16>(addition)),
        0x16 => _mm512_or_si512(_mm512_slli_si512::<0x2C>(vector), _mm512_srli_si512::<0x14>(addition)),
        0x17 => _mm512_or_si512(_mm512_slli_si512::<0x2E>(vector), _mm512_srli_si512::<0x12>(addition)),
        0x18 => _mm512_or_si512(_mm512_slli_si512::<0x30>(vector), _mm512_srli_si512::<0x10>(addition)),
        0x19 => _mm512_or_si512(_mm512_slli_si512::<0x32>(vector), _mm512_srli_si512::<0x0E>(addition)),
        0x1A => _mm512_or_si512(_mm512_slli_si512::<0x34>(vector), _mm512_srli_si512::<0x0C>(addition)),
        0x1B => _mm512_or_si512(_mm512_slli_si512::<0x36>(vector), _mm512_srli_si512::<0x0A>(addition)),
        0x1C => _mm512_or_si512(_mm512_slli_si512::<0x38>(vector), _mm512_srli_si512::<0x08>(addition)),
        0x1D => _mm512_or_si512(_mm512_slli_si512::<0x3A>(vector), _mm512_srli_si512::<0x06>(addition)),
        0x1E => _mm512_or_si512(_mm512_slli_si512::<0x3C>(vector), _mm512_srli_si512::<0x04>(addition)),
        0x1F => _mm512_or_si512(_mm512_slli_si512::<0x3E>(vector), _mm512_srli_si512::<0x02>(addition)),
        0x20 => addition,
        0x21 => _mm512_slli_si512::<0x02>(addition),
        0x22 => _mm512_slli_si512::<0x04>(addition),
        0x23 => _mm512_slli_si512::<0x06>(addition),
        0x24 => _mm512_slli_si512::<0x08>(addition),
        0x25 => _mm512_slli_si512::<0x0A>(addition),
        0x26 => _mm512_slli_si512::<0x0C>(addition),
        0x27 => _mm512_slli_si512::<0x0E>(addition),
        0x28 => _mm512_slli_si512::<0x10>(addition),
        0x29 => _mm512_slli_si512::<0x12>(addition),
        0x2A => _mm512_slli_si512::<0x14>(addition),
        0x2B => _mm512_slli_si512::<0x16>(addition),
        0x2C => _mm512_slli_si512::<0x18>(addition),
        0x2D => _mm512_slli_si512::<0x1A>(addition),
        0x2E => _mm512_slli_si512::<0x1C>(addition),
        0x2F => _mm512_slli_si512::<0x1E>(addition),
        0x30 => _mm512_slli_si512::<0x20>(addition),
        0x31 => _mm512_slli_si512::<0x22>(addition),
        0x32 => _mm512_slli_si512::<0x24>(addition),
        0x33 => _mm512_slli_si512::<0x26>(addition),
        0x34 => _mm512_slli_si512::<0x28>(addition),
        0x35 => _mm512_slli_si512::<0x2A>(addition),
        0x36 => _mm512_slli_si512::<0x2C>(addition),
        0x37 => _mm512_slli_si512::<0x2E>(addition),
        0x38 => _mm512_slli_si512::<0x30>(addition),
        0x39 => _mm512_slli_si512::<0x32>(addition),
        0x3A => _mm512_slli_si512::<0x34>(addition),
        0x3B => _mm512_slli_si512::<0x36>(addition),
        0x3C => _mm512_slli_si512::<0x38>(addition),
        0x3D => _mm512_slli_si512::<0x3A>(addition),
        0x3E => _mm512_slli_si512::<0x3C>(addition),
        0x3F => _mm512_slli_si512::<0x3E>(addition),
        _ => _mm512_setzero_si512()
    };
}

/// # Example (_mm512_arvext_epi16):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20]```
/// - Addition: ```[21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40]```
///
/// - Vext'ed right by 1 pair: ```[02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21]```
/// - Vext'ed right by 2 pairs: ```[03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22]```
/// - Vext'ed right by 3 pairs: ```[04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23]```
/// - Vext'ed right by 4 pairs: ```[05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24]```
/// - Vext'ed right by 5 pairs: ```[06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25]```
/// - Vext'ed right by 6 pairs: ```[07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26]```
/// - Vext'ed right by 7 pairs: ```[08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27]```
/// - Vext'ed right by 8 pairs: ```[09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28]```
/// - Vext'ed right by 9 pairs: ```[0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]```
/// - Vext'ed right by 10 pairs: ```[0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a]```
/// - Vext'ed right by 11 pairs: ```[0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b]```
/// - Vext'ed right by 12 pairs: ```[0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c]```
/// - Vext'ed right by 13 pairs: ```[0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d]```
/// - Vext'ed right by 14 pairs: ```[0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e]```
/// - Vext'ed right by 15 pairs: ```[10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f]```
/// - Vext'ed right by 16 pairs: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30]```
/// - Vext'ed right by 17 pairs: ```[12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31]```
/// - Vext'ed right by 18 pairs: ```[13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32]```
/// - Vext'ed right by 19 pairs: ```[14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33]```
/// - Vext'ed right by 20 pairs: ```[15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34]```
/// - Vext'ed right by 21 pairs: ```[16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35]```
/// - Vext'ed right by 22 pairs: ```[17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36]```
/// - Vext'ed right by 23 pairs: ```[18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37]```
/// - Vext'ed right by 24 pairs: ```[19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38]```
/// - Vext'ed right by 25 pairs: ```[1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39]```
/// - Vext'ed right by 26 pairs: ```[1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a]```
/// - Vext'ed right by 27 pairs: ```[1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b]```
/// - Vext'ed right by 28 pairs: ```[1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c]```
/// - Vext'ed right by 29 pairs: ```[1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d]```
/// - Vext'ed right by 30 pairs: ```[1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e]```
/// - Vext'ed right by 31 pairs: ```[20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f]```
/// - Vext'ed right by 32 pairs: ```[21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40]```
/// - Vext'ed right by 33 pairs: ```[22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00]```
/// - Vext'ed right by 34 pairs: ```[23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00]```
/// - Vext'ed right by 35 pairs: ```[24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00]```
/// - Vext'ed right by 36 pairs: ```[25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00]```
/// - Vext'ed right by 37 pairs: ```[26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 38 pairs: ```[27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 39 pairs: ```[28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 40 pairs: ```[29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 41 pairs: ```[2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 42 pairs: ```[2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 43 pairs: ```[2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 44 pairs: ```[2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 45 pairs: ```[2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 46 pairs: ```[2f, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 47 pairs: ```[30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 48 pairs: ```[31, 32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 49 pairs: ```[32, 33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 50 pairs: ```[33, 34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 51 pairs: ```[34, 35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 52 pairs: ```[35, 36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 53 pairs: ```[36, 37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 54 pairs: ```[37, 38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 55 pairs: ```[38, 39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 56 pairs: ```[39, 3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 57 pairs: ```[3a, 3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 58 pairs: ```[3b, 3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 59 pairs: ```[3c, 3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 60 pairs: ```[3d, 3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 61 pairs: ```[3e, 3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 62 pairs: ```[3f, 40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 63 pairs: ```[40, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 64 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(target_feature = "avx512f", target_feature = "avx512bw"))]
pub unsafe fn _mm512_arvext_epi16(vector: __m512i, addition: __m512i, shift: usize) -> __m512i {
    return match shift {
        0x00 => vector,
        0x01 => _mm512_or_si512(_mm512_srli_si512::<0x02>(vector), _mm512_slli_si512::<0x3E>(addition)),
        0x02 => _mm512_or_si512(_mm512_srli_si512::<0x04>(vector), _mm512_slli_si512::<0x3C>(addition)),
        0x03 => _mm512_or_si512(_mm512_srli_si512::<0x06>(vector), _mm512_slli_si512::<0x3A>(addition)),
        0x04 => _mm512_or_si512(_mm512_srli_si512::<0x08>(vector), _mm512_slli_si512::<0x38>(addition)),
        0x05 => _mm512_or_si512(_mm512_srli_si512::<0x0A>(vector), _mm512_slli_si512::<0x36>(addition)),
        0x06 => _mm512_or_si512(_mm512_srli_si512::<0x0C>(vector), _mm512_slli_si512::<0x34>(addition)),
        0x07 => _mm512_or_si512(_mm512_srli_si512::<0x0E>(vector), _mm512_slli_si512::<0x32>(addition)),
        0x08 => _mm512_or_si512(_mm512_srli_si512::<0x10>(vector), _mm512_slli_si512::<0x30>(addition)),
        0x09 => _mm512_or_si512(_mm512_srli_si512::<0x12>(vector), _mm512_slli_si512::<0x2E>(addition)),
        0x0A => _mm512_or_si512(_mm512_srli_si512::<0x14>(vector), _mm512_slli_si512::<0x2C>(addition)),
        0x0B => _mm512_or_si512(_mm512_srli_si512::<0x16>(vector), _mm512_slli_si512::<0x2A>(addition)),
        0x0C => _mm512_or_si512(_mm512_srli_si512::<0x18>(vector), _mm512_slli_si512::<0x28>(addition)),
        0x0D => _mm512_or_si512(_mm512_srli_si512::<0x1A>(vector), _mm512_slli_si512::<0x26>(addition)),
        0x0E => _mm512_or_si512(_mm512_srli_si512::<0x1C>(vector), _mm512_slli_si512::<0x24>(addition)),
        0x0F => _mm512_or_si512(_mm512_srli_si512::<0x1E>(vector), _mm512_slli_si512::<0x22>(addition)),
        0x10 => _mm512_or_si512(_mm512_srli_si512::<0x20>(vector), _mm512_slli_si512::<0x20>(addition)),
        0x11 => _mm512_or_si512(_mm512_srli_si512::<0x22>(vector), _mm512_slli_si512::<0x1E>(addition)),
        0x12 => _mm512_or_si512(_mm512_srli_si512::<0x24>(vector), _mm512_slli_si512::<0x1C>(addition)),
        0x13 => _mm512_or_si512(_mm512_srli_si512::<0x26>(vector), _mm512_slli_si512::<0x1A>(addition)),
        0x14 => _mm512_or_si512(_mm512_srli_si512::<0x28>(vector), _mm512_slli_si512::<0x18>(addition)),
        0x15 => _mm512_or_si512(_mm512_srli_si512::<0x2A>(vector), _mm512_slli_si512::<0x16>(addition)),
        0x16 => _mm512_or_si512(_mm512_srli_si512::<0x2C>(vector), _mm512_slli_si512::<0x14>(addition)),
        0x17 => _mm512_or_si512(_mm512_srli_si512::<0x2E>(vector), _mm512_slli_si512::<0x12>(addition)),
        0x18 => _mm512_or_si512(_mm512_srli_si512::<0x30>(vector), _mm512_slli_si512::<0x10>(addition)),
        0x19 => _mm512_or_si512(_mm512_srli_si512::<0x32>(vector), _mm512_slli_si512::<0x0E>(addition)),
        0x1A => _mm512_or_si512(_mm512_srli_si512::<0x34>(vector), _mm512_slli_si512::<0x0C>(addition)),
        0x1B => _mm512_or_si512(_mm512_srli_si512::<0x36>(vector), _mm512_slli_si512::<0x0A>(addition)),
        0x1C => _mm512_or_si512(_mm512_srli_si512::<0x38>(vector), _mm512_slli_si512::<0x08>(addition)),
        0x1D => _mm512_or_si512(_mm512_srli_si512::<0x3A>(vector), _mm512_slli_si512::<0x06>(addition)),
        0x1E => _mm512_or_si512(_mm512_srli_si512::<0x3C>(vector), _mm512_slli_si512::<0x04>(addition)),
        0x1F => _mm512_or_si512(_mm512_srli_si512::<0x3E>(vector), _mm512_slli_si512::<0x02>(addition)),
        0x20 => addition,
        0x21 => _mm512_srli_si512::<0x02>(addition),
        0x22 => _mm512_srli_si512::<0x04>(addition),
        0x23 => _mm512_srli_si512::<0x06>(addition),
        0x24 => _mm512_srli_si512::<0x08>(addition),
        0x25 => _mm512_srli_si512::<0x0A>(addition),
        0x26 => _mm512_srli_si512::<0x0C>(addition),
        0x27 => _mm512_srli_si512::<0x0E>(addition),
        0x28 => _mm512_srli_si512::<0x10>(addition),
        0x29 => _mm512_srli_si512::<0x12>(addition),
        0x2A => _mm512_srli_si512::<0x14>(addition),
        0x2B => _mm512_srli_si512::<0x16>(addition),
        0x2C => _mm512_srli_si512::<0x18>(addition),
        0x2D => _mm512_srli_si512::<0x1A>(addition),
        0x2E => _mm512_srli_si512::<0x1C>(addition),
        0x2F => _mm512_srli_si512::<0x1E>(addition),
        0x30 => _mm512_srli_si512::<0x20>(addition),
        0x31 => _mm512_srli_si512::<0x22>(addition),
        0x32 => _mm512_srli_si512::<0x24>(addition),
        0x33 => _mm512_srli_si512::<0x26>(addition),
        0x34 => _mm512_srli_si512::<0x28>(addition),
        0x35 => _mm512_srli_si512::<0x2A>(addition),
        0x36 => _mm512_srli_si512::<0x2C>(addition),
        0x37 => _mm512_srli_si512::<0x2E>(addition),
        0x38 => _mm512_srli_si512::<0x30>(addition),
        0x39 => _mm512_srli_si512::<0x32>(addition),
        0x3A => _mm512_srli_si512::<0x34>(addition),
        0x3B => _mm512_srli_si512::<0x36>(addition),
        0x3C => _mm512_srli_si512::<0x38>(addition),
        0x3D => _mm512_srli_si512::<0x3A>(addition),
        0x3E => _mm512_srli_si512::<0x3C>(addition),
        0x3F => _mm512_srli_si512::<0x3E>(addition),
        _ => _mm512_setzero_si512()
    };
}

/// # Example (_mm512_alvext_epi32):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Addition: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26]```
///
/// - Vext'ed left by 1 pair: ```[26, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f]```
/// - Vext'ed left by 2 pairs: ```[25, 26, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e]```
/// - Vext'ed left by 3 pairs: ```[24, 25, 26, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d]```
/// - Vext'ed left by 4 pairs: ```[23, 24, 25, 26, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c]```
/// - Vext'ed left by 5 pairs: ```[22, 23, 24, 25, 26, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b]```
/// - Vext'ed left by 6 pairs: ```[21, 22, 23, 24, 25, 26, 01, 02, 03, 04, 05, 06, 07, 08, 09, 0a]```
/// - Vext'ed left by 7 pairs: ```[20, 21, 22, 23, 24, 25, 26, 01, 02, 03, 04, 05, 06, 07, 08, 09]```
/// - Vext'ed left by 8 pairs: ```[19, 20, 21, 22, 23, 24, 25, 26, 01, 02, 03, 04, 05, 06, 07, 08]```
/// - Vext'ed left by 9 pairs: ```[18, 19, 20, 21, 22, 23, 24, 25, 26, 01, 02, 03, 04, 05, 06, 07]```
/// - Vext'ed left by 10 pairs: ```[17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 01, 02, 03, 04, 05, 06]```
/// - Vext'ed left by 11 pairs: ```[16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 01, 02, 03, 04, 05]```
/// - Vext'ed left by 12 pairs: ```[15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 01, 02, 03, 04]```
/// - Vext'ed left by 13 pairs: ```[14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 01, 02, 03]```
/// - Vext'ed left by 14 pairs: ```[13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 01, 02]```
/// - Vext'ed left by 15 pairs: ```[12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 01]```
/// - Vext'ed left by 16 pairs: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26]```
/// - Vext'ed left by 17 pairs: ```[00, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]```
/// - Vext'ed left by 18 pairs: ```[00, 00, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]```
/// - Vext'ed left by 19 pairs: ```[00, 00, 00, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]```
/// - Vext'ed left by 20 pairs: ```[00, 00, 00, 00, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22]```
/// - Vext'ed left by 21 pairs: ```[00, 00, 00, 00, 00, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]```
/// - Vext'ed left by 22 pairs: ```[00, 00, 00, 00, 00, 00, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]```
/// - Vext'ed left by 23 pairs: ```[00, 00, 00, 00, 00, 00, 00, 11, 12, 13, 14, 15, 16, 17, 18, 19]```
/// - Vext'ed left by 24 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 11, 12, 13, 14, 15, 16, 17, 18]```
/// - Vext'ed left by 25 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 11, 12, 13, 14, 15, 16, 17]```
/// - Vext'ed left by 26 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 11, 12, 13, 14, 15, 16]```
/// - Vext'ed left by 27 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 11, 12, 13, 14, 15]```
/// - Vext'ed left by 28 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 11, 12, 13, 14]```
/// - Vext'ed left by 29 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 11, 12, 13]```
/// - Vext'ed left by 30 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 11, 12]```
/// - Vext'ed left by 31 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 11]```
/// - Vext'ed left by 32 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(target_feature = "avx512f", target_feature = "avx512bw"))]
pub unsafe fn _mm512_alvext_epi32(vector: __m512i, addition: __m512i, shift: usize) -> __m512i {
    return match shift {
        0x00 => vector,
        0x01 => _mm512_or_si512(_mm512_slli_si512::<0x04>(vector), _mm512_srli_si512::<0x3C>(addition)),
        0x02 => _mm512_or_si512(_mm512_slli_si512::<0x08>(vector), _mm512_srli_si512::<0x38>(addition)),
        0x03 => _mm512_or_si512(_mm512_slli_si512::<0x0C>(vector), _mm512_srli_si512::<0x34>(addition)),
        0x04 => _mm512_or_si512(_mm512_slli_si512::<0x10>(vector), _mm512_srli_si512::<0x30>(addition)),
        0x05 => _mm512_or_si512(_mm512_slli_si512::<0x14>(vector), _mm512_srli_si512::<0x2C>(addition)),
        0x06 => _mm512_or_si512(_mm512_slli_si512::<0x18>(vector), _mm512_srli_si512::<0x28>(addition)),
        0x07 => _mm512_or_si512(_mm512_slli_si512::<0x1C>(vector), _mm512_srli_si512::<0x24>(addition)),
        0x08 => _mm512_or_si512(_mm512_slli_si512::<0x20>(vector), _mm512_srli_si512::<0x20>(addition)),
        0x09 => _mm512_or_si512(_mm512_slli_si512::<0x24>(vector), _mm512_srli_si512::<0x1C>(addition)),
        0x0A => _mm512_or_si512(_mm512_slli_si512::<0x28>(vector), _mm512_srli_si512::<0x18>(addition)),
        0x0B => _mm512_or_si512(_mm512_slli_si512::<0x2C>(vector), _mm512_srli_si512::<0x14>(addition)),
        0x0C => _mm512_or_si512(_mm512_slli_si512::<0x30>(vector), _mm512_srli_si512::<0x10>(addition)),
        0x0D => _mm512_or_si512(_mm512_slli_si512::<0x34>(vector), _mm512_srli_si512::<0x0C>(addition)),
        0x0E => _mm512_or_si512(_mm512_slli_si512::<0x38>(vector), _mm512_srli_si512::<0x08>(addition)),
        0x0F => _mm512_or_si512(_mm512_slli_si512::<0x3C>(vector), _mm512_srli_si512::<0x04>(addition)),
        0x10 => addition,
        0x11 => _mm512_slli_si512::<0x04>(addition),
        0x12 => _mm512_slli_si512::<0x08>(addition),
        0x13 => _mm512_slli_si512::<0x0C>(addition),
        0x14 => _mm512_slli_si512::<0x10>(addition),
        0x15 => _mm512_slli_si512::<0x14>(addition),
        0x16 => _mm512_slli_si512::<0x18>(addition),
        0x17 => _mm512_slli_si512::<0x1C>(addition),
        0x18 => _mm512_slli_si512::<0x20>(addition),
        0x19 => _mm512_slli_si512::<0x24>(addition),
        0x1A => _mm512_slli_si512::<0x28>(addition),
        0x1B => _mm512_slli_si512::<0x2C>(addition),
        0x1C => _mm512_slli_si512::<0x30>(addition),
        0x1D => _mm512_slli_si512::<0x34>(addition),
        0x1E => _mm512_slli_si512::<0x38>(addition),
        0x1F => _mm512_slli_si512::<0x3C>(addition),
        _ => _mm512_setzero_si512()
    };
}

/// # Example (_mm512_arvext_epi32):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10]```
/// - Addition: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26]```
///
/// - Vext'ed right by 1 pair: ```[02, 03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11]```
/// - Vext'ed right by 2 pairs: ```[03, 04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12]```
/// - Vext'ed right by 3 pairs: ```[04, 05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13]```
/// - Vext'ed right by 4 pairs: ```[05, 06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14]```
/// - Vext'ed right by 5 pairs: ```[06, 07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15]```
/// - Vext'ed right by 6 pairs: ```[07, 08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16]```
/// - Vext'ed right by 7 pairs: ```[08, 09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17]```
/// - Vext'ed right by 8 pairs: ```[09, 0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18]```
/// - Vext'ed right by 9 pairs: ```[0a, 0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]```
/// - Vext'ed right by 10 pairs: ```[0b, 0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]```
/// - Vext'ed right by 11 pairs: ```[0c, 0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]```
/// - Vext'ed right by 12 pairs: ```[0d, 0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22]```
/// - Vext'ed right by 13 pairs: ```[0e, 0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]```
/// - Vext'ed right by 14 pairs: ```[0f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]```
/// - Vext'ed right by 15 pairs: ```[10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]```
/// - Vext'ed right by 16 pairs: ```[11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26]```
/// - Vext'ed right by 17 pairs: ```[12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 00]```
/// - Vext'ed right by 18 pairs: ```[13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 00, 00]```
/// - Vext'ed right by 19 pairs: ```[14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 00, 00, 00]```
/// - Vext'ed right by 20 pairs: ```[15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 00, 00, 00, 00]```
/// - Vext'ed right by 21 pairs: ```[16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 22 pairs: ```[17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 23 pairs: ```[18, 19, 20, 21, 22, 23, 24, 25, 26, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 24 pairs: ```[19, 20, 21, 22, 23, 24, 25, 26, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 25 pairs: ```[20, 21, 22, 23, 24, 25, 26, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 26 pairs: ```[21, 22, 23, 24, 25, 26, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 27 pairs: ```[22, 23, 24, 25, 26, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 28 pairs: ```[23, 24, 25, 26, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 29 pairs: ```[24, 25, 26, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 30 pairs: ```[25, 26, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 31 pairs: ```[26, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 32 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(target_feature = "avx512f", target_feature = "avx512bw"))]
pub unsafe fn _mm512_arvext_epi32(vector: __m512i, addition: __m512i, shift: usize) -> __m512i {
    return match shift {
        0x00 => vector,
        0x01 => _mm512_or_si512(_mm512_srli_si512::<0x04>(vector), _mm512_slli_si512::<0x3C>(addition)),
        0x02 => _mm512_or_si512(_mm512_srli_si512::<0x08>(vector), _mm512_slli_si512::<0x38>(addition)),
        0x03 => _mm512_or_si512(_mm512_srli_si512::<0x0C>(vector), _mm512_slli_si512::<0x34>(addition)),
        0x04 => _mm512_or_si512(_mm512_srli_si512::<0x10>(vector), _mm512_slli_si512::<0x30>(addition)),
        0x05 => _mm512_or_si512(_mm512_srli_si512::<0x14>(vector), _mm512_slli_si512::<0x2C>(addition)),
        0x06 => _mm512_or_si512(_mm512_srli_si512::<0x18>(vector), _mm512_slli_si512::<0x28>(addition)),
        0x07 => _mm512_or_si512(_mm512_srli_si512::<0x1C>(vector), _mm512_slli_si512::<0x24>(addition)),
        0x08 => _mm512_or_si512(_mm512_srli_si512::<0x20>(vector), _mm512_slli_si512::<0x20>(addition)),
        0x09 => _mm512_or_si512(_mm512_srli_si512::<0x24>(vector), _mm512_slli_si512::<0x1C>(addition)),
        0x0A => _mm512_or_si512(_mm512_srli_si512::<0x28>(vector), _mm512_slli_si512::<0x18>(addition)),
        0x0B => _mm512_or_si512(_mm512_srli_si512::<0x2C>(vector), _mm512_slli_si512::<0x14>(addition)),
        0x0C => _mm512_or_si512(_mm512_srli_si512::<0x30>(vector), _mm512_slli_si512::<0x10>(addition)),
        0x0D => _mm512_or_si512(_mm512_srli_si512::<0x34>(vector), _mm512_slli_si512::<0x0C>(addition)),
        0x0E => _mm512_or_si512(_mm512_srli_si512::<0x38>(vector), _mm512_slli_si512::<0x08>(addition)),
        0x0F => _mm512_or_si512(_mm512_srli_si512::<0x3C>(vector), _mm512_slli_si512::<0x04>(addition)),
        0x10 => addition,
        0x11 => _mm512_srli_si512::<0x04>(addition),
        0x12 => _mm512_srli_si512::<0x08>(addition),
        0x13 => _mm512_srli_si512::<0x0C>(addition),
        0x14 => _mm512_srli_si512::<0x10>(addition),
        0x15 => _mm512_srli_si512::<0x14>(addition),
        0x16 => _mm512_srli_si512::<0x18>(addition),
        0x17 => _mm512_srli_si512::<0x1C>(addition),
        0x18 => _mm512_srli_si512::<0x20>(addition),
        0x19 => _mm512_srli_si512::<0x24>(addition),
        0x1A => _mm512_srli_si512::<0x28>(addition),
        0x1B => _mm512_srli_si512::<0x2C>(addition),
        0x1C => _mm512_srli_si512::<0x30>(addition),
        0x1D => _mm512_srli_si512::<0x34>(addition),
        0x1E => _mm512_srli_si512::<0x38>(addition),
        0x1F => _mm512_srli_si512::<0x3C>(addition),
        _ => _mm512_setzero_si512()
    };
}

/// # Example (_mm512_alvext_epi64):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08]```
/// - Addition: ```[09, 10, 11, 12, 13, 14, 15, 16]```
///
/// - Vext'ed left by 1 pair: ```[16, 01, 02, 03, 04, 05, 06, 07]```
/// - Vext'ed left by 2 pairs: ```[15, 16, 01, 02, 03, 04, 05, 06]```
/// - Vext'ed left by 3 pairs: ```[14, 15, 16, 01, 02, 03, 04, 05]```
/// - Vext'ed left by 4 pairs: ```[13, 14, 15, 16, 01, 02, 03, 04]```
/// - Vext'ed left by 5 pairs: ```[12, 13, 14, 15, 16, 01, 02, 03]```
/// - Vext'ed left by 6 pairs: ```[11, 12, 13, 14, 15, 16, 01, 02]```
/// - Vext'ed left by 7 pairs: ```[10, 11, 12, 13, 14, 15, 16, 01]```
/// - Vext'ed left by 8 pairs: ```[09, 10, 11, 12, 13, 14, 15, 16]```
/// - Vext'ed left by 9 pairs: ```[00, 09, 10, 11, 12, 13, 14, 15]```
/// - Vext'ed left by 10 pairs: ```[00, 00, 09, 10, 11, 12, 13, 14]```
/// - Vext'ed left by 11 pairs: ```[00, 00, 00, 09, 10, 11, 12, 13]```
/// - Vext'ed left by 12 pairs: ```[00, 00, 00, 00, 09, 10, 11, 12]```
/// - Vext'ed left by 13 pairs: ```[00, 00, 00, 00, 00, 09, 10, 11]```
/// - Vext'ed left by 14 pairs: ```[00, 00, 00, 00, 00, 00, 09, 10]```
/// - Vext'ed left by 15 pairs: ```[00, 00, 00, 00, 00, 00, 00, 09]```
/// - Vext'ed left by 16 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(target_feature = "avx512f", target_feature = "avx512bw"))]
pub unsafe fn _mm512_alvext_epi64(vector: __m512i, addition: __m512i, shift: usize) -> __m512i {
    return match shift {
        0x00 => vector,
        0x01 => _mm512_or_si512(_mm512_slli_si512::<0x08>(vector), _mm512_srli_si512::<0x38>(addition)),
        0x02 => _mm512_or_si512(_mm512_slli_si512::<0x10>(vector), _mm512_srli_si512::<0x30>(addition)),
        0x03 => _mm512_or_si512(_mm512_slli_si512::<0x18>(vector), _mm512_srli_si512::<0x28>(addition)),
        0x04 => _mm512_or_si512(_mm512_slli_si512::<0x20>(vector), _mm512_srli_si512::<0x20>(addition)),
        0x05 => _mm512_or_si512(_mm512_slli_si512::<0x28>(vector), _mm512_srli_si512::<0x18>(addition)),
        0x06 => _mm512_or_si512(_mm512_slli_si512::<0x30>(vector), _mm512_srli_si512::<0x10>(addition)),
        0x07 => _mm512_or_si512(_mm512_slli_si512::<0x38>(vector), _mm512_srli_si512::<0x08>(addition)),
        0x08 => addition,
        0x09 => _mm512_slli_si512::<0x08>(addition),
        0x0A => _mm512_slli_si512::<0x10>(addition),
        0x0B => _mm512_slli_si512::<0x18>(addition),
        0x0C => _mm512_slli_si512::<0x20>(addition),
        0x0D => _mm512_slli_si512::<0x28>(addition),
        0x0E => _mm512_slli_si512::<0x30>(addition),
        0x0F => _mm512_slli_si512::<0x38>(addition),
        _ => _mm512_setzero_si512()
    };
}

/// # Example (_mm512_arvext_epi64):
///
/// - Source: ```[01, 02, 03, 04, 05, 06, 07, 08]```
/// - Addition: ```[09, 10, 11, 12, 13, 14, 15, 16]```
///
/// - Vext'ed right by 1 pair: ```[02, 03, 04, 05, 06, 07, 08, 09]```
/// - Vext'ed right by 2 pairs: ```[03, 04, 05, 06, 07, 08, 09, 10]```
/// - Vext'ed right by 3 pairs: ```[04, 05, 06, 07, 08, 09, 10, 11]```
/// - Vext'ed right by 4 pairs: ```[05, 06, 07, 08, 09, 10, 11, 12]```
/// - Vext'ed right by 5 pairs: ```[06, 07, 08, 09, 10, 11, 12, 13]```
/// - Vext'ed right by 6 pairs: ```[07, 08, 09, 10, 11, 12, 13, 14]```
/// - Vext'ed right by 7 pairs: ```[08, 09, 10, 11, 12, 13, 14, 15]```
/// - Vext'ed right by 8 pairs: ```[09, 10, 11, 12, 13, 14, 15, 16]```
/// - Vext'ed right by 9 pairs: ```[10, 11, 12, 13, 14, 15, 16, 00]```
/// - Vext'ed right by 10 pairs: ```[11, 12, 13, 14, 15, 16, 00, 00]```
/// - Vext'ed right by 11 pairs: ```[12, 13, 14, 15, 16, 00, 00, 00]```
/// - Vext'ed right by 12 pairs: ```[13, 14, 15, 16, 00, 00, 00, 00]```
/// - Vext'ed right by 13 pairs: ```[15, 16, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 14 pairs: ```[14, 15, 16, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 15 pairs: ```[16, 00, 00, 00, 00, 00, 00, 00]```
/// - Vext'ed right by 16 pairs: ```[00, 00, 00, 00, 00, 00, 00, 00]```
#[cfg(all(target_feature = "avx512f", target_feature = "avx512bw"))]
pub unsafe fn _mm512_arvext_epi64(vector: __m512i, addition: __m512i, shift: usize) -> __m512i {
    return match shift {
        0x00 => vector,
        0x01 => _mm512_or_si512(_mm512_srli_si512::<0x08>(vector), _mm512_slli_si512::<0x38>(addition)),
        0x02 => _mm512_or_si512(_mm512_srli_si512::<0x10>(vector), _mm512_slli_si512::<0x30>(addition)),
        0x03 => _mm512_or_si512(_mm512_srli_si512::<0x18>(vector), _mm512_slli_si512::<0x28>(addition)),
        0x04 => _mm512_or_si512(_mm512_srli_si512::<0x20>(vector), _mm512_slli_si512::<0x20>(addition)),
        0x05 => _mm512_or_si512(_mm512_srli_si512::<0x28>(vector), _mm512_slli_si512::<0x18>(addition)),
        0x06 => _mm512_or_si512(_mm512_srli_si512::<0x30>(vector), _mm512_slli_si512::<0x10>(addition)),
        0x07 => _mm512_or_si512(_mm512_srli_si512::<0x38>(vector), _mm512_slli_si512::<0x08>(addition)),
        0x08 => addition,
        0x09 => _mm512_srli_si512::<0x08>(addition),
        0x0A => _mm512_srli_si512::<0x10>(addition),
        0x0B => _mm512_srli_si512::<0x18>(addition),
        0x0C => _mm512_srli_si512::<0x20>(addition),
        0x0E => _mm512_srli_si512::<0x28>(addition),
        0x0D => _mm512_srli_si512::<0x30>(addition),
        0x0F => _mm512_srli_si512::<0x38>(addition),
        _ => _mm512_setzero_si512()
    };
}

/// # Example (_mm512_alvext_epi128):
///
/// - Source: ```[01, 02, 03, 04]```
/// - Addition: ```[05, 06, 07, 08]```
///
/// - Vext'ed left by 1 pair: ```[08, 01, 02, 03]```
/// - Vext'ed left by 2 pairs: ```[07, 08, 01, 02]```
/// - Vext'ed left by 3 pairs: ```[06, 07, 08, 01]```
/// - Vext'ed left by 4 pairs: ```[05, 06, 07, 08]```
/// - Vext'ed left by 5 pairs: ```[00, 05, 06, 07]```
/// - Vext'ed left by 6 pairs: ```[00, 00, 05, 06]```
/// - Vext'ed left by 7 pairs: ```[00, 00, 00, 05]```
/// - Vext'ed left by 8 pairs: ```[00, 00, 00, 00]```
#[cfg(all(target_feature = "avx512f", target_feature = "avx512bw"))]
pub unsafe fn _mm512_alvext_epi128(vector: __m512i, addition: __m512i, shift: usize) -> __m512i {
    return match shift {
        0x00 => vector,
        0x01 => _mm512_or_si512(_mm512_slli_si512::<0x10>(vector), _mm512_srli_si512::<0x30>(addition)),
        0x02 => _mm512_or_si512(_mm512_slli_si512::<0x20>(vector), _mm512_srli_si512::<0x20>(addition)),
        0x03 => _mm512_or_si512(_mm512_slli_si512::<0x30>(vector), _mm512_srli_si512::<0x10>(addition)),
        0x04 => addition,
        0x05 => _mm512_slli_si512::<0x10>(addition),
        0x06 => _mm512_slli_si512::<0x20>(addition),
        0x07 => _mm512_slli_si512::<0x30>(addition),
        _ => _mm512_setzero_si512()
    };
}

/// # Example (_mm512_arvext_epi128):
///
/// - Source: ```[01, 02, 03, 04]```
/// - Addition: ```[05, 06, 07, 08]```
///
/// - Vext'ed right by 1 pair: ```[02, 03, 04, 05]```
/// - Vext'ed right by 2 pairs: ```[03, 04, 05, 06]```
/// - Vext'ed right by 3 pairs: ```[04, 05, 06, 07]```
/// - Vext'ed right by 4 pairs: ```[05, 06, 07, 08]```
/// - Vext'ed right by 5 pairs: ```[06, 07, 08, 00]```
/// - Vext'ed right by 6 pairs: ```[07, 08, 00, 00]```
/// - Vext'ed right by 7 pairs: ```[08, 00, 00, 00]```
/// - Vext'ed right by 8 pairs: ```[00, 00, 00, 00]```
#[cfg(all(target_feature = "avx512f", target_feature = "avx512bw"))]
pub unsafe fn _mm512_arvext_epi128(vector: __m512i, addition: __m512i, shift: usize) -> __m512i {
    return match shift {
        0x00 => vector,
        0x01 => _mm512_or_si512(_mm512_srli_si512::<0x10>(vector), _mm512_slli_si512::<0x30>(addition)),
        0x02 => _mm512_or_si512(_mm512_srli_si512::<0x20>(vector), _mm512_slli_si512::<0x20>(addition)),
        0x03 => _mm512_or_si512(_mm512_srli_si512::<0x30>(vector), _mm512_slli_si512::<0x10>(addition)),
        0x04 => addition,
        0x05 => _mm512_srli_si512::<0x10>(addition),
        0x06 => _mm512_srli_si512::<0x20>(addition),
        0x07 => _mm512_srli_si512::<0x30>(addition),
        _ => _mm512_setzero_si512()
    };
}

/// # Example (_mm512_alvext_epi256):
///
/// - Source: ```[01, 02]```
/// - Addition: ```[03, 04]```
///
/// - Vext'ed left by 1 pair: ```[04, 01]```
/// - Vext'ed left by 2 pairs: ```[03, 04]```
/// - Vext'ed left by 3 pairs: ```[00, 03]```
/// - Vext'ed left by 4 pairs: ```[00, 00]```
#[cfg(all(target_feature = "avx512f", target_feature = "avx512bw"))]
pub unsafe fn _mm512_alvext_epi256(vector: __m512i, addition: __m512i, shift: usize) -> __m512i {
    return match shift {
        0x00 => vector,
        0x01 => _mm512_or_si512(_mm512_slli_si512::<0x20>(vector), _mm512_srli_si512::<0x20>(addition)),
        0x02 => addition,
        0x03 => _mm512_slli_si512::<0x20>(addition),
        _ => _mm512_setzero_si512()
    };
}

/// # Example (_mm512_arvext_epi256):
///
/// - Source: ```[01, 02]```
/// - Addition: ```[03, 04]
///
/// - Vext'ed right by 1 pair: ```[02, 03]```
/// - Vext'ed right by 2 pairs: ```[03, 04]```
/// - Vext'ed right by 3 pairs: ```[04, 00]```
/// - Vext'ed right by 4 pairs: ```[00, 00]```
#[cfg(all(target_feature = "avx512f", target_feature = "avx512bw"))]
pub unsafe fn _mm512_arvext_epi256(vector: __m512i, addition: __m512i, shift: usize) -> __m512i {
    return match shift {
        0x00 => vector,
        0x01 => _mm512_or_si512(_mm512_srli_si512::<0x20>(vector), _mm512_slli_si512::<0x20>(addition)),
        0x02 => addition,
        0x03 => _mm512_srli_si512::<0x20>(addition),
        _ => _mm512_setzero_si512()
    };
}