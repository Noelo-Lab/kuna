package pkg

public class TestDelegatedMemberDefault {
   public interface Shape {
      public abstract fun area(scale: Int = ..., label: String? = ...): Int {
      }

      public abstract fun name(): String {
      }

      // $VF: Class flags could not be determined
      internal class DefaultImpls
   }

   public class Wrapper(shape: pkg.TestDelegatedMemberDefault.Shape) : TestDelegatedMemberDefault.Shape {
      private final val shape: pkg.TestDelegatedMemberDefault.Shape

      init {
         this.shape = shape
      }

      public override fun area(scale: Int = ..., label: String? = ...): Int {
         return this.shape.area(scale, label)
      }

      public override fun name(): String {
         return this.shape.name()
      }

      public fun twice(): Int {
         return TestDelegatedMemberDefault.Shape.DefaultImpls.area$default(this, 0, null, 3, null) * 2
      }
   }
}
