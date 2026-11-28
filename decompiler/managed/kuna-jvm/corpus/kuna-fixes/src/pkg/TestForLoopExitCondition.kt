package pkg

class TestForLoopExitCondition {
   class Node(val value: Int, val next: Node?)

   fun sumBackwards(a: IntArray): Int {
      var c = 0
      var i = a.size - 1
      while (i >= 0) {
         c += a[i] * i
         i--
      }
      return c
   }

   fun total(head: Node?): Int {
      var sum = 0
      var node = head
      while (node != null) {
         sum += node.value
         node = node.next
      }
      return sum
   }
}
