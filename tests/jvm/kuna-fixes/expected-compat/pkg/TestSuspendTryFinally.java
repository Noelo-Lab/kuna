package pkg;

import kotlin.Metadata;
import kotlin.Unit;
import kotlin.coroutines.Continuation;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

@Metadata(
   mv = {2, 0, 0},
   k = 1,
   xi = 48,
   d1 = "\u0000 \n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\n\u0002\u0010\b\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0010\u0002\n\u0000\u0018\u00002\u00020\u0001:\u0001\u000bB\u0007¢\u0006\u0004\b\u0002\u0010\u0003J\u0016\u0010\u0004\u001a\u00020\u00052\u0006\u0010\u0006\u001a\u00020\u0007H\u0086@¢\u0006\u0002\u0010\bJ\u0016\u0010\t\u001a\u00020\n2\u0006\u0010\u0006\u001a\u00020\u0007H\u0086@¢\u0006\u0002\u0010\b",
   d2 = {
         "Lpkg/TestSuspendTryFinally;",
         "",
         "<init>",
         "()V",
         "runThenDone",
         "",
         "step",
         "Lpkg/TestSuspendTryFinally$Step;",
         "(Lpkg/TestSuspendTryFinally$Step;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "runThenMaybeDone",
         "",
         "Step"
   }
)
public final class TestSuspendTryFinally {
   @Nullable
   public final Object runThenDone(@NotNull TestSuspendTryFinally.Step param1, @NotNull Continuation<? super Integer> param2) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 2
      // 01: instanceof pkg/TestSuspendTryFinally$runThenDone$1
      // 04: ifeq 27
      // 07: aload 2
      // 08: checkcast pkg/TestSuspendTryFinally$runThenDone$1
      // 0b: astore 6
      // 0d: aload 6
      // 0f: getfield pkg/TestSuspendTryFinally$runThenDone$1.label I
      // 12: ldc -2147483648
      // 14: iand
      // 15: ifeq 27
      // 18: aload 6
      // 1a: dup
      // 1b: getfield pkg/TestSuspendTryFinally$runThenDone$1.label I
      // 1e: ldc -2147483648
      // 20: isub
      // 21: putfield pkg/TestSuspendTryFinally$runThenDone$1.label I
      // 24: goto 32
      // 27: new pkg/TestSuspendTryFinally$runThenDone$1
      // 2a: dup
      // 2b: aload 0
      // 2c: aload 2
      // 2d: invokespecial pkg/TestSuspendTryFinally$runThenDone$1.<init> (Lpkg/TestSuspendTryFinally;Lkotlin/coroutines/Continuation;)V
      // 30: astore 6
      // 32: aload 6
      // 34: getfield pkg/TestSuspendTryFinally$runThenDone$1.result Ljava/lang/Object;
      // 37: astore 5
      // 39: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
      // 3c: astore 7
      // 3e: aload 6
      // 40: getfield pkg/TestSuspendTryFinally$runThenDone$1.label I
      // 43: tableswitch 93 0 1 21 56
      // 58: aload 5
      // 5a: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 5d: nop
      // 5e: aload 1
      // 5f: aload 6
      // 61: aload 6
      // 63: aload 1
      // 64: putfield pkg/TestSuspendTryFinally$runThenDone$1.L$0 Ljava/lang/Object;
      // 67: aload 6
      // 69: bipush 1
      // 6a: putfield pkg/TestSuspendTryFinally$runThenDone$1.label I
      // 6d: invokeinterface pkg/TestSuspendTryFinally$Step.run (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
      // 72: dup
      // 73: aload 7
      // 75: if_acmpne 8c
      // 78: aload 7
      // 7a: areturn
      // 7b: aload 6
      // 7d: getfield pkg/TestSuspendTryFinally$runThenDone$1.L$0 Ljava/lang/Object;
      // 80: checkcast pkg/TestSuspendTryFinally$Step
      // 83: astore 1
      // 84: nop
      // 85: aload 5
      // 87: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 8a: aload 5
      // 8c: astore 3
      // 8d: aload 1
      // 8e: invokeinterface pkg/TestSuspendTryFinally$Step.done ()V 1
      // 93: aload 3
      // 94: areturn
      // 95: astore 4
      // 97: aload 1
      // 98: invokeinterface pkg/TestSuspendTryFinally$Step.done ()V 1
      // 9d: aload 4
      // 9f: athrow
      // a0: new java/lang/IllegalStateException
      // a3: dup
      // a4: ldc "call to 'resume' before 'invoke' with coroutine"
      // a6: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
      // a9: athrow
      // try (34 -> 44): 62 null
      // try (53 -> 58): 62 null
      // try (62 -> 63): 62 null
   }

   @Nullable
   public final Object runThenMaybeDone(@NotNull TestSuspendTryFinally.Step param1, @NotNull Continuation<? super Unit> param2) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 2
      // 01: instanceof pkg/TestSuspendTryFinally$runThenMaybeDone$1
      // 04: ifeq 27
      // 07: aload 2
      // 08: checkcast pkg/TestSuspendTryFinally$runThenMaybeDone$1
      // 0b: astore 5
      // 0d: aload 5
      // 0f: getfield pkg/TestSuspendTryFinally$runThenMaybeDone$1.label I
      // 12: ldc -2147483648
      // 14: iand
      // 15: ifeq 27
      // 18: aload 5
      // 1a: dup
      // 1b: getfield pkg/TestSuspendTryFinally$runThenMaybeDone$1.label I
      // 1e: ldc -2147483648
      // 20: isub
      // 21: putfield pkg/TestSuspendTryFinally$runThenMaybeDone$1.label I
      // 24: goto 32
      // 27: new pkg/TestSuspendTryFinally$runThenMaybeDone$1
      // 2a: dup
      // 2b: aload 0
      // 2c: aload 2
      // 2d: invokespecial pkg/TestSuspendTryFinally$runThenMaybeDone$1.<init> (Lpkg/TestSuspendTryFinally;Lkotlin/coroutines/Continuation;)V
      // 30: astore 5
      // 32: aload 5
      // 34: getfield pkg/TestSuspendTryFinally$runThenMaybeDone$1.result Ljava/lang/Object;
      // 37: astore 4
      // 39: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
      // 3c: astore 6
      // 3e: aload 5
      // 40: getfield pkg/TestSuspendTryFinally$runThenMaybeDone$1.label I
      // 43: tableswitch 120 0 1 21 56
      // 58: aload 4
      // 5a: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 5d: nop
      // 5e: aload 1
      // 5f: aload 5
      // 61: aload 5
      // 63: aload 1
      // 64: putfield pkg/TestSuspendTryFinally$runThenMaybeDone$1.L$0 Ljava/lang/Object;
      // 67: aload 5
      // 69: bipush 1
      // 6a: putfield pkg/TestSuspendTryFinally$runThenMaybeDone$1.label I
      // 6d: invokeinterface pkg/TestSuspendTryFinally$Step.run (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
      // 72: dup
      // 73: aload 6
      // 75: if_acmpne 8c
      // 78: aload 6
      // 7a: areturn
      // 7b: aload 5
      // 7d: getfield pkg/TestSuspendTryFinally$runThenMaybeDone$1.L$0 Ljava/lang/Object;
      // 80: checkcast pkg/TestSuspendTryFinally$Step
      // 83: astore 1
      // 84: nop
      // 85: aload 4
      // 87: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 8a: aload 4
      // 8c: checkcast java/lang/Number
      // 8f: invokevirtual java/lang/Number.intValue ()I
      // 92: pop
      // 93: aload 1
      // 94: invokeinterface pkg/TestSuspendTryFinally$Step.last ()Z 1
      // 99: ifeq a2
      // 9c: aload 1
      // 9d: invokeinterface pkg/TestSuspendTryFinally$Step.done ()V 1
      // a2: goto b7
      // a5: astore 3
      // a6: aload 1
      // a7: invokeinterface pkg/TestSuspendTryFinally$Step.last ()Z 1
      // ac: ifeq b5
      // af: aload 1
      // b0: invokeinterface pkg/TestSuspendTryFinally$Step.done ()V 1
      // b5: aload 3
      // b6: athrow
      // b7: getstatic kotlin/Unit.INSTANCE Lkotlin/Unit;
      // ba: areturn
      // bb: new java/lang/IllegalStateException
      // be: dup
      // bf: ldc "call to 'resume' before 'invoke' with coroutine"
      // c1: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
      // c4: athrow
      // try (34 -> 44): 66 null
      // try (53 -> 60): 66 null
      // try (66 -> 67): 66 null
   }

   @Metadata(
      mv = {2, 0, 0},
      k = 1,
      xi = 48,
      d1 = "\u0000\u001c\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0010\b\n\u0002\b\u0002\n\u0002\u0010\u000b\n\u0000\n\u0002\u0010\u0002\bf\u0018\u00002\u00020\u0001J\u000e\u0010\u0002\u001a\u00020\u0003H¦@¢\u0006\u0002\u0010\u0004J\b\u0010\u0005\u001a\u00020\u0006H&J\b\u0010\u0007\u001a\u00020\bH&",
      d2 = {"Lpkg/TestSuspendTryFinally$Step;", "", "run", "", "(Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "last", "", "done", ""}
   )
   public interface Step {
      @Nullable
      Object run(@NotNull Continuation<? super Integer> var1);

      boolean last();

      void done();
   }
}
