package pkg;

import java.util.List;
import kotlin.Metadata;
import kotlin.coroutines.Continuation;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

@Metadata(
   mv = {2, 0, 0},
   k = 1,
   xi = 48,
   d1 = "\u0000 \n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\n\u0002\u0010\b\n\u0000\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010!\n\u0002\b\u0004\u0018\u00002\u00020\u0001:\u0001\rB\u0007¢\u0006\u0004\b\u0002\u0010\u0003J$\u0010\u0004\u001a\u00020\u00052\u0006\u0010\u0006\u001a\u00020\u00072\f\u0010\b\u001a\b\u0012\u0004\u0012\u00020\u00050\tH\u0086@¢\u0006\u0002\u0010\nJ\u0016\u0010\u000b\u001a\u00020\u00052\u0006\u0010\u0006\u001a\u00020\u0007H\u0086@¢\u0006\u0002\u0010\f",
   d2 = {
         "Lpkg/TestSuspendLoopFinally;",
         "",
         "<init>",
         "()V",
         "drain",
         "",
         "src",
         "Lpkg/TestSuspendLoopFinally$Source;",
         "out",
         "",
         "(Lpkg/TestSuspendLoopFinally$Source;Ljava/util/List;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "first",
         "(Lpkg/TestSuspendLoopFinally$Source;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "Source"
   }
)
public final class TestSuspendLoopFinally {
   @Nullable
   public final Object drain(@NotNull TestSuspendLoopFinally.Source param1, @NotNull List<Integer> param2, @NotNull Continuation<? super Integer> param3) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 3
      // 01: instanceof pkg/TestSuspendLoopFinally$drain$1
      // 04: ifeq 27
      // 07: aload 3
      // 08: checkcast pkg/TestSuspendLoopFinally$drain$1
      // 0b: astore 7
      // 0d: aload 7
      // 0f: getfield pkg/TestSuspendLoopFinally$drain$1.label I
      // 12: ldc -2147483648
      // 14: iand
      // 15: ifeq 27
      // 18: aload 7
      // 1a: dup
      // 1b: getfield pkg/TestSuspendLoopFinally$drain$1.label I
      // 1e: ldc -2147483648
      // 20: isub
      // 21: putfield pkg/TestSuspendLoopFinally$drain$1.label I
      // 24: goto 32
      // 27: new pkg/TestSuspendLoopFinally$drain$1
      // 2a: dup
      // 2b: aload 0
      // 2c: aload 3
      // 2d: invokespecial pkg/TestSuspendLoopFinally$drain$1.<init> (Lpkg/TestSuspendLoopFinally;Lkotlin/coroutines/Continuation;)V
      // 30: astore 7
      // 32: aload 7
      // 34: getfield pkg/TestSuspendLoopFinally$drain$1.result Ljava/lang/Object;
      // 37: astore 6
      // 39: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
      // 3c: astore 8
      // 3e: aload 7
      // 40: getfield pkg/TestSuspendLoopFinally$drain$1.label I
      // 43: tableswitch 172 0 1 21 66
      // 58: aload 6
      // 5a: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 5d: aconst_null
      // 5e: astore 4
      // 60: nop
      // 61: nop
      // 62: aload 1
      // 63: aload 7
      // 65: aload 7
      // 67: aload 1
      // 68: putfield pkg/TestSuspendLoopFinally$drain$1.L$0 Ljava/lang/Object;
      // 6b: aload 7
      // 6d: aload 2
      // 6e: putfield pkg/TestSuspendLoopFinally$drain$1.L$1 Ljava/lang/Object;
      // 71: aload 7
      // 73: bipush 1
      // 74: putfield pkg/TestSuspendLoopFinally$drain$1.label I
      // 77: invokeinterface pkg/TestSuspendLoopFinally$Source.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
      // 7c: dup
      // 7d: aload 8
      // 7f: if_acmpne a2
      // 82: aload 8
      // 84: areturn
      // 85: aconst_null
      // 86: astore 4
      // 88: aload 7
      // 8a: getfield pkg/TestSuspendLoopFinally$drain$1.L$1 Ljava/lang/Object;
      // 8d: checkcast java/util/List
      // 90: astore 2
      // 91: aload 7
      // 93: getfield pkg/TestSuspendLoopFinally$drain$1.L$0 Ljava/lang/Object;
      // 96: checkcast pkg/TestSuspendLoopFinally$Source
      // 99: astore 1
      // 9a: nop
      // 9b: aload 6
      // 9d: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // a0: aload 6
      // a2: checkcast java/lang/Integer
      // a5: dup
      // a6: ifnull af
      // a9: invokevirtual java/lang/Integer.intValue ()I
      // ac: goto b3
      // af: pop
      // b0: goto c4
      // b3: istore 5
      // b5: aload 2
      // b6: iload 5
      // b8: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
      // bb: invokeinterface java/util/List.add (Ljava/lang/Object;)Z 2
      // c0: pop
      // c1: goto 61
      // c4: aload 1
      // c5: aload 4
      // c7: invokeinterface pkg/TestSuspendLoopFinally$Source.close (Ljava/lang/Throwable;)V 2
      // cc: goto e5
      // cf: astore 5
      // d1: aload 5
      // d3: astore 4
      // d5: aload 5
      // d7: athrow
      // d8: astore 5
      // da: aload 1
      // db: aload 4
      // dd: invokeinterface pkg/TestSuspendLoopFinally$Source.close (Ljava/lang/Throwable;)V 2
      // e2: aload 5
      // e4: athrow
      // e5: aload 2
      // e6: invokeinterface java/util/List.size ()I 1
      // eb: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
      // ee: areturn
      // ef: new java/lang/IllegalStateException
      // f2: dup
      // f3: ldc "call to 'resume' before 'invoke' with coroutine"
      // f5: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
      // f8: athrow
      // try (36 -> 50): 87 java/lang/Throwable
      // try (65 -> 83): 87 java/lang/Throwable
      // try (36 -> 50): 92 null
      // try (65 -> 83): 92 null
      // try (87 -> 92): 92 null
      // try (92 -> 93): 92 null
   }

   @Nullable
   public final Object first(@NotNull TestSuspendLoopFinally.Source param1, @NotNull Continuation<? super Integer> param2) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 2
      // 01: instanceof pkg/TestSuspendLoopFinally$first$1
      // 04: ifeq 27
      // 07: aload 2
      // 08: checkcast pkg/TestSuspendLoopFinally$first$1
      // 0b: astore 7
      // 0d: aload 7
      // 0f: getfield pkg/TestSuspendLoopFinally$first$1.label I
      // 12: ldc -2147483648
      // 14: iand
      // 15: ifeq 27
      // 18: aload 7
      // 1a: dup
      // 1b: getfield pkg/TestSuspendLoopFinally$first$1.label I
      // 1e: ldc -2147483648
      // 20: isub
      // 21: putfield pkg/TestSuspendLoopFinally$first$1.label I
      // 24: goto 32
      // 27: new pkg/TestSuspendLoopFinally$first$1
      // 2a: dup
      // 2b: aload 0
      // 2c: aload 2
      // 2d: invokespecial pkg/TestSuspendLoopFinally$first$1.<init> (Lpkg/TestSuspendLoopFinally;Lkotlin/coroutines/Continuation;)V
      // 30: astore 7
      // 32: aload 7
      // 34: getfield pkg/TestSuspendLoopFinally$first$1.result Ljava/lang/Object;
      // 37: astore 6
      // 39: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
      // 3c: astore 8
      // 3e: aload 7
      // 40: getfield pkg/TestSuspendLoopFinally$first$1.label I
      // 43: tableswitch 131 0 1 21 58
      // 58: aload 6
      // 5a: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 5d: aconst_null
      // 5e: astore 3
      // 5f: nop
      // 60: aload 1
      // 61: aload 7
      // 63: aload 7
      // 65: aload 1
      // 66: putfield pkg/TestSuspendLoopFinally$first$1.L$0 Ljava/lang/Object;
      // 69: aload 7
      // 6b: bipush 1
      // 6c: putfield pkg/TestSuspendLoopFinally$first$1.label I
      // 6f: invokeinterface pkg/TestSuspendLoopFinally$Source.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
      // 74: dup
      // 75: aload 8
      // 77: if_acmpne 90
      // 7a: aload 8
      // 7c: areturn
      // 7d: aconst_null
      // 7e: astore 3
      // 7f: aload 7
      // 81: getfield pkg/TestSuspendLoopFinally$first$1.L$0 Ljava/lang/Object;
      // 84: checkcast pkg/TestSuspendLoopFinally$Source
      // 87: astore 1
      // 88: nop
      // 89: aload 6
      // 8b: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 8e: aload 6
      // 90: checkcast java/lang/Integer
      // 93: astore 4
      // 95: aload 4
      // 97: dup
      // 98: ifnull a1
      // 9b: invokevirtual java/lang/Integer.intValue ()I
      // 9e: goto a3
      // a1: pop
      // a2: bipush -1
      // a3: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
      // a6: astore 5
      // a8: aload 1
      // a9: aload 3
      // aa: invokeinterface pkg/TestSuspendLoopFinally$Source.close (Ljava/lang/Throwable;)V 2
      // af: aload 5
      // b1: areturn
      // b2: astore 5
      // b4: aload 5
      // b6: astore 3
      // b7: aload 5
      // b9: athrow
      // ba: astore 5
      // bc: aload 1
      // bd: aload 3
      // be: invokeinterface pkg/TestSuspendLoopFinally$Source.close (Ljava/lang/Throwable;)V 2
      // c3: aload 5
      // c5: athrow
      // c6: new java/lang/IllegalStateException
      // c9: dup
      // ca: ldc "call to 'resume' before 'invoke' with coroutine"
      // cc: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
      // cf: athrow
      // try (36 -> 46): 77 java/lang/Throwable
      // try (57 -> 72): 77 java/lang/Throwable
      // try (36 -> 46): 82 null
      // try (57 -> 72): 82 null
      // try (77 -> 82): 82 null
      // try (82 -> 83): 82 null
   }

   @Metadata(
      mv = {2, 0, 0},
      k = 1,
      xi = 48,
      d1 = "\u0000\u001c\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0010\b\n\u0002\b\u0002\n\u0002\u0010\u0002\n\u0000\n\u0002\u0010\u0003\bf\u0018\u00002\u00020\u0001J\u0010\u0010\u0002\u001a\u0004\u0018\u00010\u0003H¦@¢\u0006\u0002\u0010\u0004J\u0012\u0010\u0005\u001a\u00020\u00062\b\u0010\u0007\u001a\u0004\u0018\u00010\bH&",
      d2 = {"Lpkg/TestSuspendLoopFinally$Source;", "", "next", "", "(Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "close", "", "cause", ""}
   )
   public interface Source {
      @Nullable
      Object next(@NotNull Continuation<? super Integer> var1);

      void close(@Nullable Throwable var1);
   }
}
