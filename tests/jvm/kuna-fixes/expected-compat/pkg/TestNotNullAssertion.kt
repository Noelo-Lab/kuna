package pkg

public class TestNotNullAssertion(head: pkg.TestNotNullAssertion.Node?, name: String?) {
   public final var head: pkg.TestNotNullAssertion.Node?
      internal set

   public final var name: String?
      internal set

   init {
      this.head = head
      this.name = name
   }

   public fun nameLength(): Int {
      val var10000: java.lang.String = this.name
      return var10000.length()
   }

   public fun sum(): Int {
      var total: Int = 0
      var var10000: TestNotNullAssertion.Node = this.head
      var node: TestNotNullAssertion.Node = var10000

      while (node.value >= 0) {
         total += node.value
         var10000 = node.next
         node = var10000
      }

      return total
   }

   public fun secondLabel(): String {
      var var10000: TestNotNullAssertion.Node = this.head
      var10000 = var10000.next
      val var2: java.lang.String = var10000.label
      return var2
   }

   public fun span(other: TestNotNullAssertion): Int {
      val var10000: TestNotNullAssertion.Node = other.head
      val var2: Int = var10000.value
      var var10001: TestNotNullAssertion.Node = other.head
      var10001 = var10001.next
      return var2 - var10001.value
   }

   public fun lookup(nodes: Map<String, pkg.TestNotNullAssertion.Node>, key: String): pkg.TestNotNullAssertion.Node {
      val var10000: Any = nodes.get(key)
      return var10000 as TestNotNullAssertion.Node
   }

   public class Node(next: pkg.TestNotNullAssertion.Node?, value: Int, label: String?) {
      public final var next: pkg.TestNotNullAssertion.Node?
         internal set

      public final var value: Int
         internal set

      public final var label: String?
         internal set

      init {
         this.next = next
         this.value = value
         this.label = label
      }
   }
}
