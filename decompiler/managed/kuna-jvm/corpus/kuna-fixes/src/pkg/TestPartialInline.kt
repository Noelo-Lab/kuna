package pkg

class TestPartialInline {
    class Box(var size: Int)

    fun withDefault(text: String?): Int {
        val t = text ?: "none"
        return t.length
    }

    fun boxed(state: Any?): Int {
        val box = state as? Box ?: Box(0)
        synchronized(box) {
            return box.size
        }
    }

    fun shifted(host: String?, suffix: String): Boolean {
        var h = host
        if (h == null) h = "localhost"
        return h.length > suffix.length
    }
}
