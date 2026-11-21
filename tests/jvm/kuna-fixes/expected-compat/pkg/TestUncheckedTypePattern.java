package pkg;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class TestUncheckedTypePattern {
   public int size(Object value) {
      return value instanceof List<String> list ? ((String)list.get(0)).length() : -1;
   }

   public String fromBox(TestUncheckedTypePattern.Box<String> box) {
      String s = "";
      if (box instanceof TestUncheckedTypePattern.Leaf<String> leaf) {
         s = leaf.get();
      }

      return s;
   }

   public Integer fromPair(TestUncheckedTypePattern.Box<String> box) {
      return box instanceof TestUncheckedTypePattern.Pair<String, Integer> pair ? pair.get().length() : null;
   }

   public int wildcard(Object value) {
      int n = 0;
      if (value instanceof List<?> list) {
         n = list.size();
      }

      return n;
   }

   interface Box<T> {
      T get();
   }

   static class Grammar<N extends TestUncheckedTypePattern.Leaf<String>> {
      final Map<String, N> named = new HashMap();

      void add(Iterable<TestUncheckedTypePattern.Node> nodes) {
         for (TestUncheckedTypePattern.Node node : nodes) {
            if (node instanceof N leaf) {
               this.named.put(leaf.toString(), leaf);
            }
         }
      }
   }

   static class Leaf<T> implements TestUncheckedTypePattern.Box<T>, TestUncheckedTypePattern.Node {
      @Override
      public T get() {
         return null;
      }
   }

   interface Node {
   }

   static class Pair<A, B> implements TestUncheckedTypePattern.Box<A> {
      @Override
      public A get() {
         return null;
      }
   }
}
