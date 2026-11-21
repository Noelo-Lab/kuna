package pkg;

public class TestTryCatchNotFlag {
   private int failures;

   static Object make() throws Exception {
      return new Object();
   }

   static TestTryCatchNotFlag.Res open(String name) {
      return name.isEmpty() ? null : new TestTryCatchNotFlag.Res();
   }

   Object create(boolean param1) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: invokestatic pkg/TestTryCatchNotFlag.make ()Ljava/lang/Object;
      // 03: astore 2
      // 04: goto 17
      // 07: astore 3
      // 08: iload 1
      // 09: ifne 15
      // 0c: new java/lang/IllegalStateException
      // 0f: dup
      // 10: aload 3
      // 11: invokespecial java/lang/IllegalStateException.<init> (Ljava/lang/Throwable;)V
      // 14: athrow
      // 15: aconst_null
      // 16: astore 2
      // 17: aload 2
      // 18: areturn
      // try (0 -> 2): 3 java/lang/Throwable
   }

   void attempt(boolean param1) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: invokestatic pkg/TestTryCatchNotFlag.make ()Ljava/lang/Object;
      // 03: pop
      // 04: goto 1b
      // 07: astore 2
      // 08: iload 1
      // 09: ifne 11
      // 0c: aload 0
      // 0d: aload 2
      // 0e: invokevirtual pkg/TestTryCatchNotFlag.report (Ljava/lang/Throwable;)V
      // 11: aload 0
      // 12: dup
      // 13: getfield pkg/TestTryCatchNotFlag.failures I
      // 16: bipush 1
      // 17: iadd
      // 18: putfield pkg/TestTryCatchNotFlag.failures I
      // 1b: return
      // try (0 -> 2): 3 java/lang/Throwable
   }

   int read(String name) throws Exception {
      try (TestTryCatchNotFlag.Res res = open(name)) {
         return res.value();
      }
   }

   void report(Throwable t) {
   }

   static class Res implements AutoCloseable {
      int value() {
         return 1;
      }

      public void close() {
      }
   }
}
