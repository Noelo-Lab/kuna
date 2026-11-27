package pkg;

public class TestTypeVariableScope<T> {
   static class Box<T> {
      T value;
      T[] all;
   }

   interface Run {
      int run();
   }

   class Inner {
      T inner;
   }

   T mine;
   T[] array;
   Box raw;

   int own(TestTypeVariableScope<T> other) {
      T a = mine;
      T b = other.mine;
      int s = a.hashCode() + b.hashCode();
      for (T x : array) {
         s += x.hashCode();
      }
      return s;
   }

   int rawReceiver() {
      Object a = raw.value;
      int s = a.hashCode();
      for (Object x : raw.all) {
         s += x.hashCode();
      }
      return s;
   }

   int otherArguments(Box<String> box, TestTypeVariableScope<String> other) {
      Object a = box.value;
      Object b = other.mine;
      return a.hashCode() + b.hashCode();
   }

   <T> int hidden(T t) {
      Object a = mine;
      Object b = t;
      return a.hashCode() + b.hashCode();
   }

   <T> Run hiddenInLambda(T t) {
      return () -> {
         Object a = mine;
         return a.hashCode() + t.hashCode();
      };
   }

   int innerField(TestTypeVariableScope<String>.Inner i) {
      Object a = i.inner;
      return a.hashCode();
   }
}
