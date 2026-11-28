package pkg

public fun textBridgeCombiner(): BridgeCombiner<String> {
   return object : BridgeCombiner<String> {
      public override fun zero(): String {
         return ""
      }

      fun twice(x: String): String {
         return BridgeCombiner.DefaultImpls.twice(this, x)
      }
   }
}
