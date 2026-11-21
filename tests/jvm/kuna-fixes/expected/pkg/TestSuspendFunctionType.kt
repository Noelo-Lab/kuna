package pkg

public class TestSuspendFunctionType(action: suspend (String) -> Int) {
   public final val action: suspend (String) -> Int
   public final var hook: (suspend () -> Unit)? = null

   init {
      this.action = action
   }

   public fun pick(a: suspend () -> Unit, b: suspend (Int, String) -> Boolean, first: Boolean): Any {
      return if (first) a as Function else b as Function
   }

   public fun plain(f: (Int) -> Int): Int {
      return (f(1) as Number).toInt()
   }
}
