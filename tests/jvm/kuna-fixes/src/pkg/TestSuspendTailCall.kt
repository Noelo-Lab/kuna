package pkg

import kotlin.coroutines.CoroutineContext
import kotlin.coroutines.coroutineContext

interface TailSource {
    suspend fun next(): Int
    suspend fun done()
}

suspend fun relay(src: TailSource): Int = src.next()

suspend fun finish(src: TailSource) = src.done()

suspend fun applyTo(src: TailSource, f: suspend (TailSource) -> Int): Int = f(src)

suspend fun context(): CoroutineContext = coroutineContext
