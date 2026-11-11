package pkg

class TestKtInlineHeapRead {
    class Node(@JvmField val value: Int, @JvmField val next: Node?)

    @JvmField
    var slots: Array<Node?> = arrayOfNulls(2)

    @Volatile
    @JvmField
    var head: Node = Node(0, null)

    @Volatile
    @JvmField
    var count: Int = 0

    @Volatile
    @JvmField
    var limit: Int = 0

    @JvmField
    var items: Array<Any?> = arrayOfNulls(1)

    private val fixed: Node = Node(7, null)

    fun element(i: Int): Int {
        val e = slots[i]
        return if (e != null) e.value else 0
    }

    fun shared(): Int {
        val s = cache[0]
        return if (s != null) s.value + s.value else 0
    }

    fun nextValue(): Int {
        val n = head.next
        return if (n != null) n.value else -1
    }

    fun room(): Int {
        val r = limit - count
        return if (r > 0) r else 0
    }

    fun cast(): Node? = items[0] as? Node

    fun finalField(): Int {
        val f = fixed.next
        return if (f != null) f.value else 0
    }

    fun single(i: Int): Node? {
        val e = slots[i]
        return e
    }

    companion object {
        @JvmField
        val cache: Array<Node?> = arrayOfNulls(1)
    }
}
