package pkg;

public class TestOuterOnlySignature {
   int count;

   public TestOuterOnlySignature.Sink counting(final TestOuterOnlySignature.Sink var1) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 0: new pkg/TestOuterOnlySignature$1
      // 3: dup
      // 4: aload 0
      // 5: aload 1
      // 6: invokespecial pkg/TestOuterOnlySignature$1.<init> (Lpkg/TestOuterOnlySignature;Lpkg/TestOuterOnlySignature$Sink;)V
      // 9: areturn
   }

   public TestOuterOnlySignature.Forwarding tagged(TestOuterOnlySignature.Sink var1, final String var2) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 0: new pkg/TestOuterOnlySignature$2
      // 3: dup
      // 4: aload 0
      // 5: aload 1
      // 6: aload 2
      // 7: invokespecial pkg/TestOuterOnlySignature$2.<init> (Lpkg/TestOuterOnlySignature;Lpkg/TestOuterOnlySignature$Sink;Ljava/lang/String;)V
      // a: areturn
   }

   public Object labelled(final String var1) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: new pkg/TestOuterOnlySignature$1Labelled
      // 03: dup
      // 04: aload 0
      // 05: new pkg/TestOuterOnlySignature$Box
      // 08: dup
      // 09: invokespecial pkg/TestOuterOnlySignature$Box.<init> ()V
      // 0c: aload 1
      // 0d: invokespecial pkg/TestOuterOnlySignature$1Labelled.<init> (Lpkg/TestOuterOnlySignature;Lpkg/TestOuterOnlySignature$Box;Ljava/lang/String;)V
      // 10: areturn
   }

   public static class Box<T> {
      T value;
   }

   public static class Forwarding {
      final TestOuterOnlySignature.Sink delegate;

      Forwarding(TestOuterOnlySignature.Sink var1) {
         this.delegate = var1;
      }

      void write(String var1) {
         this.delegate.write(var1);
      }
   }

   public interface Sink {
      void write(String var1);
   }
}
