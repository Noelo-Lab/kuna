@file:SourceDebugExtension(["SMAP\nTestSuspendStateMachine.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestSuspendStateMachine.kt\npkg/TestSuspendStateMachineKt\n+ 2 fake.kt\nkotlin/jvm/internal/FakeKt\n*L\n1#1,80:1\n62#1:81\n1#2:82\n*S KotlinDebug\n*F\n+ 1 TestSuspendStateMachine.kt\npkg/TestSuspendStateMachineKt\n*L\n64#1:81\n*E\n"])

package pkg

import kotlin.coroutines.Continuation
import kotlin.jvm.internal.InlineMarker
import kotlin.jvm.internal.SourceDebugExtension

public suspend fun combine(src: StateSource, scale: Long, flag: Boolean): String {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 000: aload 4
   // 002: instanceof pkg/TestSuspendStateMachineKt$combine$1
   // 005: ifeq 029
   // 008: aload 4
   // 00a: checkcast pkg/TestSuspendStateMachineKt$combine$1
   // 00d: astore 11
   // 00f: aload 11
   // 011: getfield pkg/TestSuspendStateMachineKt$combine$1.label I
   // 014: ldc -2147483648
   // 016: iand
   // 017: ifeq 029
   // 01a: aload 11
   // 01c: dup
   // 01d: getfield pkg/TestSuspendStateMachineKt$combine$1.label I
   // 020: ldc -2147483648
   // 022: isub
   // 023: putfield pkg/TestSuspendStateMachineKt$combine$1.label I
   // 026: goto 034
   // 029: new pkg/TestSuspendStateMachineKt$combine$1
   // 02c: dup
   // 02d: aload 4
   // 02f: invokespecial pkg/TestSuspendStateMachineKt$combine$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 032: astore 11
   // 034: aload 11
   // 036: getfield pkg/TestSuspendStateMachineKt$combine$1.result Ljava/lang/Object;
   // 039: astore 10
   // 03b: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 03e: astore 12
   // 040: aload 11
   // 042: getfield pkg/TestSuspendStateMachineKt$combine$1.label I
   // 045: tableswitch 364 0 3 31 77 177 294
   // 064: aload 10
   // 066: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 069: aload 0
   // 06a: aload 11
   // 06c: aload 11
   // 06e: aload 0
   // 06f: putfield pkg/TestSuspendStateMachineKt$combine$1.L$0 Ljava/lang/Object;
   // 072: aload 11
   // 074: lload 1
   // 075: putfield pkg/TestSuspendStateMachineKt$combine$1.J$0 J
   // 078: aload 11
   // 07a: iload 3
   // 07b: putfield pkg/TestSuspendStateMachineKt$combine$1.Z$0 Z
   // 07e: aload 11
   // 080: bipush 1
   // 081: putfield pkg/TestSuspendStateMachineKt$combine$1.label I
   // 084: invokeinterface pkg/StateSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 089: dup
   // 08a: aload 12
   // 08c: if_acmpne 0ae
   // 08f: aload 12
   // 091: areturn
   // 092: aload 11
   // 094: getfield pkg/TestSuspendStateMachineKt$combine$1.Z$0 Z
   // 097: istore 3
   // 098: aload 11
   // 09a: getfield pkg/TestSuspendStateMachineKt$combine$1.J$0 J
   // 09d: lstore 1
   // 09e: aload 11
   // 0a0: getfield pkg/TestSuspendStateMachineKt$combine$1.L$0 Ljava/lang/Object;
   // 0a3: checkcast pkg/StateSource
   // 0a6: astore 0
   // 0a7: aload 10
   // 0a9: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 0ac: aload 10
   // 0ae: checkcast java/lang/Number
   // 0b1: invokevirtual java/lang/Number.intValue ()I
   // 0b4: istore 5
   // 0b6: iload 5
   // 0b8: i2d
   // 0b9: ldc2_w 1.5
   // 0bc: dmul
   // 0bd: dstore 6
   // 0bf: aload 0
   // 0c0: aload 11
   // 0c2: aload 11
   // 0c4: aload 0
   // 0c5: putfield pkg/TestSuspendStateMachineKt$combine$1.L$0 Ljava/lang/Object;
   // 0c8: aload 11
   // 0ca: lload 1
   // 0cb: putfield pkg/TestSuspendStateMachineKt$combine$1.J$0 J
   // 0ce: aload 11
   // 0d0: iload 3
   // 0d1: putfield pkg/TestSuspendStateMachineKt$combine$1.Z$0 Z
   // 0d4: aload 11
   // 0d6: iload 5
   // 0d8: putfield pkg/TestSuspendStateMachineKt$combine$1.I$0 I
   // 0db: aload 11
   // 0dd: dload 6
   // 0df: putfield pkg/TestSuspendStateMachineKt$combine$1.D$0 D
   // 0e2: aload 11
   // 0e4: bipush 2
   // 0e5: putfield pkg/TestSuspendStateMachineKt$combine$1.label I
   // 0e8: invokeinterface pkg/StateSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 0ed: dup
   // 0ee: aload 12
   // 0f0: if_acmpne 120
   // 0f3: aload 12
   // 0f5: areturn
   // 0f6: aload 11
   // 0f8: getfield pkg/TestSuspendStateMachineKt$combine$1.D$0 D
   // 0fb: dstore 6
   // 0fd: aload 11
   // 0ff: getfield pkg/TestSuspendStateMachineKt$combine$1.I$0 I
   // 102: istore 5
   // 104: aload 11
   // 106: getfield pkg/TestSuspendStateMachineKt$combine$1.Z$0 Z
   // 109: istore 3
   // 10a: aload 11
   // 10c: getfield pkg/TestSuspendStateMachineKt$combine$1.J$0 J
   // 10f: lstore 1
   // 110: aload 11
   // 112: getfield pkg/TestSuspendStateMachineKt$combine$1.L$0 Ljava/lang/Object;
   // 115: checkcast pkg/StateSource
   // 118: astore 0
   // 119: aload 10
   // 11b: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 11e: aload 10
   // 120: checkcast java/lang/Number
   // 123: invokevirtual java/lang/Number.intValue ()I
   // 126: istore 8
   // 128: aload 0
   // 129: iload 5
   // 12b: iload 8
   // 12d: iadd
   // 12e: aload 11
   // 130: aload 11
   // 132: aconst_null
   // 133: putfield pkg/TestSuspendStateMachineKt$combine$1.L$0 Ljava/lang/Object;
   // 136: aload 11
   // 138: lload 1
   // 139: putfield pkg/TestSuspendStateMachineKt$combine$1.J$0 J
   // 13c: aload 11
   // 13e: iload 3
   // 13f: putfield pkg/TestSuspendStateMachineKt$combine$1.Z$0 Z
   // 142: aload 11
   // 144: iload 5
   // 146: putfield pkg/TestSuspendStateMachineKt$combine$1.I$0 I
   // 149: aload 11
   // 14b: dload 6
   // 14d: putfield pkg/TestSuspendStateMachineKt$combine$1.D$0 D
   // 150: aload 11
   // 152: iload 8
   // 154: putfield pkg/TestSuspendStateMachineKt$combine$1.I$1 I
   // 157: aload 11
   // 159: bipush 3
   // 15a: putfield pkg/TestSuspendStateMachineKt$combine$1.label I
   // 15d: invokeinterface pkg/StateSource.text (ILkotlin/coroutines/Continuation;)Ljava/lang/Object; 3
   // 162: dup
   // 163: aload 12
   // 165: if_acmpne 193
   // 168: aload 12
   // 16a: areturn
   // 16b: aload 11
   // 16d: getfield pkg/TestSuspendStateMachineKt$combine$1.I$1 I
   // 170: istore 8
   // 172: aload 11
   // 174: getfield pkg/TestSuspendStateMachineKt$combine$1.D$0 D
   // 177: dstore 6
   // 179: aload 11
   // 17b: getfield pkg/TestSuspendStateMachineKt$combine$1.I$0 I
   // 17e: istore 5
   // 180: aload 11
   // 182: getfield pkg/TestSuspendStateMachineKt$combine$1.Z$0 Z
   // 185: istore 3
   // 186: aload 11
   // 188: getfield pkg/TestSuspendStateMachineKt$combine$1.J$0 J
   // 18b: lstore 1
   // 18c: aload 10
   // 18e: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 191: aload 10
   // 193: checkcast java/lang/String
   // 196: astore 9
   // 198: iload 3
   // 199: ifeq 1ae
   // 19c: aload 9
   // 19e: iload 5
   // 1a0: iload 8
   // 1a2: iadd
   // 1a3: lload 1
   // 1a4: dload 6
   // 1a6: invokedynamic makeConcatWithConstants (Ljava/lang/String;IJD)Ljava/lang/String; bsm=java/lang/invoke/StringConcatFactory.makeConcatWithConstants (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite; args=[ "\u0001:\u0001:\u0001:\u0001" ]
   // 1ab: goto 1b0
   // 1ae: aload 9
   // 1b0: areturn
   // 1b1: new java/lang/IllegalStateException
   // 1b4: dup
   // 1b5: ldc "call to 'resume' before 'invoke' with coroutine"
   // 1b7: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // 1ba: athrow
}

public suspend fun firstAbove(src: StateSource, limit: Int, tries: Int): Int? {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 3
   // 01: instanceof pkg/TestSuspendStateMachineKt$firstAbove$1
   // 04: ifeq 27
   // 07: aload 3
   // 08: checkcast pkg/TestSuspendStateMachineKt$firstAbove$1
   // 0b: astore 7
   // 0d: aload 7
   // 0f: getfield pkg/TestSuspendStateMachineKt$firstAbove$1.label I
   // 12: ldc -2147483648
   // 14: iand
   // 15: ifeq 27
   // 18: aload 7
   // 1a: dup
   // 1b: getfield pkg/TestSuspendStateMachineKt$firstAbove$1.label I
   // 1e: ldc -2147483648
   // 20: isub
   // 21: putfield pkg/TestSuspendStateMachineKt$firstAbove$1.label I
   // 24: goto 31
   // 27: new pkg/TestSuspendStateMachineKt$firstAbove$1
   // 2a: dup
   // 2b: aload 3
   // 2c: invokespecial pkg/TestSuspendStateMachineKt$firstAbove$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 2f: astore 7
   // 31: aload 7
   // 33: getfield pkg/TestSuspendStateMachineKt$firstAbove$1.result Ljava/lang/Object;
   // 36: astore 6
   // 38: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 3b: astore 8
   // 3d: aload 7
   // 3f: getfield pkg/TestSuspendStateMachineKt$firstAbove$1.label I
   // 42: tableswitch 134 0 1 22 77
   // 58: aload 6
   // 5a: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 5d: iload 2
   // 5e: istore 4
   // 60: iload 4
   // 62: ifle c6
   // 65: aload 0
   // 66: aload 7
   // 68: aload 7
   // 6a: aload 0
   // 6b: putfield pkg/TestSuspendStateMachineKt$firstAbove$1.L$0 Ljava/lang/Object;
   // 6e: aload 7
   // 70: iload 1
   // 71: putfield pkg/TestSuspendStateMachineKt$firstAbove$1.I$0 I
   // 74: aload 7
   // 76: iload 4
   // 78: putfield pkg/TestSuspendStateMachineKt$firstAbove$1.I$1 I
   // 7b: aload 7
   // 7d: bipush 1
   // 7e: putfield pkg/TestSuspendStateMachineKt$firstAbove$1.label I
   // 81: invokeinterface pkg/StateSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 86: dup
   // 87: aload 8
   // 89: if_acmpne ac
   // 8c: aload 8
   // 8e: areturn
   // 8f: aload 7
   // 91: getfield pkg/TestSuspendStateMachineKt$firstAbove$1.I$1 I
   // 94: istore 4
   // 96: aload 7
   // 98: getfield pkg/TestSuspendStateMachineKt$firstAbove$1.I$0 I
   // 9b: istore 1
   // 9c: aload 7
   // 9e: getfield pkg/TestSuspendStateMachineKt$firstAbove$1.L$0 Ljava/lang/Object;
   // a1: checkcast pkg/StateSource
   // a4: astore 0
   // a5: aload 6
   // a7: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // aa: aload 6
   // ac: checkcast java/lang/Number
   // af: invokevirtual java/lang/Number.intValue ()I
   // b2: istore 5
   // b4: iload 5
   // b6: iload 1
   // b7: if_icmple c0
   // ba: iload 5
   // bc: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
   // bf: areturn
   // c0: iinc 4 -1
   // c3: goto 60
   // c6: aconst_null
   // c7: areturn
   // c8: new java/lang/IllegalStateException
   // cb: dup
   // cc: ldc "call to 'resume' before 'invoke' with coroutine"
   // ce: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // d1: athrow
}

public suspend fun guarded(src: StateSource): Int {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 1
   // 01: instanceof pkg/TestSuspendStateMachineKt$guarded$1
   // 04: ifeq 27
   // 07: aload 1
   // 08: checkcast pkg/TestSuspendStateMachineKt$guarded$1
   // 0b: astore 7
   // 0d: aload 7
   // 0f: getfield pkg/TestSuspendStateMachineKt$guarded$1.label I
   // 12: ldc -2147483648
   // 14: iand
   // 15: ifeq 27
   // 18: aload 7
   // 1a: dup
   // 1b: getfield pkg/TestSuspendStateMachineKt$guarded$1.label I
   // 1e: ldc -2147483648
   // 20: isub
   // 21: putfield pkg/TestSuspendStateMachineKt$guarded$1.label I
   // 24: goto 31
   // 27: new pkg/TestSuspendStateMachineKt$guarded$1
   // 2a: dup
   // 2b: aload 1
   // 2c: invokespecial pkg/TestSuspendStateMachineKt$guarded$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 2f: astore 7
   // 31: aload 7
   // 33: getfield pkg/TestSuspendStateMachineKt$guarded$1.result Ljava/lang/Object;
   // 36: astore 6
   // 38: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 3b: astore 8
   // 3d: aload 7
   // 3f: getfield pkg/TestSuspendStateMachineKt$guarded$1.label I
   // 42: tableswitch 162 0 2 26 61 122
   // 5c: aload 6
   // 5e: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 61: nop
   // 62: aload 0
   // 63: aload 7
   // 65: aload 7
   // 67: aload 0
   // 68: putfield pkg/TestSuspendStateMachineKt$guarded$1.L$0 Ljava/lang/Object;
   // 6b: aload 7
   // 6d: bipush 1
   // 6e: putfield pkg/TestSuspendStateMachineKt$guarded$1.label I
   // 71: invokeinterface pkg/StateSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 76: dup
   // 77: aload 8
   // 79: if_acmpne 90
   // 7c: aload 8
   // 7e: areturn
   // 7f: aload 7
   // 81: getfield pkg/TestSuspendStateMachineKt$guarded$1.L$0 Ljava/lang/Object;
   // 84: checkcast pkg/StateSource
   // 87: astore 0
   // 88: nop
   // 89: aload 6
   // 8b: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 8e: aload 6
   // 90: checkcast java/lang/Number
   // 93: invokevirtual java/lang/Number.intValue ()I
   // 96: istore 4
   // 98: aload 0
   // 99: aload 7
   // 9b: aload 7
   // 9d: aconst_null
   // 9e: putfield pkg/TestSuspendStateMachineKt$guarded$1.L$0 Ljava/lang/Object;
   // a1: aload 7
   // a3: iload 4
   // a5: putfield pkg/TestSuspendStateMachineKt$guarded$1.I$0 I
   // a8: aload 7
   // aa: bipush 2
   // ab: putfield pkg/TestSuspendStateMachineKt$guarded$1.label I
   // ae: invokeinterface pkg/StateSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // b3: dup
   // b4: aload 8
   // b6: if_acmpne cb
   // b9: aload 8
   // bb: areturn
   // bc: aload 7
   // be: getfield pkg/TestSuspendStateMachineKt$guarded$1.I$0 I
   // c1: istore 4
   // c3: nop
   // c4: aload 6
   // c6: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // c9: aload 6
   // cb: astore 5
   // cd: iload 4
   // cf: aload 5
   // d1: checkcast java/lang/Number
   // d4: invokevirtual java/lang/Number.intValue ()I
   // d7: iadd
   // d8: istore 2
   // d9: goto df
   // dc: astore 3
   // dd: bipush -1
   // de: istore 2
   // df: iload 2
   // e0: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
   // e3: areturn
   // e4: new java/lang/IllegalStateException
   // e7: dup
   // e8: ldc "call to 'resume' before 'invoke' with coroutine"
   // ea: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // ed: athrow
}

public suspend fun closing(src: StateSource, log: MutableList<String>): String {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 000: aload 2
   // 001: instanceof pkg/TestSuspendStateMachineKt$closing$1
   // 004: ifeq 027
   // 007: aload 2
   // 008: checkcast pkg/TestSuspendStateMachineKt$closing$1
   // 00b: astore 7
   // 00d: aload 7
   // 00f: getfield pkg/TestSuspendStateMachineKt$closing$1.label I
   // 012: ldc -2147483648
   // 014: iand
   // 015: ifeq 027
   // 018: aload 7
   // 01a: dup
   // 01b: getfield pkg/TestSuspendStateMachineKt$closing$1.label I
   // 01e: ldc -2147483648
   // 020: isub
   // 021: putfield pkg/TestSuspendStateMachineKt$closing$1.label I
   // 024: goto 031
   // 027: new pkg/TestSuspendStateMachineKt$closing$1
   // 02a: dup
   // 02b: aload 2
   // 02c: invokespecial pkg/TestSuspendStateMachineKt$closing$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 02f: astore 7
   // 031: aload 7
   // 033: getfield pkg/TestSuspendStateMachineKt$closing$1.result Ljava/lang/Object;
   // 036: astore 6
   // 038: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 03b: astore 8
   // 03d: aload 7
   // 03f: getfield pkg/TestSuspendStateMachineKt$closing$1.label I
   // 042: tableswitch 208 0 2 26 80 150
   // 05c: aload 6
   // 05e: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 061: aload 1
   // 062: ldc "open"
   // 064: invokeinterface java/util/List.add (Ljava/lang/Object;)Z 2
   // 069: pop
   // 06a: nop
   // 06b: aload 0
   // 06c: astore 5
   // 06e: aload 0
   // 06f: aload 7
   // 071: aload 7
   // 073: aload 1
   // 074: putfield pkg/TestSuspendStateMachineKt$closing$1.L$0 Ljava/lang/Object;
   // 077: aload 7
   // 079: aload 5
   // 07b: putfield pkg/TestSuspendStateMachineKt$closing$1.L$1 Ljava/lang/Object;
   // 07e: aload 7
   // 080: bipush 1
   // 081: putfield pkg/TestSuspendStateMachineKt$closing$1.label I
   // 084: invokeinterface pkg/StateSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 089: dup
   // 08a: aload 8
   // 08c: if_acmpne 0ad
   // 08f: aload 8
   // 091: areturn
   // 092: aload 7
   // 094: getfield pkg/TestSuspendStateMachineKt$closing$1.L$1 Ljava/lang/Object;
   // 097: checkcast pkg/StateSource
   // 09a: astore 5
   // 09c: aload 7
   // 09e: getfield pkg/TestSuspendStateMachineKt$closing$1.L$0 Ljava/lang/Object;
   // 0a1: checkcast java/util/List
   // 0a4: astore 1
   // 0a5: nop
   // 0a6: aload 6
   // 0a8: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 0ab: aload 6
   // 0ad: aload 5
   // 0af: swap
   // 0b0: checkcast java/lang/Number
   // 0b3: invokevirtual java/lang/Number.intValue ()I
   // 0b6: aload 7
   // 0b8: aload 7
   // 0ba: aload 1
   // 0bb: putfield pkg/TestSuspendStateMachineKt$closing$1.L$0 Ljava/lang/Object;
   // 0be: aload 7
   // 0c0: aconst_null
   // 0c1: putfield pkg/TestSuspendStateMachineKt$closing$1.L$1 Ljava/lang/Object;
   // 0c4: aload 7
   // 0c6: bipush 2
   // 0c7: putfield pkg/TestSuspendStateMachineKt$closing$1.label I
   // 0ca: invokeinterface pkg/StateSource.text (ILkotlin/coroutines/Continuation;)Ljava/lang/Object; 3
   // 0cf: dup
   // 0d0: aload 8
   // 0d2: if_acmpne 0e9
   // 0d5: aload 8
   // 0d7: areturn
   // 0d8: aload 7
   // 0da: getfield pkg/TestSuspendStateMachineKt$closing$1.L$0 Ljava/lang/Object;
   // 0dd: checkcast java/util/List
   // 0e0: astore 1
   // 0e1: nop
   // 0e2: aload 6
   // 0e4: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 0e7: aload 6
   // 0e9: checkcast java/lang/String
   // 0ec: astore 3
   // 0ed: aload 1
   // 0ee: aload 3
   // 0ef: invokeinterface java/util/List.add (Ljava/lang/Object;)Z 2
   // 0f4: pop
   // 0f5: aload 3
   // 0f6: astore 4
   // 0f8: aload 1
   // 0f9: ldc "close"
   // 0fb: invokeinterface java/util/List.add (Ljava/lang/Object;)Z 2
   // 100: pop
   // 101: aload 4
   // 103: areturn
   // 104: astore 4
   // 106: aload 1
   // 107: ldc "close"
   // 109: invokeinterface java/util/List.add (Ljava/lang/Object;)Z 2
   // 10e: pop
   // 10f: aload 4
   // 111: athrow
   // 112: new java/lang/IllegalStateException
   // 115: dup
   // 116: ldc "call to 'resume' before 'invoke' with coroutine"
   // 118: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // 11b: athrow
}

public suspend fun StateSource.sumOf(n: Int): Int {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 2
   // 01: instanceof pkg/TestSuspendStateMachineKt$sumOf$1
   // 04: ifeq 27
   // 07: aload 2
   // 08: checkcast pkg/TestSuspendStateMachineKt$sumOf$1
   // 0b: astore 8
   // 0d: aload 8
   // 0f: getfield pkg/TestSuspendStateMachineKt$sumOf$1.label I
   // 12: ldc -2147483648
   // 14: iand
   // 15: ifeq 27
   // 18: aload 8
   // 1a: dup
   // 1b: getfield pkg/TestSuspendStateMachineKt$sumOf$1.label I
   // 1e: ldc -2147483648
   // 20: isub
   // 21: putfield pkg/TestSuspendStateMachineKt$sumOf$1.label I
   // 24: goto 31
   // 27: new pkg/TestSuspendStateMachineKt$sumOf$1
   // 2a: dup
   // 2b: aload 2
   // 2c: invokespecial pkg/TestSuspendStateMachineKt$sumOf$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 2f: astore 8
   // 31: aload 8
   // 33: getfield pkg/TestSuspendStateMachineKt$sumOf$1.result Ljava/lang/Object;
   // 36: astore 7
   // 38: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 3b: astore 9
   // 3d: aload 8
   // 3f: getfield pkg/TestSuspendStateMachineKt$sumOf$1.label I
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
   // 71: putfield pkg/TestSuspendStateMachineKt$sumOf$1.L$0 Ljava/lang/Object;
   // 74: aload 8
   // 76: iload 1
   // 77: putfield pkg/TestSuspendStateMachineKt$sumOf$1.I$0 I
   // 7a: aload 8
   // 7c: iload 4
   // 7e: putfield pkg/TestSuspendStateMachineKt$sumOf$1.I$1 I
   // 81: aload 8
   // 83: iload 5
   // 85: putfield pkg/TestSuspendStateMachineKt$sumOf$1.I$2 I
   // 88: aload 8
   // 8a: bipush 1
   // 8b: putfield pkg/TestSuspendStateMachineKt$sumOf$1.label I
   // 8e: invokeinterface pkg/StateSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 93: dup
   // 94: aload 9
   // 96: if_acmpne c0
   // 99: aload 9
   // 9b: areturn
   // 9c: aload 8
   // 9e: getfield pkg/TestSuspendStateMachineKt$sumOf$1.I$2 I
   // a1: istore 5
   // a3: aload 8
   // a5: getfield pkg/TestSuspendStateMachineKt$sumOf$1.I$1 I
   // a8: istore 4
   // aa: aload 8
   // ac: getfield pkg/TestSuspendStateMachineKt$sumOf$1.I$0 I
   // af: istore 1
   // b0: aload 8
   // b2: getfield pkg/TestSuspendStateMachineKt$sumOf$1.L$0 Ljava/lang/Object;
   // b5: checkcast pkg/StateSource
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

public suspend inline fun StateSource.pair(): Int {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 1
   // 01: instanceof pkg/TestSuspendStateMachineKt$pair$1
   // 04: ifeq 27
   // 07: aload 1
   // 08: checkcast pkg/TestSuspendStateMachineKt$pair$1
   // 0b: astore 6
   // 0d: aload 6
   // 0f: getfield pkg/TestSuspendStateMachineKt$pair$1.label I
   // 12: ldc -2147483648
   // 14: iand
   // 15: ifeq 27
   // 18: aload 6
   // 1a: dup
   // 1b: getfield pkg/TestSuspendStateMachineKt$pair$1.label I
   // 1e: ldc -2147483648
   // 20: isub
   // 21: putfield pkg/TestSuspendStateMachineKt$pair$1.label I
   // 24: goto 31
   // 27: new pkg/TestSuspendStateMachineKt$pair$1
   // 2a: dup
   // 2b: aload 1
   // 2c: invokespecial pkg/TestSuspendStateMachineKt$pair$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 2f: astore 6
   // 31: aload 6
   // 33: getfield pkg/TestSuspendStateMachineKt$pair$1.result Ljava/lang/Object;
   // 36: astore 5
   // 38: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 3b: astore 7
   // 3d: aload 6
   // 3f: getfield pkg/TestSuspendStateMachineKt$pair$1.label I
   // 42: tableswitch 156 0 2 26 62 125
   // 5c: aload 5
   // 5e: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 61: bipush 0
   // 62: istore 2
   // 63: aload 0
   // 64: aload 6
   // 66: aload 6
   // 68: aload 0
   // 69: putfield pkg/TestSuspendStateMachineKt$pair$1.L$0 Ljava/lang/Object;
   // 6c: aload 6
   // 6e: bipush 1
   // 6f: putfield pkg/TestSuspendStateMachineKt$pair$1.label I
   // 72: invokeinterface pkg/StateSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 77: dup
   // 78: aload 7
   // 7a: if_acmpne 92
   // 7d: aload 7
   // 7f: areturn
   // 80: bipush 0
   // 81: istore 2
   // 82: aload 6
   // 84: getfield pkg/TestSuspendStateMachineKt$pair$1.L$0 Ljava/lang/Object;
   // 87: checkcast pkg/StateSource
   // 8a: astore 0
   // 8b: aload 5
   // 8d: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 90: aload 5
   // 92: checkcast java/lang/Number
   // 95: invokevirtual java/lang/Number.intValue ()I
   // 98: bipush 10
   // 9a: imul
   // 9b: istore 3
   // 9c: aload 0
   // 9d: aload 6
   // 9f: aload 6
   // a1: aconst_null
   // a2: putfield pkg/TestSuspendStateMachineKt$pair$1.L$0 Ljava/lang/Object;
   // a5: aload 6
   // a7: iload 3
   // a8: putfield pkg/TestSuspendStateMachineKt$pair$1.I$0 I
   // ab: aload 6
   // ad: bipush 2
   // ae: putfield pkg/TestSuspendStateMachineKt$pair$1.label I
   // b1: invokeinterface pkg/StateSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // b6: dup
   // b7: aload 7
   // b9: if_acmpne ce
   // bc: aload 7
   // be: areturn
   // bf: bipush 0
   // c0: istore 2
   // c1: aload 6
   // c3: getfield pkg/TestSuspendStateMachineKt$pair$1.I$0 I
   // c6: istore 3
   // c7: aload 5
   // c9: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // cc: aload 5
   // ce: astore 4
   // d0: iload 3
   // d1: aload 4
   // d3: checkcast java/lang/Number
   // d6: invokevirtual java/lang/Number.intValue ()I
   // d9: iadd
   // da: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
   // dd: areturn
   // de: new java/lang/IllegalStateException
   // e1: dup
   // e2: ldc "call to 'resume' before 'invoke' with coroutine"
   // e4: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // e7: athrow
}

fun StateSource.`pair$$forInline`(`$completion`: Continuation<in Integer>) {
   InlineMarker.mark(0)
   var var10000: Any = `$this$pair`.next(`$completion`)
   InlineMarker.mark(1)
   val var3: Int = (var10000 as java.lang.Number).intValue() * 10
   InlineMarker.mark(0)
   var10000 = `$this$pair`.next(`$completion`)
   InlineMarker.mark(1)
   var3 + (var10000 as java.lang.Number).intValue()
}

public suspend fun pairs(src: StateSource): Int {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 000: aload 1
   // 001: instanceof pkg/TestSuspendStateMachineKt$pairs$1
   // 004: ifeq 027
   // 007: aload 1
   // 008: checkcast pkg/TestSuspendStateMachineKt$pairs$1
   // 00b: astore 9
   // 00d: aload 9
   // 00f: getfield pkg/TestSuspendStateMachineKt$pairs$1.label I
   // 012: ldc -2147483648
   // 014: iand
   // 015: ifeq 027
   // 018: aload 9
   // 01a: dup
   // 01b: getfield pkg/TestSuspendStateMachineKt$pairs$1.label I
   // 01e: ldc -2147483648
   // 020: isub
   // 021: putfield pkg/TestSuspendStateMachineKt$pairs$1.label I
   // 024: goto 031
   // 027: new pkg/TestSuspendStateMachineKt$pairs$1
   // 02a: dup
   // 02b: aload 1
   // 02c: invokespecial pkg/TestSuspendStateMachineKt$pairs$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 02f: astore 9
   // 031: aload 9
   // 033: getfield pkg/TestSuspendStateMachineKt$pairs$1.result Ljava/lang/Object;
   // 036: astore 8
   // 038: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 03b: astore 10
   // 03d: aload 9
   // 03f: getfield pkg/TestSuspendStateMachineKt$pairs$1.label I
   // 042: tableswitch 364 0 4 34 78 158 238 317
   // 064: aload 8
   // 066: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 069: aload 0
   // 06a: astore 2
   // 06b: bipush 0
   // 06c: istore 3
   // 06d: aload 2
   // 06e: aload 9
   // 070: aload 9
   // 072: aload 0
   // 073: putfield pkg/TestSuspendStateMachineKt$pairs$1.L$0 Ljava/lang/Object;
   // 076: aload 9
   // 078: aload 2
   // 079: putfield pkg/TestSuspendStateMachineKt$pairs$1.L$1 Ljava/lang/Object;
   // 07c: aload 9
   // 07e: bipush 1
   // 07f: putfield pkg/TestSuspendStateMachineKt$pairs$1.label I
   // 082: invokeinterface pkg/StateSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 087: dup
   // 088: aload 10
   // 08a: if_acmpne 0ab
   // 08d: aload 10
   // 08f: areturn
   // 090: bipush 0
   // 091: istore 3
   // 092: aload 9
   // 094: getfield pkg/TestSuspendStateMachineKt$pairs$1.L$1 Ljava/lang/Object;
   // 097: checkcast pkg/StateSource
   // 09a: astore 2
   // 09b: aload 9
   // 09d: getfield pkg/TestSuspendStateMachineKt$pairs$1.L$0 Ljava/lang/Object;
   // 0a0: checkcast pkg/StateSource
   // 0a3: astore 0
   // 0a4: aload 8
   // 0a6: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 0a9: aload 8
   // 0ab: checkcast java/lang/Number
   // 0ae: invokevirtual java/lang/Number.intValue ()I
   // 0b1: bipush 10
   // 0b3: imul
   // 0b4: istore 4
   // 0b6: aload 2
   // 0b7: aload 9
   // 0b9: aload 9
   // 0bb: aload 0
   // 0bc: putfield pkg/TestSuspendStateMachineKt$pairs$1.L$0 Ljava/lang/Object;
   // 0bf: aload 9
   // 0c1: aconst_null
   // 0c2: putfield pkg/TestSuspendStateMachineKt$pairs$1.L$1 Ljava/lang/Object;
   // 0c5: aload 9
   // 0c7: iload 4
   // 0c9: putfield pkg/TestSuspendStateMachineKt$pairs$1.I$0 I
   // 0cc: aload 9
   // 0ce: bipush 2
   // 0cf: putfield pkg/TestSuspendStateMachineKt$pairs$1.label I
   // 0d2: invokeinterface pkg/StateSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 0d7: dup
   // 0d8: aload 10
   // 0da: if_acmpne 0f9
   // 0dd: aload 10
   // 0df: areturn
   // 0e0: bipush 0
   // 0e1: istore 3
   // 0e2: aload 9
   // 0e4: getfield pkg/TestSuspendStateMachineKt$pairs$1.I$0 I
   // 0e7: istore 4
   // 0e9: aload 9
   // 0eb: getfield pkg/TestSuspendStateMachineKt$pairs$1.L$0 Ljava/lang/Object;
   // 0ee: checkcast pkg/StateSource
   // 0f1: astore 0
   // 0f2: aload 8
   // 0f4: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 0f7: aload 8
   // 0f9: astore 5
   // 0fb: iload 4
   // 0fd: aload 5
   // 0ff: checkcast java/lang/Number
   // 102: invokevirtual java/lang/Number.intValue ()I
   // 105: iadd
   // 106: aload 0
   // 107: astore 2
   // 108: istore 6
   // 10a: bipush 0
   // 10b: istore 3
   // 10c: aload 2
   // 10d: aload 9
   // 10f: aload 9
   // 111: aload 2
   // 112: putfield pkg/TestSuspendStateMachineKt$pairs$1.L$0 Ljava/lang/Object;
   // 115: aload 9
   // 117: iload 6
   // 119: putfield pkg/TestSuspendStateMachineKt$pairs$1.I$0 I
   // 11c: aload 9
   // 11e: bipush 3
   // 11f: putfield pkg/TestSuspendStateMachineKt$pairs$1.label I
   // 122: invokeinterface pkg/StateSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 127: dup
   // 128: aload 10
   // 12a: if_acmpne 149
   // 12d: aload 10
   // 12f: areturn
   // 130: bipush 0
   // 131: istore 3
   // 132: aload 9
   // 134: getfield pkg/TestSuspendStateMachineKt$pairs$1.I$0 I
   // 137: istore 6
   // 139: aload 9
   // 13b: getfield pkg/TestSuspendStateMachineKt$pairs$1.L$0 Ljava/lang/Object;
   // 13e: checkcast pkg/StateSource
   // 141: astore 2
   // 142: aload 8
   // 144: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 147: aload 8
   // 149: checkcast java/lang/Number
   // 14c: invokevirtual java/lang/Number.intValue ()I
   // 14f: bipush 10
   // 151: imul
   // 152: istore 4
   // 154: aload 2
   // 155: aload 9
   // 157: aload 9
   // 159: aconst_null
   // 15a: putfield pkg/TestSuspendStateMachineKt$pairs$1.L$0 Ljava/lang/Object;
   // 15d: aload 9
   // 15f: iload 4
   // 161: putfield pkg/TestSuspendStateMachineKt$pairs$1.I$0 I
   // 164: aload 9
   // 166: iload 6
   // 168: putfield pkg/TestSuspendStateMachineKt$pairs$1.I$1 I
   // 16b: aload 9
   // 16d: bipush 4
   // 16e: putfield pkg/TestSuspendStateMachineKt$pairs$1.label I
   // 171: invokeinterface pkg/StateSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 176: dup
   // 177: aload 10
   // 179: if_acmpne 196
   // 17c: aload 10
   // 17e: areturn
   // 17f: bipush 0
   // 180: istore 3
   // 181: aload 9
   // 183: getfield pkg/TestSuspendStateMachineKt$pairs$1.I$1 I
   // 186: istore 6
   // 188: aload 9
   // 18a: getfield pkg/TestSuspendStateMachineKt$pairs$1.I$0 I
   // 18d: istore 4
   // 18f: aload 8
   // 191: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 194: aload 8
   // 196: astore 5
   // 198: iload 4
   // 19a: aload 5
   // 19c: checkcast java/lang/Number
   // 19f: invokevirtual java/lang/Number.intValue ()I
   // 1a2: iadd
   // 1a3: istore 7
   // 1a5: iload 6
   // 1a7: iload 7
   // 1a9: iadd
   // 1aa: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
   // 1ad: areturn
   // 1ae: new java/lang/IllegalStateException
   // 1b1: dup
   // 1b2: ldc "call to 'resume' before 'invoke' with coroutine"
   // 1b4: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // 1b7: athrow
}

public suspend fun mapped(src: StateSource, f: (Int, Continuation<String>) -> Any?): String {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 000: aload 2
   // 001: instanceof pkg/TestSuspendStateMachineKt$mapped$1
   // 004: ifeq 027
   // 007: aload 2
   // 008: checkcast pkg/TestSuspendStateMachineKt$mapped$1
   // 00b: astore 7
   // 00d: aload 7
   // 00f: getfield pkg/TestSuspendStateMachineKt$mapped$1.label I
   // 012: ldc -2147483648
   // 014: iand
   // 015: ifeq 027
   // 018: aload 7
   // 01a: dup
   // 01b: getfield pkg/TestSuspendStateMachineKt$mapped$1.label I
   // 01e: ldc -2147483648
   // 020: isub
   // 021: putfield pkg/TestSuspendStateMachineKt$mapped$1.label I
   // 024: goto 031
   // 027: new pkg/TestSuspendStateMachineKt$mapped$1
   // 02a: dup
   // 02b: aload 2
   // 02c: invokespecial pkg/TestSuspendStateMachineKt$mapped$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 02f: astore 7
   // 031: aload 7
   // 033: getfield pkg/TestSuspendStateMachineKt$mapped$1.result Ljava/lang/Object;
   // 036: astore 6
   // 038: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 03b: astore 8
   // 03d: aload 7
   // 03f: getfield pkg/TestSuspendStateMachineKt$mapped$1.label I
   // 042: tableswitch 325 0 4 34 84 162 234 299
   // 064: aload 6
   // 066: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 069: aload 1
   // 06a: astore 4
   // 06c: aload 0
   // 06d: aload 7
   // 06f: aload 7
   // 071: aload 0
   // 072: putfield pkg/TestSuspendStateMachineKt$mapped$1.L$0 Ljava/lang/Object;
   // 075: aload 7
   // 077: aload 1
   // 078: putfield pkg/TestSuspendStateMachineKt$mapped$1.L$1 Ljava/lang/Object;
   // 07b: aload 7
   // 07d: aload 4
   // 07f: putfield pkg/TestSuspendStateMachineKt$mapped$1.L$2 Ljava/lang/Object;
   // 082: aload 7
   // 084: bipush 1
   // 085: putfield pkg/TestSuspendStateMachineKt$mapped$1.label I
   // 088: invokeinterface pkg/StateSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 08d: dup
   // 08e: aload 8
   // 090: if_acmpne 0b9
   // 093: aload 8
   // 095: areturn
   // 096: aload 7
   // 098: getfield pkg/TestSuspendStateMachineKt$mapped$1.L$2 Ljava/lang/Object;
   // 09b: checkcast kotlin/jvm/functions/Function2
   // 09e: astore 4
   // 0a0: aload 7
   // 0a2: getfield pkg/TestSuspendStateMachineKt$mapped$1.L$1 Ljava/lang/Object;
   // 0a5: checkcast kotlin/jvm/functions/Function2
   // 0a8: astore 1
   // 0a9: aload 7
   // 0ab: getfield pkg/TestSuspendStateMachineKt$mapped$1.L$0 Ljava/lang/Object;
   // 0ae: checkcast pkg/StateSource
   // 0b1: astore 0
   // 0b2: aload 6
   // 0b4: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 0b7: aload 6
   // 0b9: aload 4
   // 0bb: swap
   // 0bc: aload 7
   // 0be: aload 7
   // 0c0: aload 0
   // 0c1: putfield pkg/TestSuspendStateMachineKt$mapped$1.L$0 Ljava/lang/Object;
   // 0c4: aload 7
   // 0c6: aload 1
   // 0c7: putfield pkg/TestSuspendStateMachineKt$mapped$1.L$1 Ljava/lang/Object;
   // 0ca: aload 7
   // 0cc: aconst_null
   // 0cd: putfield pkg/TestSuspendStateMachineKt$mapped$1.L$2 Ljava/lang/Object;
   // 0d0: aload 7
   // 0d2: bipush 2
   // 0d3: putfield pkg/TestSuspendStateMachineKt$mapped$1.label I
   // 0d6: invokeinterface kotlin/jvm/functions/Function2.invoke (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object; 3
   // 0db: dup
   // 0dc: aload 8
   // 0de: if_acmpne 0fd
   // 0e1: aload 8
   // 0e3: areturn
   // 0e4: aload 7
   // 0e6: getfield pkg/TestSuspendStateMachineKt$mapped$1.L$1 Ljava/lang/Object;
   // 0e9: checkcast kotlin/jvm/functions/Function2
   // 0ec: astore 1
   // 0ed: aload 7
   // 0ef: getfield pkg/TestSuspendStateMachineKt$mapped$1.L$0 Ljava/lang/Object;
   // 0f2: checkcast pkg/StateSource
   // 0f5: astore 0
   // 0f6: aload 6
   // 0f8: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 0fb: aload 6
   // 0fd: checkcast java/lang/String
   // 100: astore 3
   // 101: aload 3
   // 102: astore 4
   // 104: aload 1
   // 105: astore 5
   // 107: aload 0
   // 108: aload 7
   // 10a: aload 7
   // 10c: aload 4
   // 10e: putfield pkg/TestSuspendStateMachineKt$mapped$1.L$0 Ljava/lang/Object;
   // 111: aload 7
   // 113: aload 5
   // 115: putfield pkg/TestSuspendStateMachineKt$mapped$1.L$1 Ljava/lang/Object;
   // 118: aload 7
   // 11a: bipush 3
   // 11b: putfield pkg/TestSuspendStateMachineKt$mapped$1.label I
   // 11e: invokeinterface pkg/StateSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 123: dup
   // 124: aload 8
   // 126: if_acmpne 147
   // 129: aload 8
   // 12b: areturn
   // 12c: aload 7
   // 12e: getfield pkg/TestSuspendStateMachineKt$mapped$1.L$1 Ljava/lang/Object;
   // 131: checkcast kotlin/jvm/functions/Function2
   // 134: astore 5
   // 136: aload 7
   // 138: getfield pkg/TestSuspendStateMachineKt$mapped$1.L$0 Ljava/lang/Object;
   // 13b: checkcast java/lang/String
   // 13e: astore 4
   // 140: aload 6
   // 142: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 145: aload 6
   // 147: aload 5
   // 149: swap
   // 14a: aload 7
   // 14c: aload 7
   // 14e: aload 4
   // 150: putfield pkg/TestSuspendStateMachineKt$mapped$1.L$0 Ljava/lang/Object;
   // 153: aload 7
   // 155: aconst_null
   // 156: putfield pkg/TestSuspendStateMachineKt$mapped$1.L$1 Ljava/lang/Object;
   // 159: aload 7
   // 15b: bipush 4
   // 15c: putfield pkg/TestSuspendStateMachineKt$mapped$1.label I
   // 15f: invokeinterface kotlin/jvm/functions/Function2.invoke (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object; 3
   // 164: dup
   // 165: aload 8
   // 167: if_acmpne 17e
   // 16a: aload 8
   // 16c: areturn
   // 16d: aload 7
   // 16f: getfield pkg/TestSuspendStateMachineKt$mapped$1.L$0 Ljava/lang/Object;
   // 172: checkcast java/lang/String
   // 175: astore 4
   // 177: aload 6
   // 179: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 17c: aload 6
   // 17e: aload 4
   // 180: swap
   // 181: invokedynamic makeConcatWithConstants (Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/String; bsm=java/lang/invoke/StringConcatFactory.makeConcatWithConstants (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite; args=[ "\u0001\u0001" ]
   // 186: areturn
   // 187: new java/lang/IllegalStateException
   // 18a: dup
   // 18b: ldc "call to 'resume' before 'invoke' with coroutine"
   // 18d: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // 190: athrow
}

public suspend fun intercepted(src: StateSource): String {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 1
   // 01: instanceof pkg/TestSuspendStateMachineKt$intercepted$1
   // 04: ifeq 27
   // 07: aload 1
   // 08: checkcast pkg/TestSuspendStateMachineKt$intercepted$1
   // 0b: astore 4
   // 0d: aload 4
   // 0f: getfield pkg/TestSuspendStateMachineKt$intercepted$1.label I
   // 12: ldc -2147483648
   // 14: iand
   // 15: ifeq 27
   // 18: aload 4
   // 1a: dup
   // 1b: getfield pkg/TestSuspendStateMachineKt$intercepted$1.label I
   // 1e: ldc -2147483648
   // 20: isub
   // 21: putfield pkg/TestSuspendStateMachineKt$intercepted$1.label I
   // 24: goto 31
   // 27: new pkg/TestSuspendStateMachineKt$intercepted$1
   // 2a: dup
   // 2b: aload 1
   // 2c: invokespecial pkg/TestSuspendStateMachineKt$intercepted$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 2f: astore 4
   // 31: aload 4
   // 33: getfield pkg/TestSuspendStateMachineKt$intercepted$1.result Ljava/lang/Object;
   // 36: astore 3
   // 37: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 3a: astore 5
   // 3c: aload 4
   // 3e: getfield pkg/TestSuspendStateMachineKt$intercepted$1.label I
   // 41: tableswitch 95 0 1 23 50
   // 58: aload 3
   // 59: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 5c: aload 0
   // 5d: aload 4
   // 5f: aload 4
   // 61: bipush 1
   // 62: putfield pkg/TestSuspendStateMachineKt$intercepted$1.label I
   // 65: invokeinterface pkg/StateSource.next (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
   // 6a: dup
   // 6b: aload 5
   // 6d: if_acmpne 78
   // 70: aload 5
   // 72: areturn
   // 73: aload 3
   // 74: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 77: aload 3
   // 78: checkcast java/lang/Number
   // 7b: invokevirtual java/lang/Number.intValue ()I
   // 7e: istore 2
   // 7f: iload 2
   // 80: aload 4
   // 82: invokeinterface kotlin/coroutines/Continuation.getContext ()Lkotlin/coroutines/CoroutineContext; 1
   // 87: getstatic kotlin/coroutines/ContinuationInterceptor.Key Lkotlin/coroutines/ContinuationInterceptor$Key;
   // 8a: checkcast kotlin/coroutines/CoroutineContext$Key
   // 8d: invokeinterface kotlin/coroutines/CoroutineContext.get (Lkotlin/coroutines/CoroutineContext$Key;)Lkotlin/coroutines/CoroutineContext$Element; 2
   // 92: ifnonnull 99
   // 95: bipush 1
   // 96: goto 9a
   // 99: bipush 0
   // 9a: invokedynamic makeConcatWithConstants (IZ)Ljava/lang/String; bsm=java/lang/invoke/StringConcatFactory.makeConcatWithConstants (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite; args=[ "\u0001:\u0001" ]
   // 9f: areturn
   // a0: new java/lang/IllegalStateException
   // a3: dup
   // a4: ldc "call to 'resume' before 'invoke' with coroutine"
   // a6: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // a9: athrow
}

public suspend fun handOff(x: Int): Int {
   // $VF: Couldn't be decompiled
   // Bytecode:
   // 00: aload 1
   // 01: instanceof pkg/TestSuspendStateMachineKt$handOff$1
   // 04: ifeq 27
   // 07: aload 1
   // 08: checkcast pkg/TestSuspendStateMachineKt$handOff$1
   // 0b: astore 8
   // 0d: aload 8
   // 0f: getfield pkg/TestSuspendStateMachineKt$handOff$1.label I
   // 12: ldc -2147483648
   // 14: iand
   // 15: ifeq 27
   // 18: aload 8
   // 1a: dup
   // 1b: getfield pkg/TestSuspendStateMachineKt$handOff$1.label I
   // 1e: ldc -2147483648
   // 20: isub
   // 21: putfield pkg/TestSuspendStateMachineKt$handOff$1.label I
   // 24: goto 31
   // 27: new pkg/TestSuspendStateMachineKt$handOff$1
   // 2a: dup
   // 2b: aload 1
   // 2c: invokespecial pkg/TestSuspendStateMachineKt$handOff$1.<init> (Lkotlin/coroutines/Continuation;)V
   // 2f: astore 8
   // 31: aload 8
   // 33: getfield pkg/TestSuspendStateMachineKt$handOff$1.result Ljava/lang/Object;
   // 36: astore 7
   // 38: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // 3b: astore 9
   // 3d: aload 8
   // 3f: getfield pkg/TestSuspendStateMachineKt$handOff$1.label I
   // 42: tableswitch 146 0 1 22 118
   // 58: aload 7
   // 5a: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // 5d: aload 8
   // 5f: iload 0
   // 60: putfield pkg/TestSuspendStateMachineKt$handOff$1.I$0 I
   // 63: aload 8
   // 65: bipush 1
   // 66: putfield pkg/TestSuspendStateMachineKt$handOff$1.label I
   // 69: aload 8
   // 6b: checkcast kotlin/coroutines/Continuation
   // 6e: astore 3
   // 6f: new kotlin/coroutines/SafeContinuation
   // 72: dup
   // 73: aload 3
   // 74: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.intercepted (Lkotlin/coroutines/Continuation;)Lkotlin/coroutines/Continuation;
   // 77: invokespecial kotlin/coroutines/SafeContinuation.<init> (Lkotlin/coroutines/Continuation;)V
   // 7a: astore 4
   // 7c: aload 4
   // 7e: checkcast kotlin/coroutines/Continuation
   // 81: astore 5
   // 83: bipush 0
   // 84: istore 6
   // 86: aload 5
   // 88: getstatic kotlin/Result.Companion Lkotlin/Result$Companion;
   // 8b: pop
   // 8c: iload 0
   // 8d: bipush 1
   // 8e: iadd
   // 8f: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
   // 92: invokestatic kotlin/Result.constructor-impl (Ljava/lang/Object;)Ljava/lang/Object;
   // 95: invokeinterface kotlin/coroutines/Continuation.resumeWith (Ljava/lang/Object;)V 2
   // 9a: nop
   // 9b: aload 4
   // 9d: invokevirtual kotlin/coroutines/SafeContinuation.getOrThrow ()Ljava/lang/Object;
   // a0: dup
   // a1: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
   // a4: if_acmpne af
   // a7: aload 8
   // a9: checkcast kotlin/coroutines/Continuation
   // ac: invokestatic kotlin/coroutines/jvm/internal/DebugProbesKt.probeCoroutineSuspended (Lkotlin/coroutines/Continuation;)V
   // af: dup
   // b0: aload 9
   // b2: if_acmpne c5
   // b5: aload 9
   // b7: areturn
   // b8: aload 8
   // ba: getfield pkg/TestSuspendStateMachineKt$handOff$1.I$0 I
   // bd: pop
   // be: aload 7
   // c0: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
   // c3: aload 7
   // c5: nop
   // c6: checkcast java/lang/Number
   // c9: invokevirtual java/lang/Number.intValue ()I
   // cc: istore 2
   // cd: iload 2
   // ce: bipush 2
   // cf: imul
   // d0: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
   // d3: areturn
   // d4: new java/lang/IllegalStateException
   // d7: dup
   // d8: ldc "call to 'resume' before 'invoke' with coroutine"
   // da: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
   // dd: athrow
}
