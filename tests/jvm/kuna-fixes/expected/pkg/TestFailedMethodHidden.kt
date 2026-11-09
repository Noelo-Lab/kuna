package pkg

import kotlin.coroutines.Continuation

public interface TestFailedMethodHidden {
   public abstract fun note(x: Int) {
   }

   public abstract suspend fun next(): Int {
   }

   public open suspend fun twice(): Int {
   }

   public open fun reset(x: Int) {
   }

   // $VF: Class flags could not be determined
   internal class DefaultImpls {
      @JvmStatic
      fun twice(`$this`: TestFailedMethodHidden, `$completion`: Continuation<in Integer>): Any? {
         // $VF: Couldn't be decompiled
         // Bytecode:
         // 00: aload 1
         // 01: instanceof pkg/TestFailedMethodHidden$twice$1
         // 04: ifeq 27
         // 07: aload 1
         // 08: checkcast pkg/TestFailedMethodHidden$twice$1
         // 0b: astore 6
         // 0d: aload 6
         // 0f: getfield pkg/TestFailedMethodHidden$twice$1.label I
         // 12: ldc -2147483648
         // 14: iand
         // 15: ifeq 27
         // 18: aload 6
         // 1a: dup
         // 1b: getfield pkg/TestFailedMethodHidden$twice$1.label I
         // 1e: ldc -2147483648
         // 20: isub
         // 21: putfield pkg/TestFailedMethodHidden$twice$1.label I
         // 24: goto 31
         // 27: new pkg/TestFailedMethodHidden$twice$1
         // 2a: dup
         // 2b: aload 1
         // 2c: invokespecial pkg/TestFailedMethodHidden$twice$1.<init> (Lkotlin/coroutines/Continuation;)V
         // 2f: astore 6
         // 31: aload 6
         // 33: getfield pkg/TestFailedMethodHidden$twice$1.result Ljava/lang/Object;
         // 36: astore 5
         // 38: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
         // 3b: astore 7
         // 3d: aload 6
         // 3f: getfield pkg/TestFailedMethodHidden$twice$1.label I
         // 42: tableswitch 149 0 2 26 60 120
         // 5c: aload 5
         // 5e: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
         // 61: aload 0
         // 62: aload 6
         // 64: aload 6
         // 66: aload 0
         // 67: putfield pkg/TestFailedMethodHidden$twice$1.L$0 Ljava/lang/Object;
         // 6a: aload 6
         // 6c: bipush 1
         // 6d: putfield pkg/TestFailedMethodHidden$twice$1.label I
         // 70: invokeinterface pkg/TestFailedMethodHidden.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
         // 75: dup
         // 76: aload 7
         // 78: if_acmpne 8e
         // 7b: aload 7
         // 7d: areturn
         // 7e: aload 6
         // 80: getfield pkg/TestFailedMethodHidden$twice$1.L$0 Ljava/lang/Object;
         // 83: checkcast pkg/TestFailedMethodHidden
         // 86: astore 0
         // 87: aload 5
         // 89: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
         // 8c: aload 5
         // 8e: checkcast java/lang/Number
         // 91: invokevirtual java/lang/Number.intValue ()I
         // 94: istore 2
         // 95: iload 2
         // 96: istore 3
         // 97: aload 0
         // 98: aload 6
         // 9a: aload 6
         // 9c: aconst_null
         // 9d: putfield pkg/TestFailedMethodHidden$twice$1.L$0 Ljava/lang/Object;
         // a0: aload 6
         // a2: iload 3
         // a3: putfield pkg/TestFailedMethodHidden$twice$1.I$0 I
         // a6: aload 6
         // a8: bipush 2
         // a9: putfield pkg/TestFailedMethodHidden$twice$1.label I
         // ac: invokeinterface pkg/TestFailedMethodHidden.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
         // b1: dup
         // b2: aload 7
         // b4: if_acmpne c7
         // b7: aload 7
         // b9: areturn
         // ba: aload 6
         // bc: getfield pkg/TestFailedMethodHidden$twice$1.I$0 I
         // bf: istore 3
         // c0: aload 5
         // c2: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
         // c5: aload 5
         // c7: astore 4
         // c9: iload 3
         // ca: aload 4
         // cc: checkcast java/lang/Number
         // cf: invokevirtual java/lang/Number.intValue ()I
         // d2: iadd
         // d3: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
         // d6: areturn
         // d7: new java/lang/IllegalStateException
         // da: dup
         // db: ldc "call to 'resume' before 'invoke' with coroutine"
         // dd: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
         // e0: athrow
      }

      @JvmStatic
      fun reset(`$this`: TestFailedMethodHidden, x: Int) {
         if (x < 0) {
            `$this`.note(0)
         } else {
            `$this`.note(x)
         }
      }
   }
}
