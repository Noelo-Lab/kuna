package pkg

import kotlin.coroutines.Continuation
import kotlin.coroutines.jvm.internal.ContinuationImpl
import kotlin.coroutines.jvm.internal.DebugMetadata

// $VF: Class flags could not be determined
@DebugMetadata(f = "TestMembersWithoutMetadata.kt", l = [25, 30], i = [0, 0, 0, 0, 1, 1, 1, 1], s = ["L$0", "J$0", "D$0", "F$0", "J$0", "D$0", "F$0", "I$0"], n = ["this", "total", "scale", "ratio", "total", "scale", "ratio", "seen"], m = "next", c = "pkg.TestMembersWithoutMetadataKt$counting$$inlined$feed$1")
internal class `TestMembersWithoutMetadataKt$counting$$inlined$feed$1$1` : ContinuationImpl {
   open int label;
   open Object L$0;
   open long J$0;
   open double D$0;
   open float F$0;
   open int I$0;

   fun `TestMembersWithoutMetadataKt$counting$$inlined$feed$1$1`(`this$0`: TestMembersWithoutMetadataKt$counting$$inlined$feed$1, `$completion`: Continuation) {
      super(`$completion`)
      this.this$0 = `this$0`
   }

   fun invokeSuspend(`$result`: Any) {
      this.result = `$result`
      this.label |= -2147483648
      this.this$0.next(this as Continuation<in Integer>)
   }
}
