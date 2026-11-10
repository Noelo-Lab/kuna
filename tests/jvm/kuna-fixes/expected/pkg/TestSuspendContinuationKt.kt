package pkg

import kotlin.coroutines.Continuation
import kotlin.coroutines.intrinsics.IntrinsicsKt
import kotlin.coroutines.jvm.internal.Boxing
import kotlin.coroutines.jvm.internal.ContinuationImpl

public suspend fun sumOf(src: SuspendSource, n: Int): Int {
   var `$continuation`: Continuation
   run label43@{
      if (`$completion` is <unrepresentable>) {
         `$continuation` = `$completion` as <unrepresentable>
         if (((`$completion` as <unrepresentable>).label and -2147483648) != 0) {
            `$continuation`.label -= -2147483648
            return@label43
         }
      }

      `$continuation` = object : ContinuationImpl(`$completion`) {
         open Object L$0;
         open int I$0;
         open int I$1;
         open int I$2;
         open int label;

         fun invokeSuspend(`$result`: Any): Any? {
            this.result = `$result`
            this.label |= -2147483648
            return TestSuspendContinuationKt.sumOf(null, 0, this as Continuation<in Integer>)
         }
      }
   }

   val `$result`: Any = `$continuation`.result
val var9: Any = IntrinsicsKt.getCOROUTINE_SUSPENDED()
var total: Int
var i: Int
   when (`$continuation`.label) {
      0 -> {
         ResultKt.throwOnFailure(`$result`)
         total = 0
         i = 0
      }
      1 -> {
         val var5: Int = `$continuation`.I$2
         i = `$continuation`.I$1
         n = `$continuation`.I$0
         src = `$continuation`.L$0 as SuspendSource
         ResultKt.throwOnFailure(`$result`)
         total = var5 + (`$result` as java.lang.Number).intValue()
         i++
      }
      else -> throw IllegalStateException("call to 'resume' before 'invoke' with coroutine")
   }

   while (i < n) {
      `$continuation`.L$0 = src
      `$continuation`.I$0 = n
      `$continuation`.I$1 = i
      `$continuation`.I$2 = total
      `$continuation`.label = 1
      val var10000: Any = src.next(`$continuation`)
      if (var10000 === var9) {
         return var9
      }

      total += (var10000 as java.lang.Number).intValue()
      i++
   }

   return Boxing.boxInt(total)
}

public suspend fun firstPositive(src: SuspendSource, tries: Int): Int? {
   var `$continuation`: Continuation
   run label52@{
      if (`$completion` is <unrepresentable>) {
         `$continuation` = `$completion` as <unrepresentable>
         if (((`$completion` as <unrepresentable>).label and -2147483648) != 0) {
            `$continuation`.label -= -2147483648
            return@label52
         }
      }

      `$continuation` = object : ContinuationImpl(`$completion`) {
         open Object L$0;
         open int I$0;
         open int label;

         fun invokeSuspend(`$result`: Any): Any? {
            this.result = `$result`
            this.label |= -2147483648
            return TestSuspendContinuationKt.firstPositive(null, 0, this as Continuation<in Integer>)
         }
      }
   }

   val `$result`: Any = `$continuation`.result
val var7: Any = IntrinsicsKt.getCOROUTINE_SUSPENDED()
var left: Int
   when (`$continuation`.label) {
      0 -> {
         ResultKt.throwOnFailure(`$result`)
         left = tries
      }
      1 -> {
         left = `$continuation`.I$0
         src = `$continuation`.L$0 as SuspendSource
         ResultKt.throwOnFailure(`$result`)
         val v: Int = (`$result` as java.lang.Number).intValue()
         if (v > 0) {
            return Boxing.boxInt(v)
         }

         left--
      }
      else -> throw IllegalStateException("call to 'resume' before 'invoke' with coroutine")
   }

   while (left > 0) {
      `$continuation`.L$0 = src
      `$continuation`.I$0 = left
      `$continuation`.label = 1
      val var10000: Any = src.next(`$continuation`)
      if (var10000 === var7) {
         return var7
      }

      val var9: Int = (var10000 as java.lang.Number).intValue()
      if (var9 > 0) {
         return Boxing.boxInt(var9)
      }

      left--
   }

   return null
}

public fun summing(src: SuspendSource): SuspendSink {
   return object : SuspendSink {
      public final var sum: Long
         internal set

      public override suspend fun put(v: Int) {
         var `$continuation`: Continuation
         run label25@{
            if (`$completion` is <unrepresentable>) {
               `$continuation` = `$completion` as <unrepresentable>
               if (((`$completion` as <unrepresentable>).label and -2147483648) != 0) {
                  `$continuation`.label -= -2147483648
                  return@label25
               }
            }

            `$continuation` = object : ContinuationImpl(`$completion`) {
               open Object L$0;
               open long J$0;
               open int I$0;
               open int label;

               fun invokeSuspend(`$result`: Any): Any {
                  this.result = `$result`
                  this.label |= -2147483648
                  return put(0, this as Continuation<in Unit>)
               }
            }
         }

         val `$result`: Any = `$continuation`.result
val var10: Any = IntrinsicsKt.getCOROUTINE_SUSPENDED()
var var3: <unrepresentable>
var var4: Long
var var6: Int
var var10000: Any
         when (`$continuation`.label) {
            0 -> {
               ResultKt.throwOnFailure(`$result`)
               var6 = v
               var4 = this.sum
               var3 = this
               val var11: SuspendSource = src
               `$continuation`.L$0 = this
               `$continuation`.J$0 = var4
               `$continuation`.I$0 = v
               `$continuation`.label = 1
               var10000 = var11.next(`$continuation`)
               if (var10000 === var10) {
                  return var10
               }
            }
            1 -> {
               var6 = `$continuation`.I$0
               var4 = `$continuation`.J$0
               var3 = `$continuation`.L$0 as <unrepresentable>
               ResultKt.throwOnFailure(`$result`)
               var10000 = `$result`
            }
            else -> throw IllegalStateException("call to 'resume' before 'invoke' with coroutine")
         }

         var3.sum = var4 + (var6 + (var10000 as java.lang.Number).intValue())
         return Unit.INSTANCE
      }
   }
}

public suspend fun SequenceScope<Int>.yieldTwice(x: Int) {
   var `$continuation`: Continuation
   run label36@{
      if (`$completion` is <unrepresentable>) {
         `$continuation` = `$completion` as <unrepresentable>
         if (((`$completion` as <unrepresentable>).label and -2147483648) != 0) {
            `$continuation`.label -= -2147483648
            return@label36
         }
      }

      `$continuation` = object : ContinuationImpl(`$completion`) {
         open Object L$0;
         open int I$0;
         open int label;

         fun invokeSuspend(`$result`: Any): Any? {
            this.result = `$result`
            this.label |= -2147483648
            return TestSuspendContinuationKt.yieldTwice(null, 0, this as Continuation<in Unit>)
         }
      }
   }

   val `$result`: Any = `$continuation`.result
val var5: Any = IntrinsicsKt.getCOROUTINE_SUSPENDED()
   when (`$continuation`.label) {
      0 -> {
         ResultKt.throwOnFailure(`$result`)
         val var10001: Int = Boxing.boxInt(x)
         `$continuation`.L$0 = `$this$yieldTwice`
         `$continuation`.I$0 = x
         `$continuation`.label = 1
         if (`$this$yieldTwice`.yield(var10001, `$continuation`) === var5) {
            return var5
         }
      }
      1 -> {
         x = `$continuation`.I$0
         `$this$yieldTwice` = `$continuation`.L$0 as SequenceScope
         ResultKt.throwOnFailure(`$result`)
      }
      2 -> {
         ResultKt.throwOnFailure(`$result`)
         return Unit.INSTANCE
      }
      else -> throw IllegalStateException("call to 'resume' before 'invoke' with coroutine")
   }

   val var6: Int = Boxing.boxInt(x + 1)
   `$continuation`.L$0 = null
   `$continuation`.label = 2
   return if (`$this$yieldTwice`.yield(var6, `$continuation`) === var5) var5 else Unit.INSTANCE
}
