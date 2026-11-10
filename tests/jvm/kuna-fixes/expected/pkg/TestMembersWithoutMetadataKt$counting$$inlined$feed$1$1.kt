package pkg

import kotlin.coroutines.Continuation
import kotlin.coroutines.jvm.internal.ContinuationImpl
import kotlin.coroutines.jvm.internal.DebugMetadata

// $VF: Class flags could not be determined
@DebugMetadata(f = "TestMembersWithoutMetadata.kt", l = [25, 30], i = [0, 0, 0, 0, 1, 1, 1, 1], s = ["L$0", "J$0", "D$0", "F$0", "J$0", "D$0", "F$0", "I$0"], n = ["this", "total", "scale", "ratio", "total", "scale", "ratio", "seen"], m = "next", c = "pkg.TestMembersWithoutMetadataKt$counting$$inlined$feed$1")
internal class `TestMembersWithoutMetadataKt$counting$$inlined$feed$1$1` : ContinuationImpl {
   @JvmField internal var label: Int = 0
   @JvmField internal var `L$0`: Any? = null
   @JvmField internal var `J$0`: Long = 0L
   @JvmField internal var `D$0`: Double = 0.0
   @JvmField internal var `F$0`: Float = 0.0F
   @JvmField internal var `I$0`: Int = 0

   public constructor(`this$0`: TestMembersWithoutMetadataKt$counting$$inlined$feed$1, `$completion`: Continuation) : super(`$completion`) {
      this.this$0 = `this$0`
   }

   fun invokeSuspend(`$result`: Any): Any {
      this.result = `$result`
      this.label = this.label or -2147483648
      return this.this$0.next(this as Continuation<in Integer>)
   }
}
