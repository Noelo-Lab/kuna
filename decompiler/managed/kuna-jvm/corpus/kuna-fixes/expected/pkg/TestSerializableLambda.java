package pkg;

import java.io.Serializable;

public class TestSerializableLambda {
   static TestSerializableLambda.Action action() {
      return (TestSerializableLambda.Action)((Serializable)((TestSerializableLambda.Action & Serializable)() -> System.out.println("run")));
   }

   static TestSerializableLambda.Measure<String> measure() {
      return (TestSerializableLambda.Measure<String> & Serializable)s -> s.length();
   }

   static TestSerializableLambda.Measure<String> reference() {
      return String::length;
   }

   static TestSerializableLambda.Stored implied() {
      return () -> System.out.println("stored");
   }

   static TestSerializableLambda.Action plain() {
      return () -> System.out.println("plain");
   }

   interface Action {
      void run();
   }

   interface Measure<T> {
      int of(T var1);
   }

   interface Stored extends TestSerializableLambda.Action, Serializable {
   }
}
