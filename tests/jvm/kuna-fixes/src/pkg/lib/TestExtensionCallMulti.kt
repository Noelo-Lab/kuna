@file:JvmMultifileClass
@file:JvmName("ExtMulti")
package pkg.lib

fun String.wrap(n: Int): String = "[" + this + n + "]"

val Int.half: Int get() = this / 2
