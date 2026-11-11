package pkg

public class TestNotNullAssertionLegacy(head: pkg.TestNotNullAssertionLegacy.Node?, name: String?) {
   public final var head: pkg.TestNotNullAssertionLegacy.Node?
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
      var node: TestNotNullAssertionLegacy.Node = this.head!!

      while (node.value >= 0) {
         total += node.value
         node = node.next!!
      }

      return total
   }

   public fun secondLabel(): String {
      return this.head!!.next!!.label!!
   }

   public fun span(other: TestNotNullAssertionLegacy): Int {
      return other.head!!.value - other.head!!.next!!.value
   }

   public fun lookup(nodes: Map<String, pkg.TestNotNullAssertionLegacy.Node>, key: String): pkg.TestNotNullAssertionLegacy.Node {
      return nodes.get(key)!! as TestNotNullAssertionLegacy.Node
   }

   public class Node(next: pkg.TestNotNullAssertionLegacy.Node?, value: Int, label: String?) {
      public final var next: pkg.TestNotNullAssertionLegacy.Node?
      public final var value: Int
      public final var label: String?

      init {
         this.next = next
         this.value = value
         this.label = label
      }
   }
}
