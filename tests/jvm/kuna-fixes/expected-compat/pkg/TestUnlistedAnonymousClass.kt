package pkg

import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestUnlistedAnonymousClass.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestUnlistedAnonymousClass.kt\npkg/TestUnlistedAnonymousClass\n+ 2 TestUnlistedAnonymousClass.kt\npkg/UnlistedQueue\n+ 3 TestUnlistedAnonymousClass.kt\npkg/TestUnlistedAnonymousClassKt\n*L\n1#1,72:1\n16#2,8:73\n30#3,3:81\n48#3:84\n42#3,14:85\n*S KotlinDebug\n*F\n+ 1 TestUnlistedAnonymousClass.kt\npkg/TestUnlistedAnonymousClass\n*L\n61#1:73,8\n66#1:81,3\n68#1:84\n68#1:85,14\n*E\n"])
public class TestUnlistedAnonymousClass(queue: UnlistedQueue) {
   private final val queue: UnlistedQueue

   public final var sent: Int
      internal set

   init {
      this.queue = queue
   }

   public fun send(id: Int, label: String) {
      this.queue.schedule(TestUnlistedAnonymousClass$send$$inlined$execute$default$1(label, true, this, id, label), 5L)
   }

   public fun offset(base: Int): UnlistedHandler {
      return TestUnlistedAnonymousClass$offset$$inlined$unlistedHandler$1(base, this)
   }

   public fun keep(source: UnlistedSource<Int>, limit: Int): UnlistedSource<Int> {
      return TestUnlistedAnonymousClass$keep$$inlined$unlistedTransform$1(source, limit, this)
   }
}
