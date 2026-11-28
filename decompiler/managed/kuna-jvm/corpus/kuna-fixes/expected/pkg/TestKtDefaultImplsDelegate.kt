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
            return `$this`.sides == 3
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
         fun describe(`$this`: TestKtDefaultImplsDelegate.Shape): String {
            return "shape with ${`$this`.sides} sides"
         }

         @JvmStatic
         fun scaled(`$this`: TestKtDefaultImplsDelegate.Shape, factor: Int): Int {
            return `$this`.sides * factor
         }

         @JvmStatic
         fun greet(`$this`: TestKtDefaultImplsDelegate.Shape, name: String): String {
            return "$name has ${`$this`.sides} sides"
         }
      }
   }

   public class Square : TestKtDefaultImplsDelegate.Polygon {
      public override val sides: Int
         public get() {
            return 4
         }


      public override fun describe(): String {
         return "square"
      }
   }

   public class Tagged(name: String) : TestKtDefaultImplsDelegate.Polygon {
      private final val name: String

      init {
         this.name = name
      }

      public override val sides: Int
         public get() {
            return this.name.length
         }


      public override fun scaled(factor: Int): Int {
         return TestKtDefaultImplsDelegate.Polygon.DefaultImpls.scaled(this, factor) + 1
      }
   }

   public class Triangle : TestKtDefaultImplsDelegate.Polygon {
      public override val sides: Int
         public get() {
            return 3
         }

   }
}
