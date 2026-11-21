package pkg

import kotlin.coroutines.Continuation

public suspend fun sumOf(src: SuspendSource, n: Int): Int {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 2
   // 01: instanceof pkg/TestSuspendContinuationKt$sumOf$1
   // 04: ifeq 27
   // 07: aload 2
   // 08: checkcast pkg/TestSuspendContinuationKt$sumOf$1
   // 0b: astore 8
   // 0d: aload 8
   // 0f: getfield pkg/TestSuspendContinuationKt$sumOf$1.label I
   // 12: ldc -2147483648
   // 14: iand
   // 15: ifeq 27
   // 18: aload 8
   // 1a: dup
   // 1b: getfield pkg/TestSuspendContinuationKt$sumOf$1.label I
   // 1e: ldc -2147483648
   // 20: isub
   // 21: putfield pkg/TestSuspendContinuationKt$sumOf$1.label I
   // 24: goto 31
   // 27: new pkg/TestSuspendContinuationKt$sumOf$1
   // 2a: dup
   // 2b: aload 2
   // 2c: invokespecial pkg/TestSuspendContinuationKt$sumOf$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 2f: astore 8
   // 31: aload 8
   // 33: getfield pkg/TestSuspendContinuationKt$sumOf$1.result Ljava/lang/Object;
   // 36: astore 7
   // 38: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 3b: astore 9
   // 3d: aload 8
   // 3f: getfield pkg/TestSuspendContinuationKt$sumOf$1.label I
   // 42: tableswitch 151 0 1 22 90
   // 58: aload 7
   // 5a: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 5d: bipush 0
   // 5e: istore 3
   // 5f: bipush 0
   // 60: istore 4
   // 62: iload 4
   // 64: iload 1
   // 65: if_icmpge d4
   // 68: iload 3
   // 69: istore 5
   // 6b: aload 0
   // 6c: aload 8
   // 6e: aload 8
   // 70: aload 0
   // 71: putfield pkg/TestSuspendContinuationKt$sumOf$1.L$0 Ljava/lang/Object;
   // 74: aload 8
   // 76: iload 1
   // 77: putfield pkg/TestSuspendContinuationKt$sumOf$1.I$0 I
   // 7a: aload 8
   // 7c: iload 4
   // 7e: putfield pkg/TestSuspendContinuationKt$sumOf$1.I$1 I
   // 81: aload 8
   // 83: iload 5
   // 85: putfield pkg/TestSuspendContinuationKt$sumOf$1.I$2 I
   // 88: aload 8
   // 8a: bipush 1
   // 8b: putfield pkg/TestSuspendContinuationKt$sumOf$1.label I
   // 8e: invokeinterface pkg/SuspendSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 93: dup
   // 94: aload 9
   // 96: if_acmpne c0
   // 99: aload 9
   // 9b: areturn
   // 9c: aload 8
   // 9e: getfield pkg/TestSuspendContinuationKt$sumOf$1.I$2 I
   // a1: istore 5
   // a3: aload 8
   // a5: getfield pkg/TestSuspendContinuationKt$sumOf$1.I$1 I
   // a8: istore 4
   // aa: aload 8
   // ac: getfield pkg/TestSuspendContinuationKt$sumOf$1.I$0 I
   // af: istore 1
   // b0: aload 8
   // b2: getfield pkg/TestSuspendContinuationKt$sumOf$1.L$0 Ljava/lang/Object;
   // b5: checkcast pkg/SuspendSource
   // b8: astore 0
   // b9: aload 7
   // bb: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // be: aload 7
   // c0: astore 6
   // c2: iload 5
   // c4: aload 6
   // c6: checkcast java/lang/Number
   // c9: invokevirtual java/lang/Number.intValue ()I
   // cc: iadd
   // cd: istore 3
   // ce: iinc 4 1
   // d1: goto 62
   // d4: iload 3
   // d5: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
   // d8: areturn
   // d9: new java/lang/IllegalStateException
   // dc: dup
   // dd: ldc "call to 'resume' before 'invoke' with coroutine"
   // df: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // e2: athrow
}

public suspend fun firstPositive(src: SuspendSource, tries: Int): Int? {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 2
   // 01: instanceof pkg/TestSuspendContinuationKt$firstPositive$1
   // 04: ifeq 27
   // 07: aload 2
   // 08: checkcast pkg/TestSuspendContinuationKt$firstPositive$1
   // 0b: astore 6
   // 0d: aload 6
   // 0f: getfield pkg/TestSuspendContinuationKt$firstPositive$1.label I
   // 12: ldc -2147483648
   // 14: iand
   // 15: ifeq 27
   // 18: aload 6
   // 1a: dup
   // 1b: getfield pkg/TestSuspendContinuationKt$firstPositive$1.label I
   // 1e: ldc -2147483648
   // 20: isub
   // 21: putfield pkg/TestSuspendContinuationKt$firstPositive$1.label I
   // 24: goto 31
   // 27: new pkg/TestSuspendContinuationKt$firstPositive$1
   // 2a: dup
   // 2b: aload 2
   // 2c: invokespecial pkg/TestSuspendContinuationKt$firstPositive$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 2f: astore 6
   // 31: aload 6
   // 33: getfield pkg/TestSuspendContinuationKt$firstPositive$1.result Ljava/lang/Object;
   // 36: astore 5
   // 38: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 3b: astore 7
   // 3d: aload 6
   // 3f: getfield pkg/TestSuspendContinuationKt$firstPositive$1.label I
   // 42: tableswitch 117 0 1 22 68
   // 58: aload 5
   // 5a: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 5d: iload 1
   // 5e: istore 3
   // 5f: iload 3
   // 60: ifle b5
   // 63: aload 0
   // 64: aload 6
   // 66: aload 6
   // 68: aload 0
   // 69: putfield pkg/TestSuspendContinuationKt$firstPositive$1.L$0 Ljava/lang/Object;
   // 6c: aload 6
   // 6e: iload 3
   // 6f: putfield pkg/TestSuspendContinuationKt$firstPositive$1.I$0 I
   // 72: aload 6
   // 74: bipush 1
   // 75: putfield pkg/TestSuspendContinuationKt$firstPositive$1.label I
   // 78: invokeinterface pkg/SuspendSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 7d: dup
   // 7e: aload 7
   // 80: if_acmpne 9c
   // 83: aload 7
   // 85: areturn
   // 86: aload 6
   // 88: getfield pkg/TestSuspendContinuationKt$firstPositive$1.I$0 I
   // 8b: istore 3
   // 8c: aload 6
   // 8e: getfield pkg/TestSuspendContinuationKt$firstPositive$1.L$0 Ljava/lang/Object;
   // 91: checkcast pkg/SuspendSource
   // 94: astore 0
   // 95: aload 5
   // 97: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 9a: aload 5
   // 9c: checkcast java/lang/Number
   // 9f: invokevirtual java/lang/Number.intValue ()I
   // a2: istore 4
   // a4: iload 4
   // a6: ifle af
   // a9: iload 4
   // ab: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
   // ae: areturn
   // af: iinc 3 -1
   // b2: goto 5f
   // b5: aconst_null
   // b6: areturn
   // b7: new java/lang/IllegalStateException
   // ba: dup
   // bb: ldc "call to 'resume' before 'invoke' with coroutine"
   // bd: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // c0: athrow
}

public fun summing(src: SuspendSource): SuspendSink {
   return object : SuspendSink {
      public final var sum: Long
         internal set

      public override suspend fun put(v: Int) {
         // $VF: Couldn't be decompiled
         // Bytecode:
         // 00: aload 2
         // 01: instanceof pkg/TestSuspendContinuationKt$summing$1$put$1
         // 04: ifeq 27
         // 07: aload 2
         // 08: checkcast pkg/TestSuspendContinuationKt$summing$1$put$1
         // 0b: astore 9
         // 0d: aload 9
         // 0f: getfield pkg/TestSuspendContinuationKt$summing$1$put$1.label I
         // 12: ldc -2147483648
         // 14: iand
         // 15: ifeq 27
         // 18: aload 9
         // 1a: dup
         // 1b: getfield pkg/TestSuspendContinuationKt$summing$1$put$1.label I
         // 1e: ldc -2147483648
         // 20: isub
         // 21: putfield pkg/TestSuspendContinuationKt$summing$1$put$1.label I
         // 24: goto 32
         // 27: new pkg/TestSuspendContinuationKt$summing$1$put$1
         // 2a: dup
         // 2b: aload 0
         // 2c: aload 2
         // 2d: invokespecial pkg/TestSuspendContinuationKt$summing$1$put$1.<init> (Lpkg/TestSuspendContinuationKt$summing$1;Lkotlin/coroutines/Continuation;)V
         // 30: astore 9
         // 32: aload 9
         // 34: getfield pkg/TestSuspendContinuationKt$summing$1$put$1.result Ljava/lang/Object;
         // 37: astore 8
         // 39: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
         // 3c: astore 10
         // 3e: aload 9
         // 40: getfield pkg/TestSuspendContinuationKt$summing$1$put$1.label I
         // 43: tableswitch 138 0 1 21 83
         // 58: aload 8
         // 5a: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
         // 5d: aload 0
         // 5e: aload 0
         // 5f: getfield pkg/TestSuspendContinuationKt$summing$1.sum J
         // 62: iload 1
         // 63: istore 6
         // 65: lstore 4
         // 67: astore 3
         // 68: aload 0
         // 69: getfield pkg/TestSuspendContinuationKt$summing$1.$src Lpkg/SuspendSource;
         // 6c: aload 9
         // 6e: aload 9
         // 70: aload 3
         // 71: putfield pkg/TestSuspendContinuationKt$summing$1$put$1.L$0 Ljava/lang/Object;
         // 74: aload 9
         // 76: lload 4
         // 78: putfield pkg/TestSuspendContinuationKt$summing$1$put$1.J$0 J
         // 7b: aload 9
         // 7d: iload 6
         // 7f: putfield pkg/TestSuspendContinuationKt$summing$1$put$1.I$0 I
         // 82: aload 9
         // 84: bipush 1
         // 85: putfield pkg/TestSuspendContinuationKt$summing$1$put$1.label I
         // 88: invokeinterface pkg/SuspendSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
         // 8d: dup
         // 8e: aload 10
         // 90: if_acmpne b4
         // 93: aload 10
         // 95: areturn
         // 96: aload 9
         // 98: getfield pkg/TestSuspendContinuationKt$summing$1$put$1.I$0 I
         // 9b: istore 6
         // 9d: aload 9
         // 9f: getfield pkg/TestSuspendContinuationKt$summing$1$put$1.J$0 J
         // a2: lstore 4
         // a4: aload 9
         // a6: getfield pkg/TestSuspendContinuationKt$summing$1$put$1.L$0 Ljava/lang/Object;
         // a9: checkcast pkg/TestSuspendContinuationKt$summing$1
         // ac: astore 3
         // ad: aload 8
         // af: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
         // b2: aload 8
         // b4: astore 7
         // b6: aload 3
         // b7: lload 4
         // b9: iload 6
         // bb: aload 7
         // bd: checkcast java/lang/Number
         // c0: invokevirtual java/lang/Number.intValue ()I
         // c3: iadd
         // c4: i2l
         // c5: ladd
         // c6: putfield pkg/TestSuspendContinuationKt$summing$1.sum J
         // c9: getstatic kotlin/Unit.INSTANCE Lkotlin/Unit;
         // cc: areturn
         // cd: new java/lang/IllegalStateException
         // d0: dup
         // d1: ldc "call to 'resume' before 'invoke' with coroutine"
         // d3: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
         // d6: athrow
      }
   }
}

public suspend fun SequenceScope<Int>.yieldTwice(x: Int) {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 2
   // 01: instanceof pkg/TestSuspendContinuationKt$yieldTwice$1
   // 04: ifeq 27
   // 07: aload 2
   // 08: checkcast pkg/TestSuspendContinuationKt$yieldTwice$1
   // 0b: astore 4
   // 0d: aload 4
   // 0f: getfield pkg/TestSuspendContinuationKt$yieldTwice$1.label I
   // 12: ldc -2147483648
   // 14: iand
   // 15: ifeq 27
   // 18: aload 4
   // 1a: dup
   // 1b: getfield pkg/TestSuspendContinuationKt$yieldTwice$1.label I
   // 1e: ldc -2147483648
   // 20: isub
   // 21: putfield pkg/TestSuspendContinuationKt$yieldTwice$1.label I
   // 24: goto 31
   // 27: new pkg/TestSuspendContinuationKt$yieldTwice$1
   // 2a: dup
   // 2b: aload 2
   // 2c: invokespecial pkg/TestSuspendContinuationKt$yieldTwice$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 2f: astore 4
   // 31: aload 4
   // 33: getfield pkg/TestSuspendContinuationKt$yieldTwice$1.result Ljava/lang/Object;
   // 36: astore 3
   // 37: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 3a: astore 5
   // 3c: aload 4
   // 3e: getfield pkg/TestSuspendContinuationKt$yieldTwice$1.label I
   // 41: tableswitch 132 0 2 27 68 122
   // 5c: aload 3
   // 5d: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 60: aload 0
   // 61: iload 1
   // 62: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
   // 65: aload 4
   // 67: aload 4
   // 69: aload 0
   // 6a: putfield pkg/TestSuspendContinuationKt$yieldTwice$1.L$0 Ljava/lang/Object;
   // 6d: aload 4
   // 6f: iload 1
   // 70: putfield pkg/TestSuspendContinuationKt$yieldTwice$1.I$0 I
   // 73: aload 4
   // 75: bipush 1
   // 76: putfield pkg/TestSuspendContinuationKt$yieldTwice$1.label I
   // 79: invokevirtual kotlin/sequences/SequenceScope.yield (Ljava/lang/Object;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;
   // 7c: dup
   // 7d: aload 5
   // 7f: if_acmpne 99
   // 82: aload 5
   // 84: areturn
   // 85: aload 4
   // 87: getfield pkg/TestSuspendContinuationKt$yieldTwice$1.I$0 I
   // 8a: istore 1
   // 8b: aload 4
   // 8d: getfield pkg/TestSuspendContinuationKt$yieldTwice$1.L$0 Ljava/lang/Object;
   // 90: checkcast kotlin/sequences/SequenceScope
   // 93: astore 0
   // 94: aload 3
   // 95: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 98: aload 3
   // 99: pop
   // 9a: aload 0
   // 9b: iload 1
   // 9c: bipush 1
   // 9d: iadd
   // 9e: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
   // a1: aload 4
   // a3: aload 4
   // a5: aconst_null
   // a6: putfield pkg/TestSuspendContinuationKt$yieldTwice$1.L$0 Ljava/lang/Object;
   // a9: aload 4
   // ab: bipush 2
   // ac: putfield pkg/TestSuspendContinuationKt$yieldTwice$1.label I
   // af: invokevirtual kotlin/sequences/SequenceScope.yield (Ljava/lang/Object;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;
   // b2: dup
   // b3: aload 5
   // b5: if_acmpne c0
   // b8: aload 5
   // ba: areturn
   // bb: aload 3
   // bc: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // bf: aload 3
   // c0: pop
   // c1: getstatic kotlin/Unit.INSTANCE Lkotlin/Unit;
   // c4: areturn
   // c5: new java/lang/IllegalStateException
   // c8: dup
   // c9: ldc "call to 'resume' before 'invoke' with coroutine"
   // cb: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // ce: athrow
}
