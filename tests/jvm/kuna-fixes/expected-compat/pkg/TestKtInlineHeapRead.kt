package pkg

import org.jetbrains.annotations.NotNull
import org.jetbrains.annotations.Nullable

public class TestKtInlineHeapRead {
   @JvmField
   @NotNull
   public final var slots: Array<pkg.TestKtInlineHeapRead.Node?>
      private set

   @JvmField
   @NotNull
   public final var head: pkg.TestKtInlineHeapRead.Node = TestKtInlineHeapRead.Node(0, null)
      private set

   @JvmField
   public final var count: Int
      private set

   @JvmField
   public final var limit: Int
      private set

   @JvmField
   @NotNull
   public final var items: Array<Any?>
      private set

   private final val fixed: pkg.TestKtInlineHeapRead.Node = TestKtInlineHeapRead.Node(7, null)

   public fun element(i: Int): Int {
      return if (this.slots[i] != null) this.slots[i].value else 0
   }

   public fun shared(): Int {
      return if (cache[0] != null) cache[0].value + cache[0].value else 0
   }

   public fun nextValue(): Int {
      return if (this.head.next != null) this.head.next.value else -1
   }

   public fun room(): Int {
      return if (this.limit - this.count > 0) this.limit - this.count else 0
   }

   public fun cast(): pkg.TestKtInlineHeapRead.Node? {
      return this.items[0] as? TestKtInlineHeapRead.Node
   }

   public fun finalField(): Int {
      return if (this.fixed.next != null) this.fixed.next.value else 0
   }

   public fun single(i: Int): pkg.TestKtInlineHeapRead.Node? {
      return this.slots[i]
   }

   public companion object {
      @JvmField
      @NotNull
      public final val cache: Array<pkg.TestKtInlineHeapRead.Node?>
   }

   public class Node(value: Int, next: pkg.TestKtInlineHeapRead.Node?) {
      @JvmField
      public final val value: Int

      @JvmField
      @Nullable
      public final val next: pkg.TestKtInlineHeapRead.Node?

      init {
         this.value = value
         this.next = next
      }
   }
}
