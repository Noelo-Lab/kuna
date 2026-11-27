package pkg

fun interface SamSink<T> {
    suspend fun emit(value: T)
}

fun interface SamIntSink {
    suspend fun emit(value: Int)
}

interface SamFlow<T> {
    suspend fun collect(sink: SamSink<T>)
}

interface SamIntFlow {
    suspend fun collect(sink: SamIntSink)
}

suspend fun forward(f: SamIntFlow, out: SamIntSink) {
    f.collect { v ->
        out.emit(v)
        out.emit(v * 2)
    }
}

suspend fun relay(f: SamIntFlow, out: SamIntSink) {
    f.collect { v -> out.emit(v + 1) }
}

suspend fun lengths(f: SamFlow<String>, out: SamSink<Int>) {
    f.collect { s ->
        if (s.isNotEmpty()) out.emit(s.length)
        out.emit(-1)
    }
}
