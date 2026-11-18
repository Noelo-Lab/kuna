package pkg

import kotlin.coroutines.Continuation

public fun twice(src: LambdaSource): (Continuation<Int>) -> Any? {
   return {
      // $VF: Could not decompile lambda - root function was not found. Is this a suspend lambda?
   } as (Continuation<in Integer>?) -> Any
}

public fun plain(x: Int): (Continuation<Int>) -> Any? {
   return {
      // $VF: Could not decompile lambda - root function was not found. Is this a suspend lambda?
   } as (Continuation<in Integer>?) -> Any
}

public fun sumOf(src: LambdaSource, n: Int): (Continuation<Int>) -> Any? {
   return {
      // $VF: Could not decompile lambda - root function was not found. Is this a suspend lambda?
   } as (Continuation<in Integer>?) -> Any
}

public fun lengthPlus(src: LambdaSource): (String, Continuation<Int>) -> Any? {
   return {
      // $VF: Could not decompile lambda - root function was not found. Is this a suspend lambda?
   } as (java.lang.String?, Continuation<in Integer>?) -> Any
}

public fun pump(src: LambdaSource): (LambdaSink<Int>, Int, Continuation<Unit>) -> Any? {
   return {
      // $VF: Could not decompile lambda - root function was not found. Is this a suspend lambda?
   } as (LambdaSink<Integer>?, Int?, Continuation<in Unit>?) -> Any
}

public fun guarded(src: LambdaSource): (Continuation<String>) -> Any? {
   return {
      // $VF: Could not decompile lambda - root function was not found. Is this a suspend lambda?
   } as (Continuation<? super java.lang.String>?) -> Any
}

public fun scaled(mode: LambdaMode, src: LambdaSource): (Continuation<Int>) -> Any? {
   return {
      // $VF: Could not decompile lambda - root function was not found. Is this a suspend lambda?
   } as (Continuation<in Integer>?) -> Any
}

public fun firstPositive(src: LambdaSource): (Continuation<Int>) -> Any? {
   return {
      // $VF: Could not decompile lambda - root function was not found. Is this a suspend lambda?
   } as (Continuation<in Integer>?) -> Any
}

public suspend fun runTwice(block: (Continuation<Unit>) -> Any?) {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 1
   // 01: instanceof pkg/TestSuspendLambdaKt$runTwice$1
   // 04: ifeq 24
   // 07: aload 1
   // 08: checkcast pkg/TestSuspendLambdaKt$runTwice$1
   // 0b: astore 3
   // 0c: aload 3
   // 0d: getfield pkg/TestSuspendLambdaKt$runTwice$1.label I
   // 10: ldc -2147483648
   // 12: iand
   // 13: ifeq 24
   // 16: aload 3
   // 17: dup
   // 18: getfield pkg/TestSuspendLambdaKt$runTwice$1.label I
   // 1b: ldc -2147483648
   // 1d: isub
   // 1e: putfield pkg/TestSuspendLambdaKt$runTwice$1.label I
   // 21: goto 2d
   // 24: new pkg/TestSuspendLambdaKt$runTwice$1
   // 27: dup
   // 28: aload 1
   // 29: invokespecial pkg/TestSuspendLambdaKt$runTwice$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 2c: astore 3
   // 2d: aload 3
   // 2e: getfield pkg/TestSuspendLambdaKt$runTwice$1.result Ljava/lang/Object;
   // 31: astore 2
   // 32: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 35: astore 4
   // 37: aload 3
   // 38: getfield pkg/TestSuspendLambdaKt$runTwice$1.label I
   // 3b: tableswitch 105 0 2 25 55 95
   // 54: aload 2
   // 55: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 58: aload 0
   // 59: aload 3
   // 5a: aload 3
   // 5b: aload 0
   // 5c: putfield pkg/TestSuspendLambdaKt$runTwice$1.L$0 Ljava/lang/Object;
   // 5f: aload 3
   // 60: bipush 1
   // 61: putfield pkg/TestSuspendLambdaKt$runTwice$1.label I
   // 64: invokeinterface kotlin/jvm/functions/Function1.invoke (Ljava/lang/Object;)Ljava/lang/Object; 2
   // 69: dup
   // 6a: aload 4
   // 6c: if_acmpne 7f
   // 6f: aload 4
   // 71: areturn
   // 72: aload 3
   // 73: getfield pkg/TestSuspendLambdaKt$runTwice$1.L$0 Ljava/lang/Object;
   // 76: checkcast kotlin/jvm/functions/Function1
   // 79: astore 0
   // 7a: aload 2
   // 7b: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 7e: aload 2
   // 7f: pop
   // 80: aload 0
   // 81: aload 3
   // 82: aload 3
   // 83: aconst_null
   // 84: putfield pkg/TestSuspendLambdaKt$runTwice$1.L$0 Ljava/lang/Object;
   // 87: aload 3
   // 88: bipush 2
   // 89: putfield pkg/TestSuspendLambdaKt$runTwice$1.label I
   // 8c: invokeinterface kotlin/jvm/functions/Function1.invoke (Ljava/lang/Object;)Ljava/lang/Object; 2
   // 91: dup
   // 92: aload 4
   // 94: if_acmpne 9f
   // 97: aload 4
   // 99: areturn
   // 9a: aload 2
   // 9b: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 9e: aload 2
   // 9f: pop
   // a0: getstatic kotlin/Unit.INSTANCE Lkotlin/Unit;
   // a3: areturn
   // a4: new java/lang/IllegalStateException
   // a7: dup
   // a8: ldc "call to 'resume' before 'invoke' with coroutine"
   // aa: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // ad: athrow
}

public fun nested(src: LambdaSource): (LambdaSink<Int>, Continuation<Unit>) -> Any? {
   return {
      // $VF: Could not decompile lambda - root function was not found. Is this a suspend lambda?
   } as (LambdaSink<Integer>?, Continuation<in Unit>?) -> Any
}

private suspend fun helper(src: LambdaSource): Int {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 1
   // 01: instanceof pkg/TestSuspendLambdaKt$helper$1
   // 04: ifeq 24
   // 07: aload 1
   // 08: checkcast pkg/TestSuspendLambdaKt$helper$1
   // 0b: astore 3
   // 0c: aload 3
   // 0d: getfield pkg/TestSuspendLambdaKt$helper$1.label I
   // 10: ldc -2147483648
   // 12: iand
   // 13: ifeq 24
   // 16: aload 3
   // 17: dup
   // 18: getfield pkg/TestSuspendLambdaKt$helper$1.label I
   // 1b: ldc -2147483648
   // 1d: isub
   // 1e: putfield pkg/TestSuspendLambdaKt$helper$1.label I
   // 21: goto 2d
   // 24: new pkg/TestSuspendLambdaKt$helper$1
   // 27: dup
   // 28: aload 1
   // 29: invokespecial pkg/TestSuspendLambdaKt$helper$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 2c: astore 3
   // 2d: aload 3
   // 2e: getfield pkg/TestSuspendLambdaKt$helper$1.result Ljava/lang/Object;
   // 31: astore 2
   // 32: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 35: astore 4
   // 37: aload 3
   // 38: getfield pkg/TestSuspendLambdaKt$helper$1.label I
   // 3b: tableswitch 63 0 1 21 46
   // 50: aload 2
   // 51: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 54: aload 0
   // 55: aload 3
   // 56: aload 3
   // 57: bipush 1
   // 58: putfield pkg/TestSuspendLambdaKt$helper$1.label I
   // 5b: invokeinterface pkg/LambdaSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 60: dup
   // 61: aload 4
   // 63: if_acmpne 6e
   // 66: aload 4
   // 68: areturn
   // 69: aload 2
   // 6a: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 6d: aload 2
   // 6e: checkcast java/lang/Number
   // 71: invokevirtual java/lang/Number.intValue ()I
   // 74: bipush 1
   // 75: iadd
   // 76: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
   // 79: areturn
   // 7a: new java/lang/IllegalStateException
   // 7d: dup
   // 7e: ldc "call to 'resume' before 'invoke' with coroutine"
   // 80: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // 83: athrow
}

public fun viaHelper(src: LambdaSource): (Continuation<Int>) -> Any? {
   return {
      // $VF: Could not decompile lambda - root function was not found. Is this a suspend lambda?
   } as (Continuation<in Integer>?) -> Any
}
