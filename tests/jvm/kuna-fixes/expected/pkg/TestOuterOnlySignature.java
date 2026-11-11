package pkg;

public class TestOuterOnlySignature {
   int count;

   public TestOuterOnlySignature.Sink counting(final TestOuterOnlySignature.Sink var1) {
      return new TestOuterOnlySignature.Sink() {
         @Override
         public void write(String var1x) {
            TestOuterOnlySignature.this.count++;
            var1.write(var1x);
         }
      };
   }

   public TestOuterOnlySignature.Forwarding tagged(TestOuterOnlySignature.Sink var1, final String var2) {
      return new TestOuterOnlySignature.Forwarding(var1) {
         @Override
         void write(String var1) {
            super.write(var2 + var1 + TestOuterOnlySignature.this.count);
         }
      };
   }

   public Object labelled(final String var1) {
      class Labelled {
         final TestOuterOnlySignature.Box<String> box;

         Labelled(TestOuterOnlySignature.Box<String> var2) {
            this.box = var2;
         }

         public String toString() {
            return var1 + this.box.value + TestOuterOnlySignature.this.count;
         }
      }

      return new Labelled(new TestOuterOnlySignature.Box<>());
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
