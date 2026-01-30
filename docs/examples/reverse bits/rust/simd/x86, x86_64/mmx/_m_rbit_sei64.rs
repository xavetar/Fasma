/*
* ╔═════════════════════════════════════════════════════════════════════════════════════╗
* ║                                 ANTI-VIRUS LICENSE                                  ║
* ║                                                                                     ║
* ║                          Code Shielded from Viral Threats                           ║
* ╟─────────────────────────────────────────────────────────────────────────────────────╢
* ║  Copyright Notice                                                                   ║
* ║                                                                                     ║
* ║  Copyright (c) 2026 Stanislav Mikhailov (xavetar)                                   ║
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

/*
 * Reverse bits: Necromancy Edition!
 *
 * - Install toolchain
 *
 * rustup default stable-i686-unknown-linux-gnu --force-non-host
 * rustup default stable-x86_64-unknown-linux-gnu --force-non-host
 *
 * - Buld only file
 *
 * rustc -C opt-level=3 --crate-type bin --target i686-unknown-linux-gnu -o _m_rbit_sei64 _m_rbit_sei64.rs
 * rustc -C opt-level=3 --crate-type bin --target x86_64-unknown-linux-gnu -o _m_rbit_sei64 _m_rbit_sei64.rs
 *
*/

extern crate core;

#[cfg(target_arch = "x86")]
use core::{
    arch::{
        asm,
        x86::*
    },
};

#[cfg(target_arch = "x86_64")]
use core::{
    arch::{
        asm,
        x86_64::*
    },
};

#[repr(C, align(8))]
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct AlignI64 {
    first: i32,
    second: i32
}

static MASK_NIBBLE: AlignI64 = AlignI64 { first: 0x0F0F0F0F_i32, second: 0x0F0F0F0F_i32 };
static MASK_2BIT: AlignI64 = AlignI64 { first: 0x33333333_i32, second: 0x33333333_i32 };
static MASK_BIT: AlignI64 = AlignI64 { first: 0x55555555_i32, second: 0x55555555_i32 };

#[unsafe(no_mangle)]
unsafe fn _llvm_rbit_sei64(value: i64) -> i64 {
    return value.reverse_bits();
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[unsafe(no_mangle)]
unsafe fn _rbit_sei64_swar(mut value: i64) -> i64 {
    #[cfg(target_arch = "x86")]
    {
        value = (
            (
                _bswap(value as i32) as i64 & 0x00000000FFFFFFFF
            ) << 0x20
        )
        |
        (
            _bswap(((value as u64) >> 0x20) as i32) as i64 & 0x00000000FFFFFFFF
        );
    }

    #[cfg(target_arch = "x86_64")]
    {
        value = _bswap64(value)
    }

    value = (value & 0x0F0F0F0F0F0F0F0F_i64) << 4 | (value >> 4) & 0x0F0F0F0F0F0F0F0F_i64;
    value = (value & 0x3333333333333333_i64) << 2 | (value >> 2) & 0x3333333333333333_i64;
    value = (value & 0x5555555555555555_i64) << 1 | (value >> 1) & 0x5555555555555555_i64;

    return value as i64;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline(never)]
#[unsafe(no_mangle)]
unsafe fn _m_rbit_sei64_mmx_pand(mut value: i64) -> i64 {
    #[cfg(target_arch = "x86")]
    {
        value = (
            (
                _bswap(value as i32) as i64 & 0x00000000FFFFFFFF
            ) << 0x20
        )
        |
        (
            _bswap(((value as u64) >> 0x20) as i32) as i64 & 0x00000000FFFFFFFF
        );
    }

    #[cfg(target_arch = "x86_64")]
    {
        value = _bswap64(value)
    }

    #[cfg(target_arch = "x86")]
    {
        asm!(
            "call 1f",
            "1:",
            "popl %ecx",

            "movq {mn}-1b(%ecx), %mm2",          // 0x0F0F0F0F0F0F0F0F
            "movq {m2}-1b(%ecx), %mm3",          // 0x3333333333333333
            "movq {mb}-1b(%ecx), %mm4",          // 0x5555555555555555

            // Оптимизация некозможна, компилятор не передаёт значение в регистре %mmN
            "movq ({value}), %mm0",              // mm0 = value
            "movq ({value}), %mm1",              // mm1 = value

            "pand %mm2, %mm1",                   // Применяем маску 0x0F к полу-байтам
            "psllw $4, %mm1",                    // Сдвигаем влево на 4 бита (полубайты)
            "psrlw $4, %mm0",                    // Сдвигаем вправо на 4 бита (полубайты)
            "pand %mm2, %mm0",                   // Применяем маску 0x0F к полу-байтам
            "por %mm1, %mm0",                    // Объединяем

            "movq %mm0, %mm1",                   // mm1 = value
            "pand %mm3, %mm1",                   // Применяем маску 0x33 к полу-байтам
            "psllw $2, %mm1",                    // Сдвигаем влево на 2 бита (полубайты)
            "psrlw $2, %mm0",                    // Сдвигаем вправо на 2 бита (полубайты)
            "pand %mm3, %mm0",                   // Применяем маску 0x33 к полу-байтам
            "por %mm1, %mm0",                    // Объединяем

            "movq %mm0, %mm1",                   // mm1 = value
            "pand %mm4, %mm1",                   // Применяем маску 0x55 к полу-байтам
            "paddb %mm1, %mm1",                  // Сдвигаем влево на 1 бит (полубайты)
            "psrlw $1, %mm0",                    // Сдвигаем вправо на 1 бит (полубайты)
            "pand %mm4, %mm0",                   // Применяем маску 0x55 к полу-байтам
            "por %mm1, %mm0",                    // Объединяем

            "movq %mm0, ({value})",              // Сохраняем результат на стек
            "emms",                              // Чистим MMX (FPU)
            options(nostack, att_syntax),
            value = in(reg) &mut value,
            mn = sym MASK_NIBBLE,                // Компилятор после оптимизации не пересчитывает относительное смещение, только (inline(never))!
            m2 = sym MASK_2BIT,                  // Компилятор после оптимизации не пересчитывает относительное смещение, только (inline(never))!
            mb = sym MASK_BIT,                   // Компилятор после оптимизации не пересчитывает относительное смещение, только (inline(never))!
            out("ecx") _
        );
    }

    #[cfg(target_arch = "x86_64")]
    {
        asm!(
            "movq {mn}(%rip), %mm2",             // 0x0F0F0F0F0F0F0F0F
            "movq {m2}(%rip), %mm3",             // 0x3333333333333333
            "movq {mb}(%rip), %mm4",             // 0x5555555555555555

            // Оптимизация некозможна, компилятор не передаёт значение в регистре %mmN
            "movq {value:r}, %mm0",              // mm0 = value
            "movq {value:r}, %mm1",              // mm1 = value

            "pand %mm2, %mm1",                   // Применяем маску 0x0F к полу-байтам
            "psllw $4, %mm1",                    // Сдвигаем влево на 4 бита (полубайты)
            "psrlw $4, %mm0",                    // Сдвигаем вправо на 4 бита (полубайты)
            "pand %mm2, %mm0",                   // Применяем маску 0x0F к полу-байтам
            "por %mm1, %mm0",                    // Объединяем

            "movq %mm0, %mm1",                   // mm1 = value
            "pand %mm3, %mm1",                   // Применяем маску 0x33 к полу-байтам
            "psllw $2, %mm1",                    // Сдвигаем влево на 2 бита (полубайты)
            "psrlw $2, %mm0",                    // Сдвигаем вправо на 2 бита (полубайты)
            "pand %mm3, %mm0",                   // Применяем маску 0x33 к полу-байтам
            "por %mm1, %mm0",                    // Объединяем

            "movq %mm0, %mm1",                   // mm1 = value
            "pand %mm4, %mm1",                   // Применяем маску 0x55 к полу-байтам
            "paddb %mm1, %mm1",                  // Сдвигаем влево на 1 бит (полубайты)
            "psrlw $1, %mm0",                    // Сдвигаем вправо на 1 бит (полубайты)
            "pand %mm4, %mm0",                   // Применяем маску 0x55 к полу-байтам
            "por %mm1, %mm0",                    // Объединяем

            "movq %mm0, {value:r}",              // Сохраняем результат в регистр общего назначения
            "emms",                              // Чистим MMX (FPU)
            options(nomem, nostack, att_syntax),
            value = in(reg) value,
            mn = sym MASK_NIBBLE,                // Компилятор после оптимизации не пересчитывает относительное смещение, только (inline(never))!
            m2 = sym MASK_2BIT,                  // Компилятор после оптимизации не пересчитывает относительное смещение, только (inline(never))!
            mb = sym MASK_BIT,                   // Компилятор после оптимизации не пересчитывает относительное смещение, только (inline(never))!
        );
    }

    return value;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline(never)]
#[unsafe(no_mangle)]
unsafe fn _m_rbit_sei64_mmx_pandn(mut value: i64) -> i64 {
    #[cfg(target_arch = "x86")]
    {
        value = (
            (
                _bswap(value as i32) as i64 & 0x00000000FFFFFFFF
            ) << 0x20
        )
        |
        (
            _bswap(((value as u64) >> 0x20) as i32) as i64 & 0x00000000FFFFFFFF
        );
    }

    #[cfg(target_arch = "x86_64")]
    {
        value = _bswap64(value)
    }

    #[cfg(target_arch = "x86")]
    {
        asm!(
            "call 1f",
            "1:",
            "popl %ecx",

            "movq {mn}-1b(%ecx), %mm2",            // 0x0F0F0F0F0F0F0F0F
            "movq {m2}-1b(%ecx), %mm3",            // 0x3333333333333333
            "movq {mb}-1b(%ecx), %mm4",            // 0x5555555555555555

            // Оптимизация некозможна, компилятор не передаёт значение в регистре %mmN
            "movq ({value}), %mm0",                // mm0 = value
            "movq ({value}), %mm1",                // mm1 = value

            "pand %mm2, %mm1",                     // Применяем маску 0x0F к полу-байтам
            "psllw $4, %mm1",                      // Сдвигаем влево на 4 бита (полубайты)
            "pandn %mm0, %mm2",                    // Применяем маску 0xF0 к полу-байтам и сохраняем в %mm2
            "psrlw $4, %mm2",                      // Сдвигаем вправо на 4 бита (полубайты)
            "por %mm2, %mm1",                      // Объединяем

            "movq %mm1, %mm0",                     // mm0 = value
            "pand %mm3, %mm1",                     // Применяем маску 0x33 к полу-байтам
            "psllw $2, %mm1",                      // Сдвигаем влево на 2 бита (полубайты)
            "pandn %mm0, %mm3",                    // Применяем маску 0xCC к полу-байтам и сохраняем в %mm3
            "psrlw $2, %mm3",                      // Сдвигаем вправо на 2 бита (полубайты)
            "por %mm3, %mm1",                      // Объединяем

            "movq %mm1, %mm0",                     // mm0 = value
            "pand %mm4, %mm1",                     // Применяем маску 0x55 к полу-байтам
            "paddb %mm1, %mm1",                    // Сдвигаем влево на 1 бит (полубайты)
            "pandn %mm0, %mm4",                    // Применяем маску 0xAA к полу-байтам и сохраняем в %mm4
            "psrlw $1, %mm4",                      // Сдвигаем вправо на 1 бита (полубайты)
            "por %mm4, %mm1",                      // Объединяем

            "movq %mm1, ({value})",                // Сохраняем результат на стек
            "emms",                                // Чистим MMX (FPU)
            options(nostack, att_syntax),
            value = in(reg) &mut value,
            mn = sym MASK_NIBBLE,                  // Компилятор после оптимизации не пересчитывает относительное смещение, только (inline(never))!
            m2 = sym MASK_2BIT,                    // Компилятор после оптимизации не пересчитывает относительное смещение, только (inline(never))!
            mb = sym MASK_BIT,                     // Компилятор после оптимизации не пересчитывает относительное смещение, только (inline(never))!
            out("ecx") _
        );
    }

    #[cfg(target_arch = "x86_64")]
    {
        asm!(
            "movq {mn}(%rip), %mm2",               // 0x0F0F0F0F0F0F0F0F
            "movq {m2}(%rip), %mm3",               // 0x3333333333333333
            "movq {mb}(%rip), %mm4",               // 0x5555555555555555

            // Оптимизация некозможна, компилятор не передаёт значение в регистре %mmN
            "movq {value:r}, %mm0",                // mm0 = value
            "movq {value:r}, %mm1",                // mm1 = value

            "pand %mm2, %mm1",                     // Применяем маску 0x0F к полу-байтам
            "psllw $4, %mm1",                      // Сдвигаем влево на 4 бита (полубайты)
            "pandn %mm0, %mm2",                    // Применяем маску 0xF0 к полу-байтам
            "psrlw $4, %mm2",                      // Сдвигаем вправо на 4 бита (полубайты)
            "por %mm2, %mm1",                      // Объединяем

            "movq %mm1, %mm0",                     // mm0 = value
            "pand %mm3, %mm1",                     // Применяем маску 0x33 к полу-байтам
            "psllw $2, %mm1",                      // Сдвигаем влево на 2 бита (полубайты)
            "pandn %mm0, %mm3",                    // Применяем маску 0xCC к полу-байтам и сохраняем в %mm3
            "psrlw $2, %mm3",                      // Сдвигаем вправо на 2 бита (полубайты)
            "por %mm3, %mm1",                      // Объединяем

            "movq %mm1, %mm0",                     // mm0 = value
            "pand %mm4, %mm1",                     // Применяем маску 0x55 к полу-байтам
            "paddb %mm1, %mm1",                    // Сдвигаем влево на 1 бит (полубайты)
            "pandn %mm0, %mm4",                    // Применяем маску 0xAA к полу-байтам и сохраняем в %mm4
            "psrlw $1, %mm4",                      // Сдвигаем вправо на 1 бита (полубайты)
            "por %mm4, %mm1",                      // Объединяем

            "movq %mm1, {value:r}",                // Сохраняем результат в регистр общего назначения
            "emms",                                // Чистим MMX (FPU)
            options(nomem, nostack, att_syntax),
            value = in(reg) value,
            mn = sym MASK_NIBBLE,                  // Компилятор после оптимизации не пересчитывает относительное смещение, только (inline(never))!
            m2 = sym MASK_2BIT,                    // Компилятор после оптимизации не пересчитывает относительное смещение, только (inline(never))!
            mb = sym MASK_BIT,                     // Компилятор после оптимизации не пересчитывает относительное смещение, только (inline(never))!
        );
    }

    return value;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[unsafe(no_mangle)]
unsafe fn _rbit_sei64_r32(value: i64) -> i64 {
    #[repr(C, align(8))]
    #[allow(dead_code)]
    union ValueI64 {
        value: i64,
        source: [i32; 2_usize],
    }

    let mut map: ValueI64 = ValueI64 { value: value };

    asm!(
        "bswapl {second:e}",                       // Свапаем байты в обратном порядке
        "bswapl {first:e}",                        // Свапаем байты в обратном порядке

        "movl {second:e}, %ecx",                   // ecx = second
        "andl $0x0F0F0F0F, %ecx",                  // Применяем маску 0x0F к полу-байтам
        "shll $4, %ecx",                           // Сдвигаем влево на 4 бита (полубайты)
        "shrl $4, {second:e}",                     // Сдвигаем вправо на 4 бита (полубайты)
        "andl $0x0F0F0F0F, {second:e}",            // Применяем маску 0x0F к полу-байтам
        "orl %ecx, {second:e}",                    // Объединяем

        "movl {second:e}, %ecx",                   // ecx = second
        "andl $0x33333333, %ecx",                  // Применяем маску 0x33 к полу-байтам
        "shrl $2, {second:e}",                     // Сдвигаем вправо на 2 бита (полубайты)
        "andl $0x33333333, {second:e}",            // Применяем маску 0x33 к полу-байтам
        "leal ({second:e}, %ecx, 4), {second:e}",  // Умножаем на 4, что эквивалентно сдвигу на 2 влево (где 0 бит на плоскости справа) и объединяем значения

        "movl {second:e}, %ecx",                   // ecx = second
        "andl $0x55555555, %ecx",                  // Применяем маску 0x55 к полу-байтам
        "shrl $1, {second:e}",                     // Сдвигаем вправо на 1 бит (полубайты)
        "andl $0x55555555, {second:e}",            // Применяем маску 0x55 к полу-байтам
        "leal ({second:e}, %ecx, 2), {second:e}",  // Умножаем на 2, что эквивалентно сдвигу на 1 влево (где 0 бит на плоскости справа) и объединяем значения

        "movl {first:e}, %ecx",                    // ecx = first
        "andl $0x0F0F0F0F, %ecx",                  // Применяем маску 0x0F к полу-байтам
        "shll $4, %ecx",                           // Сдвигаем влево на 4 бита (полубайты)
        "shrl $4, {first:e}",                      // Сдвигаем вправо на 4 бита (полубайты)
        "andl $0x0F0F0F0F, {first:e}",             // Применяем маску 0x0F к полу-байтам
        "orl %ecx, {first:e}",                     // Объединяем

        "movl {first:e}, %ecx",                    // ecx = first
        "andl $0x33333333, %ecx",                  // Применяем маску 0x33 к полу-байтам
        "shrl $2, {first:e}",                      // Сдвигаем вправо на 2 бита (полубайты)
        "andl $0x33333333, {first:e}",             // Применяем маску 0x33 к полу-байтам
        "leal ({first:e}, %ecx, 4), {first:e}",    // Умножаем на 4, что эквивалентно сдвигу на 2 влево (где 0 бит на плоскости справа) и объединяем значения

        "movl {first:e}, %ecx",                    // ecx = first
        "andl $0x55555555, %ecx",                  // Применяем маску 0x55 к полу-байтам
        "shrl $1, {first:e}",                      // Сдвигаем вправо на 1 бит (полубайты)
        "andl $0x55555555, {first:e}",             // Применяем маску 0x55 к полу-байтам
        "leal ({first:e}, %ecx, 2), {first:e}",    // Умножаем на 2, что эквивалентно сдвигу на 1 влево (где 0 бит на плоскости справа) и объединяем значения

        "xchgl {first:e}, {second:e}",             // Свапаем знаения, формируя reversed (i64)
        options(nomem, nostack, att_syntax),
        first = inout(reg) map.source[0],
        second = inout(reg) map.source[1],
        out("ecx") _
    );

    return unsafe { map.value };
}

#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
unsafe fn _rbit_sei64_r64(mut value: i64) -> i64 {
    asm!(
        "bswapq {value}",                        // Свапаем байты в обратном порядке

        "movabsq $0x0F0F0F0F0F0F0F0F, %rax",     // 0x0F0F0F0F0F0F0F0F
        "movq {value:r}, %rcx",                  // rcx = value
        "andq %rax, %rcx",                       // Применяем маску 0x0F к полу-байтам
        "shlq $4, %rcx",                         // Сдвигаем влево на 4 бита (полубайты)
        "shrq $4, {value:r}",                    // Сдвигаем вправо на 4 бита (полубайты)
        "andq %rax, {value:r}",                  // Применяем маску 0x0F к полу-байтам
        "orq %rcx, {value:r}",                   // Объединяем

        "movabsq $0x3333333333333333, %rax",     // 0x3333333333333333
        "movq {value:r}, %rcx",                  // rcx = value
        "andq %rax, %rcx",                       // Применяем маску 0x33 к полу-байтам
        "shrq $2, {value:r}",                    // Сдвигаем вправо на 2 бита (полубайты)
        "andq %rax, {value:r}",                  // Применяем маску 0x33 к полу-байтам
        "leaq ({value:r}, %rcx, 4), {value:r}",  // Умножаем на 4, что эквивалентно сдвигу на 2 влево (где 0 бит на плоскости справа) и объединяем значения

        "movabsq $0x5555555555555555, %rax",     // 0x5555555555555555
        "movq {value:r}, %rcx",                  // rcx = value
        "andq %rax, %rcx",                       // Применяем маску 0x55 к полу-байтам
        "shrq $1, {value:r}",                    // Сдвигаем вправо на 1 бит (полубайты)
        "andq %rax, {value:r}",                  // Применяем маску 0x55 к полу-байтам
        "leaq ({value:r}, %rcx, 2), {value:r}",  // Умножаем на 2, что эквивалентно сдвигу на 1 влево (где 0 бит на плоскости справа) и объединяем значения
        options(nomem, nostack, att_syntax),
        value = inout(reg) value,
        out("rax") _,
        out("rcx") _
    );

    return value;
}

#[unsafe(no_mangle)]
pub fn _splitmix_sei64(seed: &mut i64) -> i64 {
    *seed += -0x61C8864680B583EB_i64;
    let mut z: i64 = *seed;
    z = (z ^ (z >> 0x1E_i64)) * -0x40A7B892E31B1A47_i64;
    z = (z ^ (z >> 0x1B_i64)) * -0x6B2FB644ECCEEE15_i64;
    return z ^ (z >> 0x1F_i64);
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[unsafe(no_mangle)]
unsafe fn test_valid_mm_rbit_sei64(iterations: i64) {
    let mut seed: i64 = 0x000000000000002A_i64;

    for i in 0_i64..=iterations {
        let value: i64 = _splitmix_sei64(&mut seed);

        let expected_reverse: i64 = _llvm_rbit_sei64(value);

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            let result_r32: i64 = _rbit_sei64_r32(value);
            if result_r32 != expected_reverse {
                panic!(
                    "[R32-V] Error at index {}: value=0x{:016X}, got=0x{:016X}, expected=00x{:016X}",
                    i, value, result_r32, expected_reverse
                );
            }
        }

        #[cfg(target_arch = "x86_64")]
        {
            let result_r64: i64 = _rbit_sei64_r64(value);
            if result_r64 != expected_reverse {
                panic!(
                    "[R64-V] Error at index {}: value=0x{:016X}, got=0x{:016X}, expected=00x{:016X}",
                    i, value, result_r64, expected_reverse
                );
            }
        }

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            let result_swar: i64 = _rbit_sei64_swar(value);
            if result_swar != expected_reverse {
                panic!(
                    "[SWAR-V] Error at index {}: value=0x{:016X}, got=0x{:016X}, expected=00x{:016X}",
                    i, value, result_swar, expected_reverse
                );
            }
        }

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            let result_mmx_pand: i64 = _m_rbit_sei64_mmx_pand(value);
            if result_mmx_pand != expected_reverse {
                panic!(
                    "[MMX-PAND-V] Error at index {}: value=0x{:016X}, got=0x{:016X}, expected=00x{:016X}",
                    i, value, result_mmx_pand, expected_reverse
                );
            }
        }

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            let result_mmx_pandn: i64 = _m_rbit_sei64_mmx_pandn(value);
            if result_mmx_pandn != expected_reverse {
                panic!(
                    "[MMX-PANDN-V] Error at index {}: value=0x{:016X}, got=0x{:016X}, expected=00x{:016X}",
                    i, value, result_mmx_pandn, expected_reverse
                );
            }
        }
    }

    println!("RBIT validation test passed for {} values!", iterations);
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[unsafe(no_mangle)]
unsafe fn benchmark_rbit(iterations: i64) {
    let mut seed: i64 = 0x000000000000002A_i64;
    let mut result: i64 = 0x0000000000000000_i64;

    let mut start: std::time::Instant = std::time::Instant::now();

    for _ in 0..=iterations {
        result ^= _llvm_rbit_sei64(_splitmix_sei64(&mut seed));
    }

    let mut time_taken: core::time::Duration = start.elapsed();

    println!("[LLVM-V]: {} seconds (result: 0x{:016X})", time_taken.as_secs_f64(), result);

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        start = std::time::Instant::now();

        for _ in 0..=iterations {
            result ^= _rbit_sei64_r32(_splitmix_sei64(&mut seed));
        }

        time_taken = start.elapsed();

        println!("[R32-V]: {} seconds (result: 0x{:016X})", time_taken.as_secs_f64(), result);
    }

    #[cfg(target_arch = "x86_64")]
    {
        start = std::time::Instant::now();

        for _ in 0..=iterations {
            result ^= _rbit_sei64_r64(_splitmix_sei64(&mut seed));
        }

        time_taken = start.elapsed();

        println!("[R64-V]: {} seconds (result: 0x{:016X})", time_taken.as_secs_f64(), result);
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        start = std::time::Instant::now();

        for _ in 0..=iterations {
            result ^= _rbit_sei64_swar(_splitmix_sei64(&mut seed));
        }

        time_taken = start.elapsed();

        println!("[SWAR-V]: {} seconds (result: 0x{:016X})", time_taken.as_secs_f64(), result);
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        start = std::time::Instant::now();

        for _ in 0..=iterations {
            result ^= _m_rbit_sei64_mmx_pand(_splitmix_sei64(&mut seed));
        }

        time_taken = start.elapsed();

        println!("[MMX-PAND-V]: {} seconds (result: 0x{:016X})", time_taken.as_secs_f64(), result);
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        start = std::time::Instant::now();

        for _ in 0..=iterations {
            result ^= _m_rbit_sei64_mmx_pandn(_splitmix_sei64(&mut seed));
        }

        time_taken = start.elapsed();

        println!("[MMX-PANDN-V]: {} seconds (result: 0x{:016X})", time_taken.as_secs_f64(), result);
    }
}

fn main() {
    let count_iterations: i64 = 10000000_i64;

    // 1. Стандартные кейсы
    println!("Basic cases:\n");
    println!("0x0000000000000000 = 0x{:016X}", unsafe { _rbit_sei64_swar(0x0000000000000000) });
    println!("0xFFFFFFFFFFFFFFFF = 0x{:016X}", unsafe { _rbit_sei64_swar(-0x0000000000000001) });
    println!("0x0123456789ABCDEF = 0x{:016X}", unsafe { _rbit_sei64_swar(0x0123456789ABCDEF) });

    // 1. Валидация
    println!("\nValidation result: \n");
    unsafe { test_valid_mm_rbit_sei64(count_iterations) };

    // 2. Бенчмарк
    println!("\nBenchmark result: \n");
    unsafe { benchmark_rbit(count_iterations) };
}
