package pkg

abstract class UnlistedTask(val name: String, val cancelable: Boolean) {
    abstract fun runOnce(): Long
}

class UnlistedQueue {
    var last: UnlistedTask? = null
    var delay: Long = 0L

    fun schedule(task: UnlistedTask, delayNanos: Long) {
        last = task
        delay = delayNanos
    }

    inline fun execute(name: String, delayNanos: Long = 0L, cancelable: Boolean = true, crossinline block: () -> Unit) {
        schedule(object : UnlistedTask(name, cancelable) {
            override fun runOnce(): Long {
                block()
                return -1L
            }
        }, delayNanos)
    }
}

interface UnlistedHandler {
    fun handle(code: Int): Int
}

inline fun unlistedHandler(crossinline f: (Int) -> Int): UnlistedHandler = object : UnlistedHandler {
    override fun handle(code: Int): Int = f(code) + 1
}

interface UnlistedSink<T> {
    suspend fun put(value: T)
}

interface UnlistedSource<T> {
    suspend fun drain(sink: UnlistedSink<T>)
}

inline fun <T> unlistedSource(crossinline body: suspend UnlistedSink<T>.() -> Unit): UnlistedSource<T> = object : UnlistedSource<T> {
    override suspend fun drain(sink: UnlistedSink<T>) {
        sink.body()
    }
}

inline fun <T, R> UnlistedSource<T>.unlistedTransform(crossinline f: suspend UnlistedSink<R>.(T) -> Unit): UnlistedSource<R> = unlistedSource {
    val out = this
    drain(object : UnlistedSink<T> {
        override suspend fun put(value: T) {
            out.f(value)
        }
    })
}

class TestUnlistedAnonymousClass(private val queue: UnlistedQueue) {
    var sent = 0

    fun send(id: Int, label: String) {
        queue.execute(label, 5L) {
            sent += id + label.length
        }
    }

    fun offset(base: Int): UnlistedHandler = unlistedHandler { it * base + sent }

    fun keep(source: UnlistedSource<Int>, limit: Int): UnlistedSource<Int> = source.unlistedTransform { value ->
        if (value < limit) put(value + sent)
    }
}
