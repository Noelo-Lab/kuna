package pkg

class TestSuspendResultCast {
    interface Source {
        suspend fun next(): Int?
    }

    suspend fun sum(src: Source): Int {
        var total = 0
        while (true) {
            val v = src.next() ?: break
            total += v
        }
        return total
    }

    suspend fun fold(src: Source, initial: Int, op: suspend (Int, Int) -> Int): Int {
        var acc = initial
        while (true) {
            val v = src.next() ?: break
            acc = op(acc, v)
        }
        return acc
    }
}
