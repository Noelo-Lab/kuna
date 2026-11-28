package pkg

class TestKtHeadDefinitionScope {
    interface Cursor {
        suspend operator fun hasNext(): Boolean
        operator fun next(): Int
    }

    interface Source {
        operator fun iterator(): Cursor
        fun count(): Int
    }

    interface Sink {
        suspend fun put(v: Int)
    }

    suspend fun drain(sink: Sink, src: Source): Int {
        for (v in src) {
            sink.put(v)
        }
        val e = src.count()
        return e + 1
    }
}
