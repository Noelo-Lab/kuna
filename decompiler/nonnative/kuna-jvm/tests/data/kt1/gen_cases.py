"""Hand-made `kotlin/Metadata` values for KtMetadataDump's `cases` mode: wire
edge cases (packed/unpacked, unknown fields of every wire type, truncation,
overlong varints, required fields, merges, recursion, limits), string-table
and resolver corner cases, BitEncoding modes and annotation-value casts.
The expected behaviour is whatever the plugin's own classes do with them.
"""
import struct

out = []


def varint(n):
    n &= (1 << 64) - 1
    b = bytearray()
    while True:
        x = n & 0x7f
        n >>= 7
        if n:
            b.append(x | 0x80)
        else:
            b.append(x)
            return bytes(b)


def tag(f, wt):
    return varint((f << 3) | wt)


def vi(f, n):
    return tag(f, 0) + varint(n)


def ld(f, body):
    return tag(f, 2) + varint(len(body)) + body


def ldn(f, n, body):
    return tag(f, 2) + varint(n) + body


def fx32(f, n):
    return tag(f, 5) + struct.pack('<I', n & 0xffffffff)


def fx64(f, n):
    return tag(f, 1) + struct.pack('<Q', n & ((1 << 64) - 1))


def grp(f, body, end=None):
    return tag(f, 3) + body + tag(f if end is None else end, 4)


def packed(f, ns):
    return ld(f, b''.join(varint(n) for n in ns))


def delim(b):
    return varint(len(b)) + b


def units(s):
    if isinstance(s, list):
        return s
    raw = s.encode('utf-16-le', 'surrogatepass')
    return [raw[i] | (raw[i + 1] << 8) for i in range(0, len(raw), 2)]


def esc(us):
    r = []
    for c in us:
        if c <= 0x20 or c >= 0x7f or chr(c) in '\\:[]{}#':
            r.append('\\u%04x' % c)
        else:
            r.append(chr(c))
    return ''.join(r)


def S(s):
    return 's:' + esc(units(s))


def I(n, tag='I'):
    return '%s:I%d' % (tag, n)


def arr(*elems):
    return '[ ' + ' '.join(elems) + ' ]' if elems else '[ ]'


def utf8(b):
    return S([0] + list(b))


def enc8to7(b, marker=False):
    bits = []
    for x in b:
        bits.extend((x >> i) & 1 for i in range(8))
    while len(bits) % 7:
        bits.append(0)
    vals = [sum(bits[i + j] << j for j in range(7)) for i in range(0, len(bits), 7)]
    us = [(v + 1) & 0x7f for v in vals]
    return S(([0xffff] if marker else []) + us)


def case(name, k=I(1), d1=None, d2=None, xs=None, extra=()):
    out.append('case ' + name)
    if k is not None:
        out.append('k ' + k)
    if d1 is not None:
        out.append('d1 ' + d1)
    if d2 is not None:
        out.append('d2 ' + d2)
    if xs is not None:
        out.append('xs ' + xs)
    for n, v in extra:
        out.append(n + ' ' + v)
    out.append('end')


def rec(rng=None, pre=None, op=None, sub=None, sub_packed=True, rep=None, rep_packed=True, string=None):
    b = b''
    if rng is not None:
        b += vi(1, rng)
    if pre is not None:
        b += vi(2, pre)
    if op is not None:
        b += vi(3, op)
    if sub is not None:
        b += packed(4, sub) if sub_packed else b''.join(vi(4, x) for x in sub)
    if rep is not None:
        b += packed(5, rep) if rep_packed else b''.join(vi(5, x) for x in rep)
    if string is not None:
        b += ld(6, string)
    return b


def st(*records):
    return delim(b''.join(ld(1, r) for r in records))


def typ(cn=None, nullable=None, args=(), tpn=None, tan=None, extra=b''):
    b = b''.join(ld(2, a) for a in args)
    if nullable is not None:
        b += vi(3, nullable)
    if cn is not None:
        b += vi(6, cn)
    if tpn is not None:
        b += vi(9, tpn)
    if tan is not None:
        b += vi(12, tan)
    return b + extra


def targ(proj=None, t=None, tid=None):
    b = b''
    if proj is not None:
        b += vi(1, proj)
    if t is not None:
        b += ld(2, t)
    if tid is not None:
        b += vi(3, tid)
    return b


NAMES = ['kotlin/Foo', 'bar', 'Lkotlin/Baz$Inner;', 'pkg/Outer$Nested', '(I)V', 'x']
D2 = arr(*[S(n) for n in NAMES])
ST6 = st(*[rec() for _ in NAMES])
CLS = vi(3, 0)
FUN = vi(2, 1)


def k1(name, body, table=ST6, d2=D2):
    case(name, I(1), arr(utf8(table + body)), d2)


def k3(name, body, table=ST6, d2=D2):
    case(name, I(3), arr(utf8(table + body)), d2)


# ---- message kinds and annotation values
k1('class_min', CLS)
case('package_empty', I(2), arr(utf8(ST6)), D2)
k3('function_min', FUN)
case('facade', I(4), arr(S('pkg/Utils__A'), S('pkg/Utils__B')))
case('facade_with_d2', I(4), arr(S('pkg/Utils__A')), arr(S('x')))
case('facade_empty', I(4), arr())
case('part_xs', I(5), arr(utf8(ST6 + ld(3, FUN))), D2, S('pkg/Utils'))
case('part_no_xs', I(5), arr(utf8(ST6)), D2)
case('part_xs_int', I(5), arr(utf8(ST6)), D2, I(7))
case('part_xs_class', I(5), arr(utf8(ST6)), D2, 'c:pkg/Utils')
for kv in (0, 6, -1, 99):
    case('k_%d' % kv, I(kv), arr(utf8(ST6)), D2)
case('no_k', None, arr(utf8(ST6)), D2)
case('k_string', S('1'), arr(utf8(ST6 + CLS)), D2)
case('k_long', 'J:J1', arr(utf8(ST6 + CLS)), D2)
case('k_byte', I(1, 'B'), arr(utf8(ST6 + CLS)), D2)
case('k_char', I(1, 'C'), arr(utf8(ST6 + CLS)), D2)
case('k_short', I(1, 'S'), arr(utf8(ST6 + CLS)), D2)
case('k_bool', I(1, 'Z'), arr(utf8(ST6 + CLS)), D2)
case('k_class', 'c:java/lang/Integer', arr(utf8(ST6 + CLS)), D2)
case('k_array', arr(I(1)), arr(utf8(ST6 + CLS)), D2)
case('k_enum', 'e:Lkotlin/Kind;:CLASS', arr(utf8(ST6 + CLS)), D2)
case('k_float', 'F:F1065353216', arr(utf8(ST6 + CLS)), D2)
case('no_d1', I(1), None, D2)
case('no_d2', I(2), arr(utf8(b'')), None)
case('no_d2_class', I(1), arr(utf8(CLS)), None)
case('d1_not_array', I(1), utf8(ST6 + CLS), D2)
case('d1_int_element', I(1), arr(I(5)), D2)
case('d1_class_element', I(2), arr('c:\\u0000\\u0000'), arr(S('x')))
case('d1_string_of_int', I(1), arr('s:#I5'), D2)
case('d2_int_element', I(1), arr(utf8(ST6 + CLS)), arr(S('a'), I(5)))
case('d2_not_array', I(1), arr(utf8(ST6 + CLS)), S('a'))
case('keys_reordered', None, None, None, None,
     [('d2', D2), ('xs', S('ignored')), ('d1', arr(utf8(ST6 + CLS))), ('k', I(1)), ('k', I(2))])

# ---- BitEncoding
payload = ST6 + CLS + vi(1, 22)
case('bits_8to7', I(1), arr(enc8to7(payload)), D2)
case('bits_8to7_marker', I(1), arr(enc8to7(payload, True)), D2)
half = enc8to7(payload)[2:]
cut = half.index('\\', len(half) // 2)
case('bits_8to7_split', I(1), arr('s:' + half[:cut], 's:' + half[cut:]), D2)
case('bits_empty_array', I(2), arr(), None)
case('bits_empty_first', I(2), arr(S(''), utf8(ST6)), D2)
case('bits_utf8_split', I(1), arr(utf8(ST6[:3]), S([b for b in ST6[3:] + CLS])), D2)
case('bits_high_chars', I(1), arr(S([0] + [0x4100 | b for b in ST6 + CLS])), D2)
case('bits_surrogates', I(2), arr(S([0xd800, 0xdc00, 0x41])), None)
case('bits_lone_surrogate_8to7', I(2), arr(S([0xdfff, 0xd801, 0x7f, 0x80])), None)
case('bits_one_char', I(2), arr(S([5])), None)

# ---- scalars, packing, unknown fields
k1('scalar_last_wins', vi(1, 6) + CLS + vi(1, 22) + vi(4, 1) + vi(4, 2))
k1('field_order_reversed', vi(4, 1) + vi(1, 0x4006) + CLS)
case('records_packed_unpacked', I(2), arr(utf8(st(
    rec(sub=[0, 3]), rec(sub=[0, 3], sub_packed=False), rec(rep=[1, 2], rep_packed=False),
    rec(rep=[36, 46]) + rec(rep=[47]), rec(sub=[]), rec(sub=[1]) + rec(sub=[2], sub_packed=False),
    rec(sub=[0, 2], rep=[0, 2])) + b'')), D2)
k1('unknown_every_wire_type', CLS + vi(50, 7) + fx64(51, -1) + ld(52, b'xyz') +
   grp(53, vi(1, 1) + ld(2, b'ab') + grp(3, fx32(4, 9))) + fx32(54, 5) + vi(1, 0x26) +
   vi(2000000, 1) + tag(536870911, 0) + varint(3))
k1('unknown_in_nested', CLS + ld(9, FUN + vi(77, 1) + grp(78, b'') + ld(3, typ(cn=0, extra=fx32(99, 1) + vi(15, 3)))))
k1('known_number_wrong_wire', CLS + ld(1, b'\x05') + fx32(4, 1) + ld(6, typ(cn=1)) + vi(6, 3))
k1('required_wrong_wire', fx32(3, 0))
k1('wire_type_6', CLS + tag(60, 6))
k1('wire_type_7', CLS + tag(60, 7))
k1('end_group_top', CLS + tag(9, 4))
k1('end_group_nested', CLS + ld(9, FUN + tag(5, 4)))
k1('group_mismatch', CLS + grp(53, vi(1, 1), end=54))
k1('group_unterminated', CLS + tag(53, 3) + vi(1, 1))
k1('tag_zero', CLS + b'\x00')
k1('tag_field_zero', CLS + tag(0, 2) + b'\x00')
k1('tag_negative', CLS + varint(0xfffffff8) + b'\x01')

# ---- truncation and varints
k1('truncated_varint', CLS + b'\x08\x80')
k1('truncated_length', CLS + ldn(52, 10, b'abc'))
k1('truncated_fixed32', CLS + tag(54, 5) + b'\x01\x02')
k1('truncated_fixed64', CLS + tag(51, 1) + b'\x01\x02\x03')
k1('truncated_tag', CLS + b'\x80')
k1('submessage_beyond_end_at_boundary', CLS + ldn(9, 40, FUN + vi(9, 6)))
k1('submessage_beyond_end_midfield', CLS + ldn(9, 40, FUN + b'\x48'))
k1('submessage_beyond_outer', CLS + ld(9, FUN + ldn(3, 30, typ(cn=0))))
k1('negative_length', CLS + tag(9, 2) + varint(-1) + FUN)
k1('negative_length_unknown', CLS + tag(52, 2) + varint(-1))
for n in (0x7fffffff, 0x7ffffffd, 0x7ffffff0, 0x80000000, 0xffffffff):
    k1('huge_length_%x' % n, CLS + tag(9, 2) + varint(n) + FUN)
    k1('huge_length_unknown_%x' % n, CLS + tag(52, 2) + varint(n) + b'abc')
k1('overlong_varint_10', CLS + b'\x08' + b'\x86' + b'\x80' * 8 + b'\x00')
k1('overlong_varint_11', CLS + b'\x08' + b'\x86' + b'\x80' * 9 + b'\x00')
k1('overlong_varint_10_continued', CLS + b'\x08' + b'\x86' + b'\x80' * 9)
k1('int32_negative', CLS + vi(1, -1))
k1('int32_high_bits', CLS + b'\x08' + varint(0x1234567896))
k1('int32_five_bytes', CLS + b'\x08\xff\xff\xff\xff\x7f')
k1('varint_at_end_short', CLS + b'\x08\x86\x80\x80\x00')
k3('bool_overlong_fastpath', FUN + ld(3, typ(cn=0, extra=b'\x18' + b'\x80' * 9 + b'\x00' + vi(1, 1))))
k3('bool_overlong_one', FUN + ld(3, typ(cn=0, extra=b'\x18' + b'\x81' + b'\x80' * 8 + b'\x00')))
k3('bool_high_bit_only', FUN + ld(3, typ(cn=0, extra=b'\x18' + b'\x80' * 9 + b'\x01')))


def at_offset(prefix, target):
    for pad in range(4000, 4200):
        body = prefix + ld(60, b'p' * pad)
        if len(body) == target:
            return body
    raise ValueError(target)


for target, name in ((4090, 'slow'), (4086, 'fast'), (4087, 'edge'), (4095, 'last')):
    for pad in range(3900, 4200):
        inner = ld(60, b'p' * pad) + b'\x18' + b'\x80' * 9 + b'\x00' + vi(6, 0)
        msg = FUN + tag(3, 2) + varint(len(inner)) + inner
        if len(FUN) + len(tag(3, 2)) + len(varint(len(inner))) + len(ld(60, b'p' * pad)) + 1 == target:
            break
    else:
        raise ValueError(target)
    k3('bool_overlong_buffer_%s' % name, msg)

big = b'q' * 9000
k1('big_unknown_field', CLS + ld(52, big) + vi(1, 0x16))
k1('big_unknown_group', CLS + grp(53, ld(1, big)) + vi(4, 2))
case('big_record_string', I(2), arr(utf8(st(rec(string=b's' * 5000), rec()))), arr(S('a'), S('b')))
k1('message_across_buffer', CLS + ld(52, b'r' * 4080) + ld(9, FUN + ld(3, typ(cn=1, nullable=1))) + vi(1, 0x16))

# ---- enums
k3('enum_unknown', FUN + ld(3, typ(args=[targ(proj=9, t=typ(cn=0))])))
k3('enum_known_then_unknown', FUN + ld(3, typ(args=[targ(proj=1) + vi(1, 9), targ(proj=3), targ(proj=0)])))
k3('enum_negative', FUN + ld(3, typ(args=[targ(proj=-1)])) + ld(4, vi(1, 0) + vi(2, 1) + vi(4, 7) + vi(4, 0)))

# ---- merges
t1 = typ(cn=1, nullable=1, args=[targ(proj=0, t=typ(cn=0))], extra=ld(5, typ(cn=2)))
t2 = typ(cn=3, args=[targ(proj=1)], tpn=4, extra=ld(5, typ(nullable=1)) + vi(101, 1))
k3('merge_return_type', FUN + ld(3, t1) + ld(3, t2))
k3('merge_three_times', FUN + ld(3, typ(cn=1)) + ld(3, typ(tan=2)) + ld(3, typ(cn=5, nullable=0)))
k1('merge_type_table', CLS + ld(30, ld(1, typ(cn=0)) + vi(2, 1)) + ld(30, ld(1, typ(cn=1)) + vi(2, 3)))
prop_sig_1 = ld(3, vi(1, 1)) + ld(1, vi(1, 0))
prop_sig_2 = ld(4, vi(1, 2)) + ld(3, vi(2, 4)) + ld(2, vi(1, 5))
k1('merge_extension', CLS + ld(10, vi(2, 1) + ld(100, prop_sig_1) + ld(100, prop_sig_2) + vi(101, 3) + vi(101, 4)))
k3('merge_method_signature', FUN + ld(100, vi(1, 1)) + ld(100, vi(2, 4)) + ld(100, b''))
k1('merge_contract', CLS + ld(9, FUN + ld(32, ld(1, vi(1, 1))) + ld(32, ld(1, vi(1, 2) + vi(4, 2)))))
k1('merge_setter_value_parameter', CLS + ld(10, vi(2, 0) + ld(6, vi(2, 1)) + ld(6, vi(1, 2) + ld(3, typ(cn=0)))))

k3('merge_contract_twice', FUN + ld(32, b'') + ld(32, b''))
k1('merge_getter_contract', CLS + ld(10, vi(2, 0) + ld(40, b'') + ld(40, ld(1, b''))))
k1('merge_setter_contract', CLS + ld(10, vi(2, 0) + ld(41, ld(1, b'')) + ld(41, b'')))
k1('merge_version_requirement_table', CLS + ld(32, b'') + ld(32, ld(1, b'')))
case('merge_package_version_requirement_table', I(2), arr(utf8(ST6 + ld(32, b'') + ld(32, b''))), D2)
k3('merge_conclusion', FUN + ld(32, ld(1, ld(3, vi(1, 1)) + ld(3, vi(2, 1)))))
k3('merge_is_instance_type', FUN + ld(32, ld(1, ld(3, ld(4, typ(cn=1)) + ld(4, typ(nullable=1))))))
k1('merge_jvm_field_signature', CLS + ld(10, vi(2, 0) + ld(100, ld(1, vi(1, 1)) + ld(1, vi(2, 2)))))
k1('merge_property_signature_fields', CLS + ld(10, vi(2, 0) + ld(100, ld(1, vi(1, 1))) + ld(100, ld(1, vi(2, 2)))))
k1('merge_property_signature_one_field', CLS + ld(10, vi(2, 0) + ld(100, ld(3, vi(1, 1))) + ld(100, ld(1, vi(2, 2)))))
k1('merge_property_signature_first_field', CLS + ld(10, vi(2, 0) + ld(100, ld(1, vi(1, 1))) + ld(100, ld(3, vi(2, 2)))))
k1('merge_default_value', CLS + ld(8, ld(2, vi(2, 0) + ld(8, vi(1, 3) + ld(8, vi(1, 1))) + ld(8, ld(8, vi(1, 2) + ld(2, vi(1, 0) + ld(2, b'')))))))
k1('merge_value_annotation_uninit', CLS + ld(8, ld(2, vi(2, 0) + ld(8, ld(8, vi(1, 1))) + ld(8, ld(8, ld(2, vi(1, 0)))))))
k1('merge_property_signature_truncated', CLS + ld(10, vi(2, 0) + ld(100, ld(1, vi(1, 1))) + ld(100, ld(1, vi(2, 2)) + b'\x10\x80')))
k1('merge_property_signature_truncated_nofield', CLS + ld(10, vi(2, 0) + ld(100, ld(3, vi(1, 1))) + ld(100, ld(1, vi(2, 2)) + b'\x10\x80')))
k1('merge_property_signature_end_group', CLS + ld(10, vi(2, 0) + ld(100, ld(1, vi(1, 1))) + ld(100, ld(1, vi(2, 2)) + tag(9, 4))))
k1('merge_type_alias_underlying', CLS + ld(11, vi(2, 0) + ld(4, typ(cn=1)) + ld(4, typ(nullable=1))))

# ---- extensions
k3('ext_wrong_wire', FUN + vi(100, 1) + ld(101, b'\x01') + vi(101, 2) + fx32(101, 3))
k3('ext_lambda_origin', FUN + vi(101, 3))
k1('ext_class_ints', CLS + vi(101, 1) + vi(103, 2) + vi(104, 1) + vi(104, 3))
k1('ext_class_local_variable', CLS + ld(102, vi(2, 1) + vi(11, 6)) + ld(102, vi(2, 5)))
k1('ext_class_local_variable_uninit', CLS + ld(102, vi(11, 6)) + b'\x08\x80')
k1('class_property_uninit', CLS + ld(10, vi(11, 6)) + b'\x08\x80')
case('ext_package_local_variable', I(2), arr(utf8(ST6 + ld(102, vi(2, 1)) + vi(101, 0))), D2)
k1('ext_is_raw', CLS + ld(6, typ(cn=0, extra=vi(101, 1))) + ld(6, typ(cn=0, extra=vi(101, 0))))
k1('ext_constructor_signature', CLS + ld(8, vi(1, 0x16) + ld(100, vi(2, 4))) + ld(8, ld(2, vi(2, 1) + vi(1, 2))))

# ---- required fields
k1('missing_fq_name', vi(1, 6))
k3('missing_function_name', vi(9, 6))
k3('missing_type_parameter_id', FUN + ld(4, vi(2, 1)))
k3('type_parameter_complete', FUN + ld(4, vi(1, 0) + vi(2, 1) + vi(3, 1) + vi(4, 1) + ld(5, typ(cn=0))))
k3('missing_annotation_id', FUN + ld(3, typ(cn=0, extra=ld(100, b''))))
k3('annotation_complete', FUN + ld(3, typ(cn=0, extra=ld(100, vi(1, 2) + ld(2, vi(1, 1) + ld(2, vi(1, 3) + vi(2, 5)))))))
k3('missing_annotation_argument_value', FUN + ld(12, vi(1, 2) + ld(2, vi(1, 1))))
k3('nested_annotation_value_uninit', FUN + ld(12, vi(1, 2) + ld(2, vi(1, 1) + ld(2, ld(8, b'')))))
k1('missing_compiler_plugin_data', CLS + ld(33, vi(1, 1)))
k1('compiler_plugin_data_complete', CLS + ld(33, vi(1, 1) + ld(2, b'')))
k1('missing_value_parameter_name', CLS + ld(8, ld(2, vi(1, 2))))
k1('missing_type_alias_name', CLS + ld(11, ld(4, typ(cn=0))))
k1('enum_entry_ok', CLS + ld(13, vi(1, 1) + ld(2, vi(1, 0))) + ld(13, b''))
k1('version_requirement_table', CLS + ld(32, ld(1, vi(1, 1) + vi(3, 9) + vi(6, 2))) + packed(31, [1, 2]) + vi(31, 3))
k1('unconsumed_fields', CLS + packed(2, [1]) + vi(7, 2) + packed(16, [3, 4]) + vi(17, 1) + ld(18, typ(cn=0)) +
   vi(19, 2) + ld(20, typ(cn=1)) + packed(21, []) + ld(25, vi(1, 0)) + ld(5, vi(1, 0) + vi(2, 1)))
k3('unconsumed_function_fields', FUN + vi(1, 6) + vi(7, 1) + vi(8, 2) + packed(11, [5]) + ld(13, vi(2, 1)) +
   ld(34, vi(1, 0)) + ld(10, typ(cn=0)))

# ---- recursion
def nested(levels):
    t = typ(cn=0)
    n = 1
    while n + 2 <= levels:
        t = typ(args=[targ(t=t)])
        n += 2
    return t, n


def expressions(depth):
    e = vi(2, 1)
    for _ in range(depth - 4):
        e = ld(6, e)
    return FUN + ld(32, ld(1, ld(2, e)))


for depth in (64, 65, 66):
    k3('recursion_%d' % depth, expressions(depth))
t, n = nested(63)
k3('recursion_types_%d' % (n + 1), FUN + ld(3, t))


def groups(n):
    g = vi(1, 1)
    for _ in range(n):
        g = grp(9, g)
    return g


for n in (62, 63, 64, 65):
    k1('group_depth_%d' % n, CLS + tag(53, 3) + groups(n - 1) + tag(53, 4))
k3('group_inside_message_depth', FUN + ld(3, typ(cn=0, extra=tag(50, 3) + groups(61) + tag(50, 4))))

# ---- the delimited string table
case('delimited_empty', I(1), arr(utf8(b'')), D2)
case('delimited_zero', I(1), arr(utf8(b'\x00' + CLS)), D2)
case('delimited_negative', I(1), arr(utf8(varint(-1) + CLS)), D2)
case('delimited_too_long', I(1), arr(utf8(varint(100) + ld(1, rec()) + ld(1, rec()))), D2)
case('delimited_too_long_midfield', I(1), arr(utf8(varint(100) + ld(1, rec()) + b'\x0a')), D2)
case('delimited_short', I(1), arr(utf8(varint(2) + ld(1, rec(rng=6)) + CLS)), D2)
case('delimited_truncated_varint', I(1), arr(utf8(b'\x80')), D2)
case('delimited_malformed_varint', I(1), arr(utf8(b'\xff' * 11)), D2)
case('delimited_ten_byte_varint', I(1), arr(utf8(b'\x82' + b'\x80' * 8 + b'\x00' + ld(1, b'') + CLS)), D2)
case('delimited_end_group', I(1), arr(utf8(delim(ld(1, b'') + tag(3, 4)) + CLS)), D2)
case('delimited_local_name', I(1), arr(utf8(delim(ld(1, rec()) + packed(5, [0]) + vi(5, 1)) + CLS)), D2)

# ---- names
case('names_predefined', I(1), arr(utf8(st(
    rec(pre=0), rec(pre=17), rec(pre=43), rec(pre=44), rec(pre=-1), rec(pre=38, op=1), rec(pre=13, string=b'')) +
    vi(3, 0))), arr(*[S(x) for x in ('a', 'b', 'c', 'd', 'e', 'f', 'g')]))
case('names_operations', I(1), arr(utf8(st(
    rec(op=1), rec(op=2), rec(op=2), rec(op=2), rec(op=2), rec(op=3), rec(op=1, rep=[36, 95])) + vi(3, 0))),
    arr(S('a/B$C$$D'), S('La/B$C;'), S('L'), S(''), S('LX$;'), S('k$l'), S('m$n')))
case('names_substring_bug', I(1), arr(utf8(st(
    rec(sub=[0, 2]), rec(sub=[0, 2], rep=[1]), rec(sub=[0, 2], rep=[1, 3]), rec(sub=[0, 2], rep=[2, 1]),
    rec(sub=[0, 2], rep=[0, 99]), rec(sub=[5], rep=[1, 2]), rec(sub=[0, 1, 2], rep=[0, 0])) + vi(3, 0))),
    arr(*[S('abcdefgh') for _ in range(7)]))
case('names_replace_char', I(1), arr(utf8(st(
    rec(rep=[97, 65]), rec(rep=[97, 65, 98, 66]), rec(rep=[98, 97]), rec(rep=[0, 70000]), rec(rep=[-1, 5]),
    rec(rep=[65535, 65535]), rec(rep=[0xd834, 0x41]), rec(rep=[0x41])) + vi(3, 0))),
    arr(S('banana'), S('banana'), S('banana'), S('banana'), S('banana'), S([0xffff, 0x61]),
        S([0xd834, 0xdd1e, 0x62]), S('A')))
case('names_unicode_substring', I(1), arr(utf8(st(
    rec(sub=[0, 2], rep=[1, 3]), rec(sub=[0, 2], rep=[0, 2]), rec(sub=[0, 2], rep=[2, 4]), rec(op=2)) + vi(3, 0))),
    arr(S([0x61, 0xd834, 0xdd1e, 0xe9]), S([0xd834, 0xdd1e, 0x61]), S([0x61, 0xd834, 0xdd1e, 0xe9]),
        S([0xd834, 0xdd1e, 0x61, 0x24, 0xe9])))
case('names_ranges', I(1), arr(utf8(st(rec(rng=3), rec(rng=0), rec(rng=-5), rec(rng=2, op=1)) + vi(3, 4))),
     arr(*[S('n%d$x' % i) for i in range(5)]))
case('names_d2_short', I(1), arr(utf8(st(rec(rng=4), rec(pre=2)) + vi(3, 2))), arr(S('only')))
case('names_d2_empty', I(1), arr(utf8(st(rec()) + vi(3, 0))), arr())
case('names_record_string', I(1), arr(utf8(st(rec(string='héllo'.encode()), rec(string=b'\xff\xfe'),
                                            rec(rng=2)) + vi(3, 2))), arr(S('a'), S('b'), S('c'), S('d')))
case('names_no_records', I(1), arr(utf8(st() + vi(3, 0))), arr(S('a')))
case('names_class_full', I(1), arr(utf8(st(rec(op=1), rec(), rec(op=2), rec(pre=14)) +
                                        vi(3, 0) + vi(4, 1) + ld(6, typ(cn=3, tpn=1, tan=2)) +
                                        ld(9, vi(2, 1) + ld(100, vi(1, 1) + vi(2, 2)) + ld(3, typ(cn=9))))),
     arr(S('a/B$C'), S('name'), S('Lx/Y;'), S('zz')))

print('\n'.join(out))
