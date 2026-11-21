package pkg

import java.io.Closeable

class TestOverlappingRanges(private val store: Store) {
    open class IoFailure : RuntimeException()

    class InterruptedFailure : IoFailure()

    interface Store {
        fun open(name: String): Any
        fun delete(name: String)
        fun close(handle: Any, cause: Throwable?)
        fun read()
        fun clearInterrupt()
        fun interrupt()
        fun log(e: IoFailure)
    }

    fun civilized(res: Closeable, name: String): Boolean {
        res.use {
            try {
                store.delete(name)
                return true
            } catch (_: IoFailure) {
            }
        }
        store.delete(name)
        return false
    }

    fun readUninterruptibly() {
        var interrupted = false
        try {
            while (true) {
                try {
                    store.read()
                    return
                } catch (_: InterruptedFailure) {
                    store.clearInterrupt()
                    interrupted = true
                } catch (e: IoFailure) {
                    store.log(e)
                    return
                }
            }
        } finally {
            if (interrupted) {
                store.interrupt()
            }
        }
    }

    fun logged(name: String): Boolean {
        val handle = store.open(name)
        var cause: Throwable? = null
        try {
            try {
                store.delete(name)
                return true
            } catch (e: IoFailure) {
                store.log(e)
            }
        } catch (e: Throwable) {
            cause = e
            throw e
        } finally {
            store.close(handle, cause)
        }
        store.delete(name)
        return false
    }

    fun sharedTail(name: String, flag: Boolean): Boolean {
        val handle = store.open(name)
        var cause: Throwable? = null
        try {
            if (flag) {
                try {
                    store.delete(name)
                    return true
                } catch (e: IoFailure) {
                    store.log(e)
                }
            }
        } catch (e: Throwable) {
            cause = e
            throw e
        } finally {
            store.close(handle, cause)
        }
        store.delete(name)
        return false
    }

    fun countedRead(): Int {
        var interrupted = false
        var attempts = 0
        try {
            while (true) {
                attempts++
                try {
                    store.read()
                    return attempts
                } catch (_: InterruptedFailure) {
                    interrupted = true
                }
            }
        } finally {
            if (interrupted) {
                store.interrupt()
            }
        }
    }
}
