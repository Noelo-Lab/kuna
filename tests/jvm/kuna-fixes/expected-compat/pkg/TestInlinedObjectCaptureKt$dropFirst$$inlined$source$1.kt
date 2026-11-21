package pkg

import kotlin.coroutines.Continuation
import kotlin.coroutines.intrinsics.IntrinsicsKt
import kotlin.jvm.internal.SourceDebugExtension
import kotlin.jvm.internal.Ref.BooleanRef
import kotlin.jvm.internal.Ref.IntRef

@SourceDebugExtension(["SMAP\nTestInlinedObjectCapture.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestInlinedObjectCapture.kt\npkg/TestInlinedObjectCaptureKt$source$1\n+ 2 TestInlinedObjectCapture.kt\npkg/TestInlinedObjectCaptureKt\n*L\n1#1,14:1\n18#2,10:15\n*E\n"])
// $VF: local visibility outside of methodSupplier
internal class `TestInlinedObjectCaptureKt$dropFirst$$inlined$source$1` : Source {
   fun `TestInlinedObjectCaptureKt$dropFirst$$inlined$source$1`(var1: Source, var2: Int, var3: Int) {
      this.$this_dropFirst$inlined = var1
      this.$count$inlined = var2
      this.$fallback$inlined = var3
   }

   public override suspend fun drain(sink: Sink) {
      var `$continuation`: Continuation
      run label48@{
         if (`$completion` is TestInlinedObjectCaptureKt$dropFirst$$inlined$source$1$1) {
            `$continuation` = `$completion` as TestInlinedObjectCaptureKt$dropFirst$$inlined$source$1$1
            if (((`$completion` as TestInlinedObjectCaptureKt$dropFirst$$inlined$source$1$1).label and -2147483648) != 0) {
               `$continuation`.label -= -2147483648
               return@label48
            }
         }

         `$continuation` = TestInlinedObjectCaptureKt$dropFirst$$inlined$source$1$1(this, `$completion`)
      }

      val `$result`: Any = `$continuation`.result
val var5: Any = IntrinsicsKt.getCOROUTINE_SUSPENDED()
var `$this$dropFirst_u24lambda_u240`: Sink
var empty: BooleanRef
      when (`$continuation`.label) {
         0 -> {
            ResultKt.throwOnFailure(`$result`)
            `$this$dropFirst_u24lambda_u240` = sink
            val skipped: IntRef = IntRef()
            empty = BooleanRef()
            empty.element = true
            val var10000: Source = this.$this_dropFirst$inlined
            val var10001: Sink = /* $VF: Couldn't be decompiled. Class pkg/TestInlinedObjectCaptureKt$dropFirst$1$1 wasn't processed yet! */
            `$continuation`.L$0 = this
            `$continuation`.L$1 = sink
            `$continuation`.L$2 = empty
            `$continuation`.label = 1
            if (var10000.drain(var10001, `$continuation`) === var5) {
               return var5
            }
         }
         1 -> {
            empty = `$continuation`.L$2 as BooleanRef
            `$this$dropFirst_u24lambda_u240` = `$continuation`.L$1 as Sink
            this = `$continuation`.L$0 as TestInlinedObjectCaptureKt$dropFirst$$inlined$source$1
            ResultKt.throwOnFailure(`$result`)
         }
         2 -> {
            ResultKt.throwOnFailure(`$result`)
            return Unit.INSTANCE
         }
         else -> throw IllegalStateException("call to 'resume' before 'invoke' with coroutine")
      }

      if (empty.element) {
         val var12: Source = this.$this_dropFirst$inlined
         val var13: Sink = /* $VF: Couldn't be decompiled. Class pkg/TestInlinedObjectCaptureKt$dropFirst$1$2 wasn't processed yet! */
         `$continuation`.L$0 = null
         `$continuation`.L$1 = null
         `$continuation`.L$2 = null
         `$continuation`.label = 2
         if (var12.drain(var13, `$continuation`) === var5) {
            return var5
         }
      }

      return Unit.INSTANCE
   }
}
