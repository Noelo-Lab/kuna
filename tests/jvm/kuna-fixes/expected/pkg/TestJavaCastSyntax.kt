package pkg

public class TestJavaCastSyntax(items: Array<Any?>) {
   private final val items: Array<Any?>

   init {
      this.items = items
   }

   public fun narrow(l: Long): Int {
      return l.toInt()
   }

   public fun toByte(d: Double): Byte {
      return d.toInt().toByte()
   }

   public fun chr(i: Int): Char {
      return i.toChar()
   }

   public fun minLen(byteCount: Long, limit: Int): Int {
      return Math.min(byteCount, (8192 - limit).toLong()).toInt()
   }

   public fun ratio(a: Int, b: Long): Double {
      return a.toDouble() / b
   }

   public fun half(x: Int): Short {
      return (x / 2).toShort()
   }

   public fun <T> get(i: Int): T {
      return this.items[i] as T
   }

   public fun <R> run(block: () -> R): R {
      return block() as R
   }
}
