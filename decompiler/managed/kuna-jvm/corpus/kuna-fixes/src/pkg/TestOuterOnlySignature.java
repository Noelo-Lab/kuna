package pkg;

public class TestOuterOnlySignature {
   int count;

   public Sink counting(final Sink sink) {
      return new Sink() {
         @Override
         public void write(String s) {
            TestOuterOnlySignature.this.count++;
            sink.write(s);
         }
      };
   }

   public Forwarding tagged(Sink sink, final String tag) {
      return new Forwarding(sink) {
         @Override
         void write(String s) {
            super.write(tag + s + TestOuterOnlySignature.this.count);
         }
      };
   }

   public Object labelled(final String prefix) {
      class Labelled {
         final Box<String> box;

         Labelled(Box<String> box) {
            this.box = box;
         }

         @Override
         public String toString() {
            return prefix + this.box.value + TestOuterOnlySignature.this.count;
         }
      }

      return new Labelled(new Box<>());
   }

   public interface Sink {
      void write(String s);
   }

   public static class Forwarding {
      final Sink delegate;

      Forwarding(Sink delegate) {
         this.delegate = delegate;
      }

      void write(String s) {
         this.delegate.write(s);
      }
   }

   public static class Box<T> {
      T value;
   }
}
