"""Crafted Kotlin metadata for KtElementDump's `metaraw` mode: lambdas (k=3)
whose types and contracts reach the corners real metadata does not (contract
effects without a type or argument, out-of-range parameter references, every
expression shape, function and array types without arguments, ...).
Printed in the `meta` header format, d1 in UTF-8 mode."""

STRINGS = [
    "f", "kotlin/Int", "kotlin/Boolean", "x", "b", "block", "kotlin/String",
    "kotlin/Function1", "kotlin/Unit", "kotlin/Array", "T", "MyAlias",
    "kotlin/Function0", "kotlin/collections/List", "pkg/Foo", "kotlin/Any",
    "kotlin/Function2", "kotlin/collections/MutableMap.MutableEntry", "y",
    "java/util/List", "kotlin/UInt", "pkg/Outer.Inner", "kotlin/Nothing",
]
S = {s: i for i, s in enumerate(STRINGS)}


def varint(n):
    n &= (1 << 64) - 1
    out = bytearray()
    while True:
        b = n & 0x7F
        n >>= 7
        if n:
            out.append(b | 0x80)
        else:
            out.append(b)
            return bytes(out)


def tag(num, wt):
    return varint(num << 3 | wt)


def vi(num, v):
    return tag(num, 0) + varint(v)


def ld(num, body):
    return tag(num, 2) + varint(len(body)) + body


# ProtoBuf.Type
def typ(name=None, nullable=False, args=(), param=None, alias=None):
    b = b"".join(ld(2, a) for a in args)
    if nullable:
        b += vi(3, 1)
    if name is not None:
        b += vi(6, S[name])
    if param is not None:
        b += vi(9, S[param])
    if alias is not None:
        b += vi(12, S[alias])
    return b


IN, OUT, INV, STAR = 0, 1, 2, 3


def arg(t=None, projection=None, type_id=None):
    b = b""
    if projection is not None:
        b += vi(1, projection)
    if t is not None:
        b += ld(2, t)
    if type_id is not None:
        b += vi(3, type_id)
    return b


def param(name, t, flags=None, vararg=None, type_id=None):
    b = b""
    if flags is not None:
        b += vi(1, flags)
    b += vi(2, S[name]) + ld(3, t)
    if vararg is not None:
        b += ld(4, vararg)
    if type_id is not None:
        b += vi(5, type_id)
    return b


def tparam(id_, name, reified=False, variance=None, bounds=()):
    b = vi(1, id_) + vi(2, S[name])
    if reified:
        b += vi(3, 1)
    if variance is not None:
        b += vi(4, variance)
    b += b"".join(ld(5, t) for t in bounds)
    return b


# ProtoBuf.Expression; flags: 1 = IS_NEGATED, 2 = IS_NULL_CHECK_PREDICATE
TRUE, FALSE, NULL = 0, 1, 2


def expr(flags=None, ref=None, const=None, instance=None, instance_id=None, and_=(), or_=()):
    b = b""
    if flags is not None:
        b += vi(1, flags)
    if ref is not None:
        b += vi(2, ref)
    if const is not None:
        b += vi(3, const)
    if instance is not None:
        b += ld(4, instance)
    if instance_id is not None:
        b += vi(5, instance_id)
    b += b"".join(ld(6, e) for e in and_)
    b += b"".join(ld(7, e) for e in or_)
    return b


RETURNS_CONSTANT, CALLS, RETURNS_NOT_NULL = 0, 1, 2
AT_MOST_ONCE, EXACTLY_ONCE, AT_LEAST_ONCE = 0, 1, 2


def effect(type_=None, args=(), conclusion=None, kind=None):
    b = b""
    if type_ is not None:
        b += vi(1, type_)
    b += b"".join(ld(2, e) for e in args)
    if conclusion is not None:
        b += ld(3, conclusion)
    if kind is not None:
        b += vi(4, kind)
    return b


def function(params=(), ret=None, contract=None, table=(), tparams=(), receiver=None, flags=None):
    b = vi(2, S["f"])
    if ret is not None:
        b += ld(3, ret)
    b += b"".join(ld(4, t) for t in tparams)
    if receiver is not None:
        b += ld(5, receiver)
    b += b"".join(ld(6, p) for p in params)
    if flags is not None:
        b += vi(9, flags)
    if table:
        b += ld(30, b"".join(ld(1, t) for t in table))
    if contract is not None:
        b += ld(32, b"".join(ld(1, e) for e in contract))
    return b


INT = typ("kotlin/Int")
BOOL_N = typ("kotlin/Boolean", nullable=True)
BOOL = typ("kotlin/Boolean")
STRING = typ("kotlin/String")
UNIT = typ("kotlin/Unit")
FN1 = typ("kotlin/Function1", args=[arg(INT), arg(UNIT)])
X, B, BLOCK = 1, 2, 3
P3 = [param("x", INT), param("b", BOOL_N, flags=2), param("block", FN1, flags=12)]
P2 = [param("x", INT), param("y", BOOL)]

CASES = {
    "contract_shapes": function(P3, UNIT, [
        effect(CALLS, [expr(ref=BLOCK)], kind=EXACTLY_ONCE),
        effect(CALLS, [expr(ref=0)], kind=AT_LEAST_ONCE),
        effect(CALLS, [expr(ref=X)]),
        effect(),
        effect(RETURNS_CONSTANT),
        effect(RETURNS_CONSTANT, [expr(const=TRUE)], conclusion=expr(flags=3, ref=X)),
        effect(RETURNS_CONSTANT, [expr(const=NULL)], conclusion=expr(flags=2, ref=B)),
        effect(RETURNS_CONSTANT, [expr(const=FALSE, ref=B)]),
        effect(RETURNS_CONSTANT, [expr(flags=1, const=TRUE, ref=X)]),
        effect(RETURNS_CONSTANT, [expr(flags=1, const=NULL)]),
        effect(RETURNS_NOT_NULL, conclusion=expr(ref=X, instance=STRING)),
        effect(RETURNS_NOT_NULL, conclusion=expr(flags=1, ref=0, instance=typ("kotlin/collections/List", args=[arg(projection=STAR)]))),
        effect(RETURNS_NOT_NULL, conclusion=expr(ref=X, instance_id=0)),
        effect(RETURNS_NOT_NULL),
        effect(RETURNS_CONSTANT, [expr(ref=B)], conclusion=expr(flags=1, ref=B)),
        effect(RETURNS_CONSTANT, [expr()], conclusion=expr(and_=[expr(ref=B), expr(flags=3, ref=X)])),
        effect(RETURNS_CONSTANT, [expr(ref=B, and_=[expr(flags=2, ref=X)])]),
        effect(RETURNS_CONSTANT, [expr(const=TRUE)], conclusion=expr(or_=[expr(ref=B), expr(flags=2, ref=X)])),
        effect(RETURNS_CONSTANT, [expr(const=TRUE)], conclusion=expr(ref=B, and_=[expr(flags=2, ref=X)], or_=[expr(flags=1, ref=B, and_=[expr(ref=X, instance=STRING), expr(flags=3, ref=BLOCK)]), expr(ref=0, instance=INT)])),
        effect(RETURNS_CONSTANT, [expr(const=FALSE)], conclusion=expr(and_=[expr(or_=[expr(ref=B), expr(flags=1, ref=B)])])),
        effect(RETURNS_CONSTANT, [expr(const=TRUE), expr(const=FALSE)]),
    ], table=[typ("pkg/Foo", nullable=True)]),
    "contract_calls_without_argument": function(P3, UNIT, [effect(CALLS)]),
    "contract_calls_without_reference": function(P3, UNIT, [effect(CALLS, [expr(const=TRUE)])]),
    "contract_reference_past_two": function(P2, UNIT, [effect(RETURNS_CONSTANT, [expr(ref=3)])]),
    "contract_reference_negative": function(P2, UNIT, [effect(RETURNS_CONSTANT, [expr(ref=-1)])]),
    "contract_reference_past_three": function(P3, UNIT, [effect(RETURNS_CONSTANT, [expr(ref=7)])]),
    "contract_reference_without_params": function([], UNIT, [effect(RETURNS_CONSTANT, [expr(ref=1)])]),
    "contract_one_param": function([param("x", INT)], UNIT, [effect(RETURNS_CONSTANT, [expr(ref=1)], conclusion=expr(ref=2))]),
    "contract_type_id_past_table": function(P3, UNIT, [effect(RETURNS_NOT_NULL, conclusion=expr(ref=X, instance_id=4))], table=[INT]),
    "contract_type_id_without_table": function(P3, UNIT, [effect(RETURNS_NOT_NULL, conclusion=expr(ref=X, instance_id=0))]),
    "function_type_without_arguments": function([param("x", typ("kotlin/Function0"))], typ("kotlin/Function0", nullable=True)),
    "function_types": function([
        param("x", typ("kotlin/Function2", nullable=True, args=[arg(INT), arg(BOOL_N, projection=IN), arg(typ("kotlin/Function1", args=[arg(STRING), arg(typ("kotlin/Function0", nullable=True, args=[arg(UNIT)]))]))])),
        param("y", typ("kotlin/Function0", args=[arg(projection=STAR)])),
    ], typ("kotlin/Function1", args=[arg(typ("kotlin/Function1", args=[arg(INT), arg(INT)])), arg(typ(param="T", nullable=True))])),
    "array_without_arguments": function([param("x", typ("kotlin/Array"))], UNIT),
    "array_types": function([
        param("x", typ("kotlin/Array", args=[arg(STRING, projection=IN)])),
        param("y", typ("kotlin/Array", args=[arg(projection=STAR)])),
        param("b", typ("kotlin/Array", nullable=True, args=[arg(typ("kotlin/Array", args=[arg(INT, projection=OUT)]))]), vararg=STRING, flags=4),
        param("block", typ("kotlin/Array", args=[arg(typ(param="T"))]), vararg=typ(param="T"), flags=6),
    ], typ("kotlin/Array", args=[arg(typ("kotlin/Array", args=[arg(BOOL_N)]))])),
    "type_names": function([
        param("x", typ(param="T", nullable=True)),
        param("y", typ("pkg/Foo", alias="MyAlias")),
        param("b", typ(alias="MyAlias", nullable=True, args=[arg(type_id=0)])),
        param("block", typ("kotlin/collections/MutableMap.MutableEntry", args=[arg(STRING, projection=OUT), arg(typ("java/util/List"), projection=IN)])),
    ], typ("kotlin/Nothing", nullable=True), tparams=[
        tparam(0, "T", reified=True, variance=1, bounds=[typ("kotlin/Any", nullable=True), typ("pkg/Outer.Inner", args=[arg(typ(param="T"))])]),
        tparam(1, "y", variance=0),
    ], receiver=typ("kotlin/UInt", nullable=True)),
    "unsigned_and_cached": function([param("x", typ("kotlin/UInt")), param("y", typ("kotlin/UInt", nullable=True))], typ("kotlin/Unit", nullable=True), receiver=typ("kotlin/Nothing")),
}


def records():
    record = vi(1, len(STRINGS))
    return ld(1, record)


def main():
    table = records()
    for name, fn in CASES.items():
        data = varint(len(table)) + table + fn
        print("=== crafted!" + name)
        print("pkg pkg")
        print("k 3")
        print("d1 1")
        print("h 00" + data.hex())
        print("d2 %d" % len(STRINGS))
        for s in STRINGS:
            print("s " + s)
        print("--")


main()
