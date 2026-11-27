package pkg;

import java.io.Serializable;

public class TestSerializableLambda {
   interface Action {
      void run();
   }

   interface Measure<T> {
      int of(T value);
   }

   interface Stored extends Action, Serializable {
   }

   static Action action() {
      return (Action & Serializable) () -> System.out.println("run");
   }

   static Measure<String> measure() {
      return (Measure<String> & Serializable) s -> s.length();
   }

   static Measure<String> reference() {
      return (Measure<String> & Serializable) String::length;
   }

   static Stored implied() {
      return () -> System.out.println("stored");
   }

   static Action plain() {
      return () -> System.out.println("plain");
   }
}
