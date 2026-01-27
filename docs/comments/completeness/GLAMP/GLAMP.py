# Copyright 2024 Stanislav Mikhailov (xavetar)
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in
# all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
# THE SOFTWARE.

from typing import Final, Optional, Union, Dict, List, Tuple, cast


def find_solution(N: int, S: int, P: Optional[Tuple[int, ...]] = None, debug: bool = False) -> Tuple[str, str]:
    """
    Находит решение для перестановки N-битного поля в любом порядке, через циклический сумматор размером поля: S = 2^(N+∞) - 1
    Решение существует, когда общее выражение является периодическим - иначе его не существует (с заданым параметром (∞) и смещением, в поле с таким периодом).
    """

    # Исходные позиции: [N-1, N-2, ..., 0]
    # N = 8: [7, 6, 5, 4, 3, 2, 1, 0]
    source_positions: Final[List[int]] = list(range(N-1, -1, -1))

    # По-умолчанию выполняется инверсия:
    if not P: P = list(range(N-1, -1, -1))

    # Исходные позиции сумматора: [N-1, N-2, ..., 0]
    # S = 10: [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    stock_adder_positions: Final[List[int]] = list(range(S-1, -1, -1))

    (iteration, count_of_find_positions, reverse_text_mask) = cast(
        Tuple[int, int, List[str]],
        (0, 0, [])
    )

    while True:
        adder_positions: List[int] = (stock_adder_positions * (iteration + 1))

        # Исходные позиции с учётом смещения (8 bit):
        # S = N + 2: [0, 7, 6, 5, 4, 3, 2, 1, 0, '-']
        #
        # Варианты использования:
        # 1) [a % N for a in range(adder_positions.__len__() - 2, -1, -1)] + ["-"]
        # 2) ["-"] + [a % N for a in range(0, adder_positions.__len__() - 1)] | + reverse()
        source_shift_positions: List[Union[int, str]] = [a % N for a in range(adder_positions.__len__() - 2, -1, -1)] + ["-"]

        for (source_shift_position, adder_position) in zip(source_shift_positions, adder_positions):
            # Аналогично backword_source_positions == adder_position
            if isinstance(source_shift_position, int):
                if P[source_shift_position] == adder_position:
                    count_of_find_positions += 1

                    reverse_text_mask.append("*") # 1
                else:
                    reverse_text_mask.append("-") # 0
            else:
                reverse_text_mask.append("-") # 0

        if count_of_find_positions < N:
            iteration, count_of_find_positions = iteration + 1, 0
            reverse_text_mask.clear()
            source_shift_positions.clear()
        else:
            # Очистка маски от дубликатов битов (из-за увеличения периодов появляется шум и биты могут повторяться)
            while count_of_find_positions > N: reverse_text_mask[reverse_text_mask.index("*")] = "-"; count_of_find_positions -= 1;

            if debug:
                print(f"Source positions:      {source_shift_positions}, {source_shift_positions.__len__()}")
                print(f"Cycle adder positions: {adder_positions}, {adder_positions.__len__()}")
                print(f"Mask adder length:  {adder_positions.__len__()}")

            break

    # (- 1) - потому-что первый бит маски не используется или задан под период в исходных позициях битов, как смещение base multiplier/broadcast multiplier
    # Технически (- 1), может являться константой зависимой от требуемого распостранения битов и может быть другим значением
    # Вот только при изменении, придётся использовать другие методы очистки маски, чтобы избавиться от дублирующихся битов
    # Минимальное смещение, которого достаточно для распостранения (1), следовательно для правильной очистки на этом этапе этого достаточно
    clean_mask_length = (reverse_text_mask.__len__() - 1) - reverse_text_mask.index("*")

    base_multiplier: str = ((N - 2) * "0") + "1" + "0"
    broadcast_text_multiplier: str = base_multiplier * int((clean_mask_length + (N - 1)) / N)

    reverse_text_mask: str = ''.join(reverse_text_mask).replace("-", "0").replace("*", "1")

    return broadcast_text_multiplier, reverse_text_mask

def find_reverse_bits_solution():
    """
    Находит решение логико-арифметической перестановки входящего параметра в обратном порядке
    """

    # Для любого поля N, существует решение в поле циклического сумматора/модуля/периода: S = 2^(N+2) - 1
    N_fields: Final[Tuple[int, int, int, int, int, int]] = (4, 5, 8, 16, 32, 64, 128)
    # Поле циклического сумматора/модуля/периода
    S_fields: Final[Tuple[int, int, int, int, int, int]] = tuple(N + 2 for N in N_fields)

    # В резульате, маска выражения с циклическим сумматором/модулем - вернёт обратное значение для поля в N bit
    #
    # К примеру (8 bit):
    #
    # Исходные позиции битов: [7*, 6**, 5, 4, 3, 2, 1, 0] <- (find_solution() -> source_positions)
    # Поле обратных позиций:  [7, 6, 5, 4, 3, 2, 1*, 0**] -> 7 бит* в 0** бит, 6 бит** в 1 бит* и т.д
    P_fields: Final[Tuple[int, int, int, int, int, int]] = tuple(tuple(range(N-1, -1, -1)) for N in N_fields)

    for (index, N) in enumerate(N_fields):
        print(f"Search reversing bits solution for N={N}...")

        (broadcast_multiplier, reverse_mask) = cast(
            Tuple[str, str],
            find_solution(N, S_fields[index], P_fields[index])
        )

        print(f"For N={N}, the mask was found for S={S_fields[index]}.")

        # print(f"Broadcast multiplier (bin): 0b{broadcast_multiplier}")
        print(f"Broadcast multiplier (HEX): 0x{int(broadcast_multiplier, 2):02X}")

        # print(f"Inverse mask (bin): 0b{reverse_mask}")
        print(f"Inverse mask (HEX): 0x{int(reverse_mask, 2):02X}")

        # print(f"Module (dec): {2**S_fields[index]-1}")
        print(f"Module (HEX): 0x{2**S_fields[index]-1:02X}")

        print("\nReverse bits function example:\n")
        print(
        f"def _rbit_seu{N}(v: int) -> int:"
        f"""
    return ((v * 0x{int(broadcast_multiplier, 2):02X}) & 0x{int(reverse_mask, 2):02X}) % 0x{2**S_fields[index]-1:02X}
        """
        )

def find_same_bits_solution():
    """
    Находит решение логико-арифметической перестановки возвращая входящий параметр

    Возврат исходного числа (8 bit):  [0, 1, 2, 3, 4, 5, 6, 7]
    Возврат исходного числа (16 bit): [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    """

    # Для любого поля N, существует решение в поле циклического сумматора/модуля/периода: S = 2^(N+1) - 1
    N_fields: Final[Tuple[int, int, int, int, int, int]] = (4, 8, 16, 32, 64, 128)
    # Поле циклического сумматора/модуля/периода
    S_fields: Final[Tuple[int, int, int, int, int, int]] = tuple(N + 1 for N in N_fields)

    # В резульате, маска для выражения с циклическим сумматором/модулем - вернёт исходные позиции битов для поля в N bit:
    #
    # К примеру:
    #
    # Исходные позиции битов*: [7*, 6**, 5, 4, 3, 2, 1, 0] <- (find_solution() -> source_positions)
    # Поле обратных позиций:   [0, 1, 2, 3, 4, 5, 6*, 7**] -> 7 бит* в 7** бит, 6 бит** в 6 бит* и т.д
    P_fields: Final[Tuple[int, int, int, int, int, int]] = tuple(tuple(range(0, N, 1)) for N in N_fields)

    for (index, N) in enumerate(N_fields):
        print(f"Search same bits solution for N={N}...")

        (broadcast_multiplier, reverse_mask) = cast(
            Tuple[str, str],
            find_solution(N, S_fields[index], P_fields[index])
        )

        print(f"For N={N}, the mask was found for S={S_fields[index]}.")

        # print(f"Broadcast multiplier (bin): 0b{broadcast_multiplier}")
        print(f"Broadcast multiplier (HEX): 0x{int(broadcast_multiplier, 2):02X}")

        # print(f"Inverse mask (bin): 0b{reverse_mask}")
        print(f"Inverse mask (HEX): 0x{int(reverse_mask, 2):02X}")

        # print(f"Module (dec): {2**S_fields[index]-1}")
        print(f"Module (HEX): 0x{2**S_fields[index]-1:02X}")

        print("\nShuffle function example:\n")
        print(
        f"def _same_su{N}(v: int) -> int:"
        f"""
    return ((v * 0x{int(broadcast_multiplier, 2):02X}) & 0x{int(reverse_mask, 2):02X}) % 0x{2**S_fields[index]-1:02X}
        """
        )

def find_shuffle_halves_solution():
    """
    Находит решение логико-арифметической перестановки половин входящего параметра

    Перестановка частей (8 bit): [4, 5, 6, 7, 0, 1, 2, 3]
    Перестановка частей (16 bit): [8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2, 3, 4, 5, 6, 7]
    """

    # Для любого поля N, существует решение в поле циклического сумматора/модуля/периода: S = 2^(N+1) - 1
    N_fields: Final[Tuple[int, int, int, int, int, int]] = (4, 8, 16, 32, 64, 128)
    # Поле циклического сумматора/модуля/периода
    S_fields: Final[Tuple[int, int, int, int, int, int]] = tuple(N + 1 for N in N_fields)

    # В резульате, маска для выражения с циклическим сумматором/модулем - вернёт переставленные позиции половин для поля в N bit:
    P_fields: Final[Tuple[int, int, int, int, int, int]] = tuple(tuple(range(int(N/2), N, 1)) + tuple(range(0, int(N/2), 1)) for N in N_fields)

    for (index, N) in enumerate(N_fields):
        print(f"Search shuffle halves solution for N={N}...")

        (broadcast_multiplier, reverse_mask) = cast(
            Tuple[str, str],
            find_solution(N, S_fields[index], P_fields[index])
        )

        print(f"For N={N}, the mask was found for S={S_fields[index]}.")

        # print(f"Broadcast multiplier (bin): 0b{broadcast_multiplier}")
        print(f"Broadcast multiplier (HEX): 0x{int(broadcast_multiplier, 2):02X}")

        # print(f"Inverse mask (bin): 0b{reverse_mask}")
        print(f"Inverse mask (HEX): 0x{int(reverse_mask, 2):02X}")

        # print(f"Module (dec): {2**S_fields[index]-1}")
        print(f"Module (HEX): 0x{2**S_fields[index]-1:02X}")

        print("\nShuffle function example:\n")
        print(
        f"def _shuffle_su{N}(v: int) -> int:"
        f"""
    return ((v * 0x{int(broadcast_multiplier, 2):02X}) & 0x{int(reverse_mask, 2):02X}) % 0x{2**S_fields[index]-1:02X}
        """
        )

def find_permutation_N_solutions():
    """
    Находит решение логико-арифметической перестановки подполей вектора

    Определением вектора, является любое поле N > 1. Определением подполя (групп), является любое поле N < VT, зачастую кратное VT.
    """

    # Для любого векторного поля N (VT), существует решение в поле циклического сумматора/модуля/периода: S = 2^(N+1) - 1
    VT_fields: Final[Tuple[int, int, int, int, int, int, int]] = (2, 4, 8, 16, 32, 64, 128)
    # Векторные под-поля типов (группы бит), (vector_sub_fields_groups)
    VSFG_fields: Final[Tuple[int, ...]] = (1, 2, 4, 8, 16, 32, 64)
    # Поле циклического сумматора/модуля/периода
    S_fields: Final[Dict[int, int]] = { VT: S + 2 for VT, S in zip(VT_fields, VT_fields) }

    # Исходные поля битов для каждого вектора (N_source_bits_positions)
    # NSBP: Final[Tuple[int, int, int, int, int, int]] = tuple(tuple(range(0, N, 1) for N in N_fields))

    # Исходные позиции групп под-полей битов (VSFG), внутри поля вектора (VT) для дальнейшей пермутации позиций под-полей, в поле вектора VT:
    #
    # 2:   1: ((0), (1))
    # 4:   1: ((0), (1), (2), (3))
    #      : Пермутация 2-битных групп (half-nibbles в 8 bit нотации)
    #      2: ((0, 1), (2, 3))
    # 8:   1: ((0), (1), (2), (3), (4), (5), (6), (7))
    #      : Пермутация 2-битных групп (half-nibbles в 8 bit нотации)
    #      2: ((0, 1), (2, 3), (4, 5), (6, 7))
    #      4: ((0, 1, 2, 3), (4, 5, 6, 7))
    # 16:  1: ((0), (1), (2), (3), (4}, (5), (6), (7), (8), (9), (10), (11), (12), (13), (14), (15))
    #      : Пермутация 2-битных групп (half-nibbles в 8 bit нотации)
    #      2: ((0, 1), (2, 3), (4, 5), (6, 7), (8, 9), (10, 11), (12, 13), (14, 15))
    #      4: ((0, 1, 2, 3), (4, 5, 6, 7), (8, 9, 10, 11), (12, 13, 14, 15))
    #      8: ((0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11), 12, 13, 14, 15))
    # 32:  1: ((0), (1), (2), (3), (4), (5), (6), (7), (8), (9), (10), (11), (12), (13), (14), (15), (16), (17), (18), (19), (20), (21), (22), (23), (34), (25), (26), (27), (28), (29), (30), (31))
    #      : Пермутация 2-битных групп (half-nibbles в 8 bit нотации)
    #      2: ((0, 1), (2, 3), (4, 5), (6, 7), (8, 9), (10, 11), (12, 13), (14, 15), (16, 17), (18, 19), (20, 21), (22, 23), (34, 25), (26, 27), (28, 29), (30, 31))
    #      4: ((0, 1, 2, 3), (4, 5, 6, 7), (8, 9, 10, 11), (12, 13, 14, 15), (16, 17, 18, 19), (20, 21, 22, 23), (34, 25, 26, 27), (28, 29, 30, 31))
    #      8: ((0, 1, 2, 3, 4, 5, 6, 7), (8, 9, 10, 11, 12, 13, 14, 15), (16, 17, 18, 19, 20, 21, 22, 23), (34, 25, 26, 27, 28, 29, 30, 31))
    #     16: ((0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15), (16, 17, 18, 19, 20, 21, 22, 23, 34, 25, 26, 27, 28, 29, 30, 31))
    #
    # И т.д.
    VSFSG: Final[Dict[int, Dict[int, Tuple[Tuple[int, ...]]]]] = { # VSFSG - Vector sub-fields stock groups
        VT: {
            VSFG: tuple(tuple(range(i, i + VSFG)) for i in range(0, VT, VSFG) if i + VSFG <= VT) for VSFG in VSFG_fields if VSFG < VT
        } for VT in VT_fields
    }

    # Стоковые позиции под-полей (групп позиций - VSFG), внутри векторного поля, для R = VT / VBG, где VT > VBG, где R - количество под-полей, внутри вектора, для заданного под-поля
    # VSFSP_fields: Final[Dict[int, Dict[int, Tuple[int, ...]]]] = { # VSFSP - Vector sub-fields stock positions
    #     VT: {
    #         VSFG: tuple(tuple(range(0, SFSP.__len__()))) for (VSFG, SFSP) in VSFG.items() # SFSP - Sub-fields stock/groups position
    #     } for (VT, VSFG) in VSFSG.items()
    # }

    # Целевые позиции под-полей (групп позиций - VSFG), внутри векторного поля, для R = VT / VBG, где VT > VBG, где R - количество под-полей, внутри вектора, для заданного под-поля
    VSFTP_fields: Final[Dict[int, Dict[int, Tuple[int, ...]]]] = { # VSFTP - Vector sub-fields target positions
        # Перестановка битов в числе v (8 бит) с использованием перестановки подполей (SHUFFLED).
        # - SHUFFLED: перестановка подполей (2-битные или 4-битные группы) по заданным позициям (SFTP).
        # - SFTP (Sub-fields target position): целевые позиции подполей после перестановки.
        8: {
#            1: (7, 6, 5, 4, 3, 2, 1, 0),
            # Пример: для 2-битных подполей 0x1B (0b_0001_1011, подполя: [0]=00, [1]=01, [2]=10, [3]=11) ->
            #         SHUFFLED: 0xE4 (0b_1110_0100, подполя: [0]=11, [1]=10, [2]=01, [3]=00), SFTP: (3,2,1,0).

            # Варианты интерпретации для 2-битных подполей (4x2 бита):
            # Слева-направо (индексы подполей (0,1,2,3), 0 — старшая группа [7,6], 3 — младшая [1,0]):
            # 1) Вход: 0x87 (0b_1000_0111, подполя: [0]=10, [1]=00, [2]=01, [3]=11) ->
            #    SHUFFLED: 0xD2 (0b_1101_0010, подполя: [0]=11, [1]=01, [2]=00, [3]=10), SFTP: (3,2,1,0).
            #
            # Справа-налево (индексы подполей (3,2,1,0), 3 — старшая группа [7,6], 0 — младшая [1,0]):
            # 2) Вход: 0x87 (0b_1000_0111, подполя: [3]=10, [2]=00, [1]=01, [0]=11) ->
            #    SHUFFLED: 0xD2 (0b_1101_0010, подполя: [3]=11, [2]=01, [1]=00, [0]=10), SFTP: (0,1,2,3).

            # SFTP для 2-битных подполей: (3,2,1,0).
#            2: (3, 2, 1, 0), # SFTP - Sub-fields target position
            # Варианты интерпретации для 4-битных подполей (2x4 бита):
            # Слева-направо (индексы подполей (0,1), 0 — старшая группа [7..4], 1 — младшая [3..0]):
            # 3) Вход: 0x87 (0b_1000_0111, подполя: [0]=1000, [1]=0111) ->
            #    SHUFFLED: 0x78 (0b_0111_1000, подполя: [0]=0111, [1]=1000), SFTP: (0,1).
            #
            # Справа-налево (индексы подполей (1,0), 1 — старшая группа [7..4], 0 — младшая [3..0]):
            # 4) Вход: 0x87 (0b_1000_0111, подполя: [1]=1000, [0]=0111) ->
            #    SHUFFLED: 0x78 (0b_0111_1000, подполя: [1]=0111, [0]=1000), SFTP: (1,0).

            # SFTP для 4-битных подполей: (0,1).
#            4: (0, 1) # SFTP - Sub-fields target position
        },
        128: {
            1: (3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12, 19, 18, 17, 16, 23, 22, 21, 20, 27, 26, 25, 24, 31, 30, 29, 28, 35, 34, 33, 32, 39, 38, 37, 36, 43, 42, 41, 40, 47, 46, 45, 44, 51, 50, 49, 48, 55, 54, 53, 52, 59, 58, 57, 56, 63, 62, 61, 60, 67, 66, 65, 64, 71, 70, 69, 68, 75, 74, 73, 72, 79, 78, 77, 76, 83, 82, 81, 80, 87, 86, 85, 84, 91, 90, 89, 88, 95, 94, 93, 92, 99, 98, 97, 96, 103, 102, 101, 100, 107, 106, 105, 104, 111, 110, 109, 108, 115, 114, 113, 112, 119, 118, 117, 116, 123, 122, 121, 120, 127, 126, 125, 124),
#            8: (15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0)
        }
    }

    for (VT, VSFG) in VSFTP_fields.items():
        for (SFG, SFTP) in VSFG.items():
            TP: Final[Tuple[int, ...]] = sum(tuple(VSFSG[VT][SFG][TP] for TP in SFTP), ())

            print(f"Search permutation N solution for VT={VT} with VSFG={VSFG}...")

            (broadcast_multiplier, reverse_mask) = cast(
                Tuple[str, str],
                find_solution(VT, S_fields[VT], TP)
            )

            print(f"For VT={VT}, the mask was found for S={S_fields[VT]}.")

            # print(f"Broadcast multiplier (bin): 0b{broadcast_multiplier}")
            print(f"Broadcast multiplier (HEX): 0x{int(broadcast_multiplier, 2):02X}")

            # print(f"Inverse mask (bin): 0b{reverse_mask}")
            print(f"Inverse mask (HEX): 0x{int(reverse_mask, 2):02X}")

            # print(f"Module (dec): {2**S_fields[index]-1}")
            print(f"Module (HEX): 0x{2**S_fields[VT]-1:02X}")

            print("\nShuffle function example:\n")
            print(
        f"def _permute_v{VT}_g{SFG}(v: int) -> int:"
        f"""
    return ((v * 0x{int(broadcast_multiplier, 2):02X}) & 0x{int(reverse_mask, 2):02X}) % 0x{2**S_fields[VT]-1:02X}
        """
            )

def find_rpermutation_N_solutions():
    """
    Находит решение логико-арифметической перестановки подполей вектора, интерпертируя входящий параметр в обратном порядке, в порядке определяемом пользователем

    Определением вектора, является любое поле N > 1. Определением подполя (групп), является любое поле N < VT, зачастую кратное VT.
    """

    # Для любого векторного поля N (VT), существует решение в поле циклического сумматора/модуля/периода: S = 2^(N+1) - 1
    VT_fields: Final[Tuple[int, int, int, int, int, int, int]] = (2, 4, 8, 16, 32, 64, 128)
    # Векторные под-поля типов (группы бит), (vector_sub_fields_groups)
    VSFG_fields: Final[Tuple[int, ...]] = (1, 2, 4, 8, 16, 32, 64)
    # Поле циклического сумматора/модуля/периода
    S_fields: Final[Dict[int, int]] = { VT: S + 2 for VT, S in zip(VT_fields, VT_fields) }

    # Исходные поля битов для каждого вектора (N_source_bits_positions)
    # NSBP: Final[Tuple[int, int, int, int, int, int]] = tuple(tuple(range(0, N, 1) for N in N_fields))

    # Обрытные/реверснутые позиции групп под-полей битов (VSFG), внутри поля вектора (VT) для дальнейшей пермутации позиций под-полей, в поле вектора VT:
    #
    # 2:   1: ((1), (0))
    # 4:   1: ((3), (2), (1), (0))
    #      : Пермутация 2-битных групп (half-nibbles в 8 bit нотации)
    #      2: ((3, 2), (1, 0))
    # 8:   1: ((7), (6), (5), (4), (3), (2), (1), (0))
    #      : Пермутация 2-битных групп (half-nibbles в 8 bit нотации)
    #      2: ((7, 6), (5, 4), (3, 2), (1, 0))
    #      4: ((7, 6, 5, 4), (3, 2, 1, 0))
    # 16:  1: ((15), (14), (13), (12), (11), (10), (9), (8), (7), (6), (5), (4), (3), (2), (1), (0))
    #      : Пермутация 2-битных групп (half-nibbles в 8 bit нотации)
    #      2: ((15, 14), (13, 12), (11, 10), (9, 8), (7, 6), (5, 4), (3, 2), (1, 0))
    #      4: ((15, 14, 13, 12), (11, 10, 9, 8), (7, 6, 5, 4), (3, 2, 1, 0))
    #      8: ((15, 14, 13, 12, 11, 10, 9, 8), (7, 6, 5, 4, 3, 2, 1, 0))
    # 32:  1: ((31), (30), (29), (28), (27), (26), (25), (24), (23), (22), (21), (20), (19), (18), (17), (16), (15), (14), (13), (12), (11), (10), (9), (8), (7), (6), (5), (4), (3), (2), (1), (0))
    #      : Пермутация 2-битных групп (half-nibbles в 8 bit нотации)
    #      2: ((31, 30), (29, 28), (27, 26), (25, 24), (23, 22), (21, 20), (19, 18), (17, 16), (15, 14), (13, 12), (11, 10), (9, 8), (7, 6), (5, 4), (3, 2), (1, 0))
    #      4: ((31, 30, 29, 28), (27, 26, 25, 24), (23, 22, 21, 20), (19, 18, 17, 16), (15, 14, 13, 12), (11, 10, 9, 8), (7, 6, 5, 4), (3, 2, 1, 0))
    #      8: ((31, 30, 29, 28, 27, 26, 25, 24), (23, 22, 21, 20, 19, 18, 17, 16), (15, 14, 13, 12, 11, 10, 9, 8), (7, 6, 5, 4, 3, 2, 1, 0))
    #     16: ((31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16), (15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0))
    #
    # И т.д.
    VSFSG: Final[Dict[int, Dict[int, Tuple[Tuple[int, ...]]]]] = { # VSFSG - Vector sub-fields stock groups
        VT: {
            VSFG: tuple(tuple(range(i + VSFG - 1, i - 1, -1)) for i in range(VT, -1, -VSFG) if i + VSFG <= VT) for VSFG in VSFG_fields if VSFG < VT
        } for VT in VT_fields
    }

    # Стоковые позиции под-полей (групп позиций - VSFG), внутри векторного поля, для R = VT / VBG, где VT > VBG, где R - количество под-полей, внутри вектора, для заданного под-поля
    # VSFSP_fields: Final[Dict[int, Dict[int, Tuple[int, ...]]]] = { # VSFSP - Vector sub-fields stock positions
    #     VT: {
    #         VSFG: tuple(tuple(range(0, SFSP.__len__()))) for (VSFG, SFSP) in VSFG.items() # SFSP - Sub-fields stock/groups position
    #     } for (VT, VSFG) in VSFSG.items()
    # }

    # Целевые позиции под-полей (групп позиций - VSFG), внутри векторного поля, для R = VT / VBG, где VT > VBG, где R - количество под-полей, внутри вектора, для заданного под-поля
    VSFTP_fields: Final[Dict[int, Dict[int, Tuple[int, ...]]]] = { # VSFTP - Vector sub-fields target positions
        # Перестановка битов в числе v (8 бит) с комбинацией полного реверса (REVERSED-FULL) и перестановки подполей (SHUFFLED).
        # - REVERSED-FULL: полный реверс всех битов числа (например, 0x87 = 0b_1000_0111 -> 0xE1 = 0b_1110_0001).
        # - SHUFFLED: перестановка подполей (2-битные или 4-битные группы) по заданным позициям (SFTP).
        # - SFTP (Sub-fields target position): целевые позиции подполей после перестановки.
        8: {
#            1: (0, 1, 2, 3, 4, 5, 6, 7),
            # Варианты интерпретации для 2-битных подполей (4x2 бита):
            # Слева-направо (индексы подполей (0,1,2,3), 0 — старшая группа [7,6], 3 — младшая [1,0]):
            # 1) Вход: 0x87 (0b_1000_0111, подполя: [0]=10, [1]=00, [2]=01, [3]=11) ->
            #    REVERSED-FULL: 0xE1 (0b_1110_0001, подполя: [0]=11, [1]=10, [2]=00, [3]=01) ->
            #    SHUFFLED: 0xB4 (0b_1011_0100, подполя: [0]=10, [1]=11, [2]=01, [3]=00), SFTP: (1,0,3,2).
            # 2) Вход: 0x87 (0b_1000_0111, подполя: [0]=10, [1]=00, [2]=01, [3]=11) ->
            #    SHUFFLED: 0x78 (0b_0111_1000, подполя: [0]=01, [1]=11, [2]=10, [3]=00), SFTP: (2,3,0,1) ->
            #    REVERSED-2BG (реверс в пределах 2-битных групп): 0xB4 (0b_1011_0100, подполя: [0]=10, [1]=11, [2]=01, [3]=00).

            # Справа-налево (индексы подполей (3,2,1,0), 3 — старшая группа [7,6], 0 — младшая [1,0]):
            # 3) Вход: 0x87 (0b_1000_0111, подполя: [3]=10, [2]=00, [1]=01, [0]=11) ->
            #    REVERSED-FULL: 0xE1 (0b_1110_0001, подполя: [3]=11, [2]=10, [1]=00, [0]=01) ->
            #    SHUFFLED: 0xB4 (0b_1011_0100, подполя: [3]=10, [2]=11, [1]=01, [0]=00), SFTP: (2,3,0,1).
            # 4) Вход: 0x87 (0b_1000_0111, подполя: [3]=10, [2]=00, [1]=01, [0]=11) ->
            #    SHUFFLED: 0x78 (0b_0111_1000, подполя: [3]=01, [2]=11, [1]=10, [0]=00), SFTP: (1,0,3,2) ->
            #    REVERSED-2BG (реверс в пределах 2-битных групп): 0xB4 (0b_1011_0100, подполя: [3]=10, [2]=11, [1]=01, [0]=00).

            # SFTP для 2-битных подполей: (1,0,3,2).

#            2: (1, 0, 3, 2), # SFTP - Sub-fields target position
            # Варианты интерпретации для 4-битных подполей (2x4 бита):
            # Слева-направо (индексы подполей (0,1), 0 — старшая группа [7..4], 1 — младшая [3..0]):
            # 1) Вход: 0x87 (0b_1000_0111, подполя: [0]=1000, [1]=0111) ->
            #    REVERSED-FULL: 0xE1 (0b_1110_0001, подполя: [0]=1110, [1]=0001) ->
            #    SHUFFLED: 0xE1 (0b_1110_0001, подполя: [0]=1110, [1]=0001), SFTP: (1,0).
            # 2) Вход: 0x87 (0b_1000_0111, подполя: [0]=1000, [1]=0111) ->
            #    SHUFFLED: 0x78 (0b_0111_1000, подполя: [0]=0111, [1]=1000), SFTP: (0,1) ->
            #    REVERSED-4BG (реверс в пределах 4-битных групп): 0xE1 (0b_1110_0001, подполя: [0]=1110, [1]=0001).

            # Справа-налево (индексы подполей (1,0), 1 — старшая группа [7..4], 0 — младшая [3..0]):
            # 3) Вход: 0x87 (0b_1000_0111, подполя: [1]=1000, [0]=0111) ->
            #    REVERSED-FULL: 0xE1 (0b_1110_0001, подполя: [1]=1110, [0]=0001) ->
            #    SHUFFLED: 0xE1 (0b_1110_0001, подполя: [1]=1110, [0]=0001), SFTP: (0,1).
            # 4) Вход: 0x87 (0b_1000_0111, подполя: [1]=1000, [0]=0111) ->
            #    SHUFFLED: 0x78 (0b_0111_1000, подполя: [1]=0111, [0]=1000), SFTP: (1,0) ->
            #    REVERSED-4BG (реверс в пределах 4-битных групп): 0xE1 (0b_1110_0001, подполя: [1]=1110, [0]=0001).

            # SFTP для 4-битных подполей: (0,1).
#            4: (0, 1) # SFTP - Sub-fields target position
        },
        128: {
            1: (124, 125, 126, 127, 120, 121, 122, 123, 116, 117, 118, 119, 112, 113, 114, 115, 108, 109, 110, 111, 104, 105, 106, 107, 100, 101, 102, 103, 96, 97, 98, 99, 92, 93, 94, 95, 88, 89, 90, 91, 84, 85, 86, 87, 80, 81, 82, 83, 76, 77, 78, 79, 72, 73, 74, 75, 68, 69, 70, 71, 64, 65, 66, 67, 60, 61, 62, 63, 56, 57, 58, 59, 52, 53, 54, 55, 48, 49, 50, 51, 44, 45, 46, 47, 40, 41, 42, 43, 36, 37, 38, 39, 32, 33, 34, 35, 28, 29, 30, 31, 24, 25, 26, 27, 20, 21, 22, 23, 16, 17, 18, 19, 12, 13, 14, 15, 8, 9, 10, 11, 4, 5, 6, 7, 0, 1, 2, 3),
#            8: (15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0)
        }
    }

    for (VT, VSFG) in VSFTP_fields.items():
        for (SFG, SFTP) in VSFG.items():
            TP: Final[Tuple[int, ...]] = sum(tuple(VSFSG[VT][SFG][TP] for TP in SFTP), ())

            print(f"Search permutation N solution for VT={VT} with VSFG={VSFG}...")

            (broadcast_multiplier, reverse_mask) = cast(
                Tuple[str, str],
                find_solution(VT, S_fields[VT], TP)
            )

            print(f"For VT={VT}, the mask was found for S={S_fields[VT]}.")

            # print(f"Broadcast multiplier (bin): 0b{broadcast_multiplier}")
            print(f"Broadcast multiplier (HEX): 0x{int(broadcast_multiplier, 2):02X}")

            # print(f"Inverse mask (bin): 0b{reverse_mask}")
            print(f"Inverse mask (HEX): 0x{int(reverse_mask, 2):02X}")

            # print(f"Module (dec): {2**S_fields[index]-1}")
            print(f"Module (HEX): 0x{2**S_fields[VT]-1:02X}")

            print("\nShuffle function example:\n")
            print(
        f"def _rpermute_v{VT}_g{SFG}(v: int) -> int:"
        f"""
    return ((v * 0x{int(broadcast_multiplier, 2):02X}) & 0x{int(reverse_mask, 2):02X}) % 0x{2**S_fields[VT]-1:02X}
        """
            )

if __name__ == "__main__":
    find_reverse_bits_solution()
#    find_same_bits_solution()
#    find_shuffle_halves_solution()
#    find_permutation_N_solutions()
#    find_rpermutation_N_solutions()
