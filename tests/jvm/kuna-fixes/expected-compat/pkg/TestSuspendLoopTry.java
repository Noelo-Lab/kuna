package pkg;

import java.util.List;
import kotlin.Metadata;
import kotlin.Unit;
import kotlin.coroutines.Continuation;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

@Metadata(
   mv = {2, 0, 0},
   k = 1,
   xi = 48,
   d1 = "\u00002\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\n\u0002\u0010\b\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0003\n\u0002\u0010\u0002\n\u0002\b\u0002\n\u0002\u0010!\n\u0002\b\u0002\n\u0002\u0010\u0001\n\u0002\b\u0002\u0018\u00002\u00020\u0001:\u0001\u0013B\u0007¢\u0006\u0004\b\u0002\u0010\u0003J\u001e\u0010\u0004\u001a\u00020\u00052\u0006\u0010\u0006\u001a\u00020\u00072\u0006\u0010\b\u001a\u00020\u0005H\u0086@¢\u0006\u0002\u0010\tJ,\u0010\n\u001a\u00020\u000b2\u0006\u0010\u0006\u001a\u00020\u00072\u0006\u0010\f\u001a\u00020\u00072\f\u0010\r\u001a\b\u0012\u0004\u0012\u00020\u00050\u000eH\u0086@¢\u0006\u0002\u0010\u000fJ$\u0010\u0010\u001a\u00020\u00112\u0006\u0010\u0006\u001a\u00020\u00072\f\u0010\r\u001a\b\u0012\u0004\u0012\u00020\u00050\u000eH\u0086@¢\u0006\u0002\u0010\u0012",
   d2 = {
         "Lpkg/TestSuspendLoopTry;",
         "",
         "<init>",
         "()V",
         "indexOf",
         "",
         "src",
         "Lpkg/TestSuspendLoopTry$Source;",
         "x",
         "(Lpkg/TestSuspendLoopTry$Source;ILkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "pairs",
         "",
         "other",
         "out",
         "",
         "(Lpkg/TestSuspendLoopTry$Source;Lpkg/TestSuspendLoopTry$Source;Ljava/util/List;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "watch",
         "",
         "(Lpkg/TestSuspendLoopTry$Source;Ljava/util/List;Lkotlin/coroutines/Continuation;)Ljava/lang/Object;",
         "Source"
   }
)
public final class TestSuspendLoopTry {
   @Nullable
   public final Object indexOf(@NotNull TestSuspendLoopTry.Source param1, int param2, @NotNull Continuation<? super Integer> param3) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 000: aload 3
      // 001: instanceof pkg/TestSuspendLoopTry$indexOf$1
      // 004: ifeq 027
      // 007: aload 3
      // 008: checkcast pkg/TestSuspendLoopTry$indexOf$1
      // 00b: astore 9
      // 00d: aload 9
      // 00f: getfield pkg/TestSuspendLoopTry$indexOf$1.label I
      // 012: ldc -2147483648
      // 014: iand
      // 015: ifeq 027
      // 018: aload 9
      // 01a: dup
      // 01b: getfield pkg/TestSuspendLoopTry$indexOf$1.label I
      // 01e: ldc -2147483648
      // 020: isub
      // 021: putfield pkg/TestSuspendLoopTry$indexOf$1.label I
      // 024: goto 032
      // 027: new pkg/TestSuspendLoopTry$indexOf$1
      // 02a: dup
      // 02b: aload 0
      // 02c: aload 3
      // 02d: invokespecial pkg/TestSuspendLoopTry$indexOf$1.<init> (Lpkg/TestSuspendLoopTry;Lkotlin/coroutines/Continuation;)V
      // 030: astore 9
      // 032: aload 9
      // 034: getfield pkg/TestSuspendLoopTry$indexOf$1.result Ljava/lang/Object;
      // 037: astore 8
      // 039: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
      // 03c: astore 10
      // 03e: aload 9
      // 040: getfield pkg/TestSuspendLoopTry$indexOf$1.label I
      // 043: tableswitch 197 0 1 21 76
      // 058: aload 8
      // 05a: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 05d: aconst_null
      // 05e: astore 4
      // 060: bipush 0
      // 061: istore 5
      // 063: nop
      // 064: nop
      // 065: aload 1
      // 066: aload 9
      // 068: aload 9
      // 06a: aload 1
      // 06b: putfield pkg/TestSuspendLoopTry$indexOf$1.L$0 Ljava/lang/Object;
      // 06e: aload 9
      // 070: iload 2
      // 071: putfield pkg/TestSuspendLoopTry$indexOf$1.I$0 I
      // 074: aload 9
      // 076: iload 5
      // 078: putfield pkg/TestSuspendLoopTry$indexOf$1.I$1 I
      // 07b: aload 9
      // 07d: bipush 1
      // 07e: putfield pkg/TestSuspendLoopTry$indexOf$1.label I
      // 081: invokeinterface pkg/TestSuspendLoopTry$Source.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
      // 086: dup
      // 087: aload 10
      // 089: if_acmpne 0b0
      // 08c: aload 10
      // 08e: areturn
      // 08f: aload 9
      // 091: getfield pkg/TestSuspendLoopTry$indexOf$1.I$1 I
      // 094: istore 5
      // 096: aload 9
      // 098: getfield pkg/TestSuspendLoopTry$indexOf$1.I$0 I
      // 09b: istore 2
      // 09c: aconst_null
      // 09d: astore 4
      // 09f: aload 9
      // 0a1: getfield pkg/TestSuspendLoopTry$indexOf$1.L$0 Ljava/lang/Object;
      // 0a4: checkcast pkg/TestSuspendLoopTry$Source
      // 0a7: astore 1
      // 0a8: nop
      // 0a9: aload 8
      // 0ab: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 0ae: aload 8
      // 0b0: checkcast java/lang/Integer
      // 0b3: dup
      // 0b4: ifnull 0bd
      // 0b7: invokevirtual java/lang/Integer.intValue ()I
      // 0ba: goto 0c1
      // 0bd: pop
      // 0be: goto 0e2
      // 0c1: istore 6
      // 0c3: iload 6
      // 0c5: iload 2
      // 0c6: if_icmpne 0db
      // 0c9: iload 5
      // 0cb: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
      // 0ce: astore 7
      // 0d0: aload 1
      // 0d1: aload 4
      // 0d3: invokeinterface pkg/TestSuspendLoopTry$Source.close (Ljava/lang/Throwable;)V 2
      // 0d8: aload 7
      // 0da: areturn
      // 0db: nop
      // 0dc: iinc 5 1
      // 0df: goto 064
      // 0e2: aload 1
      // 0e3: aload 4
      // 0e5: invokeinterface pkg/TestSuspendLoopTry$Source.close (Ljava/lang/Throwable;)V 2
      // 0ea: goto 103
      // 0ed: astore 6
      // 0ef: aload 6
      // 0f1: astore 4
      // 0f3: aload 6
      // 0f5: athrow
      // 0f6: astore 6
      // 0f8: aload 1
      // 0f9: aload 4
      // 0fb: invokeinterface pkg/TestSuspendLoopTry$Source.close (Ljava/lang/Throwable;)V 2
      // 100: aload 6
      // 102: athrow
      // 103: bipush -1
      // 104: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
      // 107: areturn
      // 108: new java/lang/IllegalStateException
      // 10b: dup
      // 10c: ldc "call to 'resume' before 'invoke' with coroutine"
      // 10e: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
      // 111: athrow
      // try (38 -> 55): 102 java/lang/Throwable
      // try (72 -> 90): 102 java/lang/Throwable
      // try (95 -> 98): 102 java/lang/Throwable
      // try (38 -> 55): 107 null
      // try (72 -> 90): 107 null
      // try (95 -> 98): 107 null
      // try (102 -> 107): 107 null
      // try (107 -> 108): 107 null
   }

   @Nullable
   public final Object pairs(
      @NotNull TestSuspendLoopTry.Source param1,
      @NotNull TestSuspendLoopTry.Source param2,
      @NotNull List<Integer> param3,
      @NotNull Continuation<? super Unit> param4
   ) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 000: aload 4
      // 002: instanceof pkg/TestSuspendLoopTry$pairs$1
      // 005: ifeq 029
      // 008: aload 4
      // 00a: checkcast pkg/TestSuspendLoopTry$pairs$1
      // 00d: astore 8
      // 00f: aload 8
      // 011: getfield pkg/TestSuspendLoopTry$pairs$1.label I
      // 014: ldc -2147483648
      // 016: iand
      // 017: ifeq 029
      // 01a: aload 8
      // 01c: dup
      // 01d: getfield pkg/TestSuspendLoopTry$pairs$1.label I
      // 020: ldc -2147483648
      // 022: isub
      // 023: putfield pkg/TestSuspendLoopTry$pairs$1.label I
      // 026: goto 035
      // 029: new pkg/TestSuspendLoopTry$pairs$1
      // 02c: dup
      // 02d: aload 0
      // 02e: aload 4
      // 030: invokespecial pkg/TestSuspendLoopTry$pairs$1.<init> (Lpkg/TestSuspendLoopTry;Lkotlin/coroutines/Continuation;)V
      // 033: astore 8
      // 035: aload 8
      // 037: getfield pkg/TestSuspendLoopTry$pairs$1.result Ljava/lang/Object;
      // 03a: astore 7
      // 03c: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
      // 03f: astore 9
      // 041: aload 8
      // 043: getfield pkg/TestSuspendLoopTry$pairs$1.label I
      // 046: tableswitch 288 0 2 26 77 182
      // 060: aload 7
      // 062: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 065: aconst_null
      // 066: astore 5
      // 068: nop
      // 069: nop
      // 06a: aload 1
      // 06b: aload 8
      // 06d: aload 8
      // 06f: aload 1
      // 070: putfield pkg/TestSuspendLoopTry$pairs$1.L$0 Ljava/lang/Object;
      // 073: aload 8
      // 075: aload 2
      // 076: putfield pkg/TestSuspendLoopTry$pairs$1.L$1 Ljava/lang/Object;
      // 079: aload 8
      // 07b: aload 3
      // 07c: putfield pkg/TestSuspendLoopTry$pairs$1.L$2 Ljava/lang/Object;
      // 07f: aload 8
      // 081: bipush 1
      // 082: putfield pkg/TestSuspendLoopTry$pairs$1.label I
      // 085: invokeinterface pkg/TestSuspendLoopTry$Source.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
      // 08a: dup
      // 08b: aload 9
      // 08d: if_acmpne 0b9
      // 090: aload 9
      // 092: areturn
      // 093: aconst_null
      // 094: astore 5
      // 096: aload 8
      // 098: getfield pkg/TestSuspendLoopTry$pairs$1.L$2 Ljava/lang/Object;
      // 09b: checkcast java/util/List
      // 09e: astore 3
      // 09f: aload 8
      // 0a1: getfield pkg/TestSuspendLoopTry$pairs$1.L$1 Ljava/lang/Object;
      // 0a4: checkcast pkg/TestSuspendLoopTry$Source
      // 0a7: astore 2
      // 0a8: aload 8
      // 0aa: getfield pkg/TestSuspendLoopTry$pairs$1.L$0 Ljava/lang/Object;
      // 0ad: checkcast pkg/TestSuspendLoopTry$Source
      // 0b0: astore 1
      // 0b1: nop
      // 0b2: aload 7
      // 0b4: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 0b7: aload 7
      // 0b9: checkcast java/lang/Integer
      // 0bc: dup
      // 0bd: ifnull 0c6
      // 0c0: invokevirtual java/lang/Integer.intValue ()I
      // 0c3: goto 0ca
      // 0c6: pop
      // 0c7: goto 141
      // 0ca: istore 6
      // 0cc: aload 2
      // 0cd: aload 8
      // 0cf: aload 8
      // 0d1: aload 1
      // 0d2: putfield pkg/TestSuspendLoopTry$pairs$1.L$0 Ljava/lang/Object;
      // 0d5: aload 8
      // 0d7: aload 2
      // 0d8: putfield pkg/TestSuspendLoopTry$pairs$1.L$1 Ljava/lang/Object;
      // 0db: aload 8
      // 0dd: aload 3
      // 0de: putfield pkg/TestSuspendLoopTry$pairs$1.L$2 Ljava/lang/Object;
      // 0e1: aload 8
      // 0e3: iload 6
      // 0e5: putfield pkg/TestSuspendLoopTry$pairs$1.I$0 I
      // 0e8: aload 8
      // 0ea: bipush 2
      // 0eb: putfield pkg/TestSuspendLoopTry$pairs$1.label I
      // 0ee: invokeinterface pkg/TestSuspendLoopTry$Source.ready (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
      // 0f3: dup
      // 0f4: aload 9
      // 0f6: if_acmpne 129
      // 0f9: aload 9
      // 0fb: areturn
      // 0fc: aload 8
      // 0fe: getfield pkg/TestSuspendLoopTry$pairs$1.I$0 I
      // 101: istore 6
      // 103: aconst_null
      // 104: astore 5
      // 106: aload 8
      // 108: getfield pkg/TestSuspendLoopTry$pairs$1.L$2 Ljava/lang/Object;
      // 10b: checkcast java/util/List
      // 10e: astore 3
      // 10f: aload 8
      // 111: getfield pkg/TestSuspendLoopTry$pairs$1.L$1 Ljava/lang/Object;
      // 114: checkcast pkg/TestSuspendLoopTry$Source
      // 117: astore 2
      // 118: aload 8
      // 11a: getfield pkg/TestSuspendLoopTry$pairs$1.L$0 Ljava/lang/Object;
      // 11d: checkcast pkg/TestSuspendLoopTry$Source
      // 120: astore 1
      // 121: nop
      // 122: aload 7
      // 124: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 127: aload 7
      // 129: checkcast java/lang/Boolean
      // 12c: invokevirtual java/lang/Boolean.booleanValue ()Z
      // 12f: ifeq 069
      // 132: aload 3
      // 133: iload 6
      // 135: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
      // 138: invokeinterface java/util/List.add (Ljava/lang/Object;)Z 2
      // 13d: pop
      // 13e: goto 069
      // 141: aload 1
      // 142: aload 5
      // 144: invokeinterface pkg/TestSuspendLoopTry$Source.close (Ljava/lang/Throwable;)V 2
      // 149: goto 162
      // 14c: astore 6
      // 14e: aload 6
      // 150: astore 5
      // 152: aload 6
      // 154: athrow
      // 155: astore 6
      // 157: aload 1
      // 158: aload 5
      // 15a: invokeinterface pkg/TestSuspendLoopTry$Source.close (Ljava/lang/Throwable;)V 2
      // 15f: aload 6
      // 161: athrow
      // 162: getstatic kotlin/Unit.INSTANCE Lkotlin/Unit;
      // 165: areturn
      // 166: new java/lang/IllegalStateException
      // 169: dup
      // 16a: ldc "call to 'resume' before 'invoke' with coroutine"
      // 16c: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
      // 16f: athrow
      // try (36 -> 53): 141 java/lang/Throwable
      // try (72 -> 102): 141 java/lang/Throwable
      // try (124 -> 137): 141 java/lang/Throwable
      // try (36 -> 53): 146 null
      // try (72 -> 102): 146 null
      // try (124 -> 137): 146 null
      // try (141 -> 146): 146 null
      // try (146 -> 147): 146 null
   }

   @Nullable
   public final Object watch(@NotNull TestSuspendLoopTry.Source param1, @NotNull List<Integer> param2, @NotNull Continuation<?> param3) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 000: aload 3
      // 001: instanceof pkg/TestSuspendLoopTry$watch$1
      // 004: ifeq 027
      // 007: aload 3
      // 008: checkcast pkg/TestSuspendLoopTry$watch$1
      // 00b: astore 7
      // 00d: aload 7
      // 00f: getfield pkg/TestSuspendLoopTry$watch$1.label I
      // 012: ldc -2147483648
      // 014: iand
      // 015: ifeq 027
      // 018: aload 7
      // 01a: dup
      // 01b: getfield pkg/TestSuspendLoopTry$watch$1.label I
      // 01e: ldc -2147483648
      // 020: isub
      // 021: putfield pkg/TestSuspendLoopTry$watch$1.label I
      // 024: goto 032
      // 027: new pkg/TestSuspendLoopTry$watch$1
      // 02a: dup
      // 02b: aload 0
      // 02c: aload 3
      // 02d: invokespecial pkg/TestSuspendLoopTry$watch$1.<init> (Lpkg/TestSuspendLoopTry;Lkotlin/coroutines/Continuation;)V
      // 030: astore 7
      // 032: aload 7
      // 034: getfield pkg/TestSuspendLoopTry$watch$1.result Ljava/lang/Object;
      // 037: astore 6
      // 039: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
      // 03c: astore 8
      // 03e: aload 7
      // 040: getfield pkg/TestSuspendLoopTry$watch$1.label I
      // 043: tableswitch 329 0 3 29 81 196 280
      // 060: aload 6
      // 062: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 065: nop
      // 066: bipush -1
      // 067: istore 4
      // 069: nop
      // 06a: aload 1
      // 06b: aload 7
      // 06d: aload 7
      // 06f: aload 1
      // 070: putfield pkg/TestSuspendLoopTry$watch$1.L$0 Ljava/lang/Object;
      // 073: aload 7
      // 075: aload 2
      // 076: putfield pkg/TestSuspendLoopTry$watch$1.L$1 Ljava/lang/Object;
      // 079: aload 7
      // 07b: iload 4
      // 07d: putfield pkg/TestSuspendLoopTry$watch$1.I$0 I
      // 080: aload 7
      // 082: bipush 1
      // 083: putfield pkg/TestSuspendLoopTry$watch$1.label I
      // 086: invokeinterface pkg/TestSuspendLoopTry$Source.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
      // 08b: dup
      // 08c: aload 8
      // 08e: if_acmpne 0b5
      // 091: aload 8
      // 093: areturn
      // 094: aload 7
      // 096: getfield pkg/TestSuspendLoopTry$watch$1.I$0 I
      // 099: istore 4
      // 09b: aload 7
      // 09d: getfield pkg/TestSuspendLoopTry$watch$1.L$1 Ljava/lang/Object;
      // 0a0: checkcast java/util/List
      // 0a3: astore 2
      // 0a4: aload 7
      // 0a6: getfield pkg/TestSuspendLoopTry$watch$1.L$0 Ljava/lang/Object;
      // 0a9: checkcast pkg/TestSuspendLoopTry$Source
      // 0ac: astore 1
      // 0ad: nop
      // 0ae: aload 6
      // 0b0: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 0b3: aload 6
      // 0b5: checkcast java/lang/Integer
      // 0b8: dup
      // 0b9: ifnull 0c2
      // 0bc: invokevirtual java/lang/Integer.intValue ()I
      // 0bf: goto 0c4
      // 0c2: pop
      // 0c3: bipush -1
      // 0c4: istore 5
      // 0c6: iload 5
      // 0c8: iload 4
      // 0ca: if_icmpeq 0dd
      // 0cd: aload 2
      // 0ce: iload 5
      // 0d0: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
      // 0d3: invokeinterface java/util/List.add (Ljava/lang/Object;)Z 2
      // 0d8: pop
      // 0d9: iload 5
      // 0db: istore 4
      // 0dd: aload 1
      // 0de: aload 7
      // 0e0: aload 7
      // 0e2: aload 1
      // 0e3: putfield pkg/TestSuspendLoopTry$watch$1.L$0 Ljava/lang/Object;
      // 0e6: aload 7
      // 0e8: aload 2
      // 0e9: putfield pkg/TestSuspendLoopTry$watch$1.L$1 Ljava/lang/Object;
      // 0ec: aload 7
      // 0ee: iload 4
      // 0f0: putfield pkg/TestSuspendLoopTry$watch$1.I$0 I
      // 0f3: aload 7
      // 0f5: bipush 2
      // 0f6: putfield pkg/TestSuspendLoopTry$watch$1.label I
      // 0f9: invokeinterface pkg/TestSuspendLoopTry$Source.ready (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
      // 0fe: dup
      // 0ff: aload 8
      // 101: if_acmpne 128
      // 104: aload 8
      // 106: areturn
      // 107: aload 7
      // 109: getfield pkg/TestSuspendLoopTry$watch$1.I$0 I
      // 10c: istore 4
      // 10e: aload 7
      // 110: getfield pkg/TestSuspendLoopTry$watch$1.L$1 Ljava/lang/Object;
      // 113: checkcast java/util/List
      // 116: astore 2
      // 117: aload 7
      // 119: getfield pkg/TestSuspendLoopTry$watch$1.L$0 Ljava/lang/Object;
      // 11c: checkcast pkg/TestSuspendLoopTry$Source
      // 11f: astore 1
      // 120: nop
      // 121: aload 6
      // 123: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 126: aload 6
      // 128: checkcast java/lang/Boolean
      // 12b: invokevirtual java/lang/Boolean.booleanValue ()Z
      // 12e: ifne 069
      // 131: aload 1
      // 132: aload 7
      // 134: aload 7
      // 136: aload 1
      // 137: putfield pkg/TestSuspendLoopTry$watch$1.L$0 Ljava/lang/Object;
      // 13a: aload 7
      // 13c: aload 2
      // 13d: putfield pkg/TestSuspendLoopTry$watch$1.L$1 Ljava/lang/Object;
      // 140: aload 7
      // 142: iload 4
      // 144: putfield pkg/TestSuspendLoopTry$watch$1.I$0 I
      // 147: aload 7
      // 149: bipush 3
      // 14a: putfield pkg/TestSuspendLoopTry$watch$1.label I
      // 14d: invokeinterface pkg/TestSuspendLoopTry$Source.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
      // 152: dup
      // 153: aload 8
      // 155: if_acmpne 17c
      // 158: aload 8
      // 15a: areturn
      // 15b: aload 7
      // 15d: getfield pkg/TestSuspendLoopTry$watch$1.I$0 I
      // 160: istore 4
      // 162: aload 7
      // 164: getfield pkg/TestSuspendLoopTry$watch$1.L$1 Ljava/lang/Object;
      // 167: checkcast java/util/List
      // 16a: astore 2
      // 16b: aload 7
      // 16d: getfield pkg/TestSuspendLoopTry$watch$1.L$0 Ljava/lang/Object;
      // 170: checkcast pkg/TestSuspendLoopTry$Source
      // 173: astore 1
      // 174: nop
      // 175: aload 6
      // 177: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 17a: aload 6
      // 17c: pop
      // 17d: goto 069
      // 180: astore 4
      // 182: aload 1
      // 183: aconst_null
      // 184: invokeinterface pkg/TestSuspendLoopTry$Source.close (Ljava/lang/Throwable;)V 2
      // 189: aload 4
      // 18b: athrow
      // 18c: new java/lang/IllegalStateException
      // 18f: dup
      // 190: ldc "call to 'resume' before 'invoke' with coroutine"
      // 192: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
      // 195: athrow
      // try (34 -> 53): 166 null
      // try (69 -> 106): 166 null
      // try (122 -> 144): 166 null
      // try (160 -> 166): 166 null
      // try (166 -> 167): 166 null
   }

   @Metadata(
      mv = {2, 0, 0},
      k = 1,
      xi = 48,
      d1 = "\u0000\"\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0010\b\n\u0002\b\u0002\n\u0002\u0010\u000b\n\u0000\n\u0002\u0010\u0002\n\u0000\n\u0002\u0010\u0003\bf\u0018\u00002\u00020\u0001J\u0010\u0010\u0002\u001a\u0004\u0018\u00010\u0003H¦@¢\u0006\u0002\u0010\u0004J\u000e\u0010\u0005\u001a\u00020\u0006H¦@¢\u0006\u0002\u0010\u0004J\u0012\u0010\u0007\u001a\u00020\b2\b\u0010\t\u001a\u0004\u0018\u00010\nH&",
      d2 = {"Lpkg/TestSuspendLoopTry$Source;", "", "next", "", "(Lkotlin/coroutines/Continuation;)Ljava/lang/Object;", "ready", "", "close", "", "cause", ""}
   )
   public interface Source {
      @Nullable
      Object next(@NotNull Continuation<? super Integer> var1);

      @Nullable
      Object ready(@NotNull Continuation<? super Boolean> var1);

      void close(@Nullable Throwable var1);
   }
}
