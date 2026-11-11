@file:SourceDebugExtension(["SMAP\nTestCastCapture.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestCastCapture.kt\npkg/TestCastCaptureKt\n*L\n1#1,60:1\n24#1,3:61\n24#1,3:64\n*S KotlinDebug\n*F\n+ 1 TestCastCapture.kt\npkg/TestCastCaptureKt\n*L\n35#1:61,3\n51#1:64,3\n*E\n"])

package pkg

import kotlin.jvm.internal.SourceDebugExtension

public inline fun <T> withHandle(block: (CastCaptureHandle<T>) -> Unit): CastCaptureHandleImpl<T> {
   val impl: CastCaptureHandleImpl<T> = CastCaptureHandleImpl()
   block(impl)
   return impl
}

public suspend fun watch(board: CastCaptureBoard, gate: CastCaptureGate, done: () -> Unit): CastCaptureHandleImpl<Int> {
   try {
      val `impl$iv`: CastCaptureHandleImpl = CastCaptureHandleImpl()
      val handle: CastCaptureHandle<Int> = `impl$iv`
      board.onClose(object : CastCaptureCallback {
         public override fun call(code: Int) {
            handle.fire(code + 1)
         }
      })
      gate.pass()
      return `impl$iv`
   } finally {
      done()
   }
}

public suspend fun register(board: CastCaptureBoard, gate: CastCaptureGate, done: () -> Unit): CastCaptureHandleImpl<String> {
   try {
      val `impl$iv`: CastCaptureHandleImpl = CastCaptureHandleImpl()
      val handle: CastCaptureHandle<String> = `impl$iv`
      board.onError({ code: Int ->
         handle.fire("closed $code")
      } as (Int?) -> Unit)
      gate.pass()
      return `impl$iv`
   } finally {
      done()
   }
}
