package pkg

import kotlin.jvm.internal.Intrinsics

public class TestKtLegacyCheckBranchingDelegation(source: String, boundary: String) {
   public final val source: String
   public final val boundary: String

   init {
      this.source = source
      this.boundary = boundary
   }

   public constructor(response: pkg.TestKtLegacyCheckBranchingDelegation.Response)  {
      val var10001: String = response.body
      val var10002: TestKtLegacyCheckBranchingDelegation.Type? = response.type
      if (var10002 != null) {
         val var2: String? = var10002.parameter("boundary")
         if (var2 != null) {
            this(var10001, var2)
            return
         }
      }

      throw IllegalStateException("no boundary")
   }

   public class Response(body: String, type: pkg.TestKtLegacyCheckBranchingDelegation.Type?) {
      public final val body: String
      public final val type: pkg.TestKtLegacyCheckBranchingDelegation.Type?

      init {
         this.body = body
         this.type = type
      }
   }

   public class Type(params: String?) {
      private final val params: String?

      init {
         this.params = params
      }

      public fun parameter(name: String): String? {
         Intrinsics.checkParameterIsNotNull(name, "name")
         return if (name == "boundary") this.params else null
      }
   }
}
