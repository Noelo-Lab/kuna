package pkg

class TestInlineMarkerFinally {
    interface Lock {
        fun lock()
        fun unlock()
    }

    var hits = 0

    inline fun <T> takeIf(get: () -> T?, pred: (T) -> Boolean): T? = synchronized(this) {
        val first = get() ?: return null
        hits++
        if (pred(first)) first else null
    }

    inline fun <R> Lock.locked(block: () -> R): R {
        lock()
        try {
            return block()
        } finally {
            unlock()
        }
    }

    inline fun <T> lockedOrNull(lock: Lock, get: () -> T?): T? = lock.locked {
        val v = get() ?: return null
        hits++
        v
    }
}
