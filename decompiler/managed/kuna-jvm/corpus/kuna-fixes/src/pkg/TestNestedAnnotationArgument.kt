package pkg

class TestNestedAnnotationArgument {
    @Deprecated(message = "moved to val", replaceWith = ReplaceWith(expression = "size"), level = DeprecationLevel.ERROR)
    fun size(): Int = size

    val size: Int = 3

    @Deprecated("moved to extension", ReplaceWith("other.copy()", "pkg.copy"))
    fun copy(other: String): String = other + "!"
}
