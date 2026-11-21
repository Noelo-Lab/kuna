package pkg

public open class TestDefaultArgsSuperCall(a: Int, b: String = "b", c: Long = 3L) {
   public final val a: Int
   public final val b: String
   public final val c: Long

   init {
      this.a = a
      this.b = b
      this.c = c
   }

   public fun twoDefaults(x: Int): Any {
      return object {
         public final val sub: pkg.TestDefaultArgsSuperCall.twoDefaults.1.Sub = Sub()

         /* $VF: local visibility outside of method */ internal inner class Sub : TestDefaultArgsSuperCall(x)
      }
   }

   public fun oneDefault(x: Int, y: String): Any {
      return object {
         public final val sub: pkg.TestDefaultArgsSuperCall.oneDefault.1.Sub = Sub()

         /* $VF: local visibility outside of method */ internal inner class Sub : TestDefaultArgsSuperCall(x, y)
      }
   }

   public fun noDefault(x: Int): Any {
      return object {
         public final val sub: pkg.TestDefaultArgsSuperCall.noDefault.1.Sub = Sub()

         /* $VF: local visibility outside of method */ internal inner class Sub : TestDefaultArgsSuperCall(x, "x", 1L)
      }
   }

   public fun otherConstructor(x: Int): Any {
      return object {
         public final val sub: pkg.TestDefaultArgsSuperCall.otherConstructor.1.Sub = Sub()

         /* $VF: local visibility outside of method */ internal inner class Sub : TestDefaultArgsSuperCall.Overloaded(a = x)
      }
   }

   public open class Overloaded(a: Int, b: String = "b") {
      public final val a: Int
      public final val b: String

      init {
         this.a = a
         this.b = b
      }

      public constructor(n: Int) : this(n, "n")
   }
}
