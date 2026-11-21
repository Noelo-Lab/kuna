package pkg

import kotlin.jvm.internal.Intrinsics

public class TestNotNullAssertionLegacy(head: pkg.TestNotNullAssertionLegacy.Node?, name: String?) {
   public final var head: pkg.TestNotNullAssertionLegacy.Node?
      internal set

   public final var name: String?
      internal set

   init {
      this.head = head
      this.name = name
   }

   public fun nameLength(): Int {
      if (this.name == null) {
         Intrinsics.throwNpe()
      }

      return this.name.length()
   }

   public fun sum(): Int {
      var total: Int = 0
      if (this.head == null) {
         Intrinsics.throwNpe()
      }

      var node: TestNotNullAssertionLegacy.Node = this.head

      while (node.value >= 0) {
         total += node.value
         val var10000: TestNotNullAssertionLegacy.Node = node.next
         if (var10000 == null) {
            Intrinsics.throwNpe()
         }

         node = var10000
      }

      return total
   }

   public fun secondLabel(): String {
      if (this.head == null) {
         Intrinsics.throwNpe()
      }

      val var10000: TestNotNullAssertionLegacy.Node = this.head.next
      if (var10000 == null) {
         Intrinsics.throwNpe()
      }

      val var1: java.lang.String = var10000.label
      if (var1 == null) {
         Intrinsics.throwNpe()
      }

      return var1
   }

   public fun span(other: TestNotNullAssertionLegacy): Int {
      Intrinsics.checkParameterIsNotNull(other, "other")
      if (other.head == null) {
         Intrinsics.throwNpe()
      }

      val var10000: Int = other.head.value
      if (other.head == null) {
         Intrinsics.throwNpe()
      }

      val var10001: TestNotNullAssertionLegacy.Node = other.head.next
      if (var10001 == null) {
         Intrinsics.throwNpe()
      }

      return var10000 - var10001.value
   }

   public fun lookup(nodes: Map<String, pkg.TestNotNullAssertionLegacy.Node>, key: String): pkg.TestNotNullAssertionLegacy.Node {
      Intrinsics.checkParameterIsNotNull(nodes, "nodes")
      Intrinsics.checkParameterIsNotNull(key, "key")
      val var10000: Any = nodes.get(key)
      if (var10000 == null) {
         Intrinsics.throwNpe()
      }

      return var10000 as TestNotNullAssertionLegacy.Node
   }

   public class Node(next: pkg.TestNotNullAssertionLegacy.Node?, value: Int, label: String?) {
      public final var next: pkg.TestNotNullAssertionLegacy.Node?
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
