package pkg;

public class TestIfDenestFallthrough {
   private boolean closed;
   private int count;

   public void assertThenTryFinally(int var1) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 0
      // 01: getfield pkg/TestIfDenestFallthrough.closed Z
      // 04: ifeq 2b
      // 07: getstatic pkg/TestIfDenestFallthrough.$assertionsDisabled Z
      // 0a: ifne 19
      // 0d: iload 1
      // 0e: ifgt 19
      // 11: new java/lang/AssertionError
      // 14: dup
      // 15: invokespecial java/lang/AssertionError.<init> ()V
      // 18: athrow
      // 19: aload 0
      // 1a: invokevirtual pkg/TestIfDenestFallthrough.run ()V
      // 1d: aload 0
      // 1e: invokevirtual pkg/TestIfDenestFallthrough.cleanup ()V
      // 21: goto 2b
      // 24: astore 2
      // 25: aload 0
      // 26: invokevirtual pkg/TestIfDenestFallthrough.cleanup ()V
      // 29: aload 2
      // 2a: athrow
      // 2b: return
      // try (11 -> 13): 16 null
   }

   public int tryFinallyThenCode(int x) {
      label29: {
         if (!this.closed) {
            this.count++;
            return 0;
         }

         if (x <= 0) {
            return 1;
         }

         try {
            this.run();
            break label29;
         } finally {
            this.cleanup();
         }
      }
   }

   public void tryCatchReturn(int x) {
      label16:
      if (!this.closed) {
         this.count++;
      } else if (x > 0) {
         try {
            this.run();
            break label16;
         } catch (RuntimeException e) {
         }
      }
   }

   public int tryFinallyThenReturn(int x) {
      if (!this.closed) {
         return 0;
      }

      if (x <= 0) {
         return 1;
      }

      try {
         this.run();
         return 0;
      } finally {
         this.cleanup();
      }
   }

   private void run() {
   }

   private void cleanup() {
   }
}
