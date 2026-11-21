package pkg

class TestKtConstructorAccessorCall private constructor(val name: String, val size: Long) {
   private constructor(name: String) : this(name, -1L)

   fun resized(size: Long): TestKtConstructorAccessorCall = TestKtConstructorAccessorCall(name, size)

   class Factory {
      fun make(name: String): TestKtConstructorAccessorCall = TestKtConstructorAccessorCall(name, 2L)
   }

   companion object {
      fun of(name: String): TestKtConstructorAccessorCall = TestKtConstructorAccessorCall(name)

      fun sized(name: String, size: Long): TestKtConstructorAccessorCall {
         val result = TestKtConstructorAccessorCall(name, size)
         return result
      }
   }
}
