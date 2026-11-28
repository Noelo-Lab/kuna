package pkg

class TestNotNullAssertion(var head: Node?, var name: String?) {
    class Node(var next: Node?, var value: Int, var label: String?)

    fun nameLength(): Int = name!!.length

    fun sum(): Int {
        var total = 0
        var node = head!!
        while (node.value >= 0) {
            total += node.value
            node = node.next!!
        }
        return total
    }

    fun secondLabel(): String = head!!.next!!.label!!

    fun span(other: TestNotNullAssertion): Int = other.head!!.value - other.head!!.next!!.value

    fun lookup(nodes: Map<String, Node>, key: String): Node {
        val found = nodes[key]!!
        return found
    }
}
