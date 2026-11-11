package pkg

public suspend fun forward(f: SamIntFlow, out: SamIntSink) {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 0
   // 01: new pkg/TestSuspendSamKt$forward$2
   // 04: dup
   // 05: aload 1
   // 06: invokespecial pkg/TestSuspendSamKt$forward$2.<init> (Lpkg/SamIntSink;)V
   // 09: checkcast pkg/SamIntSink
   // 0c: aload 2
   // 0d: invokeinterface pkg/SamIntFlow.collect (Lpkg/SamIntSink;Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 3
   // 12: dup
   // 13: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 16: if_acmpne 1a
   // 19: areturn
   // 1a: pop
   // 1b: getstatic kotlin/Unit.INSTANCE Lkotlin/Unit;
   // 1e: areturn
}

public suspend fun relay(f: SamIntFlow, out: SamIntSink) {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 0
   // 01: new pkg/TestSuspendSamKt$relay$2
   // 04: dup
   // 05: aload 1
   // 06: invokespecial pkg/TestSuspendSamKt$relay$2.<init> (Lpkg/SamIntSink;)V
   // 09: checkcast pkg/SamIntSink
   // 0c: aload 2
   // 0d: invokeinterface pkg/SamIntFlow.collect (Lpkg/SamIntSink;Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 3
   // 12: dup
   // 13: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 16: if_acmpne 1a
   // 19: areturn
   // 1a: pop
   // 1b: getstatic kotlin/Unit.INSTANCE Lkotlin/Unit;
   // 1e: areturn
}

public suspend fun lengths(f: SamFlow<String>, out: SamSink<Int>) {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 0
   // 01: new pkg/TestSuspendSamKt$lengths$2
   // 04: dup
   // 05: aload 1
   // 06: invokespecial pkg/TestSuspendSamKt$lengths$2.<init> (Lpkg/SamSink;)V
   // 09: checkcast pkg/SamSink
   // 0c: aload 2
   // 0d: invokeinterface pkg/SamFlow.collect (Lpkg/SamSink;Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 3
   // 12: dup
   // 13: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 16: if_acmpne 1a
   // 19: areturn
   // 1a: pop
   // 1b: getstatic kotlin/Unit.INSTANCE Lkotlin/Unit;
   // 1e: areturn
}
