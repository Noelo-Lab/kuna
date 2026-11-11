package pkg;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.function.Supplier;

public class TestDeclarationScopes {
   private int[] mask;

   static boolean identical(int[] a, int[] b, int length) {
      int tmplength;
      int mask1;
      int mask2;
      int value1;
      int value2;
      int sbit = 0;

      while (sbit < length) {
         tmplength = length - sbit;
         if (tmplength > 4) {
            tmplength = 4;
         }

         mask1 = a[sbit];
         value1 = a[sbit + 1];
         mask2 = b[sbit];
         value2 = b[sbit + 1];
         if (mask1 != mask2) {
            return false;
         }

         if ((mask1 & value1) != (mask2 & value2)) {
            return false;
         }

         sbit += tmplength;
      }

      return true;
   }

   String hex() {
      int b;
      String s = "";

      String t;
      for (int i = 0; i < this.mask.length; i++) {
         b = 0xFF & this.mask[i];
         t = Integer.toString(b, 16);
         if (1 == t.length()) {
            t = "0" + t;
         }

         s = s + t;
      }

      return s;
   }

   int setCount(Map<String, Integer> map, String key, int count) {
      Integer existing;
      int old;
      if (count == 0) {
         existing = (Integer)map.remove(key);
         old = existing == null ? 0 : existing;
      } else {
         existing = (Integer)map.put(key, count);
         old = existing == null ? 0 : existing;
         if (existing == null) {
            log(key);
         }
      }

      return count - old;
   }

   static int best(int[] scores, int limit) {
      int best = 0;
      int sbit;
      int size;
      boolean context = true;

      do {
         for (sbit = 0; sbit < limit; sbit++) {
            int score = scores[sbit];
            if (score > best) {
               best = score;
            }
         }

         context = !context;
      } while (!context);

      for (size = 2; size <= 8; size++) {
         for (sbit = 0; sbit < limit - size; sbit++) {
            int score = scores[sbit] * size;
            if (score > best) {
               best = score;
            }
         }
      }

      return best;
   }

   static byte[] hash(String home, TestDeclarationScopes.Source source) {
      String base = home.trim();

      TestDeclarationScopes.Digest digest;
      try {
         digest = source.open(base);
         digest.update(base);
         return digest.finish();
      } catch (IOException ex) {
         throw new IllegalStateException(ex);
      }
   }

   static List<String> drain(Iterator<String> it) {
      String r;
      try {
         List<String> out = new ArrayList();

         while ((r = next(it)) != null) {
            out.add(r);
         }

         return out;
      } catch (IOException e) {
         throw new IllegalStateException("won't happen", e);
      }
   }

   static List<TestDeclarationScopes.Node> parents(TestDeclarationScopes.Node start) {
      TestDeclarationScopes.Node node = start.parent();
      if (node == null) {
         return List.of();
      }

      List<TestDeclarationScopes.Node> parents = new ArrayList();

      for (node = node.parent(); node != null && !parents.contains(node); node = node.parent()) {
         parents.add(node);
      }

      return parents;
   }

   static boolean prune(Map<String, Object> fields, boolean keep) {
      Iterator<Entry<String, Object>> it = fields.entrySet().iterator();

      while (it.hasNext()) {
         Object value = ((Entry)it.next()).getValue();
         boolean isMulti = false;
         if (value instanceof Map) {
            Map<?, ?> map = (Map<?, ?>)value;
            Object properties = map.get("properties");
            if (properties == null) {
               isMulti = map.containsKey("fields");
            }
         }

         if (!keep && isMulti) {
            Map<?, ?> map = (Map<?, ?>)value;
            Object subFields = map.get("fields");
            log(String.valueOf(subFields));
         }
      }

      return fields.isEmpty();
   }

   static int captured(List<String> names) {
      int total = 0;

      for (int i = 0; i < names.size(); i++) {
         String name = (String)names.get(i);
         Supplier<Integer> len = () -> name.length();
         total += len.get();
      }

      return total;
   }

   static String next(Iterator<String> it) throws IOException {
      return it.hasNext() ? (String)it.next() : null;
   }

   static void log(String message) {
   }

   interface Digest {
      void update(String var1);

      byte[] finish();
   }

   interface Node {
      TestDeclarationScopes.Node parent();
   }

   interface Source {
      TestDeclarationScopes.Digest open(String var1) throws IOException;
   }
}
