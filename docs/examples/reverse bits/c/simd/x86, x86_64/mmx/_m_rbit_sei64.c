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
 * - ASM Docs
 *
 * https://gcc.gnu.org/onlinedocs/gcc/Extended-Asm.html
 * https://gcc.gnu.org/onlinedocs/gcc/Machine-Constraints.html*
 *
 * - Build with clang:
 *
 * clang -m32 -D __i686__ -mllvm --x86-asm-syntax=att --target=i686-pc-linux-gnu -mmmx -O3 _m_rbit_sei64.c -o rbit && ./rbit && rm -rf rbit
 * clang -m64 -mllvm --x86-asm-syntax=att --target=x86_64-pc-linux-gnu -mmmx -O3 _m_rbit_sei64.c -o rbit && ./rbit && rm -rf rbit
 *
 * - Generate ASM via clang:
 *
 * clang -S -m32 -D __i686__ -mllvm --x86-asm-syntax=att --target=i686-pc-linux-gnu -mmmx -O3 _m_rbit_sei64.c -o rbit.s && vi rbit.s && rm -f rbit.s
 * clang -S -m64 -mllvm --x86-asm-syntax=att --target=x86_64-pc-linux-gnu -mmmx -O3 _m_rbit_sei64.c -o rbit.s && vi rbit.s && rm -f rbit.s
 *
 * - Build with GCC:
 *
 * gcc -m32 -mmmx -O3 _m_rbit_sei64.c -o rbit && ./rbit && rm -f rbit
 * gcc -m64 -mmmx -O3 _m_rbit_sei64.c -o rbit && ./rbit && rm -f rbit
 *
 * - Generate ASM via GCC:
 *
 * gcc -S -m32 -mmmx -O3 _m_rbit_sei64.c -o rbit.s && vi rbit.s && rm -f rbit.s
 * gcc -S -m64 -mmmx -O3 _m_rbit_sei64.c -o rbit.s && vi rbit.s && rm -f rbit.s
 *
 * - Fix GCC on RedHat header (x86_64):
 *
 * sudo sed -i 's/#ifdef __x86_64__/#if defined(__x86_64__) || defined(__i686__)/g' /usr/lib/gcc/x86_64-redhat-linux/.../include/mmintrin.h
 *
 */

#include <time.h>
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>
#include <inttypes.h>
#include <mmintrin.h>

#define STACK_i686 1
#define ASM_OPTIMIZED 1

#define NSEC_PER_SEC 1000000000L

static inline __attribute__((used, always_inline, target("mmx"), optimize("O3")))
uint64_t _rbit_sei64_swar(uint64_t v) {
    v = ((v >> 0x20) & UINT64_C(0x00000000FFFFFFFF)) | ((v << 0x20) & UINT64_C(0xFFFFFFFF00000000));
    v = ((v >> 0x10) & UINT64_C(0x0000FFFF0000FFFF)) | ((v << 0x10) & UINT64_C(0xFFFF0000FFFF0000));
    v = ((v >> 0x08) & UINT64_C(0x00FF00FF00FF00FF)) | ((v << 0x08) & UINT64_C(0xFF00FF00FF00FF00));
    v = ((v >> 0x04) & UINT64_C(0x0F0F0F0F0F0F0F0F)) | ((v << 0x04) & UINT64_C(0xF0F0F0F0F0F0F0F0));
    v = ((v >> 0x02) & UINT64_C(0x3333333333333333)) | ((v << 0x02) & UINT64_C(0xCCCCCCCCCCCCCCCC));
    v = ((v >> 0x01) & UINT64_C(0x5555555555555555)) | ((v << 0x01) & UINT64_C(0xAAAAAAAAAAAAAAAA));
    return v;
}

__attribute__((used, always_inline, target("mmx"), optimize("O3")))
static inline uint64_t _m_rbit_sei64(int64_t var) {
#if defined(__i686__)
#ifdef STACK_i686
    __m64 v = _mm_set_pi32(
        __builtin_bswap32((int32_t) var),
        __builtin_bswap32((int32_t) (((uint64_t) var) >> 0x20))
    );
#else
    __m64 v = _mm_or_si64(
        _m_psllqi(
            _mm_cvtsi32_si64(
                __builtin_bswap32(_mm_cvtsi64_si32(_m_from_int64(var)))
            ),
            0x20
        ),
        _mm_cvtsi32_si64(
            __builtin_bswap32(
                _mm_cvtsi64_si32(_m_psrlqi(_m_from_int64(var), 0x20))
            )
        )
    );
#endif
#elif defined(__x86_64__)
    __m64 v = _mm_cvtsi64_m64(__builtin_bswap64(var));
#else
    #error "Unknown or unsupported arch!"
#endif

    v = _mm_or_si64(
        _mm_slli_pi16(_mm_and_si64(v, _mm_set1_pi8(0x0F)), 0x04),
        _mm_srli_pi16(_mm_andnot_si64(_mm_set1_pi8(0x0F), v), 0x04)
    );

    v = _mm_or_si64(
        _mm_slli_pi16(_mm_and_si64(v, _mm_set1_pi8(0x33)), 0x02),
        _mm_srli_pi16(_mm_andnot_si64(_mm_set1_pi8(0x33), v), 0x02)
    );

    v = _mm_or_si64(
        _mm_slli_pi16(_mm_and_si64(v, _mm_set1_pi8(0x55)), 0x01),
        _mm_srli_pi16(_mm_andnot_si64(_mm_set1_pi8(0x55), v), 0x01)
    );

    _mm_empty();

    return (uint64_t) _mm_cvtm64_si64(v);
}

static inline __attribute__((used, always_inline, target("mmx"), optimize("O3")))
uint64_t _m_rbit_sei64_asm(int64_t var) {
#if defined(__i686__)
#ifdef STACK_i686
    __m64 value = _mm_set_pi32(
        __builtin_bswap32((int32_t) var),
        __builtin_bswap32((int32_t) (((uint64_t) var) >> 0x20))
    );
#else
    __m64 value = _mm_or_si64(
        _m_psllqi(
            _mm_cvtsi32_si64(
                __builtin_bswap32(_mm_cvtsi64_si32(_m_from_int64(var)))
            ),
            0x20
        ),
        _mm_cvtsi32_si64(
            __builtin_bswap32(
                _mm_cvtsi64_si32(_m_psrlqi(_m_from_int64(var), 0x20))
            )
        )
    );
#endif
#elif defined(__x86_64__)
    __m64 value = _m_from_int64(__builtin_bswap64(var));
#else
    #error "Unknown or unsupported arch!"
#endif

#ifndef ASM_OPTIMIZED
    __m64 result      = _mm_setzero_si64();
#endif
    __m64 mask_nibble = _m_from_int64(UINT64_C(0x0F0F0F0F0F0F0F0F));
    __m64 mask_2bit   = _m_from_int64(UINT64_C(0x3333333333333333));
    __m64 mask_bit    = _m_from_int64(UINT64_C(0x5555555555555555));

#ifdef ASM_OPTIMIZED
    __asm__ __volatile__(
        "movq %3, %%mm1\n\t"        // mm1 = value
        "pand %0, %3\n\t"           // Применяем маску 0x0F к полу-байтам
        "psllw $4, %3\n\t"          // Сдвигаем влево на 4 бита (полубайты)
        "pandn %%mm1, %0\n\t"       // Применяем маску 0xF0 к полу-байтам и сохраняем в %0 (mask_nibble)
        "psrlw $4, %0\n\t"          // Сдвигаем вправо на 4 бита
        "por %0, %3\n\t"            // Объединяем

        "movq %3, %%mm1\n\t"        // mm1 = value
        "pand %1, %3\n\t"           // Применяем маску 0x33 к полу-байтам
        "psllw $2, %3\n\t"          // Сдвигаем влево на 4 бита (полубайты)
        "pandn %%mm1, %1\n\t"       // Применяем маску 0xCC к полу-байтам и сохраняем в %1 (mask_2bit)
        "psrlw $2, %1\n\t"          // Сдвигаем вправо на 4 бита
        "por %1, %3\n\t"            // Объединяем

        "movq %3, %%mm1\n\t"        // mm1 = value
        "pand %2, %3\n\t"           // Применяем маску 0x55 к полу-байтам
        "psllw $1, %3\n\t"          // Сдвигаем влево на 4 бита (полубайты)
        "pandn %%mm1, %2\n\t"       // Применяем маску 0xAA к полу-байтам и сохраняем в %2 (mask_bit)
        "psrlw $1, %2\n\t"          // Сдвигаем вправо на 4 бита
        "por %2, %3\n\t"            // Объединяем
        : "+y"(mask_nibble), "+y"(mask_2bit), "+y"(mask_bit), "+y"(value)
        :
        : "mm1"
    );
#else
    __asm__ __volatile__(
        "movq %4, %%mm0\n\t"        // mm0 = number

        "movq %%mm0, %%mm1\n\t"     // mm1 = mm0
        "pand %1, %%mm1\n\t"        // Применяем маску 0x0F к полу-байтам
        "psllw $4, %%mm1\n\t"       // Сдвигаем влево на 4 бита (полубайты)
        "pandn %%mm0, %1\n\t"       // Применяем маску 0xF0 к полу-байтам и сохраняем в %1 (mask_nibble)
        "psrlw $4, %1\n\t"          // Сдвигаем вправо на 4 бита
        "por %1, %%mm1\n\t"         // Объединяем

        "movq %%mm1, %%mm0\n\t"     // mm0 = mm1
        "pand %2, %%mm1\n\t"        // Применяем маску 0x33 к полу-байтам
        "psllw $2, %%mm1\n\t"       // Сдвигаем влево на 4 бита (полубайты)
        "pandn %%mm0, %2\n\t"       // Применяем маску 0xCC к полу-байтам и сохраняем в %2 (mask_2bit)
        "psrlw $2, %2\n\t"          // Сдвигаем вправо на 4 бита
        "por %2, %%mm1\n\t"         // Объединяем

        "movq %%mm1, %%mm0\n\t"     // mm0 = mm1
        "pand %3, %%mm1\n\t"        // Применяем маску 0x55 к полу-байтам
        "psllw $1, %%mm1\n\t"       // Сдвигаем влево на 4 бита (полубайты)
        "pandn %%mm0, %3\n\t"       // Применяем маску 0xAA к полу-байтам и сохраняем в %3 (mask_bit)
        "psrlw $1, %3\n\t"          // Сдвигаем вправо на 4 бита
        "por %3, %%mm1\n\t"         // Объединяем

        "movq %%mm1, %0\n\t"        // Сохраняем результат
        : "=y"(result), "+y"(mask_nibble), "+y"(mask_2bit), "+y"(mask_bit)
        : "y"(value)
        : "mm0", "mm1"
    );
#endif

    _mm_empty();

#ifdef ASM_OPTIMIZED
    return (uint64_t) _mm_cvtm64_si64(value);
#else
    return (uint64_t) _mm_cvtm64_si64(result);
#endif
}

static inline __attribute__((used, always_inline, target("mmx"), optimize("O3")))
uint64_t _splitmix_seu64(uint64_t *seed) {
    uint64_t z = (*seed += UINT64_C(0x9E3779B97F4A7C15));
    z = (z ^ (z >> 0x1E)) * UINT64_C(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 0x1B)) * UINT64_C(0x94D049BB133111EB);
    return z ^ (z >> 0x1F);
}

void test_valid_mm_rbit_sei64(uint32_t iterations) {
    uint64_t seed = UINT64_C(42);

    for (uint32_t i = UINT32_C(0); i < iterations; i++) {
        uint64_t value = _splitmix_seu64(&seed);

        uint64_t result_mmx = _m_rbit_sei64(value);
        uint64_t result_asm_mmx = _m_rbit_sei64_asm(value);

        uint64_t expected_reverse = _rbit_sei64_swar(value);

        if (result_mmx != expected_reverse) {
            printf(
                "[MMX-V] Error at index %" PRIu8 ": var=0x%" PRIx64 ", got=0x%" PRIx64 ", expected=0x%" PRIx64 "\n",
                i, value, result_mmx, expected_reverse
            );
            exit(1);
        }

        if (result_asm_mmx != expected_reverse) {
            printf(
                "[ASM-MMX-V] Error at index %" PRIu8 ": var=0x%" PRIx64 ", got=0x%" PRIx64 ", expected=0x%" PRIx64 "\n",
                i, value, result_asm_mmx, expected_reverse
            );
            exit(1);
        }
    }
    printf("RBIT validation test passed for %" PRIu32 " values!\n", iterations);
}

void benchmark_rbit(uint32_t iterations) {
    uint64_t seed = UINT64_C(0x000000000000002A);
    uint64_t result = UINT64_C(0x0000000000000000);

    struct timespec start, end;

    clock_gettime(CLOCK_MONOTONIC, &start);
    for (int i = 0; i < iterations; i++) {
        uint64_t var = _splitmix_seu64(&seed);
        result ^= _rbit_sei64_swar(var);
    }
    clock_gettime(CLOCK_MONOTONIC, &end);

    double time_taken = (end.tv_sec - start.tv_sec) + (end.tv_nsec - start.tv_nsec) * NSEC_PER_SEC;
    printf(
        "%s: %f seconds (checksum: 0x%" PRIx64 ")\n",
        "SWAR-V", time_taken, result
    );

    clock_gettime(CLOCK_MONOTONIC, &start);
    for (int i = 0; i < iterations; i++) {
        uint64_t var = _splitmix_seu64(&seed);
        result ^= _m_rbit_sei64(var);
    }
    clock_gettime(CLOCK_MONOTONIC, &end);

    time_taken = (end.tv_sec - start.tv_sec) + (end.tv_nsec - start.tv_nsec) * NSEC_PER_SEC;
    printf(
        "%s: %f seconds (checksum: 0x%" PRIx64 ")\n",
        "MMX-V", time_taken, result
    );

    clock_gettime(CLOCK_MONOTONIC, &start);
    for (int i = 0; i < iterations; i++) {
        uint64_t var = _splitmix_seu64(&seed);
        result ^= _m_rbit_sei64_asm(var);
    }
    clock_gettime(CLOCK_MONOTONIC, &end);

    time_taken = (end.tv_sec - start.tv_sec) + (end.tv_nsec - start.tv_nsec) * NSEC_PER_SEC;
    printf(
        "%s: %f seconds (checksum: 0x%" PRIx64 ")\n",
        "ASM-MMX-V", time_taken, result
    );
}

int main() {
    uint32_t count_iterations = UINT32_C(10000000);

    // 1. Стандартные кейсы
    printf("Basic cases:\n\n");
    printf("0x0000000000000000 = 0x%" PRIx64 "\n", _m_rbit_sei64(0x0000000000000000));
    printf("0xFFFFFFFFFFFFFFFF = 0x%" PRIx64 "\n", _m_rbit_sei64(-0x0000000000000001)));
    printf("0x0123456789ABCDEF = 0x%" PRIx64 "\n", _m_rbit_sei64(0x0123456789ABCDEF));

    // 1. Валидация
    printf("\nValidation result: \n\n");
    test_valid_mm_rbit_sei64(count_iterations);

    // 2. Бенчмарк
    printf("\nBenchmark result: \n\n");
    benchmark_rbit(count_iterations);

    return 0;
}
