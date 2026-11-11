package pkg

public class TestJavaStatementJump {
   private final var log: Int = 0

   public fun guarded(code: Int): Int {
      var var7: Int
      when (code) {
         1 -> {
            try {
               var7 = this.work(code)
            } finally {
               val var4: Int = this.log++
            }
         }
         2 -> var7 = 7
         else -> var7 = -1
      }

      return var7 + this.log
   }

   public fun firstHit(values: IntArray): Int {
      var r: Int = -1

      for (v in values) {
         if (v > 10) {
            try {
               r = this.work(v)
               break
            } finally {
               val var7: Int = this.log++
            }
         }
      }

      return r
   }

   private fun work(code: Int): Int {
      return code * 3
   }
}
