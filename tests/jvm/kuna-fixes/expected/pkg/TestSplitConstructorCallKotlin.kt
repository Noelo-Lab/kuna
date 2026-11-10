package pkg

import java.util.ArrayList

public class TestSplitConstructorCallKotlin(count: Int, label: String?) {
   private final val count: Int
   private final val label: String?

   init {
      this.count = count
      this.label = label
   }

   public fun build(): SplitPair {
      if (this.label == null) {
         throw IllegalStateException("label == null")
      } else {
         return SplitPair(this.count, this.label)
      }
   }

   public fun orNull(): SplitPair? {
      return if (this.label == null) null else SplitPair(this.count, this.label)
   }

   public fun collect(keys: Array<String?>): List<SplitPair> {
      val out: ArrayList = ArrayList()
      var i: Int = this.count

      for (k in keys) {
         val var10001: Int = i++
         if (k != null) {
            out.add(SplitPair(var10001, k))
         }
      }

      return out as MutableList<SplitPair>
   }
}
