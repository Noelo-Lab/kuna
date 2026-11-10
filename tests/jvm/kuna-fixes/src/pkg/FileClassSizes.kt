@file:JvmMultifileClass
@file:JvmName("TestFileClassConstructor")

package pkg

fun twice(size: Int): Int = area(size, 2)

fun describe(size: Int): String = if (size > 10) "large" else "small"
