package pkg;

import java.util.function.Supplier;

public class TestSyntheticReceiverCapture {
   interface Job {
      int run();
   }

   static int total;

   static void add(Job job) {
      total += job.run();
   }

   void only(Object a, Object b, Object c) {
      Object o = c.toString();
      add(() -> o.hashCode());
   }

   void sup(Object a, Object b, Object c) {
      Object o = c.toString();
      Supplier<String> s = () -> o.toString();
      add(() -> s.get().length());
   }

   int twoArgs(String a, String b) {
      String o = a.trim();
      add(() -> o.indexOf(b));
      return total;
   }

   void parameter(Object a, Object b) {
      Object o = b;
      add(() -> o.hashCode());
   }

   void reassigned(Object a, Object b) {
      Object o = b;
      b = a;
      add(() -> o.hashCode() + 1);
   }

   void self() {
      TestSyntheticReceiverCapture me = this;
      add(() -> me.hashCode() + 2);
   }
}
