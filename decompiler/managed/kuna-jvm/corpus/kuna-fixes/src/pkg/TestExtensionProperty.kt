@file:JvmName("TestExtensionProperty")

package pkg

val String.firstChar: Char
    get() = this[0]

val <T> List<T>.secondOrNull: T?
    get() = if (size > 1) this[1] else null

var StringBuilder.lastChar: Char
    get() = this[length - 1]
    set(value) {
        setCharAt(length - 1, value)
    }
