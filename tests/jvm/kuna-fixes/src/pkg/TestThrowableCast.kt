package pkg

class TestThrowableCast {
    fun fail(n: Int): Nothing = throw IllegalArgumentException("bad: $n")

    fun require(ok: Boolean, message: String) {
        if (!ok) throw IllegalStateException(message)
    }

    fun <T : Throwable> rethrow(t: T): Nothing = throw t

    fun pick(a: RuntimeException, b: Error, first: Boolean): Nothing = throw if (first) a else b

    fun wrap(cause: Throwable): Nothing = throw RuntimeException("wrapped", cause)
}
