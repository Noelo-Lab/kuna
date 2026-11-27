@file:JvmName("ExtLib")
package pkg.lib

fun String.shout(): String = this + "!"

val String.firstChar: Char get() = this[0]

var String.tag: Int
    get() = length
    set(value) { if (value < 0) throw IllegalStateException() }

infix fun Int.pow2(y: Int): Int = this * y

fun CharSequence.describe(): String = "chars"

fun String.describe(): String = "string"

fun Any.label(): String = "any"
