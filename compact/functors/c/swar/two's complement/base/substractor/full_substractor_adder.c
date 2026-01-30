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

void full_subtractor(uint64_t A, uint64_t B, uint64_t *DIFFERENCE, uint64_t *Bout) {
    *DIFFERENCE = A ^ B;
    *Bout = (~A) & B;
}

// ADD = A - ~(B - 1)
uint64_t prepare(uint64_t B) {
    uint64_t DIFFERENCE, Bout, Bin;

    full_subtractor(B, 1, &DIFFERENCE, &Bout);

    while (Bout != 0) {
        Bin = (Bout << 1) & MASK;
        full_subtractor(DIFFERENCE, Bin, &DIFFERENCE, &Bout);
    }

    return (~DIFFERENCE) & MASK;
}

int main() {
    bool add_mode = true;

    uint32_t iterations = 0;

    uint64_t A = 0b0000000000000000000000000000000000000000000000000000000000000001;
    uint64_t B = 0b0000000000000000000000000000000000000000000000000000000010110111;
    uint64_t DIFFERENCE, Bout, Bin;

    if (add_mode == true) { B = prepare(B); };

    full_subtractor(A, B, &DIFFERENCE, &Bout);
    printf("Iter %" PRId32 ": DIFFERENCE = 0x%" PRIx64 ", Bout = 0x%" PRIx64 "\n", iterations++, DIFFERENCE, Bout);

    while (Bout != 0) {
        Bin = (Bout << 1) & MASK;
        full_subtractor(DIFFERENCE, Bin, &DIFFERENCE, &Bout);
        printf("Iter %" PRId32 ": DIFFERENCE = 0x%" PRIx64 ", Bout = 0x%" PRIx64 "\n", iterations++, DIFFERENCE, Bout);
    }

    printf("Final DIFFERENCE = 0x%" PRIx64 ", Iterations = %" PRId32 "\n", DIFFERENCE, iterations);

    return 0;
}
