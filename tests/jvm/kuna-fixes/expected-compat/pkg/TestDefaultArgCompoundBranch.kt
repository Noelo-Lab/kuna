package pkg

public class TestDefaultArgCompoundBranch {
   public fun make(names: List<String>): Any {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 0: aload 1
      // 1: ldc "names"
      // 3: invokestatic kotlin/jvm/internal/Intrinsics.checkNotNullParameter (Ljava/lang/Object;Ljava/lang/String;)V
      // 6: new pkg/TestDefaultArgCompoundBranch$make$1
      // 9: dup
      // a: aload 1
      // b: invokespecial pkg/TestDefaultArgCompoundBranch$make$1.<init> (Ljava/util/List;)V
      // e: areturn
   }
}
