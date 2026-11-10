package pkg

class TestSuspendConditionalFinally {
    interface Cursor {
        suspend fun hasMore(): Boolean
        fun take(): Int
    }

    interface Source {
        fun open(): Cursor
        fun cancel(cause: Throwable?)
        fun closeIf(flag: Boolean)
    }

    interface Sink {
        suspend fun put(v: Int)
    }

    suspend fun drain(sink: Sink, src: Source, consume: Boolean) {
        var cause: Throwable? = null
        try {
            val it = src.open()
            while (it.hasMore()) {
                sink.put(it.take())
            }
        } catch (e: Throwable) {
            cause = e
            throw e
        } finally {
            if (consume) src.cancel(cause)
        }
    }

    suspend fun sum(src: Source, cursor: Cursor, flag: Boolean): Int {
        try {
            var total = 0
            while (cursor.hasMore()) {
                total += cursor.take()
            }
            return total
        } finally {
            src.closeIf(flag)
        }
    }
}
