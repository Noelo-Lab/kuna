package pkg

public class TestJumpToNonLoopLabel(items: Array<pkg.TestJumpToNonLoopLabel.Item?>) {
   public final val items: Array<pkg.TestJumpToNonLoopLabel.Item?>
   public final var size: Int = 0
   public final var log: Int = 0

   init {
      this.items = items
   }

   public fun removeAt(index: Int): pkg.TestJumpToNonLoopLabel.Item {
      this.size += -1
      run label17@{
         if (index < this.size) {
            this.swap(index, this.size)
            val var3: Int = (index - 1) / 2
            if (index > 0 && this.items[index]!!.rank() < this.items[var3]!!.rank()) {
               this.swap(index, var3)
               this.up(var3)
               return@label17
            } else {
               this.down(index)
            }
         }
      }

      return this.items[this.size]!!
   }

   public fun matches(a: IntArray, b: IntArray): Int {
      var hits: Int = 0
      val n: Int = a.size

      repeat(n) label32@{ i ->
         val x: Int = a[i]
         val m: Int = b.size

         repeat(m) { j ->
            if (x == b[j]) {
               hits += i
               return@label32
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
      public abstract fun rank(): Int
   }
}
