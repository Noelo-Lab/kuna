package pkg

interface NothingSource {
    suspend fun next(): Int
    suspend fun never(): Nothing
}

fun fail(message: String): Nothing = throw IllegalStateException(message)

private fun unsupported(): Nothing = throw UnsupportedOperationException("unsupported")

class NothingHolder(val value: Int) {
    fun checked(): Int {
        if (value < 0) unsupported()
        return value
    }
}

fun pick(x: Int): Int {
    if (x < 0) fail("negative")
    return x
}

fun pickOrFail(x: Int): Int = if (x > 0) x else fail("not positive")

fun name(x: Int): String = when (x) {
    0 -> "zero"
    1 -> "one"
    else -> fail("other $x")
}

suspend fun nextOrNever(src: NothingSource): Int {
    val v = src.next()
    if (v < 0) src.never()
    return v
}
