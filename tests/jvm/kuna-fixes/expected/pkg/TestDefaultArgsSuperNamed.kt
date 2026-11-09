package pkg

public open class TestDefaultArgsSuperNamed(a: Int, b: String = "b", c: Long = 3L) {
   public final val a: Int
   public final val b: String
   public final val c: Long

   init {
      this.a = a
      this.b = b
      this.c = c
   }

   public fun skipsMiddle(x: Int): Any {
      return object {
         public final val sub: pkg.TestDefaultArgsSuperNamed.skipsMiddle.1.Sub = Sub()

         /* $VF: local visibility outside of method */ internal inner class Sub : TestDefaultArgsSuperNamed(a = x, c = 9L)
      }
   }

   public fun innerSuper(z: Int): Any {
      return object {
         public final val sub: pkg.TestDefaultArgsSuperNamed.innerSuper.1.Sub = Sub()

         /* $VF: local visibility outside of method */ internal inner class Sub : TestDefaultArgsSuperNamed.Inner(z)
      }
   }

   public fun picksPrimary(x: Int): Any {
      return object {
         public final val sub: pkg.TestDefaultArgsSuperNamed.picksPrimary.1.Sub = Sub()

         /* $VF: local visibility outside of method */ internal inner class Sub : TestDefaultArgsSuperNamed.Overloaded(a = x)
      }
   }

   public fun picksSecondary(x: Int): Any {
      return object {
         public final val sub: pkg.TestDefaultArgsSuperNamed.picksSecondary.1.Sub = Sub()

         /* $VF: local visibility outside of method */ internal inner class Sub : TestDefaultArgsSuperNamed.Overloaded(x)
      }
   }

   public open inner class Inner(p: Int, q: String = "q") {
      public final val p: Int
      public final val q: String

      init {
         this.p = p
         this.q = q
      }
   }

   public open class Overloaded(a: Int, b: String = "b") {
      public final val a: Int
      public final val b: String

      init {
         this.a = a
         this.b = b
      }

      public constructor(n: Int) : this(n, "n")   }
}
