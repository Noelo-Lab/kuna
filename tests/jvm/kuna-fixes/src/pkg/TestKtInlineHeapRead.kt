package pkg

class TestKtInlineHeapRead {
    class Node(@JvmField val value: Int)

    @Volatile
    @JvmField
    var state: Any? = null

    @JvmField
    var plain: Node? = null

    @JvmField
    var slots: Array<Node?> = arrayOfNulls(2)

    private val fixed: Node = Node(7)

    fun typeTest(): Int {
        val s = state
        if (s is Node) {
            return s.value
        }
        return -1
    }

    fun nonVolatile(): Int {
        val p = plain
        return if (p != null) p.value else 0
    }

    fun element(i: Int): Int {
        val e = slots[i]
        return if (e != null) e.value else 0
    }

    fun staticRead(): Int {
        val s = shared
        return if (s != null) s.value + s.value else 0
    }

    fun finalField(): Int {
        val f = fixed
        return f.value + f.value
    }

    fun safeCast(): Node? = state as? Node

    fun single(): Any? {
        val s = state
        return s
    }

    companion object {
        @Volatile
        @JvmField
        var shared: Node? = null
    }
}
