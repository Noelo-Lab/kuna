package pkg;

import java.util.function.IntSupplier;
import java.util.function.Supplier;

public class TestStaticFieldBehindLambda {
   static String label = "none";
   static int total;
   static int other;

   public Supplier<String> named(String label, int total) {
      return () -> TestStaticFieldBehindLambda.label + TestStaticFieldBehindLambda.total;
   }

   public IntSupplier local(int seed) {
      int total = seed * 2;
      return () -> TestStaticFieldBehindLambda.total + other + seed;
   }

   public Supplier<IntSupplier> nested(int total) {
      return () -> () -> TestStaticFieldBehindLambda.total + other;
   }

   public IntSupplier free(int seed) {
      return () -> total + other + seed;
   }
}
