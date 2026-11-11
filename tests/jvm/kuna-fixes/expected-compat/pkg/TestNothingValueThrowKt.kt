package pkg

import kotlin.coroutines.Continuation

public fun fail(message: String): Nothing {
   throw IllegalStateException(message)
}

private fun unsupported(): Nothing {
   throw UnsupportedOperationException("unsupported")
}

public fun pick(x: Int): Int {
   if (x < 0) {
      fail("negative")
      throw KotlinNothingValueException()
   } else {
      return x
   }
}

public fun pickOrFail(x: Int): Int {
   if (x > 0) {
      return x
   } else {
      fail("not positive")
      throw KotlinNothingValueException()
   }
}

public fun name(x: Int): String {
   var var10000: java.lang.String
   when (x) {
      0 -> var10000 = "zero"
      1 -> var10000 = "one"
      else -> {
         fail("other $x")
         throw KotlinNothingValueException()
      }
   }

   return var10000
}

public suspend fun nextOrNever(src: NothingSource): Int {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 1
   // 01: instanceof pkg/TestNothingValueThrowKt$nextOrNever$1
   // 04: ifeq 27
   // 07: aload 1
   // 08: checkcast pkg/TestNothingValueThrowKt$nextOrNever$1
   // 0b: astore 4
   // 0d: aload 4
   // 0f: getfield pkg/TestNothingValueThrowKt$nextOrNever$1.label I
   // 12: ldc -2147483648
   // 14: iand
   // 15: ifeq 27
   // 18: aload 4
   // 1a: dup
   // 1b: getfield pkg/TestNothingValueThrowKt$nextOrNever$1.label I
   // 1e: ldc -2147483648
   // 20: isub
   // 21: putfield pkg/TestNothingValueThrowKt$nextOrNever$1.label I
   // 24: goto 31
   // 27: new pkg/TestNothingValueThrowKt$nextOrNever$1
   // 2a: dup
   // 2b: aload 1
   // 2c: invokespecial pkg/TestNothingValueThrowKt$nextOrNever$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 2f: astore 4
   // 31: aload 4
   // 33: getfield pkg/TestNothingValueThrowKt$nextOrNever$1.result Ljava/lang/Object;
   // 36: astore 3
   // 37: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 3a: astore 5
   // 3c: aload 4
   // 3e: getfield pkg/TestNothingValueThrowKt$nextOrNever$1.label I
   // 41: tableswitch 133 0 2 27 60 114
   // 5c: aload 3
   // 5d: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 60: aload 0
   // 61: aload 4
   // 63: aload 4
   // 65: aload 0
   // 66: putfield pkg/TestNothingValueThrowKt$nextOrNever$1.L$0 Ljava/lang/Object;
   // 69: aload 4
   // 6b: bipush 1
   // 6c: putfield pkg/TestNothingValueThrowKt$nextOrNever$1.label I
   // 6f: invokeinterface pkg/NothingSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 74: dup
   // 75: aload 5
   // 77: if_acmpne 8b
   // 7a: aload 5
   // 7c: areturn
   // 7d: aload 4
   // 7f: getfield pkg/TestNothingValueThrowKt$nextOrNever$1.L$0 Ljava/lang/Object;
   // 82: checkcast pkg/NothingSource
   // 85: astore 0
   // 86: aload 3
   // 87: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 8a: aload 3
   // 8b: checkcast java/lang/Number
   // 8e: invokevirtual java/lang/Number.intValue ()I
   // 91: istore 2
   // 92: iload 2
   // 93: ifge c1
   // 96: aload 0
   // 97: aload 4
   // 99: aload 4
   // 9b: aconst_null
   // 9c: putfield pkg/TestNothingValueThrowKt$nextOrNever$1.L$0 Ljava/lang/Object;
   // 9f: aload 4
   // a1: bipush 2
   // a2: putfield pkg/TestNothingValueThrowKt$nextOrNever$1.label I
   // a5: invokeinterface pkg/NothingSource.never (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // aa: dup
   // ab: aload 5
   // ad: if_acmpne b8
   // b0: aload 5
   // b2: areturn
   // b3: aload 3
   // b4: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // b7: aload 3
   // b8: pop
   // b9: new kotlin/KotlinNothingValueException
   // bc: dup
   // bd: invokespecial kotlin/KotlinNothingValueException.<init> ()V
   // c0: athrow
   // c1: iload 2
   // c2: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
   // c5: areturn
   // c6: new java/lang/IllegalStateException
   // c9: dup
   // ca: ldc "call to 'resume' before 'invoke' with coroutine"
   // cc: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // cf: athrow
}
