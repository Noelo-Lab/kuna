package pkg

import kotlin.coroutines.Continuation
import kotlin.coroutines.intrinsics.IntrinsicsKt
import kotlin.coroutines.jvm.internal.Boxing
import kotlin.coroutines.jvm.internal.ContinuationImpl

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
      fun twice(`$this`: TestFailedMethodHidden, `$completion`: Continuation<in Integer>): Any? {
         var `$continuation`: Continuation
         run label32@{
            if (`$completion` is <unrepresentable>) {
               `$continuation` = `$completion` as <unrepresentable>
               if (((`$completion` as <unrepresentable>).label and -2147483648) != 0) {
                  `$continuation`.label -= -2147483648
                  return@label32
               }
            }

            `$continuation` = object : ContinuationImpl(`$completion`) {
               @JvmField internal var `L$0`: Any? = null
               @JvmField internal var `I$0`: Int = 0
               @JvmField internal var label: Int = 0

               fun invokeSuspend(`$result`: Any): Any? {
                  this.result = `$result`
                  this.label = this.label or -2147483648
                  return TestFailedMethodHidden.DefaultImpls.twice(null, this as Continuation<in Integer>)
               }
            }
         }

         var var3: Int
         var var10000: Any
         run label36@{
            val `$result`: Any = `$continuation`.result
            val var7: Any = IntrinsicsKt.getCOROUTINE_SUSPENDED()
            when (`$continuation`.label) {
               0 -> {
                  ResultKt.throwOnFailure(`$result`)
                  `$continuation`.L$0 = `$this`
                  `$continuation`.label = 1
                  var10000 = `$this`.next(`$continuation`)
                  if (var10000 === var7) {
                     return var7
                  }
               }
               1 -> {
                  `$this` = `$continuation`.L$0 as TestFailedMethodHidden
                  ResultKt.throwOnFailure(`$result`)
                  var10000 = `$result`
               }
               2 -> {
                  var3 = `$continuation`.I$0
                  ResultKt.throwOnFailure(`$result`)
                  var10000 = `$result`
                  return@label36
               }
               else -> throw IllegalStateException("call to 'resume' before 'invoke' with coroutine")
            }

            var3 = (var10000 as java.lang.Number).intValue()
            `$continuation`.L$0 = null
            `$continuation`.I$0 = var3
            `$continuation`.label = 2
            var10000 = `$this`.next(`$continuation`)
            if (var10000 === var7) {
               return var7
            }
         }

         return Boxing.boxInt(var3 + (var10000 as java.lang.Number).intValue())
      }

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
