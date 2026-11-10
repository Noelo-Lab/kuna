package pkg

public open class TestSuperPropertyAccess {
   public open var count: Int = 0
      internal set

   public open val isActive: Boolean
      public get() {
         return this.count > 0
      }


   public class Child : TestSuperPropertyAccess() {
      public override val isActive: Boolean
         public get() {
            return super.isActive && this.count < 10
         }


      public override var count: Int
         public get() {
            return super.count + 1
         }

         public set(value) {
            super.count = value * 2
         }

   }
}
