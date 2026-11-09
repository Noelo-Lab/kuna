package pkg;

public class TestCatchLoopBack {
   private TestCatchLoopBack.Source in;
   private int attempt;
   private long offset;

   public int read() throws TestCatchLoopBack.IoError {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 0
      // 01: getfield pkg/TestCatchLoopBack.attempt I
      // 04: istore 1
      // 05: aload 0
      // 06: getfield pkg/TestCatchLoopBack.in Lpkg/TestCatchLoopBack$Source;
      // 09: invokeinterface pkg/TestCatchLoopBack$Source.read ()I 1
      // 0e: istore 2
      // 0f: iload 2
      // 10: bipush -1
      // 11: if_icmpeq 1e
      // 14: aload 0
      // 15: dup
      // 16: getfield pkg/TestCatchLoopBack.offset J
      // 19: lconst_1
      // 1a: ladd
      // 1b: putfield pkg/TestCatchLoopBack.offset J
      // 1e: aload 0
      // 1f: iload 1
      // 20: invokevirtual pkg/TestCatchLoopBack.record (I)V
      // 23: iload 2
      // 24: ireturn
      // 25: astore 2
      // 26: aload 0
      // 27: iload 1
      // 28: aload 2
      // 29: invokevirtual pkg/TestCatchLoopBack.retry (ILjava/lang/Exception;)V
      // 2c: goto 05
      // 2f: astore 2
      // 30: aload 0
      // 31: iload 1
      // 32: aload 2
      // 33: invokevirtual pkg/TestCatchLoopBack.retry (ILjava/lang/Exception;)V
      // 36: goto 05
      // try (3 -> 20): 21 pkg/TestCatchLoopBack$IoError
      // try (3 -> 20): 27 pkg/TestCatchLoopBack$StateError
   }

   public int readThree() throws TestCatchLoopBack.IoError {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 0
      // 01: bipush 0
      // 02: invokevirtual pkg/TestCatchLoopBack.record (I)V
      // 05: aload 0
      // 06: getfield pkg/TestCatchLoopBack.in Lpkg/TestCatchLoopBack$Source;
      // 09: invokeinterface pkg/TestCatchLoopBack$Source.read ()I 1
      // 0e: ireturn
      // 0f: astore 1
      // 10: aload 0
      // 11: bipush 1
      // 12: invokevirtual pkg/TestCatchLoopBack.record (I)V
      // 15: goto 05
      // 18: astore 1
      // 19: aload 0
      // 1a: bipush 2
      // 1b: invokevirtual pkg/TestCatchLoopBack.record (I)V
      // 1e: goto 05
      // 21: astore 1
      // 22: aload 0
      // 23: bipush 3
      // 24: invokevirtual pkg/TestCatchLoopBack.record (I)V
      // 27: goto 05
      // try (3 -> 6): 7 pkg/TestCatchLoopBack$IoError
      // try (3 -> 6): 12 pkg/TestCatchLoopBack$StateError
      // try (3 -> 6): 17 pkg/TestCatchLoopBack$OtherError
   }

   public int readOrDefault() throws TestCatchLoopBack.IoError {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 0
      // 01: bipush 0
      // 02: invokevirtual pkg/TestCatchLoopBack.record (I)V
      // 05: aload 0
      // 06: getfield pkg/TestCatchLoopBack.in Lpkg/TestCatchLoopBack$Source;
      // 09: invokeinterface pkg/TestCatchLoopBack$Source.read ()I 1
      // 0e: ireturn
      // 0f: astore 1
      // 10: aload 0
      // 11: bipush 1
      // 12: invokevirtual pkg/TestCatchLoopBack.record (I)V
      // 15: goto 05
      // 18: astore 1
      // 19: aload 0
      // 1a: bipush 2
      // 1b: invokevirtual pkg/TestCatchLoopBack.record (I)V
      // 1e: goto 05
      // 21: astore 1
      // 22: goto 25
      // 25: bipush -1
      // 26: ireturn
      // try (3 -> 6): 7 pkg/TestCatchLoopBack$IoError
      // try (3 -> 6): 12 pkg/TestCatchLoopBack$StateError
      // try (3 -> 6): 17 pkg/TestCatchLoopBack$OtherError
   }

   public int readGuarded(boolean param1) throws TestCatchLoopBack.IoError {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: iload 1
      // 01: ifeq 09
      // 04: aload 0
      // 05: bipush 0
      // 06: invokevirtual pkg/TestCatchLoopBack.record (I)V
      // 09: aload 0
      // 0a: getfield pkg/TestCatchLoopBack.in Lpkg/TestCatchLoopBack$Source;
      // 0d: invokeinterface pkg/TestCatchLoopBack$Source.read ()I 1
      // 12: ireturn
      // 13: astore 2
      // 14: aload 0
      // 15: bipush 1
      // 16: invokevirtual pkg/TestCatchLoopBack.record (I)V
      // 19: goto 09
      // 1c: astore 2
      // 1d: aload 0
      // 1e: bipush 2
      // 1f: invokevirtual pkg/TestCatchLoopBack.record (I)V
      // 22: goto 09
      // 25: astore 2
      // 26: bipush 0
      // 27: ireturn
      // try (5 -> 8): 9 pkg/TestCatchLoopBack$IoError
      // try (5 -> 8): 14 pkg/TestCatchLoopBack$StateError
      // try (5 -> 8): 19 pkg/TestCatchLoopBack$OtherError
      // try (9 -> 19): 19 pkg/TestCatchLoopBack$OtherError
   }

   public int readFinally() throws TestCatchLoopBack.IoError {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 0
      // 01: bipush 0
      // 02: invokevirtual pkg/TestCatchLoopBack.record (I)V
      // 05: aload 0
      // 06: getfield pkg/TestCatchLoopBack.in Lpkg/TestCatchLoopBack$Source;
      // 09: invokeinterface pkg/TestCatchLoopBack$Source.read ()I 1
      // 0e: istore 1
      // 0f: aload 0
      // 10: bipush 5
      // 11: invokevirtual pkg/TestCatchLoopBack.record (I)V
      // 14: iload 1
      // 15: ireturn
      // 16: astore 1
      // 17: aload 0
      // 18: bipush 1
      // 19: invokevirtual pkg/TestCatchLoopBack.record (I)V
      // 1c: goto 05
      // 1f: astore 1
      // 20: aload 0
      // 21: bipush 2
      // 22: invokevirtual pkg/TestCatchLoopBack.record (I)V
      // 25: goto 05
      // 28: astore 2
      // 29: aload 0
      // 2a: bipush 5
      // 2b: invokevirtual pkg/TestCatchLoopBack.record (I)V
      // 2e: aload 2
      // 2f: athrow
      // try (3 -> 7): 12 pkg/TestCatchLoopBack$IoError
      // try (3 -> 7): 17 pkg/TestCatchLoopBack$StateError
      // try (3 -> 7): 22 null
      // try (12 -> 23): 22 null
   }

   public int readOneCatch() throws TestCatchLoopBack.IoError {
      this.record(0);

      while (true) {
         try {
            return this.in.read();
         } catch (TestCatchLoopBack.IoError e) {
            this.record(1);
         }
      }
   }

   void record(int a) {
   }

   void retry(int a, Exception e) {
   }

   static class IoError extends Exception {
   }

   static class OtherError extends RuntimeException {
   }

   interface Source {
      int read() throws TestCatchLoopBack.IoError;
   }

   static class StateError extends RuntimeException {
   }
}
