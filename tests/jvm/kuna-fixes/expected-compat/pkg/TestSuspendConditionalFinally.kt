package pkg

public class TestSuspendConditionalFinally {
   public suspend fun drain(sink: pkg.TestSuspendConditionalFinally.Sink, src: pkg.TestSuspendConditionalFinally.Source, consume: Boolean) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 000: aload 4
      // 002: instanceof pkg/TestSuspendConditionalFinally$drain$1
      // 005: ifeq 029
      // 008: aload 4
      // 00a: checkcast pkg/TestSuspendConditionalFinally$drain$1
      // 00d: astore 8
      // 00f: aload 8
      // 011: getfield pkg/TestSuspendConditionalFinally$drain$1.label I
      // 014: ldc -2147483648
      // 016: iand
      // 017: ifeq 029
      // 01a: aload 8
      // 01c: dup
      // 01d: getfield pkg/TestSuspendConditionalFinally$drain$1.label I
      // 020: ldc -2147483648
      // 022: isub
      // 023: putfield pkg/TestSuspendConditionalFinally$drain$1.label I
      // 026: goto 035
      // 029: new pkg/TestSuspendConditionalFinally$drain$1
      // 02c: dup
      // 02d: aload 0
      // 02e: aload 4
      // 030: invokespecial pkg/TestSuspendConditionalFinally$drain$1.<init> (Lpkg/TestSuspendConditionalFinally;Lkotlin/coroutines/Continuation;)V
      // 033: astore 8
      // 035: aload 8
      // 037: getfield pkg/TestSuspendConditionalFinally$drain$1.result Ljava/lang/Object;
      // 03a: astore 7
      // 03c: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
      // 03f: astore 9
      // 041: aload 8
      // 043: getfield pkg/TestSuspendConditionalFinally$drain$1.label I
      // 046: tableswitch 295 0 2 26 92 201
      // 060: aload 7
      // 062: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 065: aconst_null
      // 066: astore 5
      // 068: nop
      // 069: aload 2
      // 06a: invokeinterface pkg/TestSuspendConditionalFinally$Source.open ()Lpkg/TestSuspendConditionalFinally$Cursor; 1
      // 06f: astore 6
      // 071: aload 6
      // 073: aload 8
      // 075: aload 8
      // 077: aload 1
      // 078: putfield pkg/TestSuspendConditionalFinally$drain$1.L$0 Ljava/lang/Object;
      // 07b: aload 8
      // 07d: aload 2
      // 07e: putfield pkg/TestSuspendConditionalFinally$drain$1.L$1 Ljava/lang/Object;
      // 081: aload 8
      // 083: aload 6
      // 085: putfield pkg/TestSuspendConditionalFinally$drain$1.L$2 Ljava/lang/Object;
      // 088: aload 8
      // 08a: iload 3
      // 08b: putfield pkg/TestSuspendConditionalFinally$drain$1.Z$0 Z
      // 08e: aload 8
      // 090: bipush 1
      // 091: putfield pkg/TestSuspendConditionalFinally$drain$1.label I
      // 094: invokeinterface pkg/TestSuspendConditionalFinally$Cursor.hasMore (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
      // 099: dup
      // 09a: aload 9
      // 09c: if_acmpne 0cf
      // 09f: aload 9
      // 0a1: areturn
      // 0a2: aload 8
      // 0a4: getfield pkg/TestSuspendConditionalFinally$drain$1.Z$0 Z
      // 0a7: istore 3
      // 0a8: aload 8
      // 0aa: getfield pkg/TestSuspendConditionalFinally$drain$1.L$2 Ljava/lang/Object;
      // 0ad: checkcast pkg/TestSuspendConditionalFinally$Cursor
      // 0b0: astore 6
      // 0b2: aconst_null
      // 0b3: astore 5
      // 0b5: aload 8
      // 0b7: getfield pkg/TestSuspendConditionalFinally$drain$1.L$1 Ljava/lang/Object;
      // 0ba: checkcast pkg/TestSuspendConditionalFinally$Source
      // 0bd: astore 2
      // 0be: aload 8
      // 0c0: getfield pkg/TestSuspendConditionalFinally$drain$1.L$0 Ljava/lang/Object;
      // 0c3: checkcast pkg/TestSuspendConditionalFinally$Sink
      // 0c6: astore 1
      // 0c7: nop
      // 0c8: aload 7
      // 0ca: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 0cd: aload 7
      // 0cf: checkcast java/lang/Boolean
      // 0d2: invokevirtual java/lang/Boolean.booleanValue ()Z
      // 0d5: ifeq 140
      // 0d8: aload 1
      // 0d9: aload 6
      // 0db: invokeinterface pkg/TestSuspendConditionalFinally$Cursor.take ()I 1
      // 0e0: aload 8
      // 0e2: aload 8
      // 0e4: aload 1
      // 0e5: putfield pkg/TestSuspendConditionalFinally$drain$1.L$0 Ljava/lang/Object;
      // 0e8: aload 8
      // 0ea: aload 2
      // 0eb: putfield pkg/TestSuspendConditionalFinally$drain$1.L$1 Ljava/lang/Object;
      // 0ee: aload 8
      // 0f0: aload 6
      // 0f2: putfield pkg/TestSuspendConditionalFinally$drain$1.L$2 Ljava/lang/Object;
      // 0f5: aload 8
      // 0f7: iload 3
      // 0f8: putfield pkg/TestSuspendConditionalFinally$drain$1.Z$0 Z
      // 0fb: aload 8
      // 0fd: bipush 2
      // 0fe: putfield pkg/TestSuspendConditionalFinally$drain$1.label I
      // 101: invokeinterface pkg/TestSuspendConditionalFinally$Sink.put (ILkotlin/coroutines/Continuation;)Ljava/lang/Object; 3
      // 106: dup
      // 107: aload 9
      // 109: if_acmpne 13c
      // 10c: aload 9
      // 10e: areturn
      // 10f: aload 8
      // 111: getfield pkg/TestSuspendConditionalFinally$drain$1.Z$0 Z
      // 114: istore 3
      // 115: aload 8
      // 117: getfield pkg/TestSuspendConditionalFinally$drain$1.L$2 Ljava/lang/Object;
      // 11a: checkcast pkg/TestSuspendConditionalFinally$Cursor
      // 11d: astore 6
      // 11f: aconst_null
      // 120: astore 5
      // 122: aload 8
      // 124: getfield pkg/TestSuspendConditionalFinally$drain$1.L$1 Ljava/lang/Object;
      // 127: checkcast pkg/TestSuspendConditionalFinally$Source
      // 12a: astore 2
      // 12b: aload 8
      // 12d: getfield pkg/TestSuspendConditionalFinally$drain$1.L$0 Ljava/lang/Object;
      // 130: checkcast pkg/TestSuspendConditionalFinally$Sink
      // 133: astore 1
      // 134: nop
      // 135: aload 7
      // 137: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 13a: aload 7
      // 13c: pop
      // 13d: goto 071
      // 140: iload 3
      // 141: ifeq 14c
      // 144: aload 2
      // 145: aload 5
      // 147: invokeinterface pkg/TestSuspendConditionalFinally$Source.cancel (Ljava/lang/Throwable;)V 2
      // 14c: goto 169
      // 14f: astore 6
      // 151: aload 6
      // 153: astore 5
      // 155: aload 6
      // 157: athrow
      // 158: astore 6
      // 15a: iload 3
      // 15b: ifeq 166
      // 15e: aload 2
      // 15f: aload 5
      // 161: invokeinterface pkg/TestSuspendConditionalFinally$Source.cancel (Ljava/lang/Throwable;)V 2
      // 166: aload 6
      // 168: athrow
      // 169: getstatic kotlin/Unit.INSTANCE Lkotlin/Unit;
      // 16c: areturn
      // 16d: new java/lang/IllegalStateException
      // 170: dup
      // 171: ldc "call to 'resume' before 'invoke' with coroutine"
      // 173: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
      // 176: athrow
   }

   public suspend fun sum(src: pkg.TestSuspendConditionalFinally.Source, cursor: pkg.TestSuspendConditionalFinally.Cursor, flag: Boolean): Int {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 000: aload 4
      // 002: instanceof pkg/TestSuspendConditionalFinally$sum$1
      // 005: ifeq 029
      // 008: aload 4
      // 00a: checkcast pkg/TestSuspendConditionalFinally$sum$1
      // 00d: astore 8
      // 00f: aload 8
      // 011: getfield pkg/TestSuspendConditionalFinally$sum$1.label I
      // 014: ldc -2147483648
      // 016: iand
      // 017: ifeq 029
      // 01a: aload 8
      // 01c: dup
      // 01d: getfield pkg/TestSuspendConditionalFinally$sum$1.label I
      // 020: ldc -2147483648
      // 022: isub
      // 023: putfield pkg/TestSuspendConditionalFinally$sum$1.label I
      // 026: goto 035
      // 029: new pkg/TestSuspendConditionalFinally$sum$1
      // 02c: dup
      // 02d: aload 0
      // 02e: aload 4
      // 030: invokespecial pkg/TestSuspendConditionalFinally$sum$1.<init> (Lpkg/TestSuspendConditionalFinally;Lkotlin/coroutines/Continuation;)V
      // 033: astore 8
      // 035: aload 8
      // 037: getfield pkg/TestSuspendConditionalFinally$sum$1.result Ljava/lang/Object;
      // 03a: astore 7
      // 03c: invokestatic kotlin/coroutines/intrinsics/IntrinsicsKt.getCOROUTINE_SUSPENDED ()Ljava/lang/Object;
      // 03f: astore 9
      // 041: aload 8
      // 043: getfield pkg/TestSuspendConditionalFinally$sum$1.label I
      // 046: tableswitch 186 0 1 22 79
      // 05c: aload 7
      // 05e: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 061: nop
      // 062: bipush 0
      // 063: istore 5
      // 065: aload 2
      // 066: aload 8
      // 068: aload 8
      // 06a: aload 1
      // 06b: putfield pkg/TestSuspendConditionalFinally$sum$1.L$0 Ljava/lang/Object;
      // 06e: aload 8
      // 070: aload 2
      // 071: putfield pkg/TestSuspendConditionalFinally$sum$1.L$1 Ljava/lang/Object;
      // 074: aload 8
      // 076: iload 3
      // 077: putfield pkg/TestSuspendConditionalFinally$sum$1.Z$0 Z
      // 07a: aload 8
      // 07c: iload 5
      // 07e: putfield pkg/TestSuspendConditionalFinally$sum$1.I$0 I
      // 081: aload 8
      // 083: bipush 1
      // 084: putfield pkg/TestSuspendConditionalFinally$sum$1.label I
      // 087: invokeinterface pkg/TestSuspendConditionalFinally$Cursor.hasMore (Lkotlin/coroutines/Continuation;)Ljava/lang/Object; 2
      // 08c: dup
      // 08d: aload 9
      // 08f: if_acmpne 0bc
      // 092: aload 9
      // 094: areturn
      // 095: aload 8
      // 097: getfield pkg/TestSuspendConditionalFinally$sum$1.I$0 I
      // 09a: istore 5
      // 09c: aload 8
      // 09e: getfield pkg/TestSuspendConditionalFinally$sum$1.Z$0 Z
      // 0a1: istore 3
      // 0a2: aload 8
      // 0a4: getfield pkg/TestSuspendConditionalFinally$sum$1.L$1 Ljava/lang/Object;
      // 0a7: checkcast pkg/TestSuspendConditionalFinally$Cursor
      // 0aa: astore 2
      // 0ab: aload 8
      // 0ad: getfield pkg/TestSuspendConditionalFinally$sum$1.L$0 Ljava/lang/Object;
      // 0b0: checkcast pkg/TestSuspendConditionalFinally$Source
      // 0b3: astore 1
      // 0b4: nop
      // 0b5: aload 7
      // 0b7: invokestatic kotlin/ResultKt.throwOnFailure (Ljava/lang/Object;)V
      // 0ba: aload 7
      // 0bc: checkcast java/lang/Boolean
      // 0bf: invokevirtual java/lang/Boolean.booleanValue ()Z
      // 0c2: ifeq 0d3
      // 0c5: iload 5
      // 0c7: aload 2
      // 0c8: invokeinterface pkg/TestSuspendConditionalFinally$Cursor.take ()I 1
      // 0cd: iadd
      // 0ce: istore 5
      // 0d0: goto 065
      // 0d3: iload 5
      // 0d5: invokestatic kotlin/coroutines/jvm/internal/Boxing.boxInt (I)Ljava/lang/Integer;
      // 0d8: astore 6
      // 0da: aload 1
      // 0db: iload 3
      // 0dc: ifeq 0e3
      // 0df: bipush 1
      // 0e0: goto 0e4
      // 0e3: bipush 0
      // 0e4: invokeinterface pkg/TestSuspendConditionalFinally$Source.closeIf (Z)V 2
      // 0e9: aload 6
      // 0eb: areturn
      // 0ec: astore 6
      // 0ee: aload 1
      // 0ef: iload 3
      // 0f0: ifeq 0f7
      // 0f3: bipush 1
      // 0f4: goto 0f8
      // 0f7: bipush 0
      // 0f8: invokeinterface pkg/TestSuspendConditionalFinally$Source.closeIf (Z)V 2
      // 0fd: aload 6
      // 0ff: athrow
      // 100: new java/lang/IllegalStateException
      // 103: dup
      // 104: ldc "call to 'resume' before 'invoke' with coroutine"
      // 106: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/String;)V
      // 109: athrow
   }

   public interface Cursor {
      public abstract suspend fun hasMore(): Boolean {
      }

      public abstract fun take(): Int {
      }
   }

   public interface Sink {
      public abstract suspend fun put(v: Int) {
      }
   }

   public interface Source {
      public abstract fun open(): pkg.TestSuspendConditionalFinally.Cursor {
      }

      public abstract fun cancel(cause: Throwable?) {
      }

      public abstract fun closeIf(flag: Boolean) {
      }
   }
}
