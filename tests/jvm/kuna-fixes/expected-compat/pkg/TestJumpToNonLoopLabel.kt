package pkg

public class TestJumpToNonLoopLabel(vararg items: Any) {
   public final val items: Array<pkg.TestJumpToNonLoopLabel.Item?>

   public final var size: Int
      internal set

   public final var log: Int
      internal set

   init {
      this.items = items
   }

   public fun removeAt(index: Int): pkg.TestJumpToNonLoopLabel.Item {
      this.size += -1
      label19@
      if (index < this.size) {
         this.swap(index, this.size)
         val var3: Int = (index - 1) / 2
         if (index > 0) {
            val var10000: TestJumpToNonLoopLabel.Item = this.items[index]
            val var4: Int = var10000.rank()
            val var10001: TestJumpToNonLoopLabel.Item = this.items[var3]
            if (var4 < var10001.rank()) {
               this.swap(index, var3)
               this.up(var3)
               break@label19
            }
         }

         this.down(index)
      }

      val var5: TestJumpToNonLoopLabel.Item = this.items[this.size]
      return var5
   }

   public fun matches(a: IntArray, b: IntArray): Int {
      var hits: Int = 0
      val n: Int = a.length

      repeat(n) label32@{ i ->
         val x: Int = a[i]
         val m: Int = b.length

         repeat(m) { j ->
            if (x == b[j]) {
               hits += i
               continue@label32
            }
         }
      }

      return hits
   }

   public fun swap(i: Int, j: Int) {
      this.log += i * 31 + j
   }

   public fun up(i: Int) {
      this.log += i
   }

   public fun down(i: Int) {
      this.log -= i
   }

   public interface Item {
      public abstract fun rank(): Int {
      }
   }
}
