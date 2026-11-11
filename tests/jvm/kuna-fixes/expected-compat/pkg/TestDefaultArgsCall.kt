package pkg

public class TestDefaultArgsCall(base: Int) {
   public final val base: Int

   init {
      this.base = base
   }

   public fun scale(x: Int = 1, y: Boolean = false, z: Char = 122): Int {
      return if (y) x * this.base else (if (z == 113) x else x + 1)
   }

   public fun same(a: Int): Int {
      return a
   }

   public fun same(a: Int, b: Int = 0): Int {
      return a + b
   }

   public fun trailing(): String {
      return TestDefaultArgsCallKt.join$default(this.base, 0, null, 6, null)
   }

   public fun middle(): String {
      return TestDefaultArgsCallKt.join$default(this.base, 0, "m", 2, null)
   }

   public fun all(): String {
      return TestDefaultArgsCallKt.join(this.base, 5, "all")
   }

   public fun overloads(s: String): Long {
      return TestDefaultArgsCallKt.pick$default(this.base, 0L, 2, null) + TestDefaultArgsCallKt.pick$default(s, 0L, 2, null)
   }

   public fun extension(s: String): String {
      return TestDefaultArgsCallKt.twice$default(s, 0, 1, null)
   }

   public fun member(other: TestDefaultArgsCall): Int {
      return scale$default(other, 0, false, (char)0, 7, null)
         + scale$default(this, this.base, false, (char)0, 6, null)
         + scale$default(other, 0, true, (char)0, 5, null)
         + scale$default(this, 3, false, (char)113, 2, null)
      }

   public fun companion(): TestDefaultArgsCall {
      return TestDefaultArgsCall.Companion.make$default(Companion, 0, null, 3, null)
   }

   public fun named(): TestDefaultArgsCall {
      return TestDefaultArgsCall.Companion.make$default(Companion, 0, "x", 1, null)
   }

   public fun iface(s: pkg.TestDefaultArgsCall.Shape): Int {
      return TestDefaultArgsCall.Shape.DefaultImpls.area$default(s, 0, 1, null)
   }

   public fun boxes(): Int {
      return TestDefaultArgsCall.Box(this.base, 0, null, 6, null).h
         + TestDefaultArgsCall.Box(this.base, 0, "t", 2, null).w
         + TestDefaultArgsCall.Box(2, 3, null, 4, null).h
      }

   public fun nested(): String {
      return TestDefaultArgsCallKt.join$default(
         TestDefaultArgsCallKt.join$default(this.base, 0, null, 6, null).length(), 0, TestDefaultArgsCallKt.join$default(2, 3, null, 4, null), 2, null
      )
   }

   public class Box(w: Int, h: Int = 1, tag: String = "box") {
      public final val w: Int
      public final val h: Int
      public final val tag: String

      init {
         this.w = w
         this.h = h
         this.tag = tag
      }
   }

   public companion object {
      public fun make(size: Int = 4, label: String? = null): TestDefaultArgsCall {
         return TestDefaultArgsCall(size)
      }
   }

   public interface Shape {
      public abstract fun area(scale: Int = ...): Int {
      }

      // $VF: Class flags could not be determined
      internal class DefaultImpls
   }
}
