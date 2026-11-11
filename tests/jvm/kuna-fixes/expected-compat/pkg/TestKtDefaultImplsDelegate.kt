package pkg

public class TestKtDefaultImplsDelegate {
   public fun total(shapes: List<pkg.TestKtDefaultImplsDelegate.Shape>): Int {
      var sum: Int = 0

      for (s in shapes) {
         sum += s.scaled(2)
      }

      return sum
   }

   public interface Polygon : TestKtDefaultImplsDelegate.Shape {
      public open fun isTriangle(): Boolean {
      }

      // $VF: Class flags could not be determined
      internal class DefaultImpls {
         @JvmStatic
         fun isTriangle(`$this`: TestKtDefaultImplsDelegate.Polygon): Boolean {
            `$this`.getSides() == 3
         }

         @JvmStatic
         fun describe(`$this`: TestKtDefaultImplsDelegate.Polygon): java.lang.String {
            TestKtDefaultImplsDelegate.Shape.DefaultImpls.describe(`$this`)
         }

         @JvmStatic
         fun scaled(`$this`: TestKtDefaultImplsDelegate.Polygon, factor: Int): Int {
            TestKtDefaultImplsDelegate.Shape.DefaultImpls.scaled(`$this`, factor)
         }

         @JvmStatic
         fun greet(`$this`: TestKtDefaultImplsDelegate.Polygon, name: java.lang.String): java.lang.String {
            TestKtDefaultImplsDelegate.Shape.DefaultImpls.greet(`$this`, name)
         }
      }
   }

   public interface Shape {
      public val sides: Int

      public open fun describe(): String {
      }

      public open fun scaled(factor: Int): Int {
      }

      public open fun greet(name: String): String {
      }

      // $VF: Class flags could not be determined
      internal class DefaultImpls {
         @JvmStatic
         fun describe(`$this`: TestKtDefaultImplsDelegate.Shape): java.lang.String {
            "shape with ${`$this`.sides} sides"
         }

         @JvmStatic
         fun scaled(`$this`: TestKtDefaultImplsDelegate.Shape, factor: Int): Int {
            `$this`.sides * factor
         }

         @JvmStatic
         fun greet(`$this`: TestKtDefaultImplsDelegate.Shape, name: java.lang.String): java.lang.String {
            "$name has ${`$this`.sides} sides"
         }
      }
   }

   public class Square : TestKtDefaultImplsDelegate.Polygon {
      public open val sides: Int
         public open get() {
            return 4
         }


      public override fun describe(): String {
         return "square"
      }

      override fun isTriangle(): Boolean {
         TestKtDefaultImplsDelegate.Polygon.DefaultImpls.isTriangle(this)
      }

      override fun scaled(factor: Int): Int {
         TestKtDefaultImplsDelegate.Polygon.DefaultImpls.scaled(this, factor)
      }

      override fun greet(name: java.lang.String): java.lang.String {
         TestKtDefaultImplsDelegate.Polygon.DefaultImpls.greet(this, name)
      }
   }

   public class Tagged(name: String) : TestKtDefaultImplsDelegate.Polygon {
      private final val name: String

      init {
         this.name = name
      }

      public open val sides: Int
         public open get() {
            return this.name.length()
         }


      public override fun scaled(factor: Int): Int {
         return TestKtDefaultImplsDelegate.Polygon.DefaultImpls.scaled(this, factor) + 1
      }

      override fun isTriangle(): Boolean {
         TestKtDefaultImplsDelegate.Polygon.DefaultImpls.isTriangle(this)
      }

      override fun describe(): java.lang.String {
         TestKtDefaultImplsDelegate.Polygon.DefaultImpls.describe(this)
      }

      override fun greet(name: java.lang.String): java.lang.String {
         TestKtDefaultImplsDelegate.Polygon.DefaultImpls.greet(this, name)
      }
   }

   public class Triangle : TestKtDefaultImplsDelegate.Polygon {
      public open val sides: Int
         public open get() {
            return 3
         }


      override fun isTriangle(): Boolean {
         TestKtDefaultImplsDelegate.Polygon.DefaultImpls.isTriangle(this)
      }

      override fun describe(): java.lang.String {
         TestKtDefaultImplsDelegate.Polygon.DefaultImpls.describe(this)
      }

      override fun scaled(factor: Int): Int {
         TestKtDefaultImplsDelegate.Polygon.DefaultImpls.scaled(this, factor)
      }

      override fun greet(name: java.lang.String): java.lang.String {
         TestKtDefaultImplsDelegate.Polygon.DefaultImpls.greet(this, name)
      }
   }
}
