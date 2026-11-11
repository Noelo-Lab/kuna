package pkg

import kotlin.coroutines.Continuation
import kotlin.coroutines.CoroutineContext
import kotlin.coroutines.intrinsics.IntrinsicsKt

public suspend fun relay(src: TailSource): Int {
   return src.next(`$completion`)
}

public suspend fun finish(src: TailSource) {
   val var10000: Any = src.done(`$completion`)
   return if (var10000 === IntrinsicsKt.getCOROUTINE_SUSPENDED()) var10000 else Unit.INSTANCE
}

public suspend fun applyTo(src: TailSource, f: (TailSource, Continuation<Int>) -> Any?): Int {
   return f(src, `$completion`)
}

public suspend fun context(): CoroutineContext {
   return `$completion`.getContext()
}
