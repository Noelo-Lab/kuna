package pkg

import kotlin.coroutines.Continuation

public class TestSuspendFunctionType(action: (String, Continuation<Int>) -> Any?) {
   public final val action: (String, Continuation<Int>) -> Any?

   public final var hook: ((Continuation<Unit>) -> Any?)?
      internal set

   init {
      this.action = action
   }

   public fun pick(a: (Continuation<Unit>) -> Any?, b: (Int, String, Continuation<Boolean>) -> Any?, first: Boolean): Any {
      return if (first) a as Function else b as Function
   }

   public fun plain(f: (Int) -> Int): Int {
      return (f(1) as java.lang.Number).intValue()
   }
}
