package pkg

public class TestDefaultArgsCall(base: Int) {
   public final val base: Int

   init {
      this.base = base
   }

   public fun scale(x: Int = 1, y: Boolean = false, z: Char = 122): Int {
      return if (y) x * this.base else (if (z.code == 113) x else x + 1)
   }

   public fun same(a: Int): Int {
      return a
   }

   public fun same(a: Int, b: Int = 0): Int {
      return a + b
   }

   public fun trailing(): String {
      return TestDefaultArgsCallKt.join(this.base)
   }

   public fun middle(): String {
      return TestDefaultArgsCallKt.join(a = this.base, c = "m")
   }

   public fun all(): String {
      return TestDefaultArgsCallKt.join(this.base, 5, "all")
   }

   public fun overloads(s: String): Long {
      return TestDefaultArgsCallKt.pick(this.base) + TestDefaultArgsCallKt.pick(s)
   }

   public fun extension(s: String): String {
      return TestDefaultArgsCallKt.twice(s)
   }

   public fun member(other: TestDefaultArgsCall): Int {
      return other.scale() + this.scale(this.base) + other.scale(y = true) + this.scale(x = 3, z = 113.toChar())
   }

   public fun companion(): TestDefaultArgsCall {
      return Companion.make()
   }

   public fun named(): TestDefaultArgsCall {
      return Companion.make(label = "x")
   }

   public fun iface(s: pkg.TestDefaultArgsCall.Shape): Int {
      return s.area()
   }

   public fun boxes(): Int {
      return TestDefaultArgsCall.Box(this.base).h + TestDefaultArgsCall.Box(w = this.base, tag = "t").w + TestDefaultArgsCall.Box(2, 3).h
   }

   public fun nested(): String {
      return TestDefaultArgsCallKt.join(a = TestDefaultArgsCallKt.join(this.base).length(), c = TestDefaultArgsCallKt.join(2, 3))
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
