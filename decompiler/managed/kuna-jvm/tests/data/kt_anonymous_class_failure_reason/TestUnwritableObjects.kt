package pkg

suspend fun step(): Int = 1

suspend fun twoSteps(): Int {
    val a = step()
    return a + step()
}

fun each(items: IntArray, f: (Int) -> Unit) {
    for (i in items) f(i)
}

fun localReference(items: IntArray): Int {
    var total = 0
    fun add(x: Int) {
        total += x
    }
    each(items, ::add)
    return total
}

interface Source {
    suspend fun collect(sink: Sink)
}

fun interface Sink {
    suspend fun put(value: Int)
}

inline fun source(crossinline block: suspend Sink.() -> Unit): Source = object : Source {
    override suspend fun collect(sink: Sink) {
        sink.block()
    }
}

fun Source.drop(count: Int): Source = source {
    var skipped = 0
    this@drop.collect { value ->
        if (skipped >= count) put(value) else skipped++
    }
}
