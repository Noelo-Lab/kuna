package pkg

class TestKtEmptyPrimaryConstructor constructor() {
   private var executor: Any? = null
   var calls = 0

   constructor(executor: Any) : this() {
      this.executor = executor
   }

   class Builder() {
      var name: String = "x"

      internal constructor(other: TestKtEmptyPrimaryConstructor) : this() {
         name = other.calls.toString()
      }
   }
}
