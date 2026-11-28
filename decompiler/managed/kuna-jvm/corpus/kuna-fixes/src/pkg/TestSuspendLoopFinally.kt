package pkg

class TestSuspendLoopFinally {
    interface Source {
        suspend fun next(): Int?
        fun close(cause: Throwable?)
    }

    suspend fun drain(src: Source, out: MutableList<Int>): Int {
        var cause: Throwable? = null
        try {
            while (true) {
                val v = src.next() ?: break
                out.add(v)
            }
        } catch (e: Throwable) {
            cause = e
            throw e
        } finally {
            src.close(cause)
        }
        return out.size
    }

    suspend fun first(src: Source): Int {
        var cause: Throwable? = null
        try {
            val v = src.next()
            return v ?: -1
        } catch (e: Throwable) {
            cause = e
            throw e
        } finally {
            src.close(cause)
        }
    }
}
