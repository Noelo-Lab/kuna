package pkg

import kotlin.coroutines.CoroutineContext

public suspend fun relay(src: TailSource): Int {
   return src.next()
}

public suspend fun finish(src: TailSource) {
   src.done()
   return Unit.INSTANCE
}

public suspend fun applyTo(src: TailSource, f: suspend (TailSource) -> Int): Int {
   return f(src)
}

public suspend fun context(): CoroutineContext {
   return kotlin.coroutines.coroutineContext
}
