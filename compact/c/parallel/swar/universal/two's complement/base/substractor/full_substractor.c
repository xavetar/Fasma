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
// # 1
//
// A = 0b1111
// B = 0b1101
// DIFF = A ^ B       = 0b0010
// Bout = ~A & B      = 0b0000
//
// # 2
//
// A = 0b1000
// B = 0b0011
//
// DIFF = A ^ B       = 0b1011
// Bout = ~A & B      = 0b0011
//
// Bin = Bout << 1    = 0b0110
// Bout = ~DIFF & Bin = 0b0100
// DIFF = DIFF ^ Bin  = 0b1101
//
// Bin = Bout << 1    = 0b1000
// Bout = ~DIFF & Bin = 0b0000
// DIFF = DIFF ^ Bin  = 0b0101
//
// Worst cases:
//
// New max: A = 0xff, B = 0x0, Iterations = 1
// New max: A = 0xfe, B = 0x1, Iterations = 2
// New max: A = 0xfe, B = 0x3, Iterations = 3
// New max: A = 0xfe, B = 0x7, Iterations = 4
// New max: A = 0xfe, B = 0xf, Iterations = 5
// New max: A = 0xfe, B = 0x1f, Iterations = 6
// New max: A = 0xfe, B = 0x3f, Iterations = 7
// New max: A = 0xfe, B = 0x7f, Iterations = 8
// New max: A = 0xfe, B = 0xff, Iterations = 9

const uint32_t BITS = 8;
const uint64_t MASK = (1ULL << BITS) - 1;

void full_subtractor(uint64_t A, uint64_t B, uint64_t *DIFFERENCE, uint64_t *Bout) {
    *DIFFERENCE = A ^ B;
    *Bout = (~A) & B;
}

int main() {
    uint32_t iterations = 0;

    uint64_t A = 0b0000000000000000000000000000000000000000000000000000000000101000;
    uint64_t B = 0b0000000000000000000000000000000000000000000000000000000001101000;
    uint64_t DIFFERENCE, Bout, Bin;

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
