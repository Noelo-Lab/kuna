package pkg

public class TestKtConstructorAccessorCall private constructor(name: String, size: Long) {
   public final val name: String
   public final val size: Long

   init {
      this.name = name
      this.size = size
   }

   private constructor(name: String) : this(name, -1L)

   public fun resized(size: Long): TestKtConstructorAccessorCall {
      return TestKtConstructorAccessorCall(this.name, size)
   }

   public companion object {
      public fun of(name: String): TestKtConstructorAccessorCall {
         return TestKtConstructorAccessorCall(name)
      }

      public fun sized(name: String, size: Long): TestKtConstructorAccessorCall {
         return TestKtConstructorAccessorCall(name, size)
      }
   }

   public class Factory {
      public fun make(name: String): TestKtConstructorAccessorCall {
         return TestKtConstructorAccessorCall(name, 2L)
      }
   }
}
