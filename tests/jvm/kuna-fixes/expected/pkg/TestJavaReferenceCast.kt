package pkg

public class TestJavaReferenceCast<V> private constructor(values: Array<V>?, count: Long) {
   private final var values: Array<V>?
   private final var count: Long

   init {
      this.values = values as Array<V>?
      this.count = count
   }

   public constructor() : this(null, 0L)

   public fun first(): V? {
      val var10000: Array<Any> = this.values
      return (if (var10000 != null) var10000[0] else null) as V
   }

   public fun limited(): Long {
      return TestJavaReferenceCast.Overloads.least(32.toLong(), this.count)
   }

   public fun none(): Int {
      return TestJavaReferenceCast.Overloads.name(null as String?)
   }

   public fun <T> arrayOrNull(size: Int): Array<T?> {
      return arrayOfNulls(size) as Array<T>
   }

   public fun <T> boxed(element: T): pkg.TestJavaReferenceCast.Box<T> {
      return TestJavaReferenceCast.Overloads.one(element) as TestJavaReferenceCast.Box<T>
   }

   public class Box<T>(value: T) {
      public final val value: T

      init {
         this.value = value as T
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
         return TestJavaReferenceCast.Box(e) as TestJavaReferenceCast.Box<T>
      }
   }
}
