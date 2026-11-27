package pkg

class TestCopiedEquality {
    open class Base(val flag: Boolean)

    fun eqDefault(a: String, b: Boolean = a == "x"): Boolean = b
    fun neDefault(a: String, b: Boolean = a != "x"): Boolean = b
    fun idDefault(a: Any, b: Boolean = a === "x"): Boolean = b
    fun eqVarDefault(a: Any?, c: Any?, b: Boolean = a == c): Boolean = b
    fun superArg(a: String, c: String): Base = object : Base(a == c) {}
}
