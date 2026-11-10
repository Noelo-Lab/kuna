package pkg

public class TestNotNullAssertion(head: pkg.TestNotNullAssertion.Node?, name: String?) {
   public final var head: pkg.TestNotNullAssertion.Node?
   public final var name: String?

   init {
      this.head = head
      this.name = name
   }

   public fun nameLength(): Int {
      return this.name!!.length
   }

   public fun sum(): Int {
      var total: Int = 0
      var node: TestNotNullAssertion.Node = this.head!!

      while (node.value >= 0) {
         total += node.value
         node = node.next!!
      }

      return total
   }

   public fun secondLabel(): String {
      return this.head!!.next!!.label!!
   }

   public fun span(other: TestNotNullAssertion): Int {
      return other.head!!.value - other.head!!.next!!.value
   }

   public fun lookup(nodes: Map<String, pkg.TestNotNullAssertion.Node>, key: String): pkg.TestNotNullAssertion.Node {
      return nodes.get(key)!! as TestNotNullAssertion.Node
   }

   public class Node(next: pkg.TestNotNullAssertion.Node?, value: Int, label: String?) {
      public final var next: pkg.TestNotNullAssertion.Node?
      public final var value: Int
      public final var label: String?

      init {
         this.next = next
         this.value = value
         this.label = label
      }
   }
}
