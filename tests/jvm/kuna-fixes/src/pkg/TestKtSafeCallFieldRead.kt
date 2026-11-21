package pkg

class TestKtSafeCallFieldRead {
    class Node(@JvmField val value: Int) {
        fun close(): Int = value
    }

    @Volatile
    @JvmField
    var stream: Node? = null

    @JvmField
    var plain: Node? = null

    private val fixed: Node? = Node(3)

    var closed: Int = 0

    fun cancel() {
        closed = stream?.close() ?: -1
    }

    fun valueOr(): Int = plain?.value ?: -1

    fun orDefault(): Node = stream ?: Node(0)

    fun finalField(): Int = fixed?.value ?: -1
}
