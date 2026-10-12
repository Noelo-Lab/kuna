#!/usr/bin/env python3
"""Authored, Apache-2.0 ARM/Thumb ELF fixtures; no compiler or binary inputs.

Generate with: python3 arm_xref_roots.py OUTPUT [arm|thumb] [little|big] [symbols]
Variants: mixed, wide, aifprefix, be8, aifcount, aiffingerprint, splitbody,
lateprefix (with aifprefix), predreturn, noreturnpool, wideprefix, staleprefix,
stalecallee/staleblx/rejectedblx/backwardblx/corpusblx/chainprefix/longchainprefix (with staleprefix),
cycleprefix/cycleanchor (with chainprefix), selfcycleprefix (with staleprefix),
calledprefix (with cycleprefix or selfcycleprefix), interworkfp,
mappedcallee/latecallee (with interworkfp), staleisa, stalegap/interiorisa (with staleisa), prefixisa (with stalegap), prefixpool, focusmode, gapmode, splitgapcallee/gapbranch (with gapmode), gapbound (with gapbranch), rootmode (with focusmode), armcallee (with focusmode/gapmode), establishedmode (with focusmode),
establishedsplit (with establishedmode), prefixmode, backwardframe/splitframe (with focusmode/rootmode), recursiveframe/unclaimedmode/focusblocks/recoveredpublish (with splitframe), xrefpublish (with focusblocks), aifseedmode, staleaif.
The entry calls twenty leaves, stocking AIF's mov/mov fingerprint. Two callers
have unique frame prologues and no inbound direct call. The first has an early
return before a later loop; its interior matches the leaves' fingerprint.
"""
import pathlib
import struct
import sys

BASE = 0x1000


def image(thumb=False, big=False, symbols=False, mixed=False, wide=False, aifprefix=False, be8=False,
          aifcount=False, aiffingerprint=False, splitbody=False, lateprefix=False, predreturn=False, noreturnpool=False,
          wideprefix=False, staleprefix=False, stalecallee=False, staleblx=False, rejectedblx=False,
          backwardblx=False, chainprefix=False, longchainprefix=False,
          cycleprefix=False, cycleanchor=False, selfcycleprefix=False, corpusblx=False,
          calledprefix=False, interworkfp=False, mappedcallee=False, latecallee=False,
          staleisa=False, stalegap=False, interiorisa=False, prefixisa=False, prefixpool=False, focusmode=False, gapmode=False, splitgapcallee=False, gapbranch=False, gapbound=False, rootmode=False, armcallee=False, establishedmode=False, establishedsplit=False, prefixmode=False, backwardframe=False, splitframe=False, recursiveframe=False, unclaimedmode=False, focusblocks=False, xrefpublish=False, aifseedmode=False, staleaif=False, recoveredpublish=False):
    order = '>' if big or be8 else '<'
    instruction_order = '<' if be8 else order
    pack = lambda fmt, *values: struct.pack(order + fmt, *values)
    code = bytearray(0x800)

    def put(at, data):
        code[at - BASE:at - BASE + len(data)] = data

    def word(at, value):
        put(at, struct.pack(instruction_order + 'I', value))

    def half(at, value):
        put(at, struct.pack(instruction_order + 'H', value))

    def arm_branch(at, target, opcode=0xEB000000):
        word(at, opcode | (((target - at - 8) // 4) & 0xFFFFFF))

    def thumb_call(at, target):
        offset = target - at - 4
        half(at, 0xF000 | ((offset >> 12) & 0x7FF))
        half(at + 2, 0xF800 | ((offset >> 1) & 0x7FF))

    def thumb_branch(at, target, conditional=False):
        half(at, (0xD100 if conditional else 0xE000)
             | (((target - at - 4) // 2) & (0xFF if conditional else 0x7FF)))

    for at in range(BASE, BASE + len(code), 2 if thumb else 4):
        if thumb:
            half(at, 0x4770)
        else:
            word(at, 0xE12FFF1E)
    names = [(BASE, 'entry')]
    if thumb:
        half(BASE, 0xB500)
        for i in range(20):
            target = 0x1100 + 8 * i
            thumb_call(BASE + 2 + 4 * i, target)
            half(target, 0x2000 | i)
            half(target + 2, 0x2100 | i)
            names.append((target, 'leaf_' + str(i)))
        half(BASE + 82, 0xBD00)
        for at, value in [(0x1400, 0xB510), (0x1402, 0x4604), (0x1404, 0x2C00),
                          (0x1408, 0xBD10), (0x1410, 0x2007), (0x1412, 0x2100),
                          (0x1418, 0x3C01), (0x1430, 0x2C00), (0x1434, 0xBD10),
                          (0x1500, 0xB510), (0x1502, 0x4C03), (0x1508, 0xBD10),
                          (0x1520, 0xB510), (0x1522, 0x4604), (0x1528, 0xBD10),
                          (0x1600, 0xB510), (0x1602, 0x4604), (0x1608, 0xBD10),
                          (0x1700, 0xB510), (0x1702, 0x4604), (0x1708, 0xBD10)]:
            half(at, value)
        thumb_branch(0x1406, 0x1430, True)
        thumb_call(0x1414, 0x1100)
        thumb_branch(0x141A, 0x1430)
        thumb_branch(0x1432, 0x1410, True)
        for at, to in [(0x1504, 0x1100), (0x1524, 0x1108),
                       (0x1604, 0x9000), (0x1704, 0x1110)]:
            thumb_call(at, to)
        thumb_branch(0x1604, 0x1900)
        thumb_call(0x1606, 0x9000)
        half(0x160A, 0xBD10)
    else:
        word(BASE, 0xE92D4000)
        for i in range(20):
            target = 0x1100 + 16 * i
            arm_branch(BASE + 4 + 4 * i, target)
            word(target, 0xE3A00000 | i)
            word(target + 4, 0xE3A01000 | i)
            names.append((target, 'leaf_' + str(i)))
        word(BASE + 84, 0xE8BD8000)
        for at, value in [(0x1400, 0xE92D4010), (0x1404, 0xE2504000),
                          (0x140C, 0xE8BD8010), (0x1410, 0xE3A00007),
                          (0x1414, 0xE3A01000), (0x141C, 0xE2544001),
                          (0x1430, 0xE3540000), (0x1438, 0xE8BD8010),
                          (0x1500, 0xE92D4010), (0x1504, 0xE59F4004),
                          (0x150C, 0xE8BD8010), (0x1510, 0),
                          (0x1520, 0xE92D4010), (0x1524, 0xE1A04000),
                          (0x152C, 0xE8BD8010), (0x1600, 0xE92D4010),
                          (0x1604, 0xE1A04000), (0x160C, 0xE8BD8010),
                          (0x1700, 0xE92D4010), (0x1704, 0xE1A04000),
                          (0x170C, 0xE8BD8010)]:
            word(at, value)
        arm_branch(0x1408, 0x1430, 0x1A000000)
        arm_branch(0x1418, 0x1100)
        arm_branch(0x1420, 0x1430, 0xEA000000)
        arm_branch(0x1434, 0x1410, 0x1A000000)
        for at, to in [(0x1508, 0x1100), (0x1528, 0x1110),
                       (0x1608, 0x9000), (0x1708, 0x1120)]:
            arm_branch(at, to)
        arm_branch(0x1604, 0x1900, 0xEA000000)
    names += [(0x1400, 'loop_caller'), (0x1500, 'load_caller'),
              (0x1520, 'adjacent'), (0x1700, 'unreferenced')]
    if mixed:
        assert not thumb
        half(0x1700, 0xB510)
        half(0x1702, 0x4604)
        thumb_call(0x1704, 0x1740)
        half(0x1708, 0xBD10)
        for at in range(0x170A, 0x1800, 2):
            half(at, 0x4770)
        half(0x1740, 0x2001)
        half(0x1742, 0x2102)
    if wide:
        assert thumb
        half(0x1502, 0xF8DD)
        half(0x1504, 0xB510)  # ldr.w fp, [sp, #0x510], not a push at 0x1504
        thumb_call(0x1506, 0x1100)
        half(0x150A, 0xBD10)
    if aifprefix:
        assert not thumb
        word(0x14FC, 0xE1A0C00D)  # mov ip, sp before the frame
        for i in range(20):
            target = 0x1100 + 16 * i
            word(target, 0xE1A0C00D)
            word(target + 4, 0xE92D4000)
            word(target + 8, 0xE8BD8000)
    if wideprefix:
        assert thumb
        half(0x1500, 0x2007)
        half(0x1502, 0x2100)
        half(0x1504, 0xF8DD)
        half(0x1506, 0xB510)  # ldr.w fp, [sp, #0x510], not a push at 0x1506
        thumb_call(0x1508, 0x1100)
        half(0x150C, 0xBD10)
    if staleprefix:
        assert thumb
        half(BASE + 74, 0xBD00)  # entry plus eighteen leaves is below AIF's threshold
        for at in range(0x1400, 0x1800, 2):
            half(at, 0x4770)
        half(0x1500, 0x2007)
        half(0x1502, 0x2100)
        half(0x1504, 0xF8DD)
        half(0x1506, 0xE92D)  # ldr.w lr, [sp, #0x92d]
        half(0x1508, 0x4770)  # bx lr; also the false push.w register mask
        thumb_call(0x150A, 0x1700)
        half(0x150E, 0x4798)  # blx r3 in the obsolete speculative body
        half(0x1510, 0xBD10)
        half(0x1600, 0xB510)
        half(0x1602, 0x4604)
        thumb_call(0x1604, 0x1100)
        half(0x1608, 0xBD10)
        half(0x1700, 0x2A00)  # this leaf is reachable only through the phantom call
        half(0x1702, 0x2B00)
        thumb_call(0x1704, 0x1100)
    if stalecallee:
        assert staleprefix
        thumb_call(0x150A, 0x1600)
        for at in range(0x1512, 0x1600, 2):
            half(at, 0xBF00)  # no epilogue pair independently corroborates 0x1600
    if staleblx or rejectedblx:
        assert staleprefix
        half(0x150A, 0xF000)
        half(0x150C, 0xE8FA)  # blx 0x1700 after the real return
        if rejectedblx:
            thumb_branch(0x1510, 0x1900)
    if backwardblx or corpusblx:
        assert staleprefix
        half(0x150A, 0xF7FF)
        half(0x150C, 0xEDFA if corpusblx else 0xEFF2)  # blx 0x1100 or 0x14f0
    if chainprefix or longchainprefix or selfcycleprefix:
        assert staleprefix
        entries = [0x1500, 0x1580, 0x1600] if longchainprefix else [0x1500, 0x1600]
        if selfcycleprefix:
            entries = [0x1500]
        for at in range(0x1500, 0x1700, 2):
            half(at, 0xBF00)
        for i, entry in enumerate(entries):
            half(entry, 0x2007)
            half(entry + 2, 0x2100)
            half(entry + 4, 0xF8DD)
            half(entry + 6, 0xE92D)  # the second halfword is a false push.w
            half(entry + 8, 0x4770)  # real return, before the speculative call
            target = entries[i + 1] + 6 if i + 1 < len(entries) else 0x1700
            if cycleprefix or selfcycleprefix:
                target = entries[(i + 1) % len(entries)] + (0 if calledprefix else 6)
            thumb_call(entry + 10, target)
            if cycleprefix or selfcycleprefix:
                thumb_call(entry + 14, 0x1100)
                half(entry + 18, 0xBD10)
            else:
                half(entry + 14, 0xBD10)
        if cycleanchor:
            assert cycleprefix
            half(0x1700, 0xB510)
            half(0x1702, 0x4604)
            thumb_call(0x1704, 0x1506)
            half(0x1708, 0xBD10)
    if aifcount or aiffingerprint or splitbody:
        assert not thumb and not symbols
        for at in range(0x1400, 0x1800, 4):
            word(at, 0xE12FFF1E)
        word(0x1500, 0xE92D4010)
        word(0x1504, 0xE1A04000)
        word(0x1508, 0xE8BD8010)
    if aifcount or aiffingerprint:
        if aifcount:
            word(BASE + 76, 0xE8BD8000)  # entry + eighteen leaves = nineteen
        if aiffingerprint:
            for at in [0x1100, 0x1110, 0x1120]:
                word(at, 0xE92D4010)
                word(at + 4, 0xE1A04000)
                word(at + 8, 0xE8BD8010)
        word(0x1550, 0xE92D0010 if aiffingerprint else 0xE3A00007)
        word(0x1554, 0xE1A04000 if aiffingerprint else 0xE3A01000)
        word(0x1558, 0xE1A02002)
        arm_branch(0x155C, 0x1500)
        if aiffingerprint:
            word(0x1560, 0xE8BD0010)
    if splitbody:
        arm_branch(BASE + 4, 0x1500)
        arm_branch(0x1500, 0x1750, 0xEA000000)
        arm_branch(0x1750, 0x1100)
        word(0x1754, 0xE12FFF33)  # blx r3
        word(0x1600, 0xE92D4010)
        word(0x1604, 0xE1A04000)
        word(0x1608, 0xE8BD8010)
    if lateprefix:
        assert aifprefix
        word(BASE + 76, 0xE8BD8000)
        arm_branch(0x1508, 0x1700)
    if predreturn:
        assert not thumb
        for i in range(20):
            target = 0x1100 + 16 * i
            word(target, 0xE92D4010)
            word(target + 4, 0xE3500000)
            word(target + 8, 0xE8BD8010)
        for at in range(0x1400, 0x1440, 4):
            word(at, 0xE12FFF1E)
        for at, value in [(0x1400, 0xE92D4010), (0x1404, 0xE3500000),
                          (0x1408, 0x08BD8010), (0x140C, 0xE92D4010),
                          (0x1414, 0xE8BD4010), (0x1418, 0xE8BD8010)]:
            word(at, value)
        arm_branch(0x1410, 0x1100)
    if noreturnpool:
        assert not thumb
        for i in range(20):
            target = 0x1100 + 16 * i
            word(target, 0xE92D4010)
            word(target + 4, 0xE3500000)
            word(target + 8, 0xE8BD8010)
        for at in range(0x1400, 0x1480, 4):
            word(at, 0xE12FFF1E)
        # Two frames each end with a conditional return, a call to a stub that
        # never returns and literal-pool words. The first pool decodes as valid
        # instructions that fall into the next frame; the second does not decode.
        for at, value in [(0x1400, 0xE92D4010), (0x1404, 0xE3500000),
                          (0x1408, 0x08BD8010), (0x1410, 0), (0x1414, 0),
                          (0x1418, 0xE92D4010), (0x141C, 0xE1A04000),
                          (0x1424, 0xE8BD8010),
                          (0x1440, 0xE92D4010), (0x1444, 0xE3500000),
                          (0x1448, 0x08BD8010), (0x1450, 0xFFFFFFFF),
                          (0x1454, 0xFFFFFFFF),
                          (0x1458, 0xE92D4010), (0x145C, 0xE1A04000),
                          (0x1464, 0xE8BD8010),
                          (0x14C0, 0xEAFFFFFE)]:
            word(at, value)
        arm_branch(0x140C, 0x14C0)
        arm_branch(0x1420, 0x1100)
        arm_branch(0x144C, 0x14C0)
        arm_branch(0x1460, 0x1110)
    mappings = []
    if interworkfp:
        assert not thumb and not symbols
        for at in range(0x1400, 0x1800, 4):
            word(at, 0xE12FFF1E)
        for at in range(0x1400, 0x1600, 2):
            half(at, 0x4770)
        for i in range(3):
            target = 0x1100 + 16 * i
            arm_branch(BASE + 4 + 4 * i, target, 0xFA000000)
            half(target, 0x2007)
            half(target + 2, 0x2100)
            half(target + 4, 0x4770)
        half(0x1500, 0x2007)
        half(0x1502, 0x2100)
        half(0x1504, 0xF8DD)
        half(0x1506, 0xE92D)  # ldr.w lr, [sp, #0x92d], not a push
        half(0x1508, 0x4770)
        thumb_call(0x150A, 0x1100)
        half(0x150E, 0xBD10)
        word(0x1600, 0xE92D4010)
        word(0x1604, 0xE1A04000)
        arm_branch(0x1608, 0x1700, 0xFA000000)
        word(0x160C, 0xE8BD8010)
        half(0x1700, 0x2007)
        half(0x1702, 0x2100)
        thumb_call(0x1704, 0x1100)
        half(0x1708, 0x4770)
        if latecallee:
            arm_branch(0x1608, 0x16F8, 0xFA000000)
            for at, value in [(0x16F8, 0x2800), (0x16FA, 0x2900),
                              (0x16FC, 0x4600), (0x16FE, 0x4609)]:
                half(at, value)
            word(0x1620, 0xE92D4010)
            word(0x1624, 0xE1A04000)
            arm_branch(0x1628, 0x1700, 0xFA000000)
            word(0x162C, 0xE8BD8010)
        mappings = [(BASE, '$a', 0), (0x1100, '$t', 0), (0x1130, '$a', 0),
                    (0x1400, '$t', 0), (0x1600, '$a', 0)]
        if mappedcallee:
            mappings.append((0x1700, '$t', 0))
    if staleisa:
        assert not thumb and not symbols
        for at in range(0x1400 if stalegap else BASE, BASE + len(code), 4):
            word(at, 0xE12FFF1E)
        word(0x1500, 0xE92D4010)
        arm_branch(0x1504, 0x1700)
        word(0x1508, 0xE8BD8000 if big else 0xE8BD4010)
        word(0x150C, 0xE12FFF1E)
        word(0x1650, 0xE1A0C00D)
        word(0x1654, 0xE92D4010)
        arm_branch(0x1658, 0x1500, 0xFA000000)
        word(0x165C, 0xE8BD8010)
    if stalegap:
        assert staleisa
        for at in range(0x1650, 0x1660, 4):
            word(at, 0xE12FFF1E)
        word(0x1400, 0xE3A00007)
        word(0x1404, 0xE3A01000)
        word(0x1408, 0xE52DE004)
        arm_branch(0x140C, 0x1500, 0xFA000000)
        word(0x1410, 0xE49DF004)
    if prefixpool:
        assert not thumb and not symbols
        for at in [0x1110, 0x1120, 0x1130, 0x1300, 0x14FC]:
            word(at, 0xE59F4000 | (0x16B0 - at - 8))
            if at != 0x14FC:
                word(at + 4, 0xE92D4010)
                word(at + 8, 0xE8BD8010)
        word(0x1504, 0xE08F0004)
        word(0x16B0, 0x1780 - 0x150C)
        word(0x1600, 0xE92D4010)
        word(0x1604, 0xE1A04000)
        arm_branch(0x1608, 0x1300)
        word(0x160C, 0xE8BD8010)
    if prefixisa:
        assert staleisa and stalegap
        word(0x14FC, 0xE1A0C00D)
        for at in [0x1100, 0x1110, 0x1120, 0x1300]:
            word(at, 0xE1A0C00D)
            word(at + 4, 0xE92D4010)
            word(at + 8, 0xE8BD8010)
        word(0x1600, 0xE92D4010)
        word(0x1604, 0xE1A04000)
        arm_branch(0x1608, 0x1300)
        word(0x160C, 0xE8BD8010)
    if interiorisa:
        assert staleisa
        at = 0x140C if stalegap else 0x1658
        arm_branch(at, 0x1504 if big else 0x1502, 0xFA000000 if big else 0xFB000000)
    if focusmode:
        assert not thumb and not symbols
        for at in range(BASE, BASE + len(code), 4):
            word(at, 0xE12FFF1E)
        word(0x1500, 0xE92D4010)
        arm_branch(0x1504, 0x1580, 0xFA000000)
        word(0x1508, 0xE8BD8010)
        half(0x1580, 0xB510)
        half(0x1582, 0x4604)
        thumb_call(0x1584, 0x1300)
        half(0x1300, 0x4770)
        half(0x1588, 0xBD10)
        word(0x1600, 0xE52DE004)
        arm_branch(0x1604, 0x1200)
        word(0x1608, 0xE49DF004)
    if gapmode:
        assert not thumb and not symbols
        for at in range(0x1400, BASE + len(code), 4):
            word(at, 0xE12FFF1E)
        word(0x1400, 0xE3A00007)
        word(0x1404, 0xE3A01000)
        word(0x1408, 0xE52DE004)
        arm_branch(0x140C, 0x1100)
        arm_branch(0x1410, 0x1600, 0xEA000000)
        arm_branch(0x1600, 0x1200)
        word(0x1604, 0xE49DF004)
        word(0x1500, 0xE92D4010)
        arm_branch(0x1504, 0x1580, 0xFA000000)
        word(0x1508, 0xE8BD8010)
        half(0x1580, 0xB510)
        half(0x1582, 0x4604)
        thumb_call(0x1584, 0x1300)
        half(0x1588, 0xBD10)
        half(0x1300, 0x4770)
    if gapbranch:
        assert gapmode
        arm_branch(0x140C, 0x1600, 0xEA000000)
        word(0x1410, 0xE12FFF1E)
    if gapbound:
        assert gapbranch
        arm_branch(BASE + 84, 0x15C0)
        word(BASE + 88, 0xE8BD8000)
    if splitgapcallee:
        assert gapmode
        thumb_branch(0x1584, 0x1680)
        thumb_call(0x1680, 0x1300)
        half(0x1684, 0xBD10)
    if rootmode:
        assert focusmode
        word(0x1600, 0xE92D4010)
        word(0x1608, 0xE8BD8010)
    if armcallee:
        assert focusmode or gapmode
        assert not splitgapcallee
        arm_branch(0x1504, 0x1580)
        word(0x1580, 0xE92D4010)
        arm_branch(0x1584, 0x1300)
        word(0x1588, 0xE8BD8010)
        word(0x1300, 0xE12FFF1E)
    if backwardframe:
        assert focusmode and rootmode
        for at in range(0x1580, 0x158C, 4):
            word(at, 0xE12FFF1E)
        for at, target, opcode in [(0x1500, 0x1200, 0xEB000000),
                                   (0x1600, 0x1380, 0xEB000000 if armcallee else 0xFA000000)]:
            word(at, 0xE92D4010)
            arm_branch(at + 4, target, opcode)
            word(at + 8, 0xE8BD8010)
        if armcallee:
            word(0x1380, 0xE92D4010)
            arm_branch(0x1384, 0x1300)
            word(0x1388, 0xE8BD8010)
        else:
            half(0x1380, 0xB510)
            half(0x1382, 0x4604)
            thumb_call(0x1384, 0x1300)
            half(0x1388, 0xBD10)
    if splitframe:
        assert focusmode and rootmode
        for at in range(0x1380, BASE + len(code), 4):
            word(at, 0xE12FFF1E)
        caller = 0x1700 if backwardframe else 0x1400
        word(caller, 0xE92D4010)
        arm_branch(caller + 4, 0x1680, 0xEB000000 if armcallee else 0xFA000000)
        word(caller + 8, 0xE8BD8010)
        word(0x1600, 0xE92D4010)
        arm_branch(0x1604, 0x1750, 0xEA000000)
        arm_branch(0x1750, 0x1100)
        word(0x1754, 0xE8BD8010)
        if armcallee:
            word(0x1680, 0xE92D4010)
            arm_branch(0x1684, 0x1300)
            word(0x1688, 0xE8BD8010)
        else:
            half(0x1680, 0xB510)
            half(0x1682, 0x4604)
            thumb_call(0x1684, 0x1300)
            half(0x1688, 0xBD10)
    if recoveredpublish:
        assert splitframe and not backwardframe
        for at in [0x1400, 0x1404, 0x1408]:
            word(at, 0xE12FFF1E)
        word(0x1640, 0xE92D4010)
        arm_branch(0x1644, 0x1680, 0xEB000000 if armcallee else 0xFA000000)
        word(0x1648, 0xE8BD8010)
    if recursiveframe:
        assert splitframe
        arm_branch(0x1604, 0x1600)
        arm_branch(0x1608, 0x1750, 0xEA000000)
    if unclaimedmode or focusblocks:
        assert splitframe
        word(0x1600, 0xE12FFF1E)
        word(0x1604, 0xE12FFF1E)
        if focusblocks:
            word(0x1500, 0xE1A0C00E)
            arm_branch(0x1504, 0x1750, 0xEA000000)
            arm_branch(0x1750, 0x1100)
            word(0x1754, 0xE1A0E00C)
            word(0x1758, 0xE12FFF1E)
        else:
            word(0x1750, 0xE1A0C00E)
            arm_branch(0x1754, 0x1100)
            word(0x1758, 0xE1A0E00C)
            word(0x175C, 0xE12FFF1E)
    if xrefpublish:
        assert focusblocks
        word(BASE, 0xE92D4000)
        arm_branch(BASE + 4, 0x1500)
        word(BASE + 8, 0xE8BD8000)
    if aifseedmode:
        assert not thumb and not symbols
        for at in range(0x1380, BASE + len(code), 4):
            word(at, 0xE12FFF1E)
        word(0x1500, 0xE3A00007)
        word(0x1504, 0xE3A01000)
        word(0x1508, 0xE52DE004)
        arm_branch(0x150C, 0x1750, 0xEA000000)
        arm_branch(0x1750, 0x1200)
        word(0x1754, 0xE49DF004)
        word(0x1700, 0xE92D4010)
        arm_branch(0x1704, 0x1450, 0xFA000000)
        word(0x1708, 0xE8BD8010)
        half(0x1450, 0xB510)
        half(0x1452, 0x4604)
        thumb_call(0x1454, 0x1300)
        half(0x1458, 0xBD10)
        half(0x1300, 0x4770)
    if staleaif:
        assert not thumb and not symbols
        for at in range(0x1380, BASE + len(code), 4):
            word(at, 0xE12FFF1E)
        word(0x1400, 0xE92D4010)
        arm_branch(0x1404, 0x160C, 0xFA000000)
        word(0x1408, 0xE8BD8010)
        word(0x1600, 0xE3A00007)
        word(0x1604, 0xE3A01000)
        word(0x1608, 0xE1A0C00E)
        half(0x160C, 0x2007)
        half(0x160E, 0x4770)
        word(0x1300, 0xE3A00007)
        word(0x1304, 0xE3A01000)
        word(0x1308, 0xE52DE004)
        arm_branch(0x130C, 0x1200)
        word(0x1310, 0xE49DF004)
    if establishedmode:
        assert focusmode
        word(BASE, 0xE92D4000)
        arm_branch(BASE + 4, 0x1600)
        word(BASE + 8, 0xE8BD8000)
    if establishedsplit:
        assert establishedmode
        arm_branch(BASE + 4, 0x1400)
        word(0x1400, 0xE52DE004)
        arm_branch(0x1404, 0x1604, 0xEA000000)
        word(0x1600, 0xE12FFF1E)
    if prefixmode:
        assert not thumb and not symbols
        for at in range(0x1300, 0x1800, 4):
            word(at, 0xE12FFF1E)
        for at in [0x1100, 0x1110, 0x1120, 0x1300]:
            word(at, 0xE1A0C00D)
            word(at + 4, 0xE92D4000)
            word(at + 8, 0xE8BD8000)
        word(0x1500, 0xE92D4010)
        arm_branch(0x1504, 0x1300)  # fourth matching fingerprint, reached by recovery
        arm_branch(0x1508, 0x1580, 0xFA000000)
        word(0x150C, 0xE8BD8010)
        half(0x1580, 0x2001)
        half(0x1582, 0x2100)
        half(0x1584, 0x4770)
        word(0x1600, 0xE1A0C00D)
        word(0x1604, 0xE92D4010)
        arm_branch(0x1608, 0x1200)
        word(0x160C, 0xE8BD8010)
    textoff = 0x100
    shstr = b'\0.text\0.shstrtab\0.symtab\0.strtab\0'
    content = bytearray(textoff) + code
    shstroff = len(content)
    content += shstr
    sections = [(0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
                (1, 1, 6, BASE, textoff, len(code), 0, 0, 4, 0),
                (7, 3, 0, 0, shstroff, len(shstr), 0, 0, 1, 0)]
    if symbols or mixed or mappings:
        strings = bytearray(b'\0')
        syms = bytearray(16)
        records = [(at, name, 0x12) for at, name in names] if symbols else []
        if mixed:
            records += [(BASE, '$a', 0), (0x1700, '$t', 0)]
        records += mappings
        records.sort(key=lambda item: item[2])
        for at, name, info in records:
            nameoff = len(strings)
            strings += name.encode() + b'\0'
            syms += pack('IIIBBH', nameoff, at | int(thumb or (mixed and at >= 0x1700 and info == 0x12)), 0, info, 0, 1)
        while len(content) % 4:
            content += b'\0'
        symoff = len(content)
        content += syms
        stroff = len(content)
        content += strings
        sections += [(17, 2, 0, 0, symoff, len(syms), 4, 1 + sum(info == 0 for _, _, info in records), 4, 16),
                     (25, 3, 0, 0, stroff, len(strings), 0, 0, 1, 0)]
    while len(content) % 4:
        content += b'\0'
    shoff = len(content)
    content += b''.join(pack('10I', *section) for section in sections)
    ident = b'\x7fELF' + bytes([1, 2 if big or be8 else 1, 1]) + bytes(9)
    content[:52] = ident + pack('HHIIIIIHHHHHH', 2, 40, 1, BASE | int(thumb),
                              52, shoff, 0x05000000 | (0x00800000 if be8 else 0), 52, 32, 1, 40, len(sections), 2)
    content[52:84] = pack('8I', 1, textoff, BASE, BASE, len(code), len(code), 5, 4)
    return content


if __name__ == '__main__':
    pathlib.Path(sys.argv[1]).write_bytes(image('thumb' in sys.argv[2:],
                                              'big' in sys.argv[2:],
                                              'symbols' in sys.argv[2:],
                                              'mixed' in sys.argv[2:],
                                              'wide' in sys.argv[2:],
                                              'aifprefix' in sys.argv[2:],
                                              'be8' in sys.argv[2:],
                                              'aifcount' in sys.argv[2:],
                                              'aiffingerprint' in sys.argv[2:],
                                              'splitbody' in sys.argv[2:],
                                              'lateprefix' in sys.argv[2:],
                                              'predreturn' in sys.argv[2:],
                                              'noreturnpool' in sys.argv[2:],
                                              'wideprefix' in sys.argv[2:],
                                              'staleprefix' in sys.argv[2:],
                                              'stalecallee' in sys.argv[2:],
                                              'staleblx' in sys.argv[2:],
                                              'rejectedblx' in sys.argv[2:],
                                              'backwardblx' in sys.argv[2:],
                                              'chainprefix' in sys.argv[2:],
                                              'longchainprefix' in sys.argv[2:],
                                              'cycleprefix' in sys.argv[2:],
                                              'cycleanchor' in sys.argv[2:],
                                              'selfcycleprefix' in sys.argv[2:],
                                              'corpusblx' in sys.argv[2:],
                                              'calledprefix' in sys.argv[2:],
                                              'interworkfp' in sys.argv[2:],
                                              'mappedcallee' in sys.argv[2:],
                                              'latecallee' in sys.argv[2:],
                                              'staleisa' in sys.argv[2:],
                                              'stalegap' in sys.argv[2:],
                                              'interiorisa' in sys.argv[2:],
                                              'prefixisa' in sys.argv[2:],
                                              'prefixpool' in sys.argv[2:],
                                              'focusmode' in sys.argv[2:],
                                              'gapmode' in sys.argv[2:],
                                              'splitgapcallee' in sys.argv[2:],
                                              'gapbranch' in sys.argv[2:],
                                              'gapbound' in sys.argv[2:],
                                              'rootmode' in sys.argv[2:],
                                              'armcallee' in sys.argv[2:],
                                              'establishedmode' in sys.argv[2:],
                                              'establishedsplit' in sys.argv[2:],
                                              'prefixmode' in sys.argv[2:],
                                              'backwardframe' in sys.argv[2:],
                                              'splitframe' in sys.argv[2:],
                                              'recursiveframe' in sys.argv[2:],
                                              'unclaimedmode' in sys.argv[2:],
                                              'focusblocks' in sys.argv[2:],
                                              'xrefpublish' in sys.argv[2:],
                                              'aifseedmode' in sys.argv[2:],
                                              'staleaif' in sys.argv[2:],
                                              'recoveredpublish' in sys.argv[2:]))
