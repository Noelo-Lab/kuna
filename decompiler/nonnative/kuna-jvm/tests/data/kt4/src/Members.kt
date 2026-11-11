@file:OptIn(kotlin.contracts.ExperimentalContracts::class)
package kt4

import kotlin.contracts.*
import kotlin.properties.Delegates

annotation class Marker(val value: String = "dflt", val n: Int = 3, val arr: IntArray = [1, 2], val e: Color = Color.RED)

@Target(AnnotationTarget.PROPERTY)
annotation class PropAnn(val v: Int = 0)

class Defaults(val a: Int = 1, var b: String = "two", c: Long = 3L) {
    constructor(x: Double, y: Int = 7) : this(y, x.toString())

    fun simple(x: Int, y: String = "y", z: Any? = null): String = "$x$y$z"
    fun withThis(x: Int = hashCode(), y: Int = x + 1): Int = x + y
    fun <T : Number> bounded(t: T, u: T? = null): T = u ?: t
    fun <T> unbounded(t: T, u: T? = null): T? = u ?: t
    fun <T> multi(t: T, d: Int = 5): Int where T : CharSequence, T : Comparable<T> = d + t.length
    fun String.ext(n: Int = length): String = this.repeat(n)
    fun many(
        p0: Int = 0, p1: Int = 1, p2: Int = 2, p3: Int = 3, p4: Int = 4, p5: Int = 5, p6: Int = 6, p7: Int = 7,
        p8: Int = 8, p9: Int = 9, p10: Int = 10, p11: Int = 11, p12: Int = 12, p13: Int = 13, p14: Int = 14,
        p15: Int = 15, p16: Int = 16, p17: Int = 17, p18: Int = 18, p19: Int = 19, p20: Int = 20, p21: Int = 21,
        p22: Int = 22, p23: Int = 23, p24: Int = 24, p25: Int = 25, p26: Int = 26, p27: Int = 27, p28: Int = 28,
        p29: Int = 29, p30: Int = 30, p31: Int = 31, p32: Int = 32, p33: Long = 33L, p34: String = "34"
    ): Int = p0 + p31 + p32 + p34.length
    tailrec fun count(n: Int, acc: Int = 0): Int = if (n == 0) acc else count(n - 1, acc + 1)
    infix fun plusOne(o: Int): Int = o + 1
    operator fun plus(o: Defaults): Defaults = this
    inline fun inl(block: () -> Unit = {}) = block()
    suspend fun susp(x: Int = 4): Int = x
    private fun priv(x: Int = 1) = x
    internal fun intern(vararg xs: Int = intArrayOf(1)) = xs.size + priv()
    fun nullableRet(s: String? = null): String? = s
    fun arr(a: Array<String> = arrayOf("a"), d: Double = 1.5): Int = a.size + d.toInt()
}

open class Base {
    open fun over(x: Int = 1): Int = x
    open val prop: Int = 1
    override fun toString(): String = "Base"
}

class Derived : Base() {
    override fun over(x: Int): Int = x * 2
    override val prop: Int get() = 2
    fun fresh(): Int = 3
}

interface Iface {
    fun defaultMethod(x: Int = 9): Int = x
    val ip: String
    fun abstractOne(s: String): Int
}

class Impl : Iface {
    override val ip: String = "ip"
    override fun abstractOne(s: String): Int = s.length
}

object Obj {
    const val C = 42
    val lazyVal by lazy { "lz" }
    var counter = 0
    @JvmStatic fun stat(x: Int = 2) = x
    fun plain(y: String = "p") = y
}

class WithCompanion(val id: Int = 0) {
    companion object {
        const val CC = "cc"
        val cv = 1
        @JvmStatic var jsv = 2
        @JvmField val jf = 3
        @JvmStatic fun cf(x: Int = 1) = x
        fun nonStatic(y: String = "z") = y
        private val hidden = 9
    }
    lateinit var late: String
    var privSet: Int = 0
        private set
    var custom: Int
        get() = 5
        set(value) { println(value) }
    @PropAnn(4) val annotated: Int = 1
    @Deprecated("x") val deprecatedProp = 2
    @get:JvmName("fetchRenamed") val renamed = 3
    var observed: String by Delegates.observable("init") { _, _, _ -> }
    val noField: Int get() = id + 1
    protected open val prot = 7
}

class Named {
    companion object Factory {
        fun make(n: Int = 1): Named = Named()
        var made = 0
    }
}

enum class Color(val rgb: Int = 0, val label: String = "none") {
    RED(1), GREEN(2, "g"), BLUE;
    fun describe(prefix: String = "c:") = prefix + label
}

data class Point(val x: Int = 0, val y: Int = 0)

sealed class Shape {
    class Circle(val r: Double = 1.0) : Shape()
    object Empty : Shape()
}

abstract class Abs(val z: Int = 1) {
    abstract fun f(): Int
    open fun g(q: Int = 2): Int = q
}

private class PrivCls internal constructor(val v: Int = 1)

class Secondary {
    val v: Int
    constructor(a: Int) { v = a }
    constructor(s: String, extra: Int = 2) : this(s.length + extra)
    @Deprecated("old") constructor() : this(0)
}

context(String)
fun ctxFun(x: Int = 1) = this@String.length + x

fun contracted(x: Any?): Boolean {
    contract { returns(true) implies (x != null) }
    return x != null
}

inline fun runIt(block: () -> Unit) {
    contract { callsInPlace(block, InvocationKind.EXACTLY_ONCE) }
    block()
}

fun topLevel(a: Int = 1, b: String = "b"): String = a.toString() + b
fun <T : Comparable<T>> maxOf3(a: T, b: T, c: T = a): T = maxOf(a, maxOf(b, c))
fun Int.extTop(by: Int = 2): Int = this * by
fun ((Int) -> Int).applyTwice(v: Int = 0): Int = this(this(v))
val topProp: Int = 5
var topVar: String = "v"
const val TOP_CONST = 3.5
val Int.extProp: Int get() = this + 1
val lam = { x: Int -> x + 1 }
external fun nativeFun(x: Int): Int
