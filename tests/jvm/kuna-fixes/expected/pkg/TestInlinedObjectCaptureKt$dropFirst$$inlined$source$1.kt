@file:SourceDebugExtension(["SMAP\nTestInlinedObjectCapture.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestInlinedObjectCapture.kt\npkg/TestInlinedObjectCaptureKt\n*L\n1#1,37:1\n11#1,5:38\n11#1,5:43\n*S KotlinDebug\n*F\n+ 1 TestInlinedObjectCapture.kt\npkg/TestInlinedObjectCaptureKt\n*L\n17#1:38,5\n29#1:43,5\n*E\n"])

package pkg

import kotlin.coroutines.Continuation
import kotlin.coroutines.intrinsics.IntrinsicsKt
import kotlin.coroutines.jvm.internal.ContinuationImpl
import kotlin.jvm.internal.InlineMarker
import kotlin.jvm.internal.SourceDebugExtension
import kotlin.jvm.internal.Ref.BooleanRef
import kotlin.jvm.internal.Ref.IntRef

public inline fun source(crossinline block: suspend (Sink) -> Unit): Source {
   return object : Source {
      public override suspend fun drain(sink: Sink) {
         block(sink)
         return Unit.INSTANCE
      }

      fun `drain$$forInline`(sink: Sink, `$completion`: Continuation<in Unit>): Any {
         InlineMarker.mark(4)
         object : ContinuationImpl(`$completion`) {
            @JvmField internal var label: Int = 0

            fun invokeSuspend(`$result`: Any): Any {
               this.result = `$result`
               this.label = this.label or -2147483648
               return drain(null, this as Continuation<in Unit>)
            }
         }
         InlineMarker.mark(5)
         block(sink, `$completion`)
         return Unit.INSTANCE
      }
   }
}

public fun Source.dropFirst(count: Int, fallback: Int): Source {
   return object : Source {
      public override suspend fun drain(sink: Sink) {
         val `$this$dropFirst_u24lambda_u240`: Sink = sink
         val skipped: IntRef = IntRef()
         val empty: BooleanRef = BooleanRef()
         empty.element = true
         `$this$dropFirst`.drain(object : Sink {
            override final fun put(value: Int, `$completion`: Continuation<in Unit>): Any {
               empty.element = false
               if (skipped.element >= count) {
                  val var10000: Any? = `$this$dropFirst_u24lambda_u240`.put(value, `$completion`)
                  return if (var10000 === IntrinsicsKt.getCOROUTINE_SUSPENDED()) var10000 else Unit.INSTANCE
               } else {
                  val var3: Int = skipped.element++
                  return Unit.INSTANCE
               }
            }
         })
         if (empty.element) {
            `$this$dropFirst`.drain(object : Sink {
               override final fun put(it: Int, `$completion`: Continuation<in Unit>): Any {
                  val var10000: Any? = `$this$dropFirst_u24lambda_u240`.put(it + fallback, `$completion`)
                  return if (var10000 === IntrinsicsKt.getCOROUTINE_SUSPENDED()) var10000 else Unit.INSTANCE
               }
            })
         }

         return Unit.INSTANCE
      }
   }
}

public fun Source.orElse(fallback: Int): Source {
   return object : Source {
      public override suspend fun drain(sink: Sink) {
         val empty: BooleanRef = BooleanRef()
         empty.element = true
         `$this$orElse`.drain(object : Sink {
            override final fun put(value: Int, `$completion`: Continuation<in Unit>): Any {
               empty.element = false
               val var10000: Any? = sink.put(value, `$completion`)
               return if (var10000 === IntrinsicsKt.getCOROUTINE_SUSPENDED()) var10000 else Unit.INSTANCE
            }
         })
         if (empty.element) {
            sink.put(fallback)
         }

         return Unit.INSTANCE
      }
   }
}
