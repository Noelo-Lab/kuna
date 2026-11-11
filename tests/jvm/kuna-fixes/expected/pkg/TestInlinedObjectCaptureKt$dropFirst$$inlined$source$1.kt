@file:SourceDebugExtension(["SMAP\nTestInlinedObjectCapture.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestInlinedObjectCapture.kt\npkg/TestInlinedObjectCaptureKt\n*L\n1#1,37:1\n11#1,5:38\n11#1,5:43\n*S KotlinDebug\n*F\n+ 1 TestInlinedObjectCapture.kt\npkg/TestInlinedObjectCaptureKt\n*L\n17#1:38,5\n29#1:43,5\n*E\n"])

package pkg

import kotlin.jvm.internal.SourceDebugExtension
import kotlin.jvm.internal.Ref.BooleanRef
import kotlin.jvm.internal.Ref.IntRef

public inline fun source(crossinline block: suspend (Sink) -> Unit): Source {
   return object : Source {
      public override suspend fun drain(sink: Sink) {
         block(sink)
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
            override final suspend fun put(value: Int) {
               empty.element = false
               if (skipped.element >= count) {
                  `$this$dropFirst_u24lambda_u240`.put(value)
                  return Unit.INSTANCE
               } else {
                  val var3: Int = skipped.element++
                  return Unit.INSTANCE
               }
            }
         })
         if (empty.element) {
            `$this$dropFirst`.drain(object : Sink {
               override final suspend fun put(it: Int) {
                  `$this$dropFirst_u24lambda_u240`.put(it + fallback)
                  return Unit.INSTANCE
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
            override final suspend fun put(value: Int) {
               empty.element = false
               sink.put(value)
               return Unit.INSTANCE
            }
         })
         if (empty.element) {
            sink.put(fallback)
         }

         return Unit.INSTANCE
      }
   }
}
