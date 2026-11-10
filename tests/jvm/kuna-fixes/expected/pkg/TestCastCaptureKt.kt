@file:SourceDebugExtension(["SMAP\nTestCastCapture.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestCastCapture.kt\npkg/TestCastCaptureKt\n*L\n1#1,60:1\n24#1,3:61\n24#1,3:64\n*S KotlinDebug\n*F\n+ 1 TestCastCapture.kt\npkg/TestCastCaptureKt\n*L\n35#1:61,3\n51#1:64,3\n*E\n"])

package pkg

import kotlin.coroutines.Continuation
import kotlin.coroutines.intrinsics.IntrinsicsKt
import kotlin.coroutines.jvm.internal.ContinuationImpl
import kotlin.jvm.functions.Function0
import kotlin.jvm.internal.SourceDebugExtension

public inline fun <T> withHandle(block: (CastCaptureHandle<T>) -> Unit): CastCaptureHandleImpl<T> {
   val impl: CastCaptureHandleImpl = CastCaptureHandleImpl()
   block(impl)
   return impl
}

public suspend fun watch(board: CastCaptureBoard, gate: CastCaptureGate, done: () -> Unit): CastCaptureHandleImpl<Int> {
   var `$continuation`: Continuation
   run label101@{
      if (`$completion` is <unrepresentable>) {
         `$continuation` = `$completion` as <unrepresentable>
         if (((`$completion` as <unrepresentable>).label and -2147483648) != 0) {
            `$continuation`.label -= -2147483648
            return@label101
         }
      }

      `$continuation` = object : ContinuationImpl(`$completion`) {
         open Object L$0;
         open Object L$1;
         open int label;

         fun invokeSuspend(`$result`: Any): Any? {
            this.result = `$result`
            this.label |= -2147483648
            return TestCastCaptureKt.watch(null, null, null, this as Continuation<? super CastCaptureHandleImpl<Integer>>)
         }
      }
   }

   val `$result`: Any = `$continuation`.result
val var11: Any = IntrinsicsKt.getCOROUTINE_SUSPENDED()
var impl: CastCaptureHandleImpl
   when (`$continuation`.label) {
      0 -> {
         ResultKt.throwOnFailure(`$result`)
         var var18: Boolean = false /* VF: Semaphore variable */

         var var10000: Any
         try {
            var18 = true
            val `impl$iv`: CastCaptureHandleImpl = CastCaptureHandleImpl()
            board.onClose(object : CastCaptureCallback {
               public override fun call(code: Int) {
                  impl$iv.fire(code + 1)
               }
            })
            impl = `impl$iv`
            `$continuation`.L$0 = done
            `$continuation`.L$1 = `impl$iv`
            `$continuation`.label = 1
            var10000 = gate.pass(`$continuation`)
            var18 = false
         } finally {
            if (var18) {
               done()
            }
         }

         if (var10000 === var11) {
            return var11
         }
      }
      1 -> {
         impl = `$continuation`.L$1 as CastCaptureHandleImpl
         done = `$continuation`.L$0 as Function0
         var var22: Boolean = false /* VF: Semaphore variable */

         try {
            var22 = true
            ResultKt.throwOnFailure(`$result`)
            var22 = false
         } finally {
            if (var22) {
               done()
            }
         }
      }
      else -> throw IllegalStateException("call to 'resume' before 'invoke' with coroutine")
   }

   try {
      return impl
   } finally {
      done()
   }
}

public suspend fun register(board: CastCaptureBoard, gate: CastCaptureGate, done: () -> Unit): CastCaptureHandleImpl<String> {
   var `$continuation`: Continuation
   run label101@{
      if (`$completion` is <unrepresentable>) {
         `$continuation` = `$completion` as <unrepresentable>
         if (((`$completion` as <unrepresentable>).label and -2147483648) != 0) {
            `$continuation`.label -= -2147483648
            return@label101
         }
      }

      `$continuation` = object : ContinuationImpl(`$completion`) {
         open Object L$0;
         open Object L$1;
         open int label;

         fun invokeSuspend(`$result`: Any): Any? {
            this.result = `$result`
            this.label |= -2147483648
            return TestCastCaptureKt.register(null, null, null, this as Continuation<? super CastCaptureHandleImpl<java.lang.String>>)
         }
      }
   }

   val `$result`: Any = `$continuation`.result
val var11: Any = IntrinsicsKt.getCOROUTINE_SUSPENDED()
var impl: CastCaptureHandleImpl
   when (`$continuation`.label) {
      0 -> {
         ResultKt.throwOnFailure(`$result`)
         var var18: Boolean = false /* VF: Semaphore variable */

         var var10000: Any
         try {
            var18 = true
            val `impl$iv`: CastCaptureHandleImpl = CastCaptureHandleImpl()
            board.onError({ code: Int ->
               impl$iv.fire("closed $code")
            } as (Int?) -> Unit)
            impl = `impl$iv`
            `$continuation`.L$0 = done
            `$continuation`.L$1 = `impl$iv`
            `$continuation`.label = 1
            var10000 = gate.pass(`$continuation`)
            var18 = false
         } finally {
            if (var18) {
               done()
            }
         }

         if (var10000 === var11) {
            return var11
         }
      }
      1 -> {
         impl = `$continuation`.L$1 as CastCaptureHandleImpl
         done = `$continuation`.L$0 as Function0
         var var22: Boolean = false /* VF: Semaphore variable */

         try {
            var22 = true
            ResultKt.throwOnFailure(`$result`)
            var22 = false
         } finally {
            if (var22) {
               done()
            }
         }
      }
      else -> throw IllegalStateException("call to 'resume' before 'invoke' with coroutine")
   }

   try {
      return impl
   } finally {
      done()
   }
}
