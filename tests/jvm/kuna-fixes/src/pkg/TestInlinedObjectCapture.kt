package pkg

fun interface Sink {
    suspend fun put(value: Int)
}

interface Source {
    suspend fun drain(sink: Sink)
}

inline fun source(crossinline block: suspend Sink.() -> Unit): Source = object : Source {
    override suspend fun drain(sink: Sink) {
        sink.block()
    }
}

fun Source.dropFirst(count: Int, fallback: Int): Source = source {
    var skipped = 0
    var empty = true
    this@dropFirst.drain { value ->
        empty = false
        if (skipped >= count) put(value) else skipped++
    }
    if (empty) {
        this@dropFirst.drain { put(it + fallback) }
    }
}

fun Source.orElse(fallback: Int): Source = source {
    var empty = true
    this@orElse.drain { value ->
        empty = false
        put(value)
    }
    if (empty) put(fallback)
}
