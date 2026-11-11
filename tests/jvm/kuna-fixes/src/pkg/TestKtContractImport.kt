@file:OptIn(ExperimentalContracts::class)

package pkg

import kotlin.contracts.ExperimentalContracts
import kotlin.contracts.InvocationKind
import kotlin.contracts.contract

class TestKtContractImport {
    inline fun <R> once(block: () -> R): R {
        contract {
            callsInPlace(block, InvocationKind.EXACTLY_ONCE)
        }
        return block()
    }

    fun isText(x: Any?): Boolean {
        contract {
            returns(true) implies (x is String)
        }
        return x is String
    }

    fun twice(n: Int): Int = once { n * 2 }
}
