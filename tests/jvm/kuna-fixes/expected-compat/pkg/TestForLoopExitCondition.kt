package pkg

public class TestForLoopExitCondition {
   public fun sumBackwards(a: IntArray): Int {
      var c: Int = 0

      // $VF: Unable to resugar Kotlin loop from Java for loop
      var i: Int = a.length - 1
      while (true) {
         if (i >= 0) break
         c += a[i] * i

         i--
      }

      return c
   }

   public fun total(head: pkg.TestForLoopExitCondition.Node?): Int {
      var sum: Int = 0

      // $VF: Unable to resugar Kotlin loop from Java for loop
      var node: TestForLoopExitCondition.Node = head
      while (true) {
         if (node != null) break
         sum += node.value

         node = node.next
      }

      return sum
   }

   public class Node(value: Int, next: pkg.TestForLoopExitCondition.Node?) {
      public final val value: Int
      public final val next: pkg.TestForLoopExitCondition.Node?

      init {
         this.value = value
         this.next = next
      }
   }
}
