package pkg

public open class TestSuperPropertyAccess {
   public open var count: Int
      internal final set

   public open val isActive: Boolean
      public open get() {
         return this.count > 0
      }


   public class Child : TestSuperPropertyAccess {
      public open val isActive: Boolean
         public open get() {
            return this.isActive && this.count < 10
         }


      public open var count: Int
         public open get() {
            return this.count + 1
         }

         public open set(value) {
            this.count = value * 2
         }

   }
}
