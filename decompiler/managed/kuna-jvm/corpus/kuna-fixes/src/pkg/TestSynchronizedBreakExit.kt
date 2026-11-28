package pkg

class TestSynchronizedBreakExit {
    class Oops : RuntimeException()

    interface Handler

    class Finishing(@JvmField var rootCause: Any?, @JvmField var isCompleting: Boolean)

    var error = 0
    var a = false
    var b = false
    var count = 0
    @JvmField var state: Any = Finishing(null, false)

    fun close(code: Int): Boolean {
        synchronized(this) {
            if (error != 0) return false
            if (a && b) return false
            if (code < 0) throw Oops()
            error = code
        }
        count++
        return true
    }

    fun take(x: Int): Int {
        synchronized(this) {
            if (error != 0) return -1
            if (x > 100) throw Oops()
            if (a) {
                if (b) return 3
                return 2
            }
            count -= x
        }
        return 1
    }

    fun add(state: Any, node: Any): Boolean = state.hashCode() != node.hashCode()

    inline fun loopOnState(block: (Any) -> Unit): Nothing {
        while (true) block(state)
    }

    inline fun <reified T> Any.isHandlerOf(): Boolean = this is T

    fun register(onCancelling: Boolean, handler: Any, node: Any): Any? {
        loopOnState { state ->
            var rootCause: Any? = null
            var handle: Any? = null
            if (onCancelling && state is Finishing) {
                synchronized(state) {
                    rootCause = state.rootCause
                    if (rootCause == null || handler.isHandlerOf<Handler>() && !state.isCompleting) {
                        if (!add(state, node)) return@loopOnState
                        if (rootCause == null) return node
                        handle = node
                    }
                }
            }
            if (rootCause != null) return handle
            if (add(state, node)) return node
        }
    }
}
