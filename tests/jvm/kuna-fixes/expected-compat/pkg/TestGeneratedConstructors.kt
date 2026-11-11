package pkg

public class TestGeneratedConstructors(size: Int = 4, name: String? = null) {
   public final val size: Int
   public final val name: String?

   init {
      this.size = size
      this.name = name
   }

   public fun make(): Any {
      return TestGeneratedConstructors(0, null, 3, null)
   }

   public fun overloads(): Any {
      return TestGeneratedConstructors.Overloads(1, 0L, false, 6, null)
   }

   fun TestGeneratedConstructors() {
      this(0, null, 3, null)
   }

   public class Overloads @JvmOverloads  public constructor(a: Int, b: Long = 2L, c: Boolean = false) {
      public final val a: Int
      public final val b: Long
      public final val c: Boolean

      init {
         this.a = a
         this.b = b
         this.c = c
      }

      @JvmOverloads
      fun Overloads(a: Int, b: Long) {
         this(a, b, false, 4, null)
      }

      @JvmOverloads
      fun Overloads(a: Int) {
         this(a, 0L, false, 6, null)
      }
   }

   public class Secondary(a: Int, b: Int = 1) {
      public final val a: Int
      public final val b: Int

      init {
         this.a = a
         this.b = b
      }

      public constructor(s: String) : this(s.length(), 0, 2, null)   }
}
