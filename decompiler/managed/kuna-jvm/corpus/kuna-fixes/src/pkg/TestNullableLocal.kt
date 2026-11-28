package pkg

class TestNullableLocal(private val cache: Map<String, String>) {
    class Node(var next: Node?, var value: Int)

    var head: Node? = null

    fun sum(): Int {
        var total = 0
        var node = head
        while (node != null) {
            total += node.value
            node = node.next
        }
        return total
    }

    private fun suffixed(host: String?, suffix: String): Boolean {
        var h = host
        if (h == null) h = "x"
        return h.length > suffix.length
    }

    fun length(key: String): Int {
        val v = cache[key] ?: return -1
        return v.length
    }

    fun last(key: String): String {
        var found: String? = null
        for (k in cache.keys) {
            if (k.length == key.length) found = k
        }
        if (found == null) return ""
        return found
    }

    fun check(key: String): Boolean = suffixed(cache[key], key)
}
