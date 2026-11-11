package pkg

import kotlin.coroutines.Continuation

public class TestSuspendMemberMachine(src: MemberSource) {
   private final val src: MemberSource

   public final var total: Long
      internal set

   init {
      this.src = src
   }

   public suspend fun addAll(n: Int): Long {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 2
      // 01: instanceof pkg/TestSuspendMemberMachine$addAll$1
      // 04: ifeq 27
      // 07: aload 2
      // 08: checkcast pkg/TestSuspendMemberMachine$addAll$1
      // 0b: astore 9
      // 0d: aload 9
      // 0f: getfield pkg/TestSuspendMemberMachine$addAll$1.label I
      // 12: ldc -2147483648
      // 14: iand
      // 15: ifeq 27
      // 18: aload 9
      // 1a: dup
      // 1b: getfield pkg/TestSuspendMemberMachine$addAll$1.label I
      // 1e: ldc -2147483648
      // 20: isub
      // 21: putfield pkg/TestSuspendMemberMachine$addAll$1.label I
      // 24: goto 32
      // 27: new pkg/TestSuspendMemberMachine$addAll$1
      // 2a: dup
      // 2b: aload 0
      // 2c: aload 2
      // 2d: invokespecial pkg/TestSuspendMemberMachine$addAll$1.<init> (Lpkg/TestSuspendMemberMachine;Lkotlin/coroutines/Continuation;)V
      // 30: astore 9
      // 32: aload 9
      // 34: getfield pkg/TestSuspendMemberMachine$addAll$1.result Ljava/lang/Object;
      // 37: astore 8
      // 39: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
      // 3c: astore 10
      // 3e: aload 9
      // 40: getfield pkg/TestSuspendMemberMachine$addAll$1.label I
      // 43: tableswitch 177 0 1 21 100
      // 58: aload 8
      // 5a: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 5d: bipush 0
      // 5e: istore 3
      // 5f: iload 3
      // 60: iload 1
      // 61: if_icmpge ec
      // 64: aload 0
      // 65: aload 0
      // 66: getfield pkg/TestSuspendMemberMachine.total J
      // 69: lstore 5
      // 6b: astore 4
      // 6d: aload 0
      // 6e: getfield pkg/TestSuspendMemberMachine.src Lpkg/MemberSource;
      // 71: aload 9
      // 73: aload 9
      // 75: aload 0
      // 76: putfield pkg/TestSuspendMemberMachine$addAll$1.L$0 Ljava/lang/Object;
      // 79: aload 9
      // 7b: aload 4
      // 7d: putfield pkg/TestSuspendMemberMachine$addAll$1.L$1 Ljava/lang/Object;
      // 80: aload 9
      // 82: iload 1
      // 83: putfield pkg/TestSuspendMemberMachine$addAll$1.I$0 I
      // 86: aload 9
      // 88: iload 3
      // 89: putfield pkg/TestSuspendMemberMachine$addAll$1.I$1 I
      // 8c: aload 9
      // 8e: lload 5
      // 90: putfield pkg/TestSuspendMemberMachine$addAll$1.J$0 J
      // 93: aload 9
      // 95: bipush 1
      // 96: putfield pkg/TestSuspendMemberMachine$addAll$1.label I
      // 99: invokeinterface pkg/MemberSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
      // 9e: dup
      // 9f: aload 10
      // a1: if_acmpne d4
      // a4: aload 10
      // a6: areturn
      // a7: aload 9
      // a9: getfield pkg/TestSuspendMemberMachine$addAll$1.J$0 J
      // ac: lstore 5
      // ae: aload 9
      // b0: getfield pkg/TestSuspendMemberMachine$addAll$1.I$1 I
      // b3: istore 3
      // b4: aload 9
      // b6: getfield pkg/TestSuspendMemberMachine$addAll$1.I$0 I
      // b9: istore 1
      // ba: aload 9
      // bc: getfield pkg/TestSuspendMemberMachine$addAll$1.L$1 Ljava/lang/Object;
      // bf: checkcast pkg/TestSuspendMemberMachine
      // c2: astore 4
      // c4: aload 9
      // c6: getfield pkg/TestSuspendMemberMachine$addAll$1.L$0 Ljava/lang/Object;
      // c9: checkcast pkg/TestSuspendMemberMachine
      // cc: astore 0
      // cd: aload 8
      // cf: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // d2: aload 8
      // d4: astore 7
      // d6: aload 4
      // d8: lload 5
      // da: aload 7
      // dc: checkcast java/lang/Number
      // df: invokevirtual java/lang/Number.longValue ()J
      // e2: ladd
      // e3: putfield pkg/TestSuspendMemberMachine.total J
      // e6: iinc 3 1
      // e9: goto 5f
      // ec: aload 0
      // ed: getfield pkg/TestSuspendMemberMachine.total J
      // f0: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxLong (J)Ljava/lang/Long;
      // f3: areturn
      // f4: new java/lang/IllegalStateException
      // f7: dup
      // f8: ldc "call to 'resume' before 'invoke' with coroutine"
      // fa: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
      // fd: athrow
   }

   public suspend fun label(prefix: String): String {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 000: aload 2
      // 001: instanceof pkg/TestSuspendMemberMachine$label$1
      // 004: ifeq 027
      // 007: aload 2
      // 008: checkcast pkg/TestSuspendMemberMachine$label$1
      // 00b: astore 6
      // 00d: aload 6
      // 00f: getfield pkg/TestSuspendMemberMachine$label$1.label I
      // 012: ldc -2147483648
      // 014: iand
      // 015: ifeq 027
      // 018: aload 6
      // 01a: dup
      // 01b: getfield pkg/TestSuspendMemberMachine$label$1.label I
      // 01e: ldc -2147483648
      // 020: isub
      // 021: putfield pkg/TestSuspendMemberMachine$label$1.label I
      // 024: goto 032
      // 027: new pkg/TestSuspendMemberMachine$label$1
      // 02a: dup
      // 02b: aload 0
      // 02c: aload 2
      // 02d: invokespecial pkg/TestSuspendMemberMachine$label$1.<init> (Lpkg/TestSuspendMemberMachine;Lkotlin/coroutines/Continuation;)V
      // 030: astore 6
      // 032: aload 6
      // 034: getfield pkg/TestSuspendMemberMachine$label$1.result Ljava/lang/Object;
      // 037: astore 5
      // 039: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
      // 03c: astore 7
      // 03e: aload 6
      // 040: getfield pkg/TestSuspendMemberMachine$label$1.label I
      // 043: tableswitch 182 0 2 25 68 139
      // 05c: aload 5
      // 05e: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 061: aload 0
      // 062: getfield pkg/TestSuspendMemberMachine.src Lpkg/MemberSource;
      // 065: aload 6
      // 067: aload 6
      // 069: aload 0
      // 06a: putfield pkg/TestSuspendMemberMachine$label$1.L$0 Ljava/lang/Object;
      // 06d: aload 6
      // 06f: aload 1
      // 070: putfield pkg/TestSuspendMemberMachine$label$1.L$1 Ljava/lang/Object;
      // 073: aload 6
      // 075: bipush 1
      // 076: putfield pkg/TestSuspendMemberMachine$label$1.label I
      // 079: invokeinterface pkg/MemberSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
      // 07e: dup
      // 07f: aload 7
      // 081: if_acmpne 0a0
      // 084: aload 7
      // 086: areturn
      // 087: aload 6
      // 089: getfield pkg/TestSuspendMemberMachine$label$1.L$1 Ljava/lang/Object;
      // 08c: checkcast java/lang/String
      // 08f: astore 1
      // 090: aload 6
      // 092: getfield pkg/TestSuspendMemberMachine$label$1.L$0 Ljava/lang/Object;
      // 095: checkcast pkg/TestSuspendMemberMachine
      // 098: astore 0
      // 099: aload 5
      // 09b: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 09e: aload 5
      // 0a0: checkcast java/lang/Number
      // 0a3: invokevirtual java/lang/Number.intValue ()I
      // 0a6: istore 3
      // 0a7: aload 0
      // 0a8: getfield pkg/TestSuspendMemberMachine.src Lpkg/MemberSource;
      // 0ab: iload 3
      // 0ac: aload 6
      // 0ae: aload 6
      // 0b0: aload 0
      // 0b1: putfield pkg/TestSuspendMemberMachine$label$1.L$0 Ljava/lang/Object;
      // 0b4: aload 6
      // 0b6: aload 1
      // 0b7: putfield pkg/TestSuspendMemberMachine$label$1.L$1 Ljava/lang/Object;
      // 0ba: aload 6
      // 0bc: bipush 2
      // 0bd: putfield pkg/TestSuspendMemberMachine$label$1.label I
      // 0c0: invokeinterface pkg/MemberSource.text (ILkotlin/coroutines/Continuation;)Ljava/lang/Object; 3
      // 0c5: dup
      // 0c6: aload 7
      // 0c8: if_acmpne 0e7
      // 0cb: aload 7
      // 0cd: areturn
      // 0ce: aload 6
      // 0d0: getfield pkg/TestSuspendMemberMachine$label$1.L$1 Ljava/lang/Object;
      // 0d3: checkcast java/lang/String
      // 0d6: astore 1
      // 0d7: aload 6
      // 0d9: getfield pkg/TestSuspendMemberMachine$label$1.L$0 Ljava/lang/Object;
      // 0dc: checkcast pkg/TestSuspendMemberMachine
      // 0df: astore 0
      // 0e0: aload 5
      // 0e2: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 0e5: aload 5
      // 0e7: checkcast java/lang/String
      // 0ea: astore 4
      // 0ec: aload 1
      // 0ed: aload 4
      // 0ef: aload 0
      // 0f0: getfield pkg/TestSuspendMemberMachine.total J
      // 0f3: invokedynamic makeConcatWithConstants (Ljava/lang/String;Ljava/lang/String;J)Ljava/lang/String; bsm=java/lang/invoke/StringConcatFactory.makeConcatWithConstants (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite; args=[ "\u0001\u0001\u0001" ]
      // 0f8: areturn
      // 0f9: new java/lang/IllegalStateException
      // 0fc: dup
      // 0fd: ldc "call to 'resume' before 'invoke' with coroutine"
      // 0ff: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
      // 102: athrow
   }
}
