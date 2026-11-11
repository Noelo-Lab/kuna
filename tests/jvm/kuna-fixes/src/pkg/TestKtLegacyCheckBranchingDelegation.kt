package pkg

class TestKtLegacyCheckBranchingDelegation(val source: String, val boundary: String) {
   class Type(private val params: String?) {
      fun parameter(name: String): String? = if (name == "boundary") params else null
   }

   class Response(val body: String, val type: Type?)

   constructor(response: Response) : this(
      response.body,
      response.type?.parameter("boundary") ?: throw IllegalStateException("no boundary")
   )
}
