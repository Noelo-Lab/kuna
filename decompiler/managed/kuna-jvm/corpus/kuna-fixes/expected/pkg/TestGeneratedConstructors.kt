package pkg

public class TestGeneratedConstructors(size: Int = 4, name: String? = null) {
   public final val size: Int
   public final val name: String?

   init {
      this.size = size
      this.name = name
   }

   public fun make(): Any {
      return TestGeneratedConstructors()
   }

   public fun overloads(): Any {
      return TestGeneratedConstructors.Overloads(a = 1)
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
   }

   public class Secondary(a: Int, b: Int = 1) {
      public final val a: Int
      public final val b: Int

      init {
         this.a = a
         this.b = b
      }

      public constructor(s: String) : this(a = s.length)
   }
}
