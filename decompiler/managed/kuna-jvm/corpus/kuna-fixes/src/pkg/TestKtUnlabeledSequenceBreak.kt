package pkg

class TestKtUnlabeledSequenceBreak {
    private val lock = Any()
    var folded: Any? = null
    var closed = false
    var space = 0L
    var written = 0L

    fun write(n: Long): Any? {
        var count = n
        var found: Any? = null
        synchronized(lock) {
            if (closed) throw IllegalStateException("closed")
            while (count > 0) {
                folded?.let {
                    found = it
                    return@synchronized
                }
                if (space == 0L) {
                    space = 1L
                    continue
                }
                val step = minOf(space, count)
                written += step
                count -= step
            }
        }
        if (found != null) {
            written = count
        }
        return found
    }
}
