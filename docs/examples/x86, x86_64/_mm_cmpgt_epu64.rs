/// Compares two __m128i vectors containing two 64-bit unsigned integers each, determining if a > b.
/// Returns a __m128i vector where each 64-bit lane is set to 0xFFFFFFFFFFFFFFFF if a > b, or
/// 0x0000000000000000 otherwise. The most significant bit (MSB, bit 63) contributes 2^63 to the
/// magnitude of the unsigned integer.
///
/// Example:
///   a = [0x8000000000000000, 0x0000000000000005] (lane 0: 2^63, lane 1: 5)
///   b = [0x7FFFFFFFFFFFFFFF, 0x0000000000000003] (lane 0: 2^63-1, lane 1: 3)
///   Result = [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF] (lane 0: 2^63 > 2^63-1, lane 1: 5 > 3)
pub unsafe fn _mm_cmpgt_epu64(a: __m128i, b: __m128i) -> __m128i {
    // Step 1: Identify lanes with differing MSBs
    // Compute diff_signs = (b XOR a) & MSB_MASK to detect where the MSBs of a and b differ.
    // The MSB (bit 63) adds 2^63 to the unsigned value when set. The XOR operation yields 1
    // for differing MSBs (0 XOR 1 or 1 XOR 0) and 0 for matching MSBs (0 XOR 0 or 1 XOR 1).
    // The mask 0x8000000000000000 isolates the MSB, setting diff_signs[MSB]=1 in lanes where
    // MSBs differ, and 0 where they match. This distinguishes lanes requiring magnitude-based
    // handling (Step 4) from those handled by subtraction (Step 3).
    // Example:
    //   a = 0x8000000000000000 (2^63, MSB=1), b = 0x7FFFFFFFFFFFFFFF (2^63-1, MSB=0)
    //   XOR = 0x8000000000000000 ^ 0x7FFFFFFFFFFFFFFF = 0xFFFFFFFFFFFFFFFF
    //   diff_signs = 0xFFFFFFFFFFFFFFFF & 0x8000000000000000 = 0x8000000000000000
    //   For a = 0x0000000000000005 (5, MSB=0), b = 0x0000000000000003 (3, MSB=0):
    //   XOR = 0x0000000000000006, diff_signs = 0x0000000000000000 (matching MSBs)
    let diff_signs = _mm_and_si128(
        _mm_xor_si128(b, a),
        _mm_set1_epi64x(-0x8000000000000000)
    );

    // Step 2: Compute subtraction MSB
    // Compute sub_signs = (b - a) & MSB_MASK to check the MSB of the subtraction result.
    // For unsigned comparison, if b < a, the subtraction b - a overflows, producing a negative
    // result with MSB=1, indicating a > b. The mask isolates the MSB of the result.
    // Example:
    //   a = 0x0000000000000005 (5), b = 0x0000000000000003 (3)
    //   b - a = 3 - 5 = 0xFFFFFFFFFFFFFFFF - 0x0000000000000002 = 0xFFFFFFFFFFFFFFFE
    //   sub_signs = 0xFFFFFFFFFFFFFFFE & 0x8000000000000000 = 0x8000000000000000
    let sub_signs = _mm_and_si128(
        _mm_sub_epi64(b, a),
        _mm_set1_epi64x(-0x8000000000000000)
    );

    // Step 3: Filter subtraction results for matching MSBs
    // Compute r = ~diff_signs & sub_signs to retain subtraction results only in lanes where
    // a and b have matching MSBs (diff_signs[MSB]=0, from 0 XOR 0 or 1 XOR 1). When MSBs
    // match, subtraction reliably indicates if a > b (sub_signs[MSB]=1 if b < a). The
    // ~diff_signs term is 1 in lanes with matching MSBs, preserving sub_signs[MSB], and 0
    // in lanes with differing MSBs, deferring those to Step 4.
    // Example:
    //   a = 0x0000000000000005 (5, MSB=0), b = 0x0000000000000003 (3, MSB=0)
    //   diff_signs = 0x0000000000000000 (0 XOR 0)
    //   sub_signs = 0x8000000000000000 (from above)
    //   r = ~0x0000000000000000 & 0x8000000000000000 = 0x8000000000000000
    //   For a = 0x8000000000000000, b = 0x7FFFFFFFFFFFFFFF (differing MSBs):
    //   diff_signs = 0x8000000000000000, r = ~0x8000000000000000 & sub_signs = 0
    let r = _mm_andnot_si128(diff_signs, sub_signs);

    // Step 4: Handle differing MSB case
    // Compute sync = ~(b[MSB]) & a[MSB] to address lanes where MSBs differ (diff_signs[MSB]=1,
    // from 0 XOR 1 or 1 XOR 0). For unsigned comparison, when a[MSB]=1 and b[MSB]=0, a >= 2^63
    // and b < 2^63, so a > b (e.g., 2^63 > 2^63-1). This operation sets sync[MSB]=1 in such
    // lanes. The ~b[MSB] term is 1 when b[MSB]=0, and a[MSB] is 1 when a[MSB]=1, capturing
    // the case 1 XOR 0. The other differing case (a[MSB]=0, b[MSB]=1, 0 XOR 1) implies
    // a < 2^63, b >= 2^63, so a < b, and sync[MSB]=0. Matching MSB cases (0 XOR 0, 1 XOR 1)
    // are handled in Step 3, as diff_signs[MSB]=0 excludes them from needing sync. This step
    // ensures the correct result for differing MSBs by adding the result where a’s magnitude
    // dominates due to its MSB.
    // Example:
    //   a = 0x8000000000000000 (2^63, MSB=1), b = 0x7FFFFFFFFFFFFFFF (2^63-1, MSB=0)
    //   b[MSB] = 0x7FFFFFFFFFFFFFFF & 0x8000000000000000 = 0x0000000000000000
    //   ~b[MSB] = ~0x0000000000000000 = 0xFFFFFFFFFFFFFFFF
    //   a[MSB] = 0x8000000000000000 & 0x8000000000000000 = 0x8000000000000000
    //   sync = 0xFFFFFFFFFFFFFFFF & 0x8000000000000000 = 0x8000000000000000
    //   For a = 0x0000000000000005 (5, MSB=0), b = 0x8000000000000000 (2^63, MSB=1):
    //   sync = ~(0x8000000000000000) & 0x0000000000000000 = 0x0000000000000000 (a < b)
    let sync = _mm_andnot_si128(
        _mm_and_si128(b, _mm_set1_epi64x(-0x8000000000000000)),
        _mm_and_si128(a, _mm_set1_epi64x(-0x8000000000000000))
    );

    // Step 5: Combine results
    // Compute r = r | sync to merge the subtraction-based results (matching MSBs, Step 3) with
    // the magnitude-based results (differing MSBs, Step 4). If either indicates a > b, the
    // lane’s MSB is set to 1.
    // Example:
    //   r = 0x8000000000000000 (lane 0: 5 > 3, Step 3)
    //   sync = 0x8000000000000000 (lane 1: 2^63 > 2^63-1, Step 4)
    //   r = 0x8000000000000000 | 0x8000000000000000
    let r = _mm_or_si128(r, sync);

    // Step 6: Convert MSBs to 64-bit lane masks
    // The result r has MSBs set in 32-bit lanes [3, 1], corresponding to 64-bit lanes.
    // Apply _mm_srai_epi32 to sign-extend each 32-bit lane’s MSB (0xFFFFFFFF if MSB=1,
    // 0x00000000 if MSB=0). Apply _mm_shuffle_epi32 with mask 0xF5 (11 11 01 01) to copy
    // lanes [3, 1] to [3, 2] and [1, 0], forming 64-bit masks of 0xFFFFFFFFFFFFFFFF or
    // 0x0000000000000000 per lane.
    // Example:
    //   r = [0x80000000, 0x00000000, 0x80000000, 0x00000000] (32-bit lanes)
    //   _mm_srai_epi32(r, 0x1F) = [0xFFFFFFFF, 0x00000000, 0xFFFFFFFF, 0x00000000]
    //   _mm_shuffle_epi32(..., 0xF5) = [0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0x00000000]
    //   = [0xFFFFFFFFFFFFFFFF, 0x0000000000000000] (64-bit lanes)
    return _mm_shuffle_epi32::<0xF5>(_mm_srai_epi32::<0x1F>(r));
}