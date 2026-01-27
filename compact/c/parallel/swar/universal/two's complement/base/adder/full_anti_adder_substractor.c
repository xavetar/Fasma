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

#include <stdio.h>
#include <stdint.h>
#include <limits.h>
#include <stdbool.h>
#include <inttypes.h>

const uint32_t BITS = 8;
const uint64_t ONE = 1ULL << ((sizeof(ONE) * CHAR_BIT) - 1);
const uint64_t MASK = ((1ULL << BITS) - 1) << ((sizeof(MASK) * CHAR_BIT) - BITS);

void full_adder(uint64_t A, uint64_t B, uint64_t *SUM, uint64_t *Cout) {
    *SUM = A ^ B;  // SUM = A ⊕ B
    *Cout = A & B; // Cout = A ∧ B
}

uint64_t prepare(uint64_t B) {
    uint64_t SUM, Cout, Cin;

    full_adder(~B, ONE, &SUM, &Cout);

    while (Cout != 0) {
        Cin = (Cout >> 1) & MASK;          // Cin = Cout >> 1
        full_adder(SUM, Cin, &SUM, &Cout); // SUM = SUM ⊕ Cin, Cout = SUM ∧ Cin
    }

    return SUM & MASK;
}

int main() {
    bool sub_mode = true;

    uint32_t iterations = 0;

    uint64_t A = 0b0101010100000000000000000000000000000000000000000000000000000000;
    uint64_t B = 0b1010101000000000000000000000000000000000000000000000000000000000;
    uint64_t SUM, Cout, Cin;

    if (sub_mode == true) { B = prepare(B); };

    // Первая итерация (полу-анти-сумматор)
    full_adder(A, B, &SUM, &Cout);
    printf("Iter %" PRId32 ": SUM = 0x%" PRIx64 ", Cout = 0x%" PRIx64 "\n", iterations++, SUM, Cout);

    // Итерации с переносом, пока Cout != 0
    while (Cout != 0) {
        Cin = (Cout >> 1) & MASK;    // Cin = Cout >> 1
        full_adder(SUM, Cin, &SUM, &Cout); // SUM = SUM ⊕ Cin, Cout = SUM ∧ Cin
        printf("Iter %" PRId32 ": SUM = 0x%" PRIx64 ", Cout = 0x%" PRIx64 "\n", iterations++, SUM, Cout);
    }

    printf("Final SUM = 0x%" PRIx64 ", Iterations = %" PRId32 "\n", SUM, iterations);

    return 0;
}
