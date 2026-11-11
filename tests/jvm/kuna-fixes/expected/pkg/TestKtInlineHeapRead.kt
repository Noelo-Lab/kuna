package pkg

import org.jetbrains.annotations.NotNull
import org.jetbrains.annotations.Nullable

public class TestKtInlineHeapRead {
   @JvmField
   @NotNull
   public final var slots: Array<pkg.TestKtInlineHeapRead.Node?> = arrayOfNulls(2)

   @JvmField
   @NotNull
   @Volatile
   public final var head: pkg.TestKtInlineHeapRead.Node = TestKtInlineHeapRead.Node(0, null)

   @JvmField
   @Volatile
   public final var count: Int = 0

   @JvmField
   @Volatile
   public final var limit: Int = 0

   @JvmField
   @NotNull
   public final var items: Array<Any?> = arrayOfNulls(1)

   private final val fixed: pkg.TestKtInlineHeapRead.Node = TestKtInlineHeapRead.Node(7, null)

   public fun element(i: Int): Int {
      val e: TestKtInlineHeapRead.Node = this.slots[i]
      return if (e != null) e.value else 0
   }

   public fun shared(): Int {
      val s: TestKtInlineHeapRead.Node = cache[0]
      return if (s != null) s.value + s.value else 0
   }

   public fun nextValue(): Int {
      val n: TestKtInlineHeapRead.Node = this.head.next
      return if (n != null) n.value else -1
   }

   public fun room(): Int {
      val r: Int = this.limit - this.count
      return if (r > 0) r else 0
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
      public final val cache: Array<pkg.TestKtInlineHeapRead.Node?> = arrayOfNulls(1)
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
