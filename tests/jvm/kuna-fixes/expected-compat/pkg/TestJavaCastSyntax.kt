package pkg

public class TestJavaCastSyntax(vararg items: Any) {
   private final val items: Array<Any?>

   init {
      this.items = items
   }

   public fun narrow(l: Long): Int {
      return (int)l
   }

   public fun toByte(d: Double): Byte {
      return (byte)d
   }

   public fun chr(i: Int): Char {
      return (char)i
   }

   public fun minLen(byteCount: Long, limit: Int): Int {
      return (int)Math.min(byteCount, (long)(8192 - limit))
   }

   public fun ratio(a: Int, b: Long): Double {
      return (double)a / b
   }

   public fun half(x: Int): Short {
      return (short)(x / 2)
   }

   public fun <T> get(i: Int): T {
      return (T)this.items[i]
   }

   public fun <R> run(block: () -> R): R {
      return (R)block()
   }
}
