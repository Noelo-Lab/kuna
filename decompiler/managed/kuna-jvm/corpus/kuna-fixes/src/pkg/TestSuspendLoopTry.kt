package pkg

class TestSuspendLoopTry {
    interface Source {
        suspend fun next(): Int?
        suspend fun ready(): Boolean
        fun close(cause: Throwable?)
    }

    suspend fun indexOf(src: Source, x: Int): Int {
        var cause: Throwable? = null
        var i = 0
        try {
            while (true) {
                val v = src.next() ?: break
                if (v == x) return i
                i++
            }
        } catch (e: Throwable) {
            cause = e
            throw e
        } finally {
            src.close(cause)
        }
        return -1
    }

    suspend fun pairs(src: Source, other: Source, out: MutableList<Int>) {
        var cause: Throwable? = null
        try {
            while (true) {
                val v = src.next() ?: break
                if (!other.ready()) continue
                out.add(v)
            }
        } catch (e: Throwable) {
            cause = e
            throw e
        } finally {
            src.close(cause)
        }
    }

    suspend fun watch(src: Source, out: MutableList<Int>): Nothing {
        try {
            var old = -1
            while (true) {
                val v = src.next() ?: -1
                if (v != old) {
                    out.add(v)
                    old = v
                }
                if (!src.ready()) {
                    src.next()
                }
            }
        } finally {
            src.close(null)
        }
    }
}
