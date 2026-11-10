package pkg

import org.jetbrains.annotations.NotNull
import org.jetbrains.annotations.Nullable

public class TestKtInlineHeapRead {
   @JvmField
   @Nullable
   public final var state: Any?
      private set

   @JvmField
   @Nullable
   public final var plain: pkg.TestKtInlineHeapRead.Node?
      private set

   @JvmField
   @NotNull
   public final var slots: Array<pkg.TestKtInlineHeapRead.Node?>
      private set

   private final val fixed: pkg.TestKtInlineHeapRead.Node = TestKtInlineHeapRead.Node(7)

   public fun typeTest(): Int {
      return if (this.state is TestKtInlineHeapRead.Node) (this.state as TestKtInlineHeapRead.Node).value else -1
   }

   public fun nonVolatile(): Int {
      return if (this.plain != null) this.plain.value else 0
   }

   public fun element(i: Int): Int {
      return if (this.slots[i] != null) this.slots[i].value else 0
   }

   public fun staticRead(): Int {
      return if (shared != null) shared.value + shared.value else 0
   }

   public fun finalField(): Int {
      return this.fixed.value + this.fixed.value
   }

   public fun safeCast(): pkg.TestKtInlineHeapRead.Node? {
      return this.state as? TestKtInlineHeapRead.Node
   }

   public fun single(): Any? {
      return this.state
   }

   public companion object {
      @JvmField
      @Nullable
      public final var shared: pkg.TestKtInlineHeapRead.Node?
         private set
   }

   public class Node(value: Int) {
      @JvmField
      public final val value: Int

      init {
         this.value = value
      }
   }
}
