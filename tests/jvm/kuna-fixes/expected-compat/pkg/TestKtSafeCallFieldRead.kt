package pkg

import org.jetbrains.annotations.Nullable

public class TestKtSafeCallFieldRead {
   @JvmField
   @Nullable
   public final var stream: pkg.TestKtSafeCallFieldRead.Node?
      private set

   @JvmField
   @Nullable
   public final var plain: pkg.TestKtSafeCallFieldRead.Node?
      private set

   private final val fixed: pkg.TestKtSafeCallFieldRead.Node? = TestKtSafeCallFieldRead.Node(3)

   public final var closed: Int
      internal set

   public fun cancel() {
      this.closed = if (this.stream != null) this.stream.close() else -1
   }

   public fun valueOr(): Int {
      return if (this.plain != null) this.plain.value else -1
   }

   public fun orDefault(): pkg.TestKtSafeCallFieldRead.Node {
      var var10000: TestKtSafeCallFieldRead.Node = this.stream
      if (this.stream == null) {
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
