package pkg;

import java.util.ArrayList;
import java.util.List;
import java.util.function.BiFunction;
import java.util.function.Supplier;

public class TestParameterNamesNoDebug {
   static int total;
   static String label = "none";
   private final List<String> items;
   private int limit;

   public TestParameterNamesNoDebug(List<String> items, int limit) {
      this.items = new ArrayList<>(items);
      this.limit = limit;
   }

   public static int record(int total, String label) {
      TestParameterNamesNoDebug.total += total;
      TestParameterNamesNoDebug.label = label;
      return TestParameterNamesNoDebug.total + label.length();
   }

   public boolean accept(String items) {
      if (this.items.size() >= limit) {
         return false;
      }
      return this.items.add(items);
   }

   public Supplier<String> later(String label, int limit) {
      return () -> label + this.limit + limit + TestParameterNamesNoDebug.label;
   }

   public Supplier<String> outside(String label, int total) {
      return () -> TestParameterNamesNoDebug.label + TestParameterNamesNoDebug.total;
   }

   public BiFunction<String, Integer, String> joiner(String separator) {
      return (label, total) -> label + separator + total + items.size();
   }

   public Comparable<String> ordered(String limit) {
      return new Comparable<String>() {
         @Override
         public int compareTo(String items) {
            return items.compareTo(limit) + TestParameterNamesNoDebug.this.limit;
         }

         public boolean same(Object limit) {
            return limit == this;
         }
      };
   }

   public int count(int limit) {
      class Counter {
         int from(int total, int items) {
            return total + items + limit + TestParameterNamesNoDebug.total;
         }
      }

      return new Counter().from(limit, this.limit);
   }
}
