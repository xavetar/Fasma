# Unified `_mm_cmpgt_epu64` and `_mm_cmpgt_epi64` Logic and Implementation

This document unifies the logic of `_mm_cmpgt_epu64` and `_mm_cmpgt_epi64`, methods for unsigned and signed 64-bit integer comparisons, respectively, in two `__m128i` vectors using SSE2 intrinsics. Both functions compare `a` and `b`, returning `0xFFFFFFFFFFFFFFFF` per 64-bit lane where `a > b`, and `0x0000000000000000` where `a ≤ b`. The unsigned version (`_mm_cmpgt_epu64`) leverages subtraction borrow and bitwise operations to handle unsigned comparisons, while the signed version (`_mm_cmpgt_epi64`) uses subtraction sign detection and prioritizes cases where `a` is positive and `b` is negative. Both implementations bypass *two's complement* pitfalls by focusing on MSB differences and subtraction outcomes. Below, we provide C code with detailed comments and a pseudocode summary in markdown.

## C Implementation with Comments

```c
#include <emmintrin.h> // SSE2 intrinsics

// Compares unsigned 64-bit integers in two __m128i vectors
// Returns: 0xFFFFFFFFFFFFFFFF per 64-bit lane where a > b, 0x0000000000000000 where a ≤ b
__m128i _mm_cmpgt_epu64(__m128i a, __m128i b) {
    // 1. Extract MSB (bit 63) for both inputs to focus on high-order bit differences
    //    - Mask: 0x8000000000000000 isolates the most significant bit
    __m128i msb_mask = _mm_set1_epi64x(0x8000000000000000);
    __m128i a_sign = _mm_and_si128(a, msb_mask); // a_sign = a & 0x8000...
    __m128i b_sign = _mm_and_si128(b, msb_mask); // b_sign = b & 0x8000...

    // 2. Compute differences in MSB (where a and b differ in bit 63)
    //    - XOR: 1 where bits differ, 0 where they are the same
    __m128i diff_signs = _mm_xor_si128(b_sign, a_sign); // diff_signs = b_sign ⊕ a_sign

    // 3. Perform subtraction (b - a) and isolate MSB to detect borrow
    //    - Borrow (MSB = 1 in result) indicates b < a, hence a > b in unsigned context
    __m128i sub = _mm_sub_epi64(b, a); // sub = b - a
    __m128i sub_signs = _mm_and_si128(sub, msb_mask); // sub_signs = (b - a) & 0x8000...

    // 4. Filter borrow to cases where MSB of a and b are the same
    //    - ANDNOT: ¬diff_signs ∧ sub_signs ensures borrow is only counted when a[63] == b[63]
    //    - This avoids false positives from differing MSBs (e.g., a = 0x7F..., b = 0x80...)
    __m128i result = _mm_andnot_si128(diff_signs, sub_signs); // result = ¬diff_signs ∧ sub_signs

    // 5. Account for cases where a[63] = 1 and b[63] = 0 (e.g., a = 0x80..., b = 0x7F...)
    //    - ANDNOT: a_sign ∧ ¬b_sign captures a > b when a has MSB set and b does not
    //    - Critical for edge cases like 0x8000000000000000 > 0x7FFFFFFFFFFFFFFF
    result = _mm_or_si128(result, _mm_andnot_si128(b_sign, a_sign)); // result |= a_sign ∧ ¬b_sign

    // 6. Propagate the result MSB to all 64 bits in each lane
    //    - Arithmetic right shift (psrad) by 31 fills 32-bit lanes with the sign bit
    //    - Shuffle (pshufd) duplicates the high 32 bits to fill the 64-bit lane
    //    - _MM_SHUFFLE(3,3,1,1) = 0xF5 ensures correct lane alignment
    result = _mm_srai_epi32(result, 31); // Sign-extend MSB to 32 bits
    result = _mm_shuffle_epi32(result, _MM_SHUFFLE(3, 3, 1, 1)); // Duplicate to fill 64 bits

    return result;
}

// Compares signed 64-bit integers in two __m128i vectors
// Returns: 0xFFFFFFFFFFFFFFFF per 64-bit lane where a > b, 0x0000000000000000 where a ≤ b
__m128i _mm_cmpgt_epi64(__m128i a, __m128i b) {
    // 1. Extract MSB (bit 63, sign bit) for both inputs to focus on sign differences
    //    - Mask: 0x8000000000000000 isolates the sign bit
    __m128i msb_mask = _mm_set1_epi64x(0x8000000000000000);
    __m128i a_sign = _mm_and_si128(a, msb_mask); // a_sign = a & 0x8000...
    __m128i b_sign = _mm_and_si128(b, msb_mask); // b_sign = b & 0x8000...

    // 2. Compute differences in sign bits (where a and b differ in bit 63)
    //    - XOR: 1 where signs differ, 0 where they are the same
    __m128i diff_signs = _mm_xor_si128(b_sign, a_sign); // diff_signs = b_sign ⊕ a_sign

    // 3. Perform subtraction (b - a) and isolate MSB to detect sign
    //    - Negative result (MSB = 1) indicates b - a < 0, hence a > b in signed context
    __m128i sub = _mm_sub_epi64(b, a); // sub = b - a
    __m128i sub_signs = _mm_and_si128(sub, msb_mask); // sub_signs = (b - a) & 0x8000...

    // 4. Filter sign to cases where signs of a and b are the same
    //    - ANDNOT: ¬diff_signs ∧ sub_signs ensures sign is only counted when a[63] == b[63]
    //    - This avoids false positives from differing signs (e.g., a = -1, b = 1)
    __m128i result = _mm_andnot_si128(diff_signs, sub_signs); // result = ¬diff_signs ∧ sub_signs

    // 5. Account for cases where a[63] = 0 and b[63] = 1 (e.g., a = 0x7F..., b = 0x80...)
    //    - ANDNOT: ¬a_sign ∧ b_sign captures a > b when a is positive and b is negative
    //    - Critical for edge cases like 0x7FFFFFFFFFFFFFFF > 0x8000000000000000
    result = _mm_or_si128(result, _mm_andnot_si128(a_sign, b_sign)); // result |= ¬a_sign ∧ b_sign

    // 6. Propagate the result MSB to all 64 bits in each lane
    //    - Same as unsigned: shift and shuffle to produce full 64-bit mask
    result = _mm_srai_epi32(result, 31); // Sign-extend MSB to 32 bits
    result = _mm_shuffle_epi32(result, _MM_SHUFFLE(3, 3, 1, 1)); // Duplicate to fill 64 bits

    return result;
}
```

## Pseudocode Logic

The `_mm_cmpgt_epu64` and `_mm_cmpgt_epi64` methods compare unsigned and signed 64-bit integers, respectively, in two `__m128i` vectors by analyzing subtraction outcomes (borrow for unsigned, sign for signed) and MSB differences. Below is the step-by-step logic for both:

### `_mm_cmpgt_epu64` (Unsigned)
1. **Extract MSB (bit 63)**:
   - Compute `a_sign = a & 0x8000000000000000`.
   - Compute `b_sign = b & 0x8000000000000000`.
   - *Purpose*: Focus on the most significant bit, as it dominates unsigned comparisons (e.g., `0x8000...` > `0x7FFF...`).

2. **Compute MSB Differences**:
   - Compute `diff_signs = b_sign ⊕ a_sign`.
   - *Purpose*: Identify lanes where `a` and `b` have different MSBs (1 if different, 0 if same). This filters borrow cases.

3. **Detect Borrow via Subtraction**:
   - Compute `sub = b - a`.
   - Compute `sub_signs = sub & 0x8000000000000000`.
   - *Purpose*: Borrow (MSB = 1) indicates `b < a`, thus `a > b`.

4. **Filter Borrow for Same MSBs**:
   - Compute `result = ¬diff_signs ∧ sub_signs`.
   - *Purpose*: Count borrow only when `a` and `b` have the same MSB, avoiding false positives (e.g., `a = 0x7F...`, `b = 0x80...`).

5. **Handle Different MSBs**:
   - Compute `result |= a_sign ∧ ¬b_sign`.
   - *Purpose*: Capture `a[63] = 1`, `b[63] = 0` (e.g., `0x8000...` > `0x7FFF...`).

6. **Propagate Result**:
   - Shift right arithmetically by 31 bits to fill 32-bit lanes.
   - Shuffle to duplicate high 32 bits across 64-bit lanes.
   - *Purpose*: Convert MSB to full 64-bit mask (`0xFFFFFFFFFFFFFFFF` or `0x0000000000000000`).

### `_mm_cmpgt_epi64` (Signed)
1. **Extract MSB (bit 63, sign bit)**:
   - Compute `a_sign = a & 0x8000000000000000`.
   - Compute `b_sign = b & 0x8000000000000000`.
   - *Purpose*: Focus on the sign bit, critical for signed comparisons (e.g., `0x7FFF...` > `0x8000...`).

2. **Compute Sign Bit Differences**:
   - Compute `diff_signs = b_sign ⊕ a_sign`.
   - *Purpose*: Identify lanes where `a` and `b` have different signs.

3. **Detect Sign via Subtraction**:
   - Compute `sub = b - a`.
   - Compute `sub_signs = sub & 0x8000000000000000`.
   - *Purpose*: Negative result (MSB = 1) indicates `b - a < 0`, thus `a > b`.

4. **Filter Sign for Same Signs**:
   - Compute `result = ¬diff_signs ∧ sub_signs`.
   - *Purpose*: Count negative result only when `a` and `b` have the same sign, avoiding false positives (e.g., `a = -1`, `b = 1`).

5. **Handle Different Signs**:
   - Compute `result |= ¬a_sign ∧ b_sign`.
   - *Purpose*: Capture `a[63] = 0`, `b[63] = 1` (a positive, b negative, e.g., `0x7FFF...` > `0x8000...`).

6. **Propagate Result**:
   - Same as unsigned: shift and shuffle to produce 64-bit mask.

## Key Insights

- **Subtraction-Based Comparison**:
  - **Unsigned**: Borrow (MSB = 1 in `b - a`) reliably detects `a > b`, unlike addition, which requires complex carry handling.
  - **Signed**: Negative result (MSB = 1 in `b - a`) indicates `a > b` when signs match, with `¬a_sign ∧ b_sign` handling different signs.
- **MSB Focus**:
  - **Unsigned**: MSB = 1 indicates larger values (e.g., `0x8000...` > `0x7FFF...`).
  - **Signed**: MSB = 1 indicates negative values (e.g., `0x8000...` < `0x7FFF...`).
- **Bitwise Operations**: `⊕`, `∧`, `¬`, `∨` emulate missing comparison instructions in SSE2, keeping instruction count low (~9).
- **Edge Case Handling**:
  - **Unsigned**: `a_sign ∧ ¬b_sign` ensures `0x8000000000000000` > `0x7FFFFFFFFFFFFFFF`.
  - **Signed**: `¬a_sign ∧ b_sign` ensures `0x7FFFFFFFFFFFFFFF` > `0x8000000000000000`.
- **Two's Complement**:
  - **Unsigned**: Ignores signed overflow, treating all operations as unsigned.
  - **Signed**: Uses sign bit and subtraction result to maintain correct ordering.
- **Unified Logic**: Both functions share the same structure, differing only in the MSB difference term (`a_sign ∧ ¬b_sign` vs. `¬a_sign ∧ b_sign`).

## Performance
- **Instruction Count**: ~9 instructions (`pand x3`, `pxor`, `psubq`, `pandn x2`, `por`, `psrad`, `pshufd`) for both functions.
- **Scalability**: Logic extends to AVX2 (`_mm256_cmpgt_epu64`/`_epi64`) and AVX512 (`_mm512_cmpgt_epu64`/`_epi64`) with minor adjustments.
- **Optimization**: Precomputing `a_sign` and `b_sign` enhances clarity with minimal performance impact.

## Testing Recommendations

To verify correctness, test the following edge cases:
- **Unsigned**:
  - `a = 0x8000000000000000`, `b = 0x7FFFFFFFFFFFFFFF` (expect `a > b`).
  - `a = 0x7FFFFFFFFFFFFFFF`, `b = 0xFFFFFFFFFFFFFFFF` (expect `a < b`).
  - `a = 0x0000000000000001`, `b = 0x0000000000000000` (expect `a > b`).
- **Signed**:
  - `a = 0x8000000000000000` (-2^{63}), `b = 0x7FFFFFFFFFFFFFFF` (2^{63} - 1) (expect `a < b`).
  - `a = 0xFFFFFFFFFFFFFFFF` (-1), `b = 0xFFFFFFFFFFFFFFFE` (-2) (expect `a > b`).
  - `a = 0x0000000000000001` (1), `b = 0xFFFFFFFFFFFFFFFF` (-1) (expect `a > b`).

This unified implementation balances readability and performance, making it suitable for production and educational purposes. It handles all edge cases (e.g., `0x8000...`, `0x7FFF...`, `0xFFFF...`) and is robust against *two's complement* overflows for both unsigned and signed comparisons.