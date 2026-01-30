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
#include <stdbool.h>
#include <inttypes.h>

const uint32_t BITS = 8;
const uint64_t MASK = (1ULL << BITS) - 1;

void full_adder(uint64_t A, uint64_t B, uint64_t *SUM, uint64_t *Cout) {
    *SUM = A ^ B;  // SUM = A ⊕ B
    *Cout = A & B; // Cout = A ∧ B
}

uint64_t prepare(uint64_t B) {
    uint64_t SUM, Cout, Cin;

    full_adder(~B, 1, &SUM, &Cout);

    while (Cout != 0) {
        Cin = (Cout << 1) & MASK;          // Cin = Cout << 1
        full_adder(SUM, Cin, &SUM, &Cout); // SUM = SUM ⊕ Cin, Cout = SUM ∧ Cin
    }

    return SUM & MASK;
}

int count_iterations(uint64_t A, uint64_t B, uint64_t *final_sum) {
    uint32_t iterations = 0;

    uint64_t SUM, Cout, Cin;

    full_adder(A, B, &SUM, &Cout);
    iterations++;

    while (Cout != 0) {
        Cin = (Cout << 1) & MASK;
        full_adder(SUM, Cin, &SUM, &Cout);
        iterations++;
    }

    *final_sum = SUM;
    return iterations;
}

int main() {
    bool sub_mode = true;

    uint32_t errors = 0;

    uint64_t max_iterations = 0;
    uint64_t worst_A = 0, worst_B = 0;

    for (uint64_t A = 0xFF; ; ) {
        for (uint64_t B = 0x00; ; ) {

            uint64_t B_temp = B;
            uint64_t final_sum = 0;

            if (sub_mode == true) {
                B_temp = prepare(B);
            }

            uint32_t iterations = count_iterations(A, B_temp, &final_sum);

            // Ожидаемое вычитание или сложение в 8-битной арифметике
            uint64_t expected;
            if (sub_mode == true) {
                expected = (A - B) & MASK;
            } else {
                expected = (A + B) & MASK;
            }

            if (final_sum != expected) {
                printf(
                    "Error: A = 0x%" PRIx64 ", B = 0x%" PRIx64 ", SUM = 0x%" PRIx64 ", Expected = 0x%" PRIx64 ", Iterations = %" PRId32 "\n",
                    A, B, final_sum, expected, iterations
                );
                errors++;
            }

            if (iterations > max_iterations) {
                max_iterations = iterations;
                worst_A = A;
                worst_B = B;
                printf("New max: A = 0x%" PRIx64 ", B = 0x%" PRIx64 ", Iterations = %" PRId32 "\n", A, B, iterations);
            }

            if (B == 0xFF) { break; }; B++;
        }

        if (A == 0x00) { break; }; A--;
    }

    printf("Worst case: A = 0x%" PRIx64 ", B = 0x%" PRIx64 ", Max iterations = %" PRIu64 "\n", worst_A, worst_B, max_iterations);
    printf("Total errors: %" PRId32 "\n", errors);

    return 0;
}

