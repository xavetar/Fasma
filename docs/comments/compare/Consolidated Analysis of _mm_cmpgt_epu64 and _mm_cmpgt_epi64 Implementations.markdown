# Consolidated Analysis of `_mm_cmpgt_epu64` and `_mm_cmpgt_epi64` Implementations

The `_mm_cmpgt_epu64` and `_mm_cmpgt_epi64` functions perform unsigned and signed 64-bit comparisons, respectively, between two `__m128i` vectors, `a` and `b`, in SSE2, where direct 64-bit comparison instructions are unavailable. Both generate a mask: `0xFFFFFFFFFFFFFFFF` for each 64-bit lane where `a > b`, and `0x0000000000000000` where `a ≤ b`. The unsigned version (`_mm_cmpgt_epu64`) uses subtraction (`b - a`) to detect borrow, filtered by MSB matching, and accounts for cases where `a[MSB] = 1` and `b[MSB] = 0`. The signed version (`_mm_cmpgt_epi64`) uses subtraction to detect the sign of the result, filtered similarly, and prioritizes cases where `a[MSB] = 0` and `b[MSB] = 1` (a positive, b negative). This document consolidates insights from multiple implementations for both functions, including an optimized version, a decomposed MSB-focused variant, and a clarified implementation with precomputed MSBs. It explains the logic, addresses optimization trade-offs, evaluates subtraction versus addition, and provides example code for each variant.

## Evolution of Implementations

### Optimized Implementation

The performance-optimized method is concise and applies to both unsigned and signed comparisons (Little-Endian):

#### Any (Pseudo-code)

```pseudo-code
_cmpgt_epu64(a: 128bit, b: 128bit) -> 128bit {
    result: 128bit = (¬(b ⊕ a)) ∧ (b - a);    // Filter borrow
    result = result ∨ (a ∧ ¬b);               // Account for MSB differences (unsigned: a[63]=1, b[63]=0)
    result = broadcast_sign(result);
    return shuffle_block::<32>::<3, 3, 1, 1>(result)
}

_cmpgt_epi64(a: 128bit, b: 128bit) -> 128bit {
    result: 128bit = (¬(b ⊕ a)) ∧ (b - a);    // Filter borrow
    result = result ∨ (¬a ∧ b);               // Account for MSB differences (signed: a[63]=0, b[63]=1)
    result = broadcast_sign(result);
    return shuffle_block::<32>::<3, 3, 1, 1>(result)
}
```

#### x86, x86_64 (Rust)

```rust
unsafe fn _mm_cmpgt_epu64(a: __m128i, b: __m128i) -> __m128i {
    return _mm_shuffle_epi32::<0xF5>(
        _mm_srai_epi32::<0x1F>(
            _mm_or_si128(
                _mm_andnot_si128(_mm_xor_si128(b, a), _mm_sub_epi64(b, a)), // Filter borrow
                _mm_andnot_si128(b, a)                                      // Account for MSB differences
            )
        )
    );
}

unsafe fn _mm_cmpgt_epi64(a: __m128i, b: __m128i) -> __m128i {
    return _mm_shuffle_epi32::<0xF5>(
        _mm_srai_epi32::<0x1F>(
            _mm_or_si128(
                _mm_andnot_si128(_mm_xor_si128(b, a), _mm_sub_epi64(b, a)), // Filter borrow
                _mm_andnot_si128(a, b)                                      // Account for MSB differences
            )
        )
    );
}
```

- **Strengths**: Minimizes instructions (~8 SSE2 instructions: `pxor`, `psubq`, `pandn`, `pandn`, `por`, `psrad`, `pshufd`), leveraging full bitwise operations for both signed and unsigned comparisons.
- **Weaknesses**: Processes all bits in `(¬(b ⊕ a)) ∧ (b - a)` and `a ∧ ¬b` (unsigned) or `¬a ∧ b` (signed), obscuring the critical role of the MSB. This reduces semantic clarity, as borrow (unsigned) or sign (signed) detection is implicit.
- **Insight**: Optimization prioritizes performance over transparency, making the logic less intuitive without decomposition.

### Decomposed MSB-Focused Implementation

The decomposed variant explicitly focuses on the MSB (`-0x8000000000000000`):

```rust
pub unsafe fn _mm_cmpgt_epu64(a: __m128i, b: __m128i) -> __m128i {
    let diff_signs = _mm_and_si128(_mm_xor_si128(b, a), _mm_set1_epi64x(-0x8000000000000000));
    let sub_signs = _mm_and_si128(_mm_sub_epi64(b, a), _mm_set1_epi64x(-0x8000000000000000));
    let result_signs = _mm_andnot_si128(diff_signs, sub_signs);
    let result_signs = _mm_or_si128(result_signs, _mm_andnot_si128(_mm_and_si128(b, _mm_set1_epi64x(-0x8000000000000000)), _mm_and_si128(a, _mm_set1_epi64x(-0x8000000000000000))));
    return _mm_shuffle_epi32::<0xF5>(_mm_srai_epi32::<0x1F>(result_signs));
}

pub unsafe fn _mm_cmpgt_epi64(a: __m128i, b: __m128i) -> __m128i {
    let diff_signs = _mm_and_si128(_mm_xor_si128(b, a), _mm_set1_epi64x(-0x8000000000000000));
    let sub_signs = _mm_and_si128(_mm_sub_epi64(b, a), _mm_set1_epi64x(-0x8000000000000000));
    let result_signs = _mm_andnot_si128(diff_signs, sub_signs);
    let result_signs = _mm_or_si128(result_signs, _mm_andnot_si128(_mm_and_si128(a, _mm_set1_epi64x(-0x8000000000000000)), _mm_and_si128(b, _mm_set1_epi64x(-0x8000000000000000))));
    return _mm_shuffle_epi32::<0xF5>(_mm_srai_epi32::<0x1F>(result_signs));
}
```

- **Improvements**: Restricts operations to the MSB, clarifying that comparison hinges on borrow (unsigned) or sign (signed) and MSB differences. Explicit steps enhance readability.
- **Trade-off**: Maintains efficiency (~8 instructions) while sacrificing minor generality (all-bit operations), which is unnecessary for both comparison types.
- **Insight**: Decomposition restores semantic clarity by focusing on the MSB, critical for both unsigned and signed comparison outcomes.

### Clarified Implementation

The clarified implementation precomputes MSB values for maximum clarity:

```rust
pub unsafe fn _mm_cmpgt_epu64(a: __m128i, b: __m128i) -> __m128i {
    let a_sign = _mm_and_si128(a, _mm_set1_epi64x(-0x8000000000000000));
    let b_sign = _mm_and_si128(b, _mm_set1_epi64x(-0x8000000000000000));

    // Step 1: diff = b ⊕ a (MSB differences)
    let diff_signs = _mm_xor_si128(b_sign, a_sign); // pxor

    // Step 2: sub = b - a (borrow at MSB)
    let sub_signs = _mm_and_si128(_mm_sub_epi64(b, a), _mm_set1_epi64x(-0x8000000000000000)); // psubq

    // Step 3: r = ~(diff) & sub (filter borrow where MSBs match)
    let result_signs = _mm_andnot_si128(diff_signs, sub_signs); // pandn

    // Step 4: r |= a & ~b (account for a[MSB]=1, b[MSB]=0)
    let result_signs = _mm_or_si128(r, _mm_andnot_si128(b_sign, a_sign)); // pandn, por
    
    // Step 5: Propagate sign from 32-bit lanes [3, 1] to 64-bit lanes
    return _mm_shuffle_epi32::<0xF5>(_mm_srai_epi32::<0x1F>(result_signs)); // psrad, pshufd
}

pub unsafe fn _mm_cmpgt_epi64(a: __m128i, b: __m128i) -> __m128i {
    let a_sign = _mm_and_si128(a, _mm_set1_epi64x(-0x8000000000000000));
    let b_sign = _mm_and_si128(b, _mm_set1_epi64x(-0x8000000000000000));

    // Step 1: diff = b ⊕ a (MSB differences)
    let diff_signs = _mm_xor_si128(b_sign, a_sign); // pxor

    // Step 2: sub = b - a (borrow at MSB)
    let sub_signs = _mm_and_si128(_mm_sub_epi64(b, a), _mm_set1_epi64x(-0x8000000000000000)); // psubq

    // Step 3: r = ~(diff) & sub (filter borrow where MSBs match)
    let result_signs = _mm_andnot_si128(diff_signs, sub_signs); // pandn

    // Step 4: r |= ~a & b (account for a[MSB]=0, b[MSB]=1)
    let result_signs = _mm_or_si128(result_signs, _mm_andnot_si128(a_sign, b_sign)); // pandn, por
    
    // Step 5: Propagate sign from 32-bit lanes [3, 1] to 64-bit lanes
    return _mm_shuffle_epi32::<0xF5>(_mm_srai_epi32::<0x1F>(result_signs)); // psrad, pshufd
}
```

- **Improvements**:
  - Precomputes `a_sign` and `b_sign`, isolating MSB values upfront.
  - Clearly labels steps (e.g., "MSB differences", "borrow at MSB"), aligning with logical flow.
- **Instruction Count**: ~9 instructions (`pand`, `pand`, `pxor`, `psubq`, `pand`, `pandn`, `pandn`, `por`, `psrad`, `pshufd`), slightly higher due to precomputing MSBs.
- **Insight**: Prioritizes clarity, making it an excellent reference for understanding SIMD comparison logic for both signed and unsigned cases.

## Algorithm Logic

Both functions compare 64-bit integers in `__m128i` vectors using *two's complement* arithmetic, but differ in their interpretation of the MSB (bit 63).

### Unsigned Comparison (`_mm_cmpgt_epu64`)

Detects borrow in `b - a` (MSB = 1 indicates `b < a`) and accounts for `a[63] = 1`, `b[63] = 0`. Steps:
1. **Extract MSBs**: Compute `a_sign = a & 0x80...`, `b_sign = b & 0x80...`.
2. **MSB Differences**: XOR `b_sign` and `a_sign` to identify differing MSBs.
3. **Detect Borrow**: Perform `b - a` and extract MSB to check for borrow.
4. **Filter Borrow**: Retain borrow only where MSBs match (`~diff_signs & sub_signs`).
5. **MSB Differences**: Include cases where `a[63] = 1`, `b[63] = 0` (`a_sign & ~b_sign`).
6. **Propagate Mask**: Convert MSB to 64-bit mask via shift and shuffle.

### Signed Comparison (`_mm_cmpgt_epi64`)

Detects sign in `b - a` (MSB = 1 indicates `b - a < 0`, suggesting `a > b`) and accounts for `a[63] = 0`, `b[63] = 1`. Steps:
1. **Extract MSBs**: Compute `a_sign` and `b_sign` to isolate sign bit.
2. **MSB Differences**: XOR `b_sign` and `a_sign` to identify differing signs.
3. **Detect Sign**: Perform `b - a` and extract MSB to check if `b - a < 0`.
4. **Filter Sign**: Retain sign bit only where signs match (`~diff_signs & sub_signs`).
5. **Sign Differences**: Include cases where `a[63] = 0`, `b[63] = 1` (`~a_sign & b_sign`).
6. **Propagate Mask**: Convert MSB to 64-bit mask.

### Step-by-Step Analysis (8-bit Example)

#### Unsigned: `a = 0x80` (128), `b = 0x7F` (127)
- **Expectation**: `a > b` (128 > 127).
- **Steps**:
  - `a_sign = 0x80`, `b_sign = 0x00`.
  - `diff_signs = 0x80 ⊕ 0x00 = 0x80`.
  - `sub_signs = (0x7F - 0x80 = 0xFF) & 0x80 = 0x80` (borrow).
  - `r = ¬0x80 & 0x80 = 0x00`.
  - `a_sign & ¬b_sign = 0x80 & 0xFF = 0x80`.
  - `r = 0x00 | 0x80 = 0x80` → `a > b`.
  - Propagate: `_mm_srai_epi32(0x80, 0x1F)` → `0xFFFFFFFF`, `_mm_shuffle_epi32::<0xF5>` → `0xFFFFFFFFFFFFFFFF`.

#### Signed: `a = 0x80` (-128), `b = 0x7F` (127)
- **Expectation**: `a < b` (-128 < 127).
- **Steps**:
  - `a_sign = 0x80`, `b_sign = 0x00`.
  - `diff_signs = 0x80`.
  - `sub_signs = (0xFF) & 0x80 = 0x80` (negative result).
  - `r = ¬0x80 & 0x80 = 0x00`.
  - `~a_sign & b_sign = ¬0x80 & 0x00 = 0x00` (a negative, b positive).
  - `r = 0x00 | 0x00 = 0x00` → `a < b`.
  - Propagate: `_mm_srai_epi32(0x00, 0x1F)` → `0x00000000`, `_mm_shuffle_epi32::<0xF5>` → `0x0000000000000000`.

## Comparison Across Implementations

- **Optimized**: Processes all bits, optimizing for performance (~8 instructions) but lacking clarity.
- **Decomposed**: Restricts to MSB, balancing clarity and efficiency (~8 instructions).
- **Clarified**: Precomputes MSBs, prioritizing clarity (~9 instructions).
- **Equivalence**:
  - Optimized: `(¬(b ⊕ a)) ∧ (b - a)` and `a ∧ ¬b` (unsigned) or `¬a ∧ b` (signed) for all bits.
  - Decomposed: `(b ⊕ a) & 0x80...`, `(b - a) & 0x80...`, `(a ∧ ¬b) & 0x80...` (unsigned) or `(¬a ∧ b) & 0x80...` (signed).
  - Clarified: Same as decomposed, with precomputed `a_sign`, `b_sign`.
- **Trade-off**: Clarified variant sacrifices one instruction for readability.

## Consolidated Insights

1. **Subtraction vs. Addition**:
   - **Subtraction**:
     - Unsigned: Optimal, as borrow (MSB = 1 in `b - a`) indicates `b < a` (e.g., `0x00 - 0x01 = 0xFF`, MSB = 1).
     - Signed: Optimal, as `b - a < 0` (MSB = 1) indicates `a > b` when signs match.
   - **Addition**: Less suitable for unsigned comparison, as carry (`a + b ≥ 2^{64}`) does not correlate with `a > b` (e.g., `a = 0x01`, `b = 0xFF...` → carry, but `1 < 2^{64} - 1`). For signed comparison, overflow detection adds complexity (~3–4 instructions).
   - **Conclusion**: Subtraction is simpler and more direct for both.

2. **Optimization vs. Clarity**:
   - Optimized versions prioritize performance, obscuring MSB focus.
   - Decomposed and clarified variants emphasize MSB operations, enhancing readability while maintaining near-optimal performance.

3. **Two's Complement**:
   - **Unsigned**: MSB = 1 indicates large values. `a ∧ ¬b` ensures `0x8000000000000000` (2^{63}) > `0x7FFFFFFFFFFFFFFF` (2^{63} - 1).
   - **Signed**: MSB = 1 indicates negative values. `¬a ∧ b` ensures `0x8000000000000000` (-2^{63}) < `0x7FFFFFFFFFFFFFFF` (2^{63} - 1).
   - Subtraction: `b - a = b + (¬a + 1)` produces borrow (unsigned) or negative result (signed) when MSB = 1.

4. **Decomposition**:
   - All-bit operations were decomposed to focus on MSB, clarifying borrow (unsigned) and sign (signed) detection.
   - Precomputed MSBs in the clarified variant enhance pedagogical value.

## Edge Case Analysis

### Unsigned Cases
1. **Case 1**: `a = 0x8000000000000000` (2^{63}), `b = 0x7FFFFFFFFFFFFFFF` (2^{63} - 1).
   - Expectation: `a > b`.
   - Clarified: `a_sign = 0x80...`, `b_sign = 0x00`, `diff_signs = 0x80...`, `sub_signs = 0x80...`, `r = 0x00 | 0x80... = 0x80...` → `a > b`.

2. **Case 2**: `a = 0x7FFFFFFFFFFFFFFF` (2^{63} - 1), `b = 0xFFFFFFFFFFFFFFFF` (2^{64} - 1).
   - Expectation: `a < b`.
   - Clarified: `a_sign = 0x00`, `b_sign = 0x80...`, `diff_signs = 0x80...`, `sub_signs = 0x80...`, `r = 0x00 | 0x00 = 0x00` → `a < b`.

3. **Case 3**: `a = 0x0000000000000001` (1), `b = 0x0000000000000000` (0).
   - Expectation: `a > b`.
   - Clarified: `a_sign = 0x00`, `b_sign = 0x00`, `diff_signs = 0x00`, `sub_signs = 0x80...`, `r = 0x80... | 0x00 = 0x80...` → `a > b`.

### Signed Cases
1. **Case 1**: `a = 0x8000000000000000` (-2^{63}), `b = 0x7FFFFFFFFFFFFFFF` (2^{63} - 1).
   - Expectation: `a < b`.
   - Clarified: `a_sign = 0x80...`, `b_sign = 0x00`, `diff_signs = 0x80...`, `sub_signs = 0x80...`, `r = 0x00 | 0x00 = 0x00` → `a < b`.

2. **Case 2**: `a = 0xFFFFFFFFFFFFFFFF` (-1), `b = 0xFFFFFFFFFFFFFFFE` (-2).
   - Expectation: `a > b`.
   - Clarified: `a_sign = 0x80...`, `b_sign = 0x80...`, `diff_signs = 0x00`, `sub_signs = 0x80...`, `r = 0x80... | 0x00 = 0x80...` → `a > b`.

3. **Case 3**: `a = 0x0000000000000001` (1), `b = 0xFFFFFFFFFFFFFFFF` (-1).
   - Expectation: `a > b`.
   - Clarified: `a_sign = 0x00`, `b_sign = 0x80...`, `diff_signs = 0x80...`, `sub_signs = 0x80...`, `r = 0x00 | 0x80... = 0x80...` → `a > b`.

## Performance and Scalability

- **Instruction Counts**:
  - Optimized: ~8 instructions.
  - Decomposed: ~8 instructions.
  - Clarified: ~9 instructions (additional `pand` for `a_sign`, `b_sign`).
- **Scalability**: Logic applies to `_mm256_cmpgt_epu64`/`_epi64` (AVX2) and `_mm512_cmpgt_epu64`/`_epi64` (AVX512).
- **Verification**:
  ```bash
  cargo rustc --release -- --emit asm
  ```

## Testing Recommendations

```rust
let tests = [
    // Unsigned
    (
        _mm_setr_epi64x(0x8000000000000000, 0x7FFFFFFFFFFFFFFF),
        _mm_setr_epi64x(0x7FFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF),
        "Unsigned edge cases: 0x80... vs 0x7F..., 0x7F... vs 0xFF...",
    ),
    (
        _mm_setr_epi64x(0x0000000000000001, 0x0000000000000000),
        _mm_setr_epi64x(0x0000000000000000, 0x0000000000000000),
        "Unsigned crossing 0x00: 0x01 vs 0x00, equal values",
    ),
    // Signed
    (
        _mm_setr_epi64x(0x8000000000000000, 0x7FFFFFFFFFFFFFFF),
        _mm_setr_epi64x(0x7FFFFFFFFFFFFFFF, 0x8000000000000000),
        "Signed edge cases: -2^63 vs 2^63-1, 2^63-1 vs -2^63",
    ),
    (
        _mm_setr_epi64x(0xFFFFFFFFFFFFFFFF, 0x0000000000000001),
        _mm_setr_epi64x(0xFFFFFFFFFFFFFFFE, 0xFFFFFFFFFFFFFFFF),
        "Signed: -1 vs -2, 1 vs -1",
    ),
];
```

## Conclusion

The `_mm_cmpgt_epu64` and `_mm_cmpgt_epi64` implementations demonstrate a progression from performance-optimized to clarity-focused designs. The clarified variants, with precomputed MSBs and corrected shift (`_mm_srai_epi32::<0x1F>`), offer exceptional readability while maintaining near-optimal performance. Key insights include:
- Subtraction is optimal for both unsigned (borrow detection) and signed (sign detection) comparisons.
- Addition is less reliable due to carry (unsigned) or overflow complexity (signed).
- Decomposition emphasizes the MSB, critical for edge cases like `0x8000000000000000` vs. `0x7FFFFFFFFFFFFFFF`.
- The clarified implementations serve as performant solutions and clear references for SIMD programming, with `a ∧ ¬b` (unsigned) and `¬a ∧ b` (signed) distinguishing their logic.