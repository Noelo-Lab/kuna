package pkg

public class TestDefaultValueCoercionCast {
   public fun none(
      tag: CharSequence = "tag" as java.lang.CharSequence,
      box: pkg.TestDefaultValueCoercionCast.Base? = null,
      base: pkg.TestDefaultValueCoercionCast.Base = TestDefaultValueCoercionCast.Derived() as TestDefaultValueCoercionCast.Base,
      arr: Array<pkg.TestDefaultValueCoercionCast.Base> = arrayOf(TestDefaultValueCoercionCast.Derived())
   ): Int {
      return tag.length() + (if (box == null) 0 else 1) + (if (base is TestDefaultValueCoercionCast.Derived) 2 else 3) + arr.length
   }

   public fun call(action: () -> Unit = { 
         System.out.println("x")
         Unit.INSTANCE
      }, n: Int = 1) {
      action()
      System.out.println(n)
   }

   public open class Base

   public class Derived : TestDefaultValueCoercionCast.Base
}
