# GLAMP (General Logical-Arithmetic Modular Permutation)

*Universal Bit Reversal for Any N, Powered by Logic*  
**By Stanislav Mikhailov (xavetar)** | *Anti-Virus AND MIT AND the Apache License (Version 2.0)*

## Overview

The Generator Logical-Arithmetic Module Inverter (GLAMP) is a computational framework for manipulating bits in arbitrary order or sequence within a field of N bits, using a larger cyclic field of S bits (S ≥ N + 2). Unlike traditional bit manipulation methods that rely on iterative shifts or lookup tables, GLAMP uses logical operations (bitwise AND, multiplication, and modulo) to achieve constant-time (O(1)) permutations at runtime. This makes it highly efficient for embedded systems, hardware accelerators (e.g., FPGAs), and software requiring flexible bit transformations.

GLAMP is not limited to standard bit reversal (LSB to MSB); it supports any bit permutation, including swapping halves, nibbles, or custom patterns, in any field S ≥ N + 2. This implementation provides precomputed parameters for standard bit reversal for N from 8 to 64 bits (in increments of 8), but the method is theoretically extensible to any N and any permutation, limited only by computational resources.

## Principle of Operation

### Core Concept

GLAMP performs bit permutations by mapping input bits to desired output positions using a cyclic field defined by a modulus 2^S - 1, where S ≥ N + 2. The method relies entirely on logical operations (bitwise AND, multiplication as a shift simulation, and modulo as a wrap-around), executed by the processor without arithmetic abstractions. The approach by classical bit-swapping methods (e.g., 0x55/0xAA for adjacent bits) is a direct subset of this method. This method uses logical shifts and masking to achieve any desired bit permutation.

For a given N-bit input v (0 ≤ v < 2^N), GLAMP constructs a multiplier and mask to permute bits according to a specified order, such as reversal (\[N-1, N-2, ..., 0\] → \[0, 1, ..., N-1\]) or custom patterns (e.g., nibble swaps). The cyclic field 2^S - 1 provides a logical framework for bit position manipulation, with S = N + 2 being the minimal practical choice.

### Algorithmic Steps

1. **Field Definition**:

   - Define N as the bit width of the input value v (0 ≤ v < 2^N).
   - Set S ≥ N + 2, typically S = N + 2, to define the cyclic field.
   - Use modulus M = 2^S - 1 to simulate cyclic wrap-around via logical operations.

2. **Broadcast Multiplier Construction**:

   - Create a base multiplier: (N-2) zeros followed by "10" (binary, representing 2^{N-1}).
   - Repeat this pattern k times to cover the cycle length required for the S-bit field, ensuring all bit positions are addressable.

3. **Mask Computation for Permutations**:

   - Define source positions as \[N-1, N-2, ..., 0\] for standard reversal, or any permutation \[p_0, p_1, ..., p\_{N-1}\] for custom ordering.
   - Generate adder positions \[S-1, S-2, ..., 0\], extended cyclically to match the period.
   - Compute shifted source positions modulo N to align with the target permutation.
   - Build a mask by matching shifted positions against adder positions: matches yield '1' (denoted '\*'), non-matches yield '0' ('-').
   - Converge rapidly (typically in one or few iterations) to a mask with exactly N '1' bits, removing duplicates if necessary.

4. **Bit Permutation**:

   - Compute intermediate = (v \* MUL) & MASK.
   - Return intermediate % M.
   - This operation permutes the bits of v according to the specified order, using logical operations to simulate shifts and selections.

The method’s universality allows it to handle any bit permutation (reversal, half-swaps, nibble rearrangements, or non-deterministic patterns) in any field S ≥ N + 2.

### Logical Foundation

GLAMP operates entirely through logical operations: multiplication simulates bit shifts, AND selects specific positions, and modulo enforces cyclic behavior. The modulus 2^S - 1 is a property of the field size, not a reliance on mathematical structures like Mersenne numbers or Galois fields. The algorithm avoids arithmetic abstractions, focusing on processor-level logic to achieve permutations. The method is not derived from classical bit-swapping techniques but treats them as a subset of possible permutations achievable through mask configuration.

## Implementation Details

### Generator Script

The `find_solution(N, S, permutation=None)` function generates the broadcast multiplier and permutation mask as binary strings, converted to hexadecimal. It converges rapidly (often in one iteration) for S = N + 2, producing parameters for any specified bit order. For non-standard permutations, the permutation parameter defines the target bit mapping.

### Permutation Functions

For each supported N, a function `_rbit_seuN(v: int) -> int` is provided for standard reversal, using Python’s arbitrary-precision integers. The general form is:

```python
def _rbit_seuN(v: int, MOD: int, MUL: int, MASK: int) -> int:
    """
    Performs N-bit permutation using GLAMP parameters.
    
    Args:
        v (int): Input value (0 <= v < 2**N).
    
    Returns:
        int: Permuted bit value.
    """
    return ((v * MUL) & MASK) % MOD
```

## Supported Bit Widths and Parameters

Below are precomputed parameters for standard bit reversal (LSB to MSB) for N from 8 to 64 bits, using S = N + 2.

| Bit Width (N) | Modulus (M = 2^{N+2} - 1) | Broadcast Multiplier (MUL, HEX) | Reversal Mask (MASK, HEX) |
| --- | --- | --- | --- |
| 8 | 0x3FF | 0x0202020202 | 0x0010884422010 |
| 16 | 0x3FFFF | 0x000200020002000200020002000200020002 | 0x00001008080404020201010080804040202000100 |
| 24 | 0x3FFFFFF | 0x000002000002000002000002000002000002000002000002000002000002000002000002000002 | 0x0000001000800800400400200200100100080080040040020020010010008008004004002002000001000 |
| 32 | 0x3FFFFFFFF | 0x0000000200000002000000020000000200000002000000020000000200000002000000020000000200000002000000020000000200000002000000020000000200000002 | 0x0000000010000800080004000400020002000100010000800080004000400020002000100010000800080004000400020002000100010000800080004000400020002000000010000 |
| 40 | 0x3FFFFFFFFFF | 0x000000000200000000020000000002000000000200000000020000000002000000000200000000020000000002000000000200000000020000000002000000000200000000020000000002000000000200000000020000000002000000000200000000020000000002 | 0x00000000001000008000080000400004000020000200001000010000080000800004000040000200002000010000100000800008000040000400002000020000100001000008000080000400004000020000200001000010000080000800004000040000200002000000000100000 |
| 48 | 0x3FFFFFFFFFFFF | 0x000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002000000000002 | 0x0000000000001000000800000800000400000400000200000200000100000100000080000080000040000040000020000020000010000010000008000008000004000004000002000002000001000001000000800000800000400000400000200000200000100000100000080000080000040000040000020000020000010000010000008000008000004000004000002000002000000000001000000 |
| 56 | 0x3FFFFFFFFFFFFFF | 0x0000000000000200000000000002000000000000020000000000000200000000000002000000000000020000000000000200000000000002000000000000020000000000000200000000000002000000000000020000000000000200000000000002000000000000020000000000000200000000000002000000000000020000000000000200000000000002000000000000020000000000000200000000000002000000000000020000000000000200000000000002000000000000020000000000000200000000000002 | 0x0000000000000010000000800000080000004000000400000020000002000000100000010000000800000080000004000000400000020000002000000100000010000000800000080000004000000400000020000002000000100000010000000800000080000004000000400000020000002000000100000010000000800000080000004000000400000020000002000000100000010000000800000080000004000000400000020000002000000100000010000000800000080000004000000400000020000002000000000000010000000 |
| 64 | 0x3FFFFFFFFFFFFFFFF | 0x000000000000000200000000000000020000000000000002000000000000000200000000000000020000000000000002000000000000000200000000000000020000000000000002000000000000000200000000000000020000000000000002000000000000000200000000000000020000000000000002000000000000000200000000000000020000000000000002000000000000000200000000000000020000000000000002000000000000000200000000000000020000000000000002000000000000000200000000000000020000000000000002000000000000000200000000000000020000000000000002000000000000000200000000000000020000000000000002 | 0x00000000000000001000000008000000080000000400000004000000020000000200000001000000010000000080000000800000004000000040000000200000002000000010000000100000000800000008000000040000000400000002000000020000000100000001000000008000000080000000400000004000000020000000200000001000000010000000080000000800000004000000040000000200000002000000010000000100000000800000008000000040000000400000002000000020000000100000001000000008000000080000000400000004000000020000000200000001000000010000000080000000800000004000000040000000200000002000000000000000100000000 |

Note: Hexadecimal values are uppercase and zero-padded. Custom permutations require regenerating the mask with the desired bit order.

## Usage Examples

### Python Integration

Incorporate GLAMP functions into Python code as follows:

```python
# Example for N=32 (standard reversal)
def _rbit_seu32(v: int) -> int:
    MOD = 0x3FFFFFFFF
    MUL = 0x0000000200000002000000020000000200000002000000020000000200000002000000020000000200000002000000020000000200000002000000020000000200000002
    MASK = 0x0000000010000800080004000400020002000100010000800080004000400020002000100010000800080004000400020002000100010000800080004000400020002000000010000
    return ((v * MUL) & MASK) % MOD

# Custom permutation example (e.g., swap adjacent bits)
def _rbit_custom32(v: int, permutation: list) -> int:
    MOD = 0x3FFFFFFFF
    MUL, MASK = find_solution(32, 34, permutation=permutation)  # Custom permutation
    return ((v * MUL) & MASK) % MOD

# Usage
input_value = 0b000000000000000000000001010101  # Example 32-bit value
reversed_value = _rbit_seu32(input_value)
print(f"Input: 0x{input_value:08X}, Reversed: 0x{reversed_value:08X}")
```

### Verification

Verify correctness using a reference implementation:

```python
def reference_permute(v: int, n: int, permutation: list = None) -> int:
    """Reference bit permutation using string manipulation."""
    binary = bin(v)[2:].zfill(n)
    if permutation is None:
        permutation = list(range(n-1, -1, -1))  # Standard reversal
    result = ['0'] * n
    for i, p in enumerate(permutation):
        result[p] = binary[i]
    return int(''.join(result), 2)

# Test
assert _rbit_seu32(1) == reference_permute(1, 32)  # LSB to MSB
```

## Testing and Validation

### Test Suite Outline

A comprehensive test suite should include:

1. **Boundary Cases**:

   - v = 0: Expected 0 for any permutation.
   - v = 2^N - 1: Expected 2^N - 1 for standard reversal.

2. **Single Bit Tests**:

   - v = 2^k for k in 0 to N-1: Expected 2^{permutation\[k\]}.

3. **Custom Permutations**:

   - Test patterns like adjacent bit swaps, nibble reversals, or random shuffles.

4. **Random Inputs**:

   - Generate 1000 random v < 2^N and verify against reference.

Example test for N=8 (standard reversal):

| Input (DEC) | Input (BIN) | Expected Output (BIN) | GLAMP Output (DEC) |
| --- | --- | --- | --- |
| 0 | 00000000 | 00000000 | 0 |
| 1 | 00000001 | 10000000 | 128 |
| 85 | 01010101 | 10101010 | 170 |
| 255 | 11111111 | 11111111 | 255 |

All tests pass for the provided parameters.

## Performance Considerations

- **Runtime**: O(1) per permutation, using logical operations (MUL, AND, MOD). Multiplication complexity is O(N^2) in Python due to big-integer handling.
- **Memory**: MUL and MASK scale with N and S; for N=64, approximately 256 bits.
- **Hardware Suitability**: Ideal for systems with wide registers or multi-precision logic units. For N&gt;64, use hardware acceleration (e.g., FPGA, GPU).
- **Mask Generation**: Converges in minimal iterations (often one) for S = N + 2. Parallelize for large N or complex permutations.

## Limitations and Extensions

- **Scalability**: Supports any N and S ≥ N + 2, with no theoretical upper bound. Practical limits depend on register sizes or memory (e.g., N=128 requires \~8452-bit masks).
- **Custom Permutations**: Fully extensible to any bit order (reversal, half-swaps, nibble shuffles, or non-deterministic patterns) by adjusting the permutation parameter.
- **Hardware**: Large N requires wide registers. The method is purely logical, avoiding arithmetic abstractions, making it ideal for hardware implementation.
- **Signed Integers**: Requires additional handling for negative values.

## License

File is part of project [Fasma](https://github.com/xavetar/Fasma). Licensed under the Anti-Virus AND MIT AND the Apache License (Version 2.0). See [LICENSE](https://github.com/xavetar/Fasma/blob/main/LICENSE) for details.

## Author

Developed by [Stanislav Mikhailov (xavetar)](https://github.com/xavetar/).

## References

- HAKMEM (Richard Charles Schroeppel - Item 167) [MIT AIM-239](https://www.inwap.com/pdp10/hbaker/hakmem/hacks.html#item167)
- Bithacks ([Reverse the bits in a byte with 3 operations (64-bit multiply and modulus division](https://graphics.stanford.edu/~seander/bithacks.html#ReverseByteWith64BitsDiv))