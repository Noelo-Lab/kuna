@file:JvmName("TestTopLevelModality")

package pkg

val GREETING: String = "hello"

var counter: Int = 3
    get() = field + 1
    private set

var limit: Long = 8L
    set(value) {
        field = if (value < 0L) 0L else value
    }

fun bumpCounter(): Int = ++counter
