package pkg

@JvmInline
value class Span(val raw: Long)

class Shifted(val base: Long, val span: Span) {
    fun end(): Long = base + span.raw
}

class TestStaleSequence {
    fun make(base: Long, span: Span): Shifted = Shifted(base, span)
}
