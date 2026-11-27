package pkg

interface InlineSink<T> {
    suspend fun emit(value: T)
}

interface InlineFlow<T> {
    suspend fun collect(sink: InlineSink<T>)
}

inline fun <T> InlineFlow<T>.peek(crossinline action: suspend (T) -> Unit): InlineFlow<T> = object : InlineFlow<T> {
    override suspend fun collect(sink: InlineSink<T>) {
        this@peek.collect(object : InlineSink<T> {
            override suspend fun emit(value: T) {
                action(value)
                sink.emit(value)
            }
        })
    }
}
