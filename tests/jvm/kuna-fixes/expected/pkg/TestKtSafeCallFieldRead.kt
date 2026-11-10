package pkg

import org.jetbrains.annotations.Nullable

public class TestKtSafeCallFieldRead {
   @JvmField
   @Nullable
   @Volatile
   public final var stream: pkg.TestKtSafeCallFieldRead.Node? = null

   @JvmField
   @Nullable
   public final var plain: pkg.TestKtSafeCallFieldRead.Node? = null

   private final val fixed: pkg.TestKtSafeCallFieldRead.Node? = TestKtSafeCallFieldRead.Node(3)
   public final var closed: Int = 0

   public fun cancel() {
      val var10001: TestKtSafeCallFieldRead.Node = this.stream
      this.closed = if (var10001 != null) var10001.close() else -1
   }

   public fun valueOr(): Int {
      val var10000: TestKtSafeCallFieldRead.Node = this.plain
      return if (var10000 != null) var10000.value else -1
   }

   public fun orDefault(): pkg.TestKtSafeCallFieldRead.Node {
      var var10000: TestKtSafeCallFieldRead.Node = this.stream
      if (var10000 == null) {
         var10000 = TestKtSafeCallFieldRead.Node(0)
      }

      return var10000
   }

   public fun finalField(): Int {
      return if (this.fixed != null) this.fixed.value else -1
   }

   public class Node(value: Int) {
      @JvmField
      public final val value: Int

      init {
         this.value = value
      }

      public fun close(): Int {
         return this.value
      }
   }
}
