package pkg

import java.util.ArrayList

public object TestObjectInstance {
   private final val pool: ArrayList<IntArray> = ArrayList()
   public final var taken: Int = 0

   public fun take(): IntArray {
      val var1: Int = taken++
      var var10000: IntArray
      if (pool.isEmpty()) {
         var10000 = IntArray(8)
      } else {
         var10000 = pool.removeAt(pool.size - 1) as IntArray
         var10000 = var10000
      }

      return var10000
   }

   public fun recycle(segment: IntArray) {
      pool.add(segment)
   }

   public class User {
      public fun cycle(): Int {
         val s: IntArray = TestObjectInstance.take()
         TestObjectInstance.recycle(s)
         return TestObjectInstance.taken + s.size
      }
   }
}
