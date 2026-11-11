package pkg;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class TestUncheckedTypePattern {
   public int size(Object value) {
      if (value instanceof List) {
         List<String> list = (List<String>)value;
         return ((String)list.get(0)).length();
      }

      return -1;
   }

   public String fromBox(TestUncheckedTypePattern.Box<String> box) {
      String s = "";
      if (box instanceof TestUncheckedTypePattern.Leaf<String> leaf) {
         s = leaf.get();
      }

      return s;
   }

   public Integer fromPair(TestUncheckedTypePattern.Box<String> box) {
      if (box instanceof TestUncheckedTypePattern.Pair) {
         TestUncheckedTypePattern.Pair<String, Integer> pair = (TestUncheckedTypePattern.Pair<String, Integer>)box;
         return pair.get().length();
      }

      return null;
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
            if (node instanceof TestUncheckedTypePattern.Leaf) {
               N leaf = (N)node;
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
