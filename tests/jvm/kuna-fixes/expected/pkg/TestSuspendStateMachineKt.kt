@file:SourceDebugExtension(["SMAP\nTestSuspendStateMachine.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestSuspendStateMachine.kt\npkg/TestSuspendStateMachineKt\n+ 2 fake.kt\nkotlin/jvm/internal/FakeKt\n*L\n1#1,80:1\n62#1:81\n1#2:82\n*S KotlinDebug\n*F\n+ 1 TestSuspendStateMachine.kt\npkg/TestSuspendStateMachineKt\n*L\n64#1:81\n*E\n"])

package pkg

import kotlin.coroutines.Continuation
import kotlin.coroutines.ContinuationInterceptor
import kotlin.coroutines.SafeContinuation
import kotlin.coroutines.CoroutineContext.Key
import kotlin.coroutines.intrinsics.IntrinsicsKt
import kotlin.coroutines.jvm.internal.ContinuationImpl
import kotlin.coroutines.jvm.internal.DebugProbesKt
import kotlin.jvm.internal.InlineMarker
import kotlin.jvm.internal.SourceDebugExtension

public suspend fun combine(src: StateSource, scale: Long, flag: Boolean): String {
   val a: Int = src.next()
   val d: Double = a * 1.5
   val b: Int = src.next()
   val s: String = src.text(a + b)
   return if (flag) "$s:${a + b}:$scale:$d" else s
}

public suspend fun firstAbove(src: StateSource, limit: Int, tries: Int): Int? {
   // $VF: Unable to resugar Kotlin loop from Java for loop
   var left: Int = tries
   while (left > 0) {
      val v: Int = src.next()
      if (v > limit) {
         return v
      }

      left--
   }

   return null
}

public suspend fun guarded(src: StateSource): Int {
   var var2: Int
   try {
      var2 = src.next() + src.next()
   } catch (var9: IllegalStateException) {
      var2 = -1
   }

   return var2
}

public suspend fun closing(src: StateSource, log: MutableList<String>): String {
   log.add("open")

   try {
      val t: String = src.text(src.next())
      log.add(t)
      return t
   } finally {
      log.add("close")
   }
}

public suspend fun StateSource.sumOf(n: Int): Int {
   var t: Int = 0

   repeat(n) { i ->
      t += this.next()
   }

   return t
}

public suspend inline fun StateSource.pair(): Int {
   return this.next() * 10 + this.next()
}

fun StateSource.`pair$$forInline`(`$completion`: Continuation<in Int>): Any {
   InlineMarker.mark(0)
   var var10000: Any? = `$this$pair`.next(`$completion`)
   InlineMarker.mark(1)
   val var3: Int = (var10000 as Number).toInt() * 10
   InlineMarker.mark(0)
   var10000 = `$this$pair`.next(`$completion`)
   InlineMarker.mark(1)
   return var3 + (var10000 as Number).toInt()
}

public suspend fun pairs(src: StateSource): Int {
   return src.next() * 10 + src.next() + src.next() * 10 + src.next()
}

public suspend fun mapped(src: StateSource, f: suspend (Int) -> String): String {
   return "${f(src.next()) as String}${f(src.next())}"
}

public suspend fun intercepted(src: StateSource): String {
   return "${src.next()}:${kotlin.coroutines.coroutineContext.get(ContinuationInterceptor.Key as Key) == null}"
}

public suspend fun handOff(x: Int): Int {
   var `$continuation`: Continuation
   run label29@{
      if (`$completion` is <unrepresentable>) {
         `$continuation` = `$completion` as <unrepresentable>
         if ((`$continuation`.label and -2147483648) != 0) {
            `$continuation`.label -= -2147483648
            return@label29
         }
      }

      `$continuation` = object : ContinuationImpl(`$completion`) {
         @JvmField internal var `I$0`: Int = 0
         @JvmField internal var label: Int = 0

         fun invokeSuspend(`$result`: Any): Any? {
            this.result = `$result`
            this.label = this.label or -2147483648
            return handOff(0, this as Continuation<in Int>)
         }
      }
   }

   val `$result`: Any = `$continuation`.result
   val var9: Any = IntrinsicsKt.getCOROUTINE_SUSPENDED()
   var var10000: Any
   when (`$continuation`.label) {
      0 -> {
         ResultKt.throwOnFailure(`$result`)
         `$continuation`.`I$0` = x
         `$continuation`.label = 1
         val var4: SafeContinuation = SafeContinuation(IntrinsicsKt.intercepted(`$continuation`))
         (var4 as Continuation).resumeWith(Result.constructor_impl/* $VF was: constructor-impl */(x + 1))
         var10000 = var4.getOrThrow()
         if (var10000 === IntrinsicsKt.getCOROUTINE_SUSPENDED()) {
            DebugProbesKt.probeCoroutineSuspended(`$continuation`)
         }

         if (var10000 === var9) {
            return var9
         }
      }
      1 -> {
         ResultKt.throwOnFailure(`$result`)
         var10000 = `$result`
      }
      else -> throw IllegalStateException("call to 'resume' before 'invoke' with coroutine")
   }

   return (var10000 as Number).toInt() * 2
}
