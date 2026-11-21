@file:SourceDebugExtension(["SMAP\nTestCastCapture.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestCastCapture.kt\npkg/TestCastCaptureKt\n*L\n1#1,60:1\n24#1,3:61\n24#1,3:64\n*S KotlinDebug\n*F\n+ 1 TestCastCapture.kt\npkg/TestCastCaptureKt\n*L\n35#1:61,3\n51#1:64,3\n*E\n"])

package pkg

import kotlin.jvm.internal.SourceDebugExtension

public inline fun <T> withHandle(block: (CastCaptureHandle<T>) -> Unit): CastCaptureHandleImpl<T> {
   val impl: CastCaptureHandleImpl = CastCaptureHandleImpl()
   block(impl)
   return impl
}

public suspend fun watch(board: CastCaptureBoard, gate: CastCaptureGate, done: () -> Unit): CastCaptureHandleImpl<Int> {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 3
   // 01: instanceof pkg/TestCastCaptureKt$watch$1
   // 04: ifeq 27
   // 07: aload 3
   // 08: checkcast pkg/TestCastCaptureKt$watch$1
   // 0b: astore 10
   // 0d: aload 10
   // 0f: getfield pkg/TestCastCaptureKt$watch$1.label I
   // 12: ldc -2147483648
   // 14: iand
   // 15: ifeq 27
   // 18: aload 10
   // 1a: dup
   // 1b: getfield pkg/TestCastCaptureKt$watch$1.label I
   // 1e: ldc -2147483648
   // 20: isub
   // 21: putfield pkg/TestCastCaptureKt$watch$1.label I
   // 24: goto 31
   // 27: new pkg/TestCastCaptureKt$watch$1
   // 2a: dup
   // 2b: aload 3
   // 2c: invokespecial pkg/TestCastCaptureKt$watch$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 2f: astore 10
   // 31: aload 10
   // 33: getfield pkg/TestCastCaptureKt$watch$1.result Ljava/lang/Object;
   // 36: astore 9
   // 38: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 3b: astore 11
   // 3d: aload 10
   // 3f: getfield pkg/TestCastCaptureKt$watch$1.label I
   // 42: tableswitch 163 0 1 22 109
   // 58: aload 9
   // 5a: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 5d: nop
   // 5e: bipush 0
   // 5f: istore 5
   // 61: new pkg/CastCaptureHandleImpl
   // 64: dup
   // 65: invokespecial pkg/CastCaptureHandleImpl.<init> ()V
   // 68: astore 6
   // 6a: aload 6
   // 6c: checkcast pkg/CastCaptureHandle
   // 6f: astore 7
   // 71: bipush 0
   // 72: istore 8
   // 74: aload 0
   // 75: new pkg/TestCastCaptureKt$watch$impl$1$1
   // 78: dup
   // 79: aload 7
   // 7b: invokespecial pkg/TestCastCaptureKt$watch$impl$1$1.<init> (Lpkg/CastCaptureHandle;)V
   // 7e: checkcast pkg/CastCaptureCallback
   // 81: invokeinterface pkg/CastCaptureBoard.onClose (Lpkg/CastCaptureCallback;)V 2
   // 86: nop
   // 87: aload 6
   // 89: astore 4
   // 8b: aload 1
   // 8c: aload 10
   // 8e: aload 10
   // 90: aload 2
   // 91: putfield pkg/TestCastCaptureKt$watch$1.L$0 Ljava/lang/Object;
   // 94: aload 10
   // 96: aload 4
   // 98: putfield pkg/TestCastCaptureKt$watch$1.L$1 Ljava/lang/Object;
   // 9b: aload 10
   // 9d: bipush 1
   // 9e: putfield pkg/TestCastCaptureKt$watch$1.label I
   // a1: invokeinterface pkg/CastCaptureGate.pass (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // a6: dup
   // a7: aload 11
   // a9: if_acmpne ca
   // ac: aload 11
   // ae: areturn
   // af: aload 10
   // b1: getfield pkg/TestCastCaptureKt$watch$1.L$1 Ljava/lang/Object;
   // b4: checkcast pkg/CastCaptureHandleImpl
   // b7: astore 4
   // b9: aload 10
   // bb: getfield pkg/TestCastCaptureKt$watch$1.L$0 Ljava/lang/Object;
   // be: checkcast kotlin/jvm/functions/Function0
   // c1: astore 2
   // c2: nop
   // c3: aload 9
   // c5: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // c8: aload 9
   // ca: pop
   // cb: aload 4
   // cd: astore 5
   // cf: aload 2
   // d0: invokeinterface kotlin/jvm/functions/Function0.invoke ()Ljava/lang/Object; 1
   // d5: pop
   // d6: aload 5
   // d8: areturn
   // d9: astore 5
   // db: aload 2
   // dc: invokeinterface kotlin/jvm/functions/Function0.invoke ()Ljava/lang/Object; 1
   // e1: pop
   // e2: aload 5
   // e4: athrow
   // e5: new java/lang/IllegalStateException
   // e8: dup
   // e9: ldc "call to 'resume' before 'invoke' with coroutine"
   // eb: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // ee: athrow
}

public suspend fun register(board: CastCaptureBoard, gate: CastCaptureGate, done: () -> Unit): CastCaptureHandleImpl<String> {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 3
   // 01: instanceof pkg/TestCastCaptureKt$register$1
   // 04: ifeq 27
   // 07: aload 3
   // 08: checkcast pkg/TestCastCaptureKt$register$1
   // 0b: astore 10
   // 0d: aload 10
   // 0f: getfield pkg/TestCastCaptureKt$register$1.label I
   // 12: ldc -2147483648
   // 14: iand
   // 15: ifeq 27
   // 18: aload 10
   // 1a: dup
   // 1b: getfield pkg/TestCastCaptureKt$register$1.label I
   // 1e: ldc -2147483648
   // 20: isub
   // 21: putfield pkg/TestCastCaptureKt$register$1.label I
   // 24: goto 31
   // 27: new pkg/TestCastCaptureKt$register$1
   // 2a: dup
   // 2b: aload 3
   // 2c: invokespecial pkg/TestCastCaptureKt$register$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 2f: astore 10
   // 31: aload 10
   // 33: getfield pkg/TestCastCaptureKt$register$1.result Ljava/lang/Object;
   // 36: astore 9
   // 38: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 3b: astore 11
   // 3d: aload 10
   // 3f: getfield pkg/TestCastCaptureKt$register$1.label I
   // 42: tableswitch 163 0 1 22 109
   // 58: aload 9
   // 5a: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 5d: nop
   // 5e: bipush 0
   // 5f: istore 5
   // 61: new pkg/CastCaptureHandleImpl
   // 64: dup
   // 65: invokespecial pkg/CastCaptureHandleImpl.<init> ()V
   // 68: astore 6
   // 6a: aload 6
   // 6c: checkcast pkg/CastCaptureHandle
   // 6f: astore 7
   // 71: bipush 0
   // 72: istore 8
   // 74: aload 0
   // 75: new pkg/TestCastCaptureKt$register$impl$1$1
   // 78: dup
   // 79: aload 7
   // 7b: invokespecial pkg/TestCastCaptureKt$register$impl$1$1.<init> (Lpkg/CastCaptureHandle;)V
   // 7e: checkcast kotlin/jvm/functions/Function1
   // 81: invokeinterface pkg/CastCaptureBoard.onError (Lkotlin/jvm/functions/Function1;)V 2
   // 86: nop
   // 87: aload 6
   // 89: astore 4
   // 8b: aload 1
   // 8c: aload 10
   // 8e: aload 10
   // 90: aload 2
   // 91: putfield pkg/TestCastCaptureKt$register$1.L$0 Ljava/lang/Object;
   // 94: aload 10
   // 96: aload 4
   // 98: putfield pkg/TestCastCaptureKt$register$1.L$1 Ljava/lang/Object;
   // 9b: aload 10
   // 9d: bipush 1
   // 9e: putfield pkg/TestCastCaptureKt$register$1.label I
   // a1: invokeinterface pkg/CastCaptureGate.pass (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // a6: dup
   // a7: aload 11
   // a9: if_acmpne ca
   // ac: aload 11
   // ae: areturn
   // af: aload 10
   // b1: getfield pkg/TestCastCaptureKt$register$1.L$1 Ljava/lang/Object;
   // b4: checkcast pkg/CastCaptureHandleImpl
   // b7: astore 4
   // b9: aload 10
   // bb: getfield pkg/TestCastCaptureKt$register$1.L$0 Ljava/lang/Object;
   // be: checkcast kotlin/jvm/functions/Function0
   // c1: astore 2
   // c2: nop
   // c3: aload 9
   // c5: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // c8: aload 9
   // ca: pop
   // cb: aload 4
   // cd: astore 5
   // cf: aload 2
   // d0: invokeinterface kotlin/jvm/functions/Function0.invoke ()Ljava/lang/Object; 1
   // d5: pop
   // d6: aload 5
   // d8: areturn
   // d9: astore 5
   // db: aload 2
   // dc: invokeinterface kotlin/jvm/functions/Function0.invoke ()Ljava/lang/Object; 1
   // e1: pop
   // e2: aload 5
   // e4: athrow
   // e5: new java/lang/IllegalStateException
   // e8: dup
   // e9: ldc "call to 'resume' before 'invoke' with coroutine"
   // eb: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // ee: athrow
}
