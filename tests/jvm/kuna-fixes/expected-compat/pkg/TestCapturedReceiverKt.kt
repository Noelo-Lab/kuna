package pkg

public operator fun Array<String>.iterator(): Iterator<String> {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 0
   // 01: ldc "<this>"
   // 03: invokestatic kotlin/jvm/internal/Intrinsics.checkNotNullParameter (Ljava/lang/Object;Ljava/lang/String;)V
   // 06: new pkg/TestCapturedReceiverKt$iterator$1
   // 09: dup
   // 0a: aload 0
   // 0b: invokespecial pkg/TestCapturedReceiverKt$iterator$1.<init> ([Ljava/lang/String;)V
   // 0e: checkcast java/util/Iterator
   // 11: areturn
}
