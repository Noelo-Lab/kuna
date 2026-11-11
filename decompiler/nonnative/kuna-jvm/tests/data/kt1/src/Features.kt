@file:OptIn(kotlin.contracts.ExperimentalContracts::class)
package pkg

import kotlin.contracts.*
import kotlin.reflect.KProperty

typealias StrMap<V> = Map<String, V>
typealias Callback = (Int, String?) -> Unit

data class Point(val x: Int, val y: Int = 0) {
    companion object Factory {
        const val ORIGIN_NAME = "origin"
        @JvmStatic fun origin(): Point = Point(0, 0)
        var counter: Long = 0L
    }
    operator fun plus(o: Point) = Point(x + o.x, y + o.y)
    infix fun dot(o: Point): Int = x * o.x + y * o.y
}

sealed class Shape {
    abstract val area: Double
    object Empty : Shape() { override val area: Double get() = 0.0 }
    class Circle(val r: Double) : Shape() { override val area get() = 3.14 * r * r }
    data class Rect(val w: Double, val h: Double) : Shape() { override val area get() = w * h }
}

enum class Color(val rgb: Int) { RED(0xff0000), GREEN(0x00ff00), BLUE(0x0000ff);
    fun hex() = rgb.toString(16)
}

annotation class Marker(val name: String, val level: Int = 1)

@JvmInline value class Meters(val value: Double)

fun interface Visitor<in T> { fun visit(t: T): Boolean }

interface Repo<K : Comparable<K>, out V : Any> {
    fun get(key: K): V?
    fun keys(): List<K> = emptyList()
    val size: Int
}

open class Base<T>(protected open val item: T) {
    open fun describe(): String = item.toString()
    internal fun secret() = 1
    private fun hidden() = 2
    protected open fun prot() = 3
}

class Derived(override val item: String) : Base<String>(item), Comparable<Derived> {
    lateinit var late: String
    val lazyVal: Int by lazy { item.length }
    var observed: String by Delegate()
    override fun describe(): String = "derived $item"
    override fun compareTo(other: Derived): Int = item.compareTo(other.item)
    inner class Inner(val z: Int) { fun outer() = item }
    constructor(n: Int) : this(n.toString())
    private constructor(a: Int, b: Int) : this(a + b)
}

class Delegate {
    operator fun getValue(thisRef: Any?, property: KProperty<*>): String = property.name
    operator fun setValue(thisRef: Any?, property: KProperty<*>, value: String) {}
}

abstract class Node<T : Number> where T : Comparable<T> {
    abstract fun <R : T> map(f: (T) -> R): Node<R>?
    abstract fun children(): Array<out Node<*>>
}

inline fun <reified T : Any> typeName(): String = T::class.java.name

inline fun runTwice(crossinline a: () -> Unit, noinline b: () -> Unit) { a(); b(); a() }

fun sum(vararg xs: Int, scale: Int = 1): Int = xs.sum() * scale

fun String?.isBlankOrNull(): Boolean {
    contract { returns(false) implies (this@isBlankOrNull != null) }
    return this == null || this.isBlank()
}

fun requireLen(s: Any?, block: () -> Unit) {
    contract {
        returns() implies (s is String)
        callsInPlace(block, InvocationKind.EXACTLY_ONCE)
    }
    block()
    if (s !is String) throw IllegalArgumentException()
}

fun <T> T?.orElse(other: T): T {
    contract { returnsNotNull() implies (this@orElse != null) }
    return this ?: other
}

tailrec fun gcd(a: Int, b: Int): Int = if (b == 0) a else gcd(b, a % b)

suspend fun delayed(x: Int): Int = x + 1

external fun nativeThing(x: Int): Int

val String.firstChar: Char get() = this[0]
var topCounter: Int = 0
    private set
const val TOP_CONST = 42

fun useCallback(cb: Callback, m: StrMap<Int>, mm: MutableMap<String, in Int>, f: suspend (Int) -> String, g: Int.(String) -> Unit) {
    cb(m.size, null)
}

fun lambdas(): List<() -> Int> {
    val captured = 5
    val anon = object : Visitor<String> { override fun visit(t: String) = t.isEmpty() }
    return listOf({ captured }, { captured * 2 }, { if (anon.visit("")) 1 else 0 })
}

fun localDelegates(): String {
    val local: String by lazy { "x" }
    fun localFun(q: Int) = q * 2
    return local + localFun(1)
}

fun projections(a: Array<in String>, b: MutableList<out Number>, c: Map<*, List<*>>): Array<Array<Int?>> = arrayOf()
