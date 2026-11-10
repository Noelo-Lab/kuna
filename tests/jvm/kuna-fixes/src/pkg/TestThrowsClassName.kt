package pkg

class TestThrowsClassName {
    class Failure(message: String) : Exception(message)

    @Throws(Failure::class)
    fun load(path: String): Int {
        if (path.isEmpty()) throw Failure("empty")
        return path.length
    }

    @Throws(Failure::class, IllegalStateException::class, Throwable::class)
    fun check(n: Int) {
        if (n < 0) throw IllegalStateException("negative")
    }
}
