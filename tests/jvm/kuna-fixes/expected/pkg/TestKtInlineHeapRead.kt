package pkg

import org.jetbrains.annotations.NotNull
import org.jetbrains.annotations.Nullable

public class TestKtInlineHeapRead {
   @JvmField
   @Nullable
   @Volatile
   public final var state: Any? = null

   @JvmField
   @Nullable
   public final var plain: pkg.TestKtInlineHeapRead.Node? = null

   @JvmField
   @NotNull
   public final var slots: Array<pkg.TestKtInlineHeapRead.Node?> = arrayOfNulls(2)

   private final val fixed: pkg.TestKtInlineHeapRead.Node = TestKtInlineHeapRead.Node(7)

   public fun typeTest(): Int {
      val s: Any = this.state
      return if (s is TestKtInlineHeapRead.Node) (s as TestKtInlineHeapRead.Node).value else -1
   }

   public fun nonVolatile(): Int {
      val p: TestKtInlineHeapRead.Node = this.plain
      return if (p != null) p.value else 0
   }

   public fun element(i: Int): Int {
      val e: TestKtInlineHeapRead.Node = this.slots[i]
      return if (e != null) e.value else 0
   }

   public fun staticRead(): Int {
      val s: TestKtInlineHeapRead.Node = shared
      return if (s != null) s.value + s.value else 0
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
      @Volatile
      public final var shared: pkg.TestKtInlineHeapRead.Node? = null
   }

   public class Node(value: Int) {
      @JvmField
      public final val value: Int

      init {
         this.value = value
      }
   }
}
