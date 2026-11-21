package pkg

public class TestJavaReferenceCast<V> private constructor(vararg values: Any, count: Long) {
   private final var values: Array<Any>?
   private final var count: Long

   init {
      this.values = (V[])values
      this.count = count
   }

   public constructor() : this(null, 0L)
   public fun first(): Any? {
      return if (this.values != null) this.values[0] else null
   }

   public fun limited(): Long {
      return TestJavaReferenceCast.Overloads.least((long)32, this.count)
   }

   public fun none(): Int {
      return TestJavaReferenceCast.Overloads.name((java.lang.String)null)
   }

   public fun <T> arrayOrNull(size: Int): Array<T?> {
      return (T[])arrayOfNulls(size)
   }

   public fun <T> boxed(element: T): pkg.TestJavaReferenceCast.Box<T> {
      return (TestJavaReferenceCast.Box<T>)TestJavaReferenceCast.Overloads.one(element)
   }

   public class Box<T>(value: Any) {
      public final val value: Any

      init {
         this.value = (T)value
      }
   }

   public object Overloads {
      @JvmStatic
      public fun least(a: Long, b: Long): Long {
         return if (a < b) a else b
      }

      @JvmStatic
      public fun name(s: String?): Int {
         return 1
      }

      @JvmStatic
      public fun name(s: pkg.TestJavaReferenceCast.Box<String>?): Int {
         return 2
      }

      @JvmStatic
      public fun <T> one(e: T): pkg.TestJavaReferenceCast.Box<T> {
         return (TestJavaReferenceCast.Box<T>)TestJavaReferenceCast.Box<>(e)
      }
   }
}
