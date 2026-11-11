@file:SourceDebugExtension(["SMAP\nTestInlinedObjectCapture.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestInlinedObjectCapture.kt\npkg/TestInlinedObjectCaptureKt\n*L\n1#1,37:1\n11#1,5:38\n11#1,5:43\n*S KotlinDebug\n*F\n+ 1 TestInlinedObjectCapture.kt\npkg/TestInlinedObjectCaptureKt\n*L\n17#1:38,5\n29#1:43,5\n*E\n"])

package pkg

import kotlin.coroutines.Continuation
import kotlin.coroutines.intrinsics.IntrinsicsKt
import kotlin.coroutines.jvm.internal.ContinuationImpl
import kotlin.jvm.internal.InlineMarker
import kotlin.jvm.internal.SourceDebugExtension
import kotlin.jvm.internal.Ref.BooleanRef
import kotlin.jvm.internal.Ref.IntRef

public inline fun source(crossinline block: (Sink, Continuation<Unit>) -> Any?): Source {
   return object : Source {
      public override suspend fun drain(sink: Sink) {
         val var10000: Any = block(sink, `$completion`)
         return if (var10000 === IntrinsicsKt.getCOROUTINE_SUSPENDED()) var10000 else Unit.INSTANCE
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
         var `$continuation`: Continuation
         run label48@{
            if (`$completion` is <unrepresentable>) {
               `$continuation` = `$completion` as <unrepresentable>
               if ((`$continuation`.label and -2147483648) != 0) {
                  `$continuation`.label -= -2147483648
                  return@label48
               }
            }

            `$continuation` = object : ContinuationImpl(`$completion`) {
               @JvmField internal var label: Int = 0
               @JvmField internal var `L$0`: Any? = null
               @JvmField internal var `L$1`: Any? = null
               @JvmField internal var `L$2`: Any? = null

               fun invokeSuspend(`$result`: Any): Any {
                  this.result = `$result`
                  this.label = this.label or -2147483648
                  return drain(null, this as Continuation<in Unit>)
               }
            }
         }

         val `$result`: Any = `$continuation`.result
         val var5: Any = IntrinsicsKt.getCOROUTINE_SUSPENDED()
         val `$this$dropFirst_u24lambda_u240`: Sink
         val empty: BooleanRef
         when (`$continuation`.label) {
            0 -> {
               ResultKt.throwOnFailure(`$result`)
               `$this$dropFirst_u24lambda_u240` = sink
               val skipped: IntRef = IntRef()
               empty = BooleanRef()
               empty.element = true
               val var10000: Source = `$this$dropFirst`
               val var10001: Sink = object : Sink {
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
               }
               `$continuation`.`L$0` = this
               `$continuation`.`L$1` = `$this$dropFirst_u24lambda_u240`
               `$continuation`.`L$2` = empty
               `$continuation`.label = 1
               if (var10000.drain(var10001, `$continuation`) === var5) {
                  return var5
               }
            }
            1 -> {
               empty = `$continuation`.`L$2` as BooleanRef
               `$this$dropFirst_u24lambda_u240` = `$continuation`.`L$1` as Sink
               this = `$continuation`.`L$0` as <unrepresentable>
               ResultKt.throwOnFailure(`$result`)
            }
            2 -> {
               ResultKt.throwOnFailure(`$result`)
               return Unit.INSTANCE
            }
            else -> throw IllegalStateException("call to 'resume' before 'invoke' with coroutine")
         }

         if (empty.element) {
            val var12: Source = `$this$dropFirst`
            val var13: Sink = object : Sink {
               override final fun put(it: Int, `$completion`: Continuation<in Unit>): Any {
                  val var10000: Any? = `$this$dropFirst_u24lambda_u240`.put(it + fallback, `$completion`)
                  return if (var10000 === IntrinsicsKt.getCOROUTINE_SUSPENDED()) var10000 else Unit.INSTANCE
               }
            }
            `$continuation`.`L$0` = null
            `$continuation`.`L$1` = null
            `$continuation`.`L$2` = null
            `$continuation`.label = 2
            if (var12.drain(var13, `$continuation`) === var5) {
               return var5
            }
         }

         return Unit.INSTANCE
      }
   }
}

public fun Source.orElse(fallback: Int): Source {
   return object : Source {
      public override suspend fun drain(sink: Sink) {
         var `$continuation`: Continuation
         run label48@{
            if (`$completion` is <unrepresentable>) {
               `$continuation` = `$completion` as <unrepresentable>
               if ((`$continuation`.label and -2147483648) != 0) {
                  `$continuation`.label -= -2147483648
                  return@label48
               }
            }

            `$continuation` = object : ContinuationImpl(`$completion`) {
               @JvmField internal var label: Int = 0
               @JvmField internal var `L$0`: Any? = null
               @JvmField internal var `L$1`: Any? = null
               @JvmField internal var `L$2`: Any? = null

               fun invokeSuspend(`$result`: Any): Any {
                  this.result = `$result`
                  this.label = this.label or -2147483648
                  return drain(null, this as Continuation<in Unit>)
               }
            }
         }

         val `$result`: Any = `$continuation`.result
         val var5: Any = IntrinsicsKt.getCOROUTINE_SUSPENDED()
         val `$this$orElse_u24lambda_u241`: Sink
         val empty: BooleanRef
         when (`$continuation`.label) {
            0 -> {
               ResultKt.throwOnFailure(`$result`)
               `$this$orElse_u24lambda_u241` = sink
               empty = BooleanRef()
               empty.element = true
               val var10000: Source = `$this$orElse`
               val var10001: Sink = object : Sink {
                  override final fun put(value: Int, `$completion`: Continuation<in Unit>): Any {
                     empty.element = false
                     val var10000: Any? = `$this$orElse_u24lambda_u241`.put(value, `$completion`)
                     return if (var10000 === IntrinsicsKt.getCOROUTINE_SUSPENDED()) var10000 else Unit.INSTANCE
                  }
               }
               `$continuation`.`L$0` = this
               `$continuation`.`L$1` = `$this$orElse_u24lambda_u241`
               `$continuation`.`L$2` = empty
               `$continuation`.label = 1
               if (var10000.drain(var10001, `$continuation`) === var5) {
                  return var5
               }
            }
            1 -> {
               empty = `$continuation`.`L$2` as BooleanRef
               `$this$orElse_u24lambda_u241` = `$continuation`.`L$1` as Sink
               this = `$continuation`.`L$0` as <unrepresentable>
               ResultKt.throwOnFailure(`$result`)
            }
            2 -> {
               ResultKt.throwOnFailure(`$result`)
               return Unit.INSTANCE
            }
            else -> throw IllegalStateException("call to 'resume' before 'invoke' with coroutine")
         }

         if (empty.element) {
            val var11: Int = fallback
            `$continuation`.`L$0` = null
            `$continuation`.`L$1` = null
            `$continuation`.`L$2` = null
            `$continuation`.label = 2
            if (`$this$orElse_u24lambda_u241`.put(var11, `$continuation`) === var5) {
               return var5
            }
         }

         return Unit.INSTANCE
      }
   }
}
