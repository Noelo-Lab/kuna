package pkg

interface CastCaptureHandle<T> {
    fun fire(value: T)
}

class CastCaptureHandleImpl<T> : CastCaptureHandle<T> {
    var last: T? = null
    override fun fire(value: T) {
        last = value
    }
}

interface CastCaptureCallback {
    fun call(code: Int)
}

interface CastCaptureBoard {
    fun onClose(callback: CastCaptureCallback)
    fun onError(callback: (Int) -> Unit)
}

inline fun <T> withHandle(block: (CastCaptureHandle<T>) -> Unit): CastCaptureHandleImpl<T> {
    val impl = CastCaptureHandleImpl<T>()
    block(impl)
    return impl
}

interface CastCaptureGate {
    suspend fun pass()
}

suspend fun watch(board: CastCaptureBoard, gate: CastCaptureGate, done: () -> Unit): CastCaptureHandleImpl<Int> {
    try {
        val impl = withHandle<Int> { handle ->
            board.onClose(object : CastCaptureCallback {
                override fun call(code: Int) {
                    handle.fire(code + 1)
                }
            })
        }
        gate.pass()
        return impl
    } finally {
        done()
    }
}

suspend fun register(board: CastCaptureBoard, gate: CastCaptureGate, done: () -> Unit): CastCaptureHandleImpl<String> {
    try {
        val impl = withHandle<String> { handle ->
            board.onError { code -> handle.fire("closed $code") }
        }
        gate.pass()
        return impl
    } finally {
        done()
    }
}
