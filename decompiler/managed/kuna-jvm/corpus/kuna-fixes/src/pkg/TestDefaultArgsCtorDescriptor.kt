package pkg

class TestDefaultArgsCtorDescriptor {
    inner class Sink(val finished: Boolean = false, val limit: Long = 16L)

    class Nested(val size: Int = 8)

    fun sink(): Sink = Sink()
}
