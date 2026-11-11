package pkg

import kotlin.coroutines.intrinsics.IntrinsicsKt
import kotlin.coroutines.jvm.internal.Boxing
import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestUnlistedAnonymousClass.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestUnlistedAnonymousClass.kt\npkg/TestUnlistedAnonymousClass\n+ 2 TestUnlistedAnonymousClass.kt\npkg/UnlistedQueue\n+ 3 TestUnlistedAnonymousClass.kt\npkg/TestUnlistedAnonymousClassKt\n*L\n1#1,72:1\n16#2,8:73\n30#3,3:81\n48#3:84\n42#3,14:85\n*S KotlinDebug\n*F\n+ 1 TestUnlistedAnonymousClass.kt\npkg/TestUnlistedAnonymousClass\n*L\n61#1:73,8\n66#1:81,3\n68#1:84\n68#1:85,14\n*E\n"])
public class TestUnlistedAnonymousClass(queue: UnlistedQueue) {
   private final val queue: UnlistedQueue
   public final var sent: Int = 0

   init {
      this.queue = queue
   }

   public fun send(id: Int, label: String) {
      val `cancelable$iv`: Boolean = true
      this.queue.schedule(object : UnlistedTask(label, `cancelable$iv`) {
         public override fun runOnce(): Long {
            this@TestUnlistedAnonymousClass.sent = this@TestUnlistedAnonymousClass.sent + id + label.length
            return -1L
         }
      }, 5L)
   }

   public fun offset(base: Int): UnlistedHandler {
      return object : UnlistedHandler {
         public override fun handle(code: Int): Int {
            return code * base + this@TestUnlistedAnonymousClass.sent + 1
         }
      }
   }

   public fun keep(source: UnlistedSource<Int>, limit: Int): UnlistedSource<Int> {
      val `$this$unlistedTransform$iv`: UnlistedSource = source
      return object : UnlistedSource<Any> {
         public override suspend fun drain(sink: UnlistedSink<Any>) {
            val `$this$unlistedTransform_u24lambda_u240`: UnlistedSink = sink
            val out: UnlistedSink = `$this$unlistedTransform_u24lambda_u240`
            val var10000: Any? = `$this$unlistedTransform$iv`.drain(object : UnlistedSink<Any> {
               public override suspend fun put(value: Any) {
                  val var10000: UnlistedSink = out
                  val valuex: Int = (value as Number).toInt()
                  if (valuex < limit) {
                     val var7: Any? = var10000.put(Boxing.boxInt(valuex + var10/* $VF was: TestUnlistedAnonymousClass.this */.sent), `$completion`)
                     if (var7 === IntrinsicsKt.getCOROUTINE_SUSPENDED()) {
                        return var7
                     }
                  }

                  return Unit.INSTANCE
               }
            }, `$completion`)
            return if (var10000 === IntrinsicsKt.getCOROUTINE_SUSPENDED()) var10000 else Unit.INSTANCE
         }
      }
   }
}
