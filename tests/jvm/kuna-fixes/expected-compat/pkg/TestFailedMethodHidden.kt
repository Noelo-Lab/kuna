package pkg

import kotlin.coroutines.Continuation

public interface TestFailedMethodHidden {
   public abstract fun note(x: Int) {
   }

   public abstract suspend fun next(): Int {
   }

   public open suspend fun twice(): Int {
   }

   public open fun reset(x: Int) {
   }

   // $VF: Class flags could not be determined
   internal class DefaultImpls {
      @JvmStatic
      fun reset(`$this`: TestFailedMethodHidden, x: Int) {
         if (x < 0) {
            `$this`.note(0)
         } else {
            `$this`.note(x)
         }
      }
   }
}
