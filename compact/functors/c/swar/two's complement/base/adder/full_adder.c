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
#include <inttypes.h>

// Logical scheme:
//
// A = 0b1101
// B = 0b1111
//
// SUM = A ^ B      = 0b00010
// Cout = A & B     = 0b01101
//
// Cin = Cout << 1  = 0b11010
// Cout = Cin & SUM = 0b00010
// SUM = SUM ^ Cin  = 0b11000
//
// Cin = Cout << 1  = 0b00100
// Cout = Cin & SUM = 0b00000
// SUM = SUM ^ Cin  = 0b11100
//
// Worst cases:
//
// New max: A = 0x0, B = 0x0, Iterations = 1
// New max: A = 0x1, B = 0x1, Iterations = 2
// New max: A = 0x1, B = 0x3, Iterations = 3
// New max: A = 0x1, B = 0x7, Iterations = 4
// New max: A = 0x1, B = 0xf, Iterations = 5
// New max: A = 0x1, B = 0x1f, Iterations = 6
// New max: A = 0x1, B = 0x3f, Iterations = 7
// New max: A = 0x1, B = 0x7f, Iterations = 8
// New max: A = 0x1, B = 0xff, Iterations = 9
// Worst case: A = 0x1, B = 0xff, Max iterations = 9
// Total errors: 0

const uint32_t BITS = 8;
const uint64_t MASK = (1ULL << BITS) - 1;

void full_adder(uint64_t A, uint64_t B, uint64_t *SUM, uint64_t *Cout) {
    *SUM = A ^ B;  // SUM = A ⊕ B
    *Cout = A & B; // Cout = A ∧ B
}

int main() {
    uint32_t iterations = 0;

    uint64_t A = 0b0000000000000000000000000000000000000000000000000000000000000001;
    uint64_t B = 0b0000000000000000000000000000000000000000000000000000000011111111;
    uint64_t SUM, Cout, Cin;

    // Первая итерация (полусумматор)
    full_adder(A, B, &SUM, &Cout);
    printf("Iter %" PRId32 ": SUM = 0x%" PRIx64 ", Cout = 0x%" PRIx64 "\n", iterations++, SUM, Cout);

    // Итерации с переносом, пока Cout != 0
    while (Cout != 0) {
        Cin = (Cout << 1) & MASK;    // Cin = Cout << 1
        full_adder(SUM, Cin, &SUM, &Cout); // SUM = SUM ⊕ Cin, Cout = SUM ∧ Cin
        printf("Iter %" PRId32 ": SUM = 0x%" PRIx64 ", Cout = 0x%" PRIx64 "\n", iterations++, SUM, Cout);
    }

    printf("Final SUM = 0x%" PRIx64 ", Iterations = %" PRId32 "\n", SUM, iterations);

    return 0;
}
