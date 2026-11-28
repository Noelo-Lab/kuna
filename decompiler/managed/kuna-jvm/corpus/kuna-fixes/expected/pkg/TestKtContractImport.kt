package pkg

import kotlin.contracts.ExperimentalContracts
import kotlin.contracts.InvocationKind
import kotlin.contracts.contract
import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestKtContractImport.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestKtContractImport.kt\npkg/TestKtContractImport\n*L\n1#1,26:1\n14#1:27\n*S KotlinDebug\n*F\n+ 1 TestKtContractImport.kt\npkg/TestKtContractImport\n*L\n24#1:27\n*E\n"])
public class TestKtContractImport {
   @OptIn(ExperimentalContracts::class)
   public inline fun <R> once(block: () -> R): R {
      contract {
         callsInPlace(block, InvocationKind.EXACTLY_ONCE)
      }

      return block() as R
   }

   @OptIn(ExperimentalContracts::class)
   public fun isText(x: Any?): Boolean {
      contract {
         returns(true) implies (x is String)
      }

      return x is String
   }

   public fun twice(n: Int): Int {
      return n * 2
   }
}
