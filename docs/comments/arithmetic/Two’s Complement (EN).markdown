The term is the result of a general property possessing a moment of **collapsing**, assuming a **symmetric ratio of magnitudes in binary logic-arithmetic form**, during **operations over reflections**. It does not encode numbers, signs, or operations; the term expresses the form of a **linear ratio** between **binary representation**, expressed using **logical reduction of inversion** to a **symmetric** and **identical** ratio of **symmetric linear magnitudes**, capable of being expressed as a **linear symmetric ratio**, through **operations over reflections**:

1) Where **expansion and contraction** preserves **proportions**, implying that the **linear ratio** is already **predetermined** over these **linear magnitudes**
2) Where the result of an **operation** over **linear magnitudes** assumes the possibility to be expressed through a **linear ratio**
3) Where the result of an **operation** logic-arithmetically assumes **symmetry** or the possibility to be expressed through a **symmetric ratio**
4) Where the **symmetric ratio of linear magnitudes** is expressed symmetrically, relative to one **numerical plane**
5) Where the **linear ratio** is determined not by the absolute **binary representation**, but by its **position** and **direction** relative to a **fixed** center of the neutral point on the **numerical plane**
6) Where **interpretation** determines the value of the **linear ratio** of magnitudes (signed, unsigned)

The form **logically** expressing the **result of inversion**, through reducing it to a symmetric **logic-arithmetic** operation (`adjusting logic-arithmetically its result to the answer`). Discretely expressing the continuous ratio of **linear magnitudes and their symmetric or asymmetric ratios**, capable of being expressed **symmetrically**. Regardless of whether **arithmetic**, **logic**, or **nothing at all** will be defined over the form.

## Definitions

- **Complementarity** — this is an **asymmetric reflection**, obeying the identity of connectivity, striving for **annihilation** of the original structure to a state of rest (zero or another definition of rest).
- **Collapsing** — the point of singularity, the beginning and end of `reflection`.
- **Identity of connectivity** — for `i8`/`u8`, **(~V + 0x01) ≡ ~(V - 0x01)** ≡ **(~V - 0xFF) ≡ ~(V + 0xFF)** , reflects formation and cancellation, expansion and contraction, defines the connectivity of **asymmetric reflections** (as `abs` and `backward abs` or `V ⇌ -V`) - reflecting the `phase` of displacement relative to the moment/center of inversion.
- **Field cyclicity** — for `i8`/`u8`, the field is closed in a ring, where `V + 0x01 ⇌ V - 0xFF` ≡ `V - 0x01 ⇌ V + 0xFF` or `+1 = -255` ⇌ `-1 = +255`, mutual equivalence of representations (valid for simulations using **TC** instructions for **signed** and **unsigned** data types).
- **Operations over reflection** - performing operations over reflections in an 8-bit ring, the values completely destroy each other (**collapse**):

    - For signed reflections (`[-128, -1] => [-128, 127] => [0, 127]`):

        1) Addition (**expansion**) paired with `reflections` (`A = V`, `B = -V`):
            
                - `-x = ~(B - 1) = ~(B + 255)`:
                
                    **A ⊕ B** ≡ **A ⊖ ¬(B ⊖ 1)**
                    **A + B** ≡ **A - ~(B - 1)**
                    
                    **A ⊕ B** ≡ **A ⊖ ¬(B ⊕ 255)**
                    **A + B** ≡ **A - ~(B + 255)**
                
                - `-x = (~B + 1) = (~B - 255)`:
                
                    **A ⊕ B** ≡ **A ⊖ (¬B ⊕ 1)**
                    **A + B** ≡ **A - (~B + 1)**
                    
                    **A ⊕ B** ≡ **A ⊖ (¬B ⊖ 255)**
                    **A + B** ≡ **A - (~B - 255)** 
                
        2) Subtraction (**contraction**) paired with `number` (`A = V`, `B = V`):
            
                - `-x = ~(B - 1) = ~(B + 255)`:
                
                    **A ⊖ B** ≡ **A ⊕ ~(B ⊖ 1)**
                    **A - B** ≡ **A + ~(B - 1)**
                    
                    **A ⊖ B** ≡ **A ⊕ ~(B ⊕ 255)**
                    **A - B** ≡ **A + ~(B + 255)**
                    
                - `-x = (~B + 1) = (~B - 255)`:
                
                    **A ⊖ B** ≡ **A ⊕ (~B ⊕ 1)**
                    **A - B** ≡ **A + (~B + 1)**
                    
                    **A ⊖ B** ≡ **A ⊕ (~B ⊖ 255)**
                    **A - B** ≡ **A + (~B - 255)**

    - For unsigned reflections (`[0, 255]`):

        1) Addition (**expansion**) paired with `reflections (magnitudes of reflections - inversely proportional - to magnitudes of presence)` (`A = V`, `B = -V`):
            
                - `-x = ~(B - 1) = ~(B + 255)`:
                
                    **A ⊕ B** ≡ **A ⊖ ¬(B ⊖ 1)** 
                    **A + B** ≡ **A - ~(B - 1)**
                    
                    **A ⊕ B** ≡ **A ⊖ ¬(B ⊕ 255)**
                    **A + B** ≡ **A - ~(B + 255)**
                
                - `-x = (~B + 1) = (~B - 255)`:
                
                    **A ⊕ B** ≡ **A ⊖ (¬B ⊕ 1)**
                    **A + B** ≡ **A - (~B + 1)** 
                    
                    **A ⊕ B** ≡ **A ⊖ (¬B ⊖ 255)**
                    **A + B** ≡ **A - (~B - 255)**
            
            Addition upward does not create a trap for imitating **unsigned** logic **LPU:LAU:ALU** and **TC**, through its simulation.
            
        2) Subtraction (**contraction**) paired with `number` (`A = V`, `B = V`):
                
                - `-x = ~(B - 1) = ~(B + 255)`:

                    **if A < B { B ⊖ A } else { A ⊖ B }** ≡ **if A < B { ¬A ⊕ (B ⊕ 1) } else { A ⊕ ¬(B ⊖ 1) }**
                    **if A < B { B - A } else { A - B }** ≡ **if A < B { ~A + (B + 1) } else { A + ~(B - 1) }**
                    
                    **if A < B { B ⊖ A } else { A ⊖ B }** ≡ **if A < B { ¬A ⊕ (B ⊖ 255) } else { A ⊕ ¬(B ⊕ 255) }**
                    **if A < B { B - A } else { A - B }** ≡ **if A < B { ~A + (B - 255) } else { A + ~(B + 255) }**
                    
                    **if A < B { B ⊖ A } else { A ⊖ B }** ≡ **if A < B { B ⊕ (¬A ⊕ 1) } else { A ⊕ ¬(B ⊖ 1) }**
                    **if A < B { B - A } else { A - B }** ≡ **if A < B { B + (~A + 1) } else { A + ~(B - 1) }**
                    
                    **if A < B { B ⊖ A } else { A ⊖ B }** ≡ **if A < B { B ⊕ (¬A ⊖ 255) } else { A ⊕ ¬(B ⊕ 255) }**
                    **if A < B { B - A } else { A - B }** ≡ **if A < B { B + (~A - 255) } else { A + ~(B + 255) }**
                    
                    **if A < B { ¬A ⊖ ¬B } else { A ⊖ B }** ≡ **if A < B { ¬A ⊕ (B ⊕ 1) } else { A ⊕ ¬(B ⊖ 1) }**
                    **if A < B { ~A - ~B } else { A - B }** ≡ **if A < B { ~A + (B + 1) } else { A + ~(B - 1) }**
                    
                    **if A < B { ¬A ⊖ ¬B } else { A ⊖ B }** ≡ **if A < B { ¬A ⊕ (B ⊖ 255) } else { A ⊕ ¬(B ⊕ 255) }**
                    **if A < B { ~A - ~B } else { A - B }** ≡ **if A < B { ~A + (B - 255) } else { A + ~(B + 255) }**
                    
                    **if A < B { ¬A ⊖ ¬B } else { A ⊖ B }** ≡ **if A < B { B ⊕ (¬A ⊕ 1) } else { A ⊕ ¬(B ⊖ 1) }**
                    **if A < B { ~A - ~B } else { A - B }** ≡ **if A < B { B + (~A + 1) } else { A + ~(B - 1) }**
                    
                    **if A < B { ¬A ⊖ ¬B } else { A ⊖ B }** ≡ **if A < B { B ⊕ (¬A ⊖ 255) } else { A ⊕ ¬(B ⊕ 255) }**
                    **if A < B { ~A - ~B } else { A - B }** ≡ **if A < B { B + (~A - 255) } else { A + ~(B + 255) }**
                
                - `-x = (~B + 1) = (~B - 255)`:
                
                    **if A < B { B ⊖ A } else { A ⊖ B }** ≡ **if A < B { ¬A ⊕ (B ⊕ 1) } else { A ⊕ (¬B ⊕ 1) }**
                    **if A < B { B - A } else { A - B }** ≡ **if A < B { ~A + (B + 1) } else { A + (~B + 1) }**
                    
                    **if A < B { B ⊖ A } else { A ⊖ B }** ≡ **if A < B { ¬A ⊕ (B ⊖ 255) } else { A ⊕ (¬B ⊖ 255) }**
                    **if A < B { B - A } else { A - B }** ≡ **if A < B { ~A + (B - 255) } else { A + (~B - 255) }**
                    
                    **if A < B { B ⊖ A } else { A ⊖ B }** ≡ **if A < B { B ⊕ (¬A ⊕ 1) } else { A ⊕ (¬B ⊕ 1) }**
                    **if A < B { B - A } else { A - B }** ≡ **if A < B { B + (~A + 1) } else { A + (~B + 1) }**
                    
                    **if A < B { B ⊖ A } else { A ⊖ B }** ≡ **if A < B { B ⊕ (¬A ⊖ 255) } else { A ⊕ (¬B ⊖ 255) }**
                    **if A < B { B - A } else { A - B }** ≡ **if A < B { B + (~A - 255) } else { A + (~B - 255) }**
                    
                    **if A < B { ¬A ⊖ ¬B } else { A ⊖ B }** ≡ **if A < B { ¬A ⊕ (B ⊕ 1) } else { A ⊕ (¬B ⊕ 1) }**
                    **if A < B { ~A - ~B } else { A - B }** ≡ **if A < B { ~A + (B + 1) } else { A + (~B + 1) }**
                    
                    **if A < B { ¬A ⊖ ¬B } else { A ⊖ B }** ≡ **if A < B { ¬A ⊕ (B ⊖ 255) } else { A ⊕ (¬B ⊖ 255) }**
                    **if A < B { ~A - ~B } else { A - B }** ≡ **if A < B { ~A + (B - 255) } else { A + (~B - 255) }**
                    
                    **if A < B { ¬A ⊖ ¬B } else { A ⊖ B }** ≡ **if A < B { B ⊕ (¬A ⊕ 1) } else { A ⊕ (¬B ⊕ 1) }**
                    **if A < B { ~A - ~B } else { A - B }** ≡ **if A < B { B + (~A + 1) } else { A + (~B + 1) }**
                    
                    **if A < B { ¬A ⊖ ¬B } else { A ⊖ B }** ≡ **if A < B { B ⊕ (¬A ⊖ 255) } else { A ⊕ (¬B ⊖ 255) }**
                    **if A < B { ~A - ~B } else { A - B }** ≡ **if A < B { B + (~A - 255) } else { A + (~B - 255) }**
                    
            - Additional formulas: `A < B` : `~(A - ~B)`, `~(A - B) + 1`, `~(A + ~(B - 1)) + 1`, `~(A + (~B + 1)) + 1`, `~(A + ~(B + 255)) - 255`, `~(A + (~B - 255)) - 255`.
            
            Subtraction downward creates a trap for imitating **unsigned** logic **LPU:LAU:ALU** and **TC**, through its simulation. Subtraction in **unsigned** logic corresponds to the **absolute distance** between numbers.
            
            Through the mathematics of **TC** it is explained by the ring **ℤ/256ℤ** (where borrow arises from the “virtual bit”: 2⁸ for 8-bit numbers), however **mathematics** possesses symmetry and is a derivative of **arithmetic** based on **logic**, and any **arithmetic-mathematical** justification is **incomplete and secondary**. Using **mathematics or arithmetic** to prove **logic** is akin to the idea: `using nails to chop an axe`.
            
            Compilers use simplified **logic** for **unsigned** data types — creating conditions for a **logical simulation error (for unsigned types)**: `0 (0x00) - 255 (0xFF) = 1 (0x01)`, which is logically equivalent to the **law of double negation** (`involution`) for signed numbers: `0 (0x00) - (-1) (0xFF) = 1 (0x01)`.
            
            Ironic joke: `A friend had 0x00 rubles, gave a friend 0xFF rubles, how much money does he have now?`.
            
        Downward overflow is **prohibited**! In **unsigned** arithmetic, one cannot speak of the **absence** of something, only of its **presence**. However, the **absence** of something is **expressed by the presence of its opposite**.

## Description

**Symmetric reflection** of the binary representation of a **linear magnitude** is reproduced using **logical identities** (arithmetic operations expressed through logic that lead to a symmetric mapping of **linear magnitudes** on the **numerical plane**, resulting in **conformity** between **logic and arithmetic**):

- `-x = (~B + 1) = (~B - 255)`:

    - For signed reflections (`[-128, -1] => [-128, 127] => [0, 127]`):

        1) Logic-arithmetic addition of one to an inverted **positive number** reactively forms a **complementary representation** (`corrects the distortion that occurred, for symmetry of the linear ratio`). It brings the number closer to **0**, compensating the **arithmetic asymmetry of logical inversion** relative to the **numerical plane** for the subsequent operation over **linear reflections of magnitudes** within the **numerical plane**: `(~1 + 1) = -1 (0xFF)`
        2) Logic-arithmetic addition of one to an inverted **negative number** reactively forms a **complementary representation** (`corrects the distortion that occurred, for symmetry of the linear ratio`). It moves the number away from **0**, compensating the **arithmetic asymmetry of logical inversion** relative to the **numerical plane** for the subsequent operation over **linear reflections of magnitudes** within the **numerical plane**: `(~-1 + 1) = 1 (0x01)`

    - For unsigned reflections (`[0, 255]`):

        1) Logic-arithmetic addition of one to an inverted **number** reactively forms a **magnitude of reflection inversely proportional to the magnitude of presence** (`corrects the distortion that occurred, for symmetry of the linear ratio`). It compensates the **arithmetic asymmetry of logical inversion** relative to the **numerical plane** for the subsequent operation over **linear reflections of magnitudes** within the **numerical plane**: `(~1 + 1) = 255 (0xFF)`

- `-x = ~(B - 1) = ~(B + 255)`:

    - For signed reflections (`[-128, -1] => [-128, 127] => [0, 127]`):

        1) Logic-arithmetic subtraction of one from a **positive number** (before inversion) proactively forms a **complementary representation** (`prepares the distortion, for symmetry of the linear ratio`). It brings the number closer to **0** after inversion, compensating the **arithmetic asymmetry of logical inversion** relative to the **numerical plane** for the subsequent operation over **linear reflections of magnitudes** within the **numerical plane**: `~(1 - 1) = -1 (0xFF)`, `~(1 - 1) = 255 (0xFF)`
        2) Logic-arithmetic subtraction of one from a **negative number** (before inversion) proactively forms a **complementary representation** (`prepares the distortion, for symmetry of the linear ratio`). It moves the number away from **0** after inversion, compensating the **arithmetic asymmetry of logical inversion** relative to the **numerical plane** for the subsequent operation over **linear reflections of magnitudes** within the **numerical plane**: `~(-1 - 1) = 1 (0x01)`

    - For unsigned reflections (`[0, 255]`):

        1) Logic-arithmetic subtraction of one from a **number** (before inversion) proactively forms a **complementary representation** (`prepares the distortion, for symmetry of the linear ratio`). It compensates the **arithmetic asymmetry of logical inversion** relative to the **numerical plane** for the subsequent operation over **linear reflections of magnitudes** within the **numerical plane**: `~(1 - 1) = 255 (0xFF)`

## Fundamental transformations

### Formulas (SFG = `i8`/`u8`)

Addition (`a + b`): **a - ~(b - 0x01) ≡ a - (~b + 0x01)** ⇌ **a - ~(b + 0xFF) ≡ a - (~b - 0xFF)**

Subtraction (`a - b`): **a + (~b + 0x01) ≡ a + ~(b - 0x01)** ⇌ **a + (~b - 0xFF) ≡ a + ~(b + 0xFF)**

#### Addition

##### Data type: `i8`/`u8` (what happens inside **TC**-based hardware instructions)

| a   | b   | a + b = a + b (HEX)                | Result |
| --- | --- | ---------------------------------- | ------ |
| 5   | 3   | `5 + 3 = 0x05 + 0x03 = 0x08`       | 8      |
| 5   | -3  | `5 + (-3) = 0x05 + 0xFD = 0x02`    | 2      |
| -5  | -3  | `(-5) + (-3) = 0xFB + 0xFD = 0xF8` | -8     |

| a   | b   | a + b = a - ~(b - 1) (HEX)                     | Result |
| --- | --- | ---------------------------------------------- | ------ |
| 5   | 3   | `5 + ~(3 - 1) = 0x05 - ~(0x03 - 1) = 0x08`     | 8      |
| 5   | -3  | `5 + ~(-3 - 1) = 0x05 - ~(0xFD - 1) = 0x02`    | 2      |
| -5  | -3  | `(-5) + ~(-3 - 1) = 0xFB - ~(0xFD - 1) = 0xF8` | -8     |

| a   | b   | a + b = a - ~(b + 255) =  a - ~(b + (-1)) (HEX)                              | Result |
| --- | --- | ---------------------------------------------------------------------------- | ------ |
| 5   | 3   | `5 - ~(3 + 255) = 5 - ~(3 + (-1)) = 0x05 - ~(0x03 - 1) = 0x08`               | 8      |
| 5   | -3  | `5 - ~((-3) + 255) = = 5 - ~((-3) + (-1)) = 0x05 - ~(0xFD - 1) = 0x02`       | 2      |
| -5  | -3  | `(-5) - ~((-3) + 255) = = (-5) - ~((-3) + (-1)) = 0xFB - ~(0xFD - 1) = 0xF8` | -8     |

| a   | b   | a + b = a - (~b + 1) (HEX)                       | Result |
| --- | --- | ------------------------------------------------ | ------ |
| 5   | 3   | `5 - (~3 + 1) = 0x05 - ~(0x03 - 1) = 0x08`       | 8      |
| 5   | -3  | `5 - (~(-3) + 1) = 0x05 - ~(0xFD - 1) = 0x02`    | 2      |
| -5  | -3  | `(-5) - (~(-3) + 1) = 0xFB - ~(0xFD - 1) = 0xF8` | -8     |

| a   | b   | a + b = a - (~b - 255) = a - (~b - (-1)) (HEX)                             | Result |
| --- | --- | -------------------------------------------------------------------------- | ------ |
| 5   | 3   | `5 - (~3 - 255) = 5 - (~3 - (-1)) = 0x05 - ~(0x03 - 1) = 0x08`             | 8      |
| 5   | -3  | `5 - (~(-3) - 255) = 5 - (~(-3) - (-1)) = 0x05 - ~(0xFD - 1) = 0x02`       | 2      |
| -5  | -3  | `(-5) - (~(-3) - 255) = (-5) - (~(-3) - (-1)) = 0xFB - ~(0xFD - 1) = 0xF8` | -8     |

#### Subtraction

##### Data type: `i8`/`u8` (what happens inside **TC**-based hardware instructions)

| a   | b   | a - b = a - b (HEX)                | Result |
| --- | --- | ---------------------------------- | ------ |
| 5   | 3   | `5 - 3 = 0x05 - 0x03 = 0x02`       | 2      |
| 5   | -3  | `5 - (-3) = 0x05 - 0xFD = 0x08`    | 8      |
| -5  | -3  | `(-5) - (-3) = 0xFB - 0xFD = 0xFE` | -2     |

| a   | b   | a - b = a + (~b + 1) (HEX)                | Result |
| --- | --- | ----------------------------------------- | ------ |
| 5   | 3   | `5 + (~3 + 1) = 0x05 + 0xFD = 0x02`       | 2      |
| 5   | -3  | `5 + (~(-3) + 1) = 0x05 + 0x03 = 0x08`    | 8      |
| -5  | -3  | `(-5) + (~(-3) + 1) = 0xFB + 0x03 = 0xFE` | -2     |

| a   | b   | a - b = a + (~b - 255) = a + (~b - (-1)) (HEX)                      | Result |
| --- | --- | ------------------------------------------------------------------- | ------ |
| 5   | 3   | `5 + (~3 - 255) = 5 + (~3 - (-1)) = 0x05 + 0xFD = 0x02`             | 2      |
| 5   | -3  | `5 + (~(-3) - 255) = 5 + (~(-3) - (-1)) = 0x05 + 0x03 = 0x08`       | 8      |
| -5  | -3  | `(-5) + (~(-3) - 255) = (-5) + (~(-3) - (-1)) = 0xFB + 0x03 = 0xFE` | -2     |

| a   | b   | a - b = a + ~(b - 1) (HEX)              | Result |
| --- | --- | --------------------------------------- | ------ |
| 5   | 3   | `5 + ~(3 - 1) = 0x05 + 0xFD = 0x02`     | 2      |
| 5   | -3  | `5 + ~(-3 - 1) = 0x05 + 0x03 = 0x08`    | 8      |
| -5  | -3  | `(-5) + ~(-3 - 1) = 0xFB + 0x03 = 0xFE` | -2     |

| a   | b   | a - b = a + ~(b + 255) = a + ~(b + (-1)) (HEX)                      | Result |
| --- | --- | ------------------------------------------------------------------- | ------ |
| 5   | 3   | `5 + ~(3 + 255) = 5 + ~(3 + (-1)) = 0x05 + 0xFD = 0x02`             | 2      |
| 5   | -3  | `5 + ~((-3) + 255) = 5 + ~((-3) + (-1)) = 0x05 + 0x03 = 0x08`       | 8      |
| -5  | -3  | `(-5) + ~((-3) + 255) = (-5) + ~((-3) + (-1)) = 0xFB + 0x03 = 0xFE` | -2     |

Implicitly arises an **identity** for **linear symmetric ratios** (when `left operand < right operand`):

1) `0_u8 - 255_u8 => 0_i8 - (-1_i8) = 1`
2) `0_u8 - 1_u8 => 0_i8 - 1_i8 = -1`

This reveals itself as an **unsigned data types simulation error**: in **unsigned** arithmetic, one cannot speak of the **absence** of something, only of its **presence**.

## Operations in the model (PL)

| Task                                                          | Formula                                                            | Note                                                                                                                              |
| ------------------------------------------------------------- | ------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------- |
| Forced formation of complemented representation from `v`      | `N = ilog2(V) + 1`<br>`R = ((-1) ^ V) ^ ((1 << N) - 1)`            | Applicable when in another environment the number had a different representation with sign extension (byte stream, C, Rust, hardware register). |
| Formation and cancellation of complementary representation    | `R = (~V + 1)` ≡ `R = (~R + 1)`<br>`R = ~(V - 1)` ≡ `R = ~(R - 1)` | Applicable for identical transformations within the standard asymmetric set of definitions.                                       |

## Examples of minimal functions

### Formalization of abs (|x|)

#### General idea of the algorithm (with sign-propagation instruction available):

- For 32-bit:

1. x >> 31 → arithmetic shift right (unlike logical shifts (sll, srl), it propagates the sign)
        → the most significant bit (sign) is extended (sra; there is no sla — the sign is at the beginning of the register, and sla would contradict x86 architecture)
        → for negative: 0xFFFFFFFF (-1)
        → for positive: 0x00000000 (0)
2. (x ^ mask) - mask
        → if x ≥ 0 and x ≤ (2**(32-1) - 1):
            (x ^ 0) - 0 = x - 0 = x
        → if x ≥ -(2**(32-1)) and x ≤ 0:
            (x ^ (-1)) - (-1) = ~x + 1

#### Implementation:

- Equivalent in C:

```c
static inline __attribute__((always_inline, used))
int8_t abs(int8_t x) {
    /*
    Correct |x| for a value in two's complement
    */
    if (x < 0) { return !x + 0x01; } else { return x; }
}
```

```c
static inline __attribute__((always_inline, used))
int8_t abs(int8_t x) {
    /*
    Correct |x| for a value in two's complement
    */
    int8_t mask = x >> 7;
    return (mask ^ x) - mask;
}
```

- Equivalent in Rust:

```rust
#[unsafe(no_mangle)]
fn abs(x: i8) -> i8 {
    /*
    Correct |x| for a value in two's complement
    */
    if x < 0 { return !x + 0x01; } else { return x; }
}
```

```rust
#[unsafe(no_mangle)]
fn abs(x: i8) -> i8 {
    /*
    Correct |x| for a value in two's complement
    */
    let mask: i8 = x >> 7;
    return ((mask ^ x) - mask);
}
```

- Equivalent in Python:

```python
def abs_two_complement(x: int, bits: int = 32) -> int:
    """
    Correct |x| for a value in two's complement (to ±2^{bits-1}).
    """
    min_u: int = -1 ^ ((1 << (bits - 1)) - 1)
    max_u: int = 0 ^ ((1 << (bits - 1)) - 1)
    if not (min_u <= x <= max_u):
        raise ValueError("Value out of signed bits-bit range")
    # The most significant sign bit = 1
    if x >> (bits - 1):
        return (((-1) ^ x) + 1) & max_u
    return x
```

- Equivalent in Python:

```python
def abs_two_complement(x: int, bits: int = 32) -> int:
    """
    Correct |x| for a value in two's complement (covering ±2^{bits})-1).
    """
    min_u: int = -1 ^ ((1 << bits) - 1)
    max_u: int = 0 ^ ((1 << bits) - 1)
    if not (min_u <= x <= max_u):
        raise ValueError("Value out of unsigned bits-bit range")
    # The most significant sign bit = 1
    if x >> (bits - 1):
        x = (-1 ^ x) + 1
        # We guarantee a positive result
        return x & ((1 << (bits - 1)) - 1)
    return x
```

### Formalization babs (-x)

#### Implementation:

- Equivalent in C:

```c
static inline __attribute__((always_inline, used))
int8_t babs(int8_t x) { // -x
    /*
    Correct -x for value in two's complement
    */
    return ~x + 0x01;
}
```

```c
static inline __attribute__((always_inline, used))
uint8_t babs(uint8_t x) {
    /*
    Correctly returns the magnitude of reflection inversely proportional to the magnitude of presence for unsigned data type in two's complement
    */
    return ~x + 0x01;
}
```

- Equivalent in Rust:

```rust
#[unsafe(no_mangle)]
#[inline(always)]
fn babs(x: i8) -> i8 { // -x
    /*
    Correct -x for value in two's complement
    */
    return !x + 0x01;
}
```

```rust
#[unsafe(no_mangle)]
#[inline(always)]
fn babs(x: u8) -> u8 { // -x
    /*
    Correctly returns the magnitude of reflection inversely proportional to the magnitude of presence for unsigned data type in two's complement
    */
    return !x + 0x01;
}
```

- Equivalent in Python:

```python
def babs(x: int, bits: int = 32) -> int: # -x
    """
    Correct -x for value in two's complement (to ±2^{bits-1})
    """
    min_u: int = -1 << (bits - 1)
    max_u: int = (1 << (bits - 1)) - 1
    if not (min_u <= x <= max_u):
        raise ValueError("Value out of signed bits-bit range")
    # The most significant sign bit = 1
    if x >> (bits - 1):
        if x == min_u:
            return min_u
        else:
            return (((-1) ^ x) + 1) & max_u
    else:
        return (((-1) ^ x) + 1) | min_u
```

```python
def babs(x: int, bits: int = 32) -> int: # -x
    """
    Correct -x for value in two's complement (covering ±2^{bits})-1)
    """
    min_u: int = -1 << bits
    max_u: int = (1 << bits) - 1
    if not (min_u <= x <= max_u):
        raise ValueError("Value out of unsigned bits-bit range")
    # The most significant sign bit = 1
    if x >> (bits - 1):
        if x == min_u:
            return min_u
        else:
            return (((-1) ^ x) + 1) & max_u
    else:
        return (((-1) ^ x) + 1) | min_u
```

### Formalization nabs (-|x|)

#### Implementation:

- Equivalent in C:

```c
#include <stdint.h>

int8_t nabs(uint8_t x) {
    /*
    Correct negative abs for the value
    */
    if (x <= UINT8_C(0x80)) {
        return (int8_t) (~x + UINT8_C(0x01));
    } else {
        return (int8_t) x;
    }
}
```

```c
#include <stdint.h>

int8_t nabs(uint8_t x) {
    /*
    Correct negative abs for the value
    */
    if (x <= UINT8_C(0x80)) {
        return (int8_t) (~x + UINT8_C(0x01));
    } else {
        return (int8_t) (~(~x - UINT8_MAX) + UINT8_C(0x01));
    }
}
```

- Equivalent in Rust:

```rust
#[unsafe(no_mangle)]
fn nabs(x: u8) -> i8 {
    /*
    Correct negative abs for the value
    */
    if x <= 0x80 {
        return (!x + 0x01) as i8;
    } else {
        return (x as i8);
    }
}
```

```rust
#[unsafe(no_mangle)]
fn nabs(x: u8) -> i8 {
    /*
    Correct negative abs for the value
    */
    if x <= 0x80 {
        return (!x + 0x01) as i8;
    } else {
        return (!(!x - u8::MAX) + 0x01) as i8;
    }
}
```

### Interpretation of the negative representation from the variable gamma field

#### Implementation:

- Equivalent in C:

```c
#include <stdint.h>

static inline __attribute__((always_inline, used))
int8_t uas(uint8_t x) {
    /*
    Correct unsigned as signed for value
    */
    if (x == 0) { return x; }
    return (int8_t) (
        (
            UINT8_C(0xFF) ^ x
        ) ^ (
            (uint8_t) ((0x0001U << (0x0020U - __builtin_clz(x))) - 0x0001U)
        )
    );
}
```

```c
#include <stdint.h>

static inline __attribute__((always_inline, used))
int8_t uas(uint8_t x) {
    /*
    Correct unsigned as signed for value, distributing the sign
    */
    return (int8_t) (
        (
            UINT8_C(0xFF) ^ x
        ) ^ (
            (x | x >> 1)
        | ((x | x >> 1) >> 2)
        | (((x | x >> 1) | ((num | num >> 1) >> 2)) >> 4)
        )
    );
}
```

```c
#include <stdint.h>

static inline __attribute__((always_inline, used)) 
int64_t uas(uint64_t value, uint8_t sfg) {
    /* 
    * sign_shift: distance to the edge of the number plane.
    * We use 64 bits (0x40), subtracting the subfield length.
    * Masking (sfg & 0x3F) ensures that we do not exceed 63.
    * Example (subfield group): SFG = 8 (for uint8_t)

    */
    uint8_t sign_shift = UINT8_C(0x0040) - (sfg & UINT8_C(0x3F));

    /*
    * 1. A left shift moves the sign bit of the subfield to the position of the i64 sign bit.
    * 2. An arithmetic right shift “pulls” this bit back, 
    *    filling the gap and restoring linear symmetry (propagating the sign).

    */
    return (((int64_t) value) << sign_shift) >> sign_shift;
}
```

- Equivalent in Rust:

```rust
#[unsafe(no_mangle)]
#[inline(always)]
fn uas(x: u8) -> i8 {
    /*
    Correct unsigned as signed for value, distributing the sign
    */
    return (
        (
            0xFF_u8 ^ x
        ) ^ (
            (
                (0x00000001_u32 << (0x00000008_u32 - x.leading_zeros())) - 0x00000001_u32
            ) as u8
        )
    ) as i8;
}
```

```rust
#[unsafe(no_mangle)]
#[inline(always)]
fn uas(x: u8) -> i8 {
    /*
    Correct unsigned as signed for value, distributing the sign
    */
    return (
        (
            0xFF_u8 ^ x
        ) ^ (
            (
                (0x00000001_u32 << (x.ilog2() + 0x00000001_u32)) - 0x00000001_u32
            ) as u8
        )
    ) as i8;
}
```

```rust
#[unsafe(no_mangle)]
#[inline(always)]
fn uas(x: u8) -> i8 {
    /*
    Correct unsigned as signed for value, distributing the sign
    */
    return (
        (
            0xFF_u8 ^ x
        ) ^ (
            (x | x >> 1)
        | ((x | x >> 1) >> 2)
        | (((x | x >> 1) | ((x | x >> 1) >> 2)) >> 4)
        )
    ) as i8;
}
```

```rust
#[unsafe(no_mangle)]
#[inline(always)]
pub const fn uas<const SFG: u8>(value: u128) -> i128 {
    /*
    Correct unsigned as signed for the value of any length subfield within the length field of the data type, distributing the sign
    */
    let sign_shift: i128 = 0x80_i128 - (SFG & 0x7F_u8) as i128;
    return ((value as i128) << sign_shift) >> sign_shift;
}
```

- Equivalent in Python:

```python
def uas(x: int, bits: int) -> int:
    min_u: int = 0
    max_u: int = (1 << n) - 1
    if not (min_u <= x <= max_u):
        raise ValueError("Value out of signed bits-bit range")
    return ((-1) & (-1 << n)) | x
```

```python
def uas(x: int, bits: int) -> int:
    min_u: int = 0
    max_u: int = (1 << n) - 1
    if not (min_u <= x <= max_u):
        raise ValueError("Value out of signed bits-bit range")
    return (((-1) ^ x) ^ ((1 << n) - 1))
```

## License

File is part of project [Fasma](https://github.com/xavetar/Fasma). Licensed under the Anti-Virus AND MIT AND the Apache License (Version 2.0). See [LICENSE](https://github.com/xavetar/Fasma/blob/main/LICENSE) for details.

## Author

Developed by [Stanislav Mikhailov (xavetar)](https://github.com/xavetar/).

## Translation

Thanks to [Grok](https://grok.com/)
