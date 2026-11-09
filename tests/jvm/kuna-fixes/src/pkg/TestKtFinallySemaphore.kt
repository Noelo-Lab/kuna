package pkg

class TestKtFinallySemaphore {
    interface Src {
        fun more(): Boolean
        fun next(): Any?
        fun cancel(cause: Throwable?)
    }

    var first: Any? = null
    var count: Int = 0

    inline fun Src.consume(block: Src.() -> Unit) {
        var cause: Throwable? = null
        try {
            block()
        } catch (e: Throwable) {
            cause = e
            throw e
        } finally {
            cancel(cause)
        }
    }

    inline fun removeIf(p: (Any) -> Boolean): Any? = synchronized(this) {
        val f = first ?: return null
        if (p(f)) f else null
    }

    inline fun firstNull(src: Src): Int {
        src.consume {
            do {
                if (next() == null) return count
                count++
            } while (more())
        }
        return -1
    }
}
