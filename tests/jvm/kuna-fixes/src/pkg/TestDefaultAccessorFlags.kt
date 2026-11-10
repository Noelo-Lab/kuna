package pkg

open class TestDefaultAccessorFlags {
   var plain: Int = 0
   var privateSet: Int = 0
      private set
   @set:JvmSynthetic
   var syntheticSet: Int = 0
   protected var prot: Int = 0
   open var openVar: Int = 0
   lateinit var late: String

   object Counter {
      var count: Long = 0L
   }
}
