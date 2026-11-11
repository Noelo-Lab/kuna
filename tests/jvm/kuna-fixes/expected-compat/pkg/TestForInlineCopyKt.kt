package pkg

import kotlin.coroutines.Continuation
import kotlin.coroutines.intrinsics.IntrinsicsKt
import kotlin.jvm.internal.InlineMarker

public inline fun <T> InlineFlow<T>.peek(crossinline action: (T, Continuation<Unit>) -> Any?): InlineFlow<T> {
   return object : InlineFlow<Any> {
      public override suspend fun collect(sink: InlineSink<Any>) {
         val var10000: Any = $this$peek.collect(
            object : InlineSink<Any> {
               public override suspend fun emit(value: Any) {
                  // $VF: Couldn't be decompiled
                  // Bytecode:
                  // 00: aload 2
                  // 01: instanceof pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1
                  // 04: ifeq 27
                  // 07: aload 2
                  // 08: checkcast pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1
                  // 0b: astore 4
                  // 0d: aload 4
                  // 0f: getfield pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1.label I
                  // 12: ldc -2147483648
                  // 14: iand
                  // 15: ifeq 27
                  // 18: aload 4
                  // 1a: dup
                  // 1b: getfield pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1.label I
                  // 1e: ldc -2147483648
                  // 20: isub
                  // 21: putfield pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1.label I
                  // 24: goto 32
                  // 27: new pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1
                  // 2a: dup
                  // 2b: aload 0
                  // 2c: aload 2
                  // 2d: invokespecial pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1.<init> (Lpkg/TestForInlineCopyKt$peek$1$collect$2;Lkotlin/coroutines/Continuation;)V
                  // 30: astore 4
                  // 32: aload 4
                  // 34: getfield pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1.result Ljava/lang/Object;
                  // 37: astore 3
                  // 38: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
                  // 3b: astore 5
                  // 3d: aload 4
                  // 3f: getfield pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1.label I
                  // 42: tableswitch 139 0 2 26 69 129
                  // 5c: aload 3
                  // 5d: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
                  // 60: aload 0
                  // 61: getfield pkg/TestForInlineCopyKt$peek$1$collect$2.$action Lkotlin/jvm/functions/Function2;
                  // 64: aload 1
                  // 65: aload 4
                  // 67: aload 4
                  // 69: aload 0
                  // 6a: putfield pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1.L$0 Ljava/lang/Object;
                  // 6d: aload 4
                  // 6f: aload 1
                  // 70: putfield pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1.L$1 Ljava/lang/Object;
                  // 73: aload 4
                  // 75: bipush 1
                  // 76: putfield pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1.label I
                  // 79: invokeinterface kotlin/jvm/functions/Function2.invoke (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object; 3
                  // 7e: dup
                  // 7f: aload 5
                  // 81: if_acmpne 9b
                  // 84: aload 5
                  // 86: areturn
                  // 87: aload 4
                  // 89: getfield pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1.L$1 Ljava/lang/Object;
                  // 8c: astore 1
                  // 8d: aload 4
                  // 8f: getfield pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1.L$0 Ljava/lang/Object;
                  // 92: checkcast pkg/TestForInlineCopyKt$peek$1$collect$2
                  // 95: astore 0
                  // 96: aload 3
                  // 97: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
                  // 9a: aload 3
                  // 9b: pop
                  // 9c: aload 0
                  // 9d: getfield pkg/TestForInlineCopyKt$peek$1$collect$2.$sink Lpkg/InlineSink;
                  // a0: aload 1
                  // a1: aload 4
                  // a3: aload 4
                  // a5: aconst_null
                  // a6: putfield pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1.L$0 Ljava/lang/Object;
                  // a9: aload 4
                  // ab: aconst_null
                  // ac: putfield pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1.L$1 Ljava/lang/Object;
                  // af: aload 4
                  // b1: bipush 2
                  // b2: putfield pkg/TestForInlineCopyKt$peek$1$collect$2$emit$1.label I
                  // b5: invokeinterface pkg/InlineSink.emit (Ljava/lang/Object;Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 3
                  // ba: dup
                  // bb: aload 5
                  // bd: if_acmpne c8
                  // c0: aload 5
                  // c2: areturn
                  // c3: aload 3
                  // c4: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
                  // c7: aload 3
                  // c8: pop
                  // c9: getstatic kotlin/Unit.INSTANCE Lkotlin/Unit;
                  // cc: areturn
                  // cd: new java/lang/IllegalStateException
                  // d0: dup
                  // d1: ldc "call to 'resume' before 'invoke' with coroutine"
                  // d3: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
                  // d6: athrow
               }
            },
            `$completion`
         )
         return if (var10000 === IntrinsicsKt.getCOROUTINE_SUSPENDED()) var10000 else Unit.INSTANCE
      }
   }
}
