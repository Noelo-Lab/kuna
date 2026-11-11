package pkg;

import kotlin.Metadata;
import kotlin.Unit;
import kotlin.coroutines.Continuation;
import kotlin.coroutines.jvm.internal.Boxing;
import kotlin.jvm.functions.Function0;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

@Metadata(
   mv = {2, 0, 0},
   k = 2,
   xi = 48,
   d1 = "\u0000\u001e\n\u0000\n\u0002\u0010\b\n\u0002\b\u0002\n\u0002\u0010\u0002\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0004\n\u0002\u0010\u000b\n\u0000\u001a\u000e\u0010\u0000\u001a\u00020\u0001H\u0086@¢\u0006\u0002\u0010\u0002\u001a\u001c\u0010\u0003\u001a\u00020\u00042\f\u0010\u0005\u001a\b\u0012\u0004\u0012\u00020\u00040\u0006H\u0086@¢\u0006\u0002\u0010\u0007\u001a\u001c\u0010\b\u001a\u00020\u00012\f\u0010\u0005\u001a\b\u0012\u0004\u0012\u00020\u00040\u0006H\u0086@¢\u0006\u0002\u0010\u0007\u001a$\u0010\t\u001a\u00020\u00012\u0006\u0010\n\u001a\u00020\u000b2\f\u0010\u0005\u001a\b\u0012\u0004\u0012\u00020\u00040\u0006H\u0086@¢\u0006\u0002\u0010\f",
   d2 = {
         "step",
         "",
         "(Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "once",
         "",
         "done",
         "Lkotlin/Function0;",
         "(Lkotlin/jvm/functions/Function0;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "twice",
         "early",
         "flag",
         "",
         "(ZLkotlin/jvm/functions/Function0;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;"
   }
)
public final class TestSuspendFinallyKt {
   @Nullable
   public static final Object step(@NotNull Continuation<? super Integer> $completion) {
      return Boxing.boxInt(1);
   }

   @Nullable
   public static final Object once(@NotNull Function0<Unit> param0, @NotNull Continuation<? super Unit> param1) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 1
      // 01: instanceof pkg/TestSuspendFinallyKt$once$1
      // 04: ifeq 27
      // 07: aload 1
      // 08: checkcast pkg/TestSuspendFinallyKt$once$1
      // 0b: astore 4
      // 0d: aload 4
      // 0f: getfield pkg/TestSuspendFinallyKt$once$1.label I
      // 12: ldc -2147483648
      // 14: iand
      // 15: ifeq 27
      // 18: aload 4
      // 1a: dup
      // 1b: getfield pkg/TestSuspendFinallyKt$once$1.label I
      // 1e: ldc -2147483648
      // 20: isub
      // 21: putfield pkg/TestSuspendFinallyKt$once$1.label I
      // 24: goto 31
      // 27: new pkg/TestSuspendFinallyKt$once$1
      // 2a: dup
      // 2b: aload 1
      // 2c: invokespecial pkg/TestSuspendFinallyKt$once$1.<init> (Lkotlin/coroutines/Continuation;)V
      // 2f: astore 4
      // 31: aload 4
      // 33: getfield pkg/TestSuspendFinallyKt$once$1.result Ljava/lang/Object;
      // 36: astore 3
      // 37: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
      // 3a: astore 5
      // 3c: aload 4
      // 3e: getfield pkg/TestSuspendFinallyKt$once$1.label I
      // 41: tableswitch 100 0 1 23 54
      // 58: aload 3
      // 59: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 5c: nop
      // 5d: aload 4
      // 5f: aload 4
      // 61: aload 0
      // 62: putfield pkg/TestSuspendFinallyKt$once$1.L$0 Ljava/lang/Object;
      // 65: aload 4
      // 67: bipush 1
      // 68: putfield pkg/TestSuspendFinallyKt$once$1.label I
      // 6b: invokestatic pkg/TestSuspendFinallyKt.step (Lkotlin/coroutines/Continuation;)Ljava/lang/Object;
      // 6e: dup
      // 6f: aload 5
      // 71: if_acmpne 86
      // 74: aload 5
      // 76: areturn
      // 77: aload 4
      // 79: getfield pkg/TestSuspendFinallyKt$once$1.L$0 Ljava/lang/Object;
      // 7c: checkcast kotlin/jvm/functions/Function0
      // 7f: astore 0
      // 80: nop
      // 81: aload 3
      // 82: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 85: aload 3
      // 86: checkcast java/lang/Number
      // 89: invokevirtual java/lang/Number.intValue ()I
      // 8c: pop
      // 8d: aload 0
      // 8e: invokeinterface kotlin/jvm/functions/Function0.invoke ()Ljava/lang/Object; 1
      // 93: pop
      // 94: goto a1
      // 97: astore 2
      // 98: aload 0
      // 99: invokeinterface kotlin/jvm/functions/Function0.invoke ()Ljava/lang/Object; 1
      // 9e: pop
      // 9f: aload 2
      // a0: athrow
      // a1: getstatic kotlin/Unit.INSTANCE Lkotlin/Unit;
      // a4: areturn
      // a5: new java/lang/IllegalStateException
      // a8: dup
      // a9: ldc "call to 'resume' before 'invoke' with coroutine"
      // ab: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
      // ae: athrow
      // try (33 -> 42): 62 null
      // try (51 -> 58): 62 null
      // try (62 -> 63): 62 null
   }

   @Nullable
   public static final Object twice(@NotNull Function0<Unit> param0, @NotNull Continuation<? super Integer> param1) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 1
      // 01: instanceof pkg/TestSuspendFinallyKt$twice$1
      // 04: ifeq 27
      // 07: aload 1
      // 08: checkcast pkg/TestSuspendFinallyKt$twice$1
      // 0b: astore 6
      // 0d: aload 6
      // 0f: getfield pkg/TestSuspendFinallyKt$twice$1.label I
      // 12: ldc -2147483648
      // 14: iand
      // 15: ifeq 27
      // 18: aload 6
      // 1a: dup
      // 1b: getfield pkg/TestSuspendFinallyKt$twice$1.label I
      // 1e: ldc -2147483648
      // 20: isub
      // 21: putfield pkg/TestSuspendFinallyKt$twice$1.label I
      // 24: goto 31
      // 27: new pkg/TestSuspendFinallyKt$twice$1
      // 2a: dup
      // 2b: aload 1
      // 2c: invokespecial pkg/TestSuspendFinallyKt$twice$1.<init> (Lkotlin/coroutines/Continuation;)V
      // 2f: astore 6
      // 31: aload 6
      // 33: getfield pkg/TestSuspendFinallyKt$twice$1.result Ljava/lang/Object;
      // 36: astore 5
      // 38: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
      // 3b: astore 7
      // 3d: aload 6
      // 3f: getfield pkg/TestSuspendFinallyKt$twice$1.label I
      // 42: tableswitch 172 0 2 26 58 114
      // 5c: aload 5
      // 5e: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 61: nop
      // 62: aload 6
      // 64: aload 6
      // 66: aload 0
      // 67: putfield pkg/TestSuspendFinallyKt$twice$1.L$0 Ljava/lang/Object;
      // 6a: aload 6
      // 6c: bipush 1
      // 6d: putfield pkg/TestSuspendFinallyKt$twice$1.label I
      // 70: invokestatic pkg/TestSuspendFinallyKt.step (Lkotlin/coroutines/Continuation;)Ljava/lang/Object;
      // 73: dup
      // 74: aload 7
      // 76: if_acmpne 8d
      // 79: aload 7
      // 7b: areturn
      // 7c: aload 6
      // 7e: getfield pkg/TestSuspendFinallyKt$twice$1.L$0 Ljava/lang/Object;
      // 81: checkcast kotlin/jvm/functions/Function0
      // 84: astore 0
      // 85: nop
      // 86: aload 5
      // 88: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 8b: aload 5
      // 8d: checkcast java/lang/Number
      // 90: invokevirtual java/lang/Number.intValue ()I
      // 93: istore 2
      // 94: aload 6
      // 96: aload 6
      // 98: aload 0
      // 99: putfield pkg/TestSuspendFinallyKt$twice$1.L$0 Ljava/lang/Object;
      // 9c: aload 6
      // 9e: iload 2
      // 9f: putfield pkg/TestSuspendFinallyKt$twice$1.I$0 I
      // a2: aload 6
      // a4: bipush 2
      // a5: putfield pkg/TestSuspendFinallyKt$twice$1.label I
      // a8: invokestatic pkg/TestSuspendFinallyKt.step (Lkotlin/coroutines/Continuation;)Ljava/lang/Object;
      // ab: dup
      // ac: aload 7
      // ae: if_acmpne cb
      // b1: aload 7
      // b3: areturn
      // b4: aload 6
      // b6: getfield pkg/TestSuspendFinallyKt$twice$1.I$0 I
      // b9: istore 2
      // ba: aload 6
      // bc: getfield pkg/TestSuspendFinallyKt$twice$1.L$0 Ljava/lang/Object;
      // bf: checkcast kotlin/jvm/functions/Function0
      // c2: astore 0
      // c3: nop
      // c4: aload 5
      // c6: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // c9: aload 5
      // cb: checkcast java/lang/Number
      // ce: invokevirtual java/lang/Number.intValue ()I
      // d1: istore 3
      // d2: iload 2
      // d3: iload 3
      // d4: iadd
      // d5: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
      // d8: astore 4
      // da: aload 0
      // db: invokeinterface kotlin/jvm/functions/Function0.invoke ()Ljava/lang/Object; 1
      // e0: pop
      // e1: aload 4
      // e3: areturn
      // e4: astore 3
      // e5: aload 0
      // e6: invokeinterface kotlin/jvm/functions/Function0.invoke ()Ljava/lang/Object; 1
      // eb: pop
      // ec: aload 3
      // ed: athrow
      // ee: new java/lang/IllegalStateException
      // f1: dup
      // f2: ldc "call to 'resume' before 'invoke' with coroutine"
      // f4: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
      // f7: athrow
      // try (33 -> 42): 98 null
      // try (51 -> 69): 98 null
      // try (81 -> 93): 98 null
      // try (98 -> 99): 98 null
   }

   @Nullable
   public static final Object early(boolean param0, @NotNull Function0<Unit> param1, @NotNull Continuation<? super Integer> param2) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 2
      // 01: instanceof pkg/TestSuspendFinallyKt$early$1
      // 04: ifeq 27
      // 07: aload 2
      // 08: checkcast pkg/TestSuspendFinallyKt$early$1
      // 0b: astore 6
      // 0d: aload 6
      // 0f: getfield pkg/TestSuspendFinallyKt$early$1.label I
      // 12: ldc -2147483648
      // 14: iand
      // 15: ifeq 27
      // 18: aload 6
      // 1a: dup
      // 1b: getfield pkg/TestSuspendFinallyKt$early$1.label I
      // 1e: ldc -2147483648
      // 20: isub
      // 21: putfield pkg/TestSuspendFinallyKt$early$1.label I
      // 24: goto 31
      // 27: new pkg/TestSuspendFinallyKt$early$1
      // 2a: dup
      // 2b: aload 2
      // 2c: invokespecial pkg/TestSuspendFinallyKt$early$1.<init> (Lkotlin/coroutines/Continuation;)V
      // 2f: astore 6
      // 31: aload 6
      // 33: getfield pkg/TestSuspendFinallyKt$early$1.result Ljava/lang/Object;
      // 36: astore 5
      // 38: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
      // 3b: astore 7
      // 3d: aload 6
      // 3f: getfield pkg/TestSuspendFinallyKt$early$1.label I
      // 42: tableswitch 111 0 1 22 72
      // 58: aload 5
      // 5a: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 5d: nop
      // 5e: iload 0
      // 5f: ifeq 70
      // 62: bipush 0
      // 63: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
      // 66: astore 3
      // 67: aload 1
      // 68: invokeinterface kotlin/jvm/functions/Function0.invoke ()Ljava/lang/Object; 1
      // 6d: pop
      // 6e: aload 3
      // 6f: areturn
      // 70: aload 6
      // 72: aload 6
      // 74: aload 1
      // 75: putfield pkg/TestSuspendFinallyKt$early$1.L$0 Ljava/lang/Object;
      // 78: aload 6
      // 7a: bipush 1
      // 7b: putfield pkg/TestSuspendFinallyKt$early$1.label I
      // 7e: invokestatic pkg/TestSuspendFinallyKt.step (Lkotlin/coroutines/Continuation;)Ljava/lang/Object;
      // 81: dup
      // 82: aload 7
      // 84: if_acmpne 9b
      // 87: aload 7
      // 89: areturn
      // 8a: aload 6
      // 8c: getfield pkg/TestSuspendFinallyKt$early$1.L$0 Ljava/lang/Object;
      // 8f: checkcast kotlin/jvm/functions/Function0
      // 92: astore 1
      // 93: nop
      // 94: aload 5
      // 96: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 99: aload 5
      // 9b: astore 3
      // 9c: aload 1
      // 9d: invokeinterface kotlin/jvm/functions/Function0.invoke ()Ljava/lang/Object; 1
      // a2: pop
      // a3: aload 3
      // a4: areturn
      // a5: astore 4
      // a7: aload 1
      // a8: invokeinterface kotlin/jvm/functions/Function0.invoke ()Ljava/lang/Object; 1
      // ad: pop
      // ae: aload 4
      // b0: athrow
      // b1: new java/lang/IllegalStateException
      // b4: dup
      // b5: ldc "call to 'resume' before 'invoke' with coroutine"
      // b7: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
      // ba: athrow
      // try (33 -> 39): 71 null
      // try (44 -> 52): 71 null
      // try (61 -> 66): 71 null
      // try (71 -> 72): 71 null
   }
}
