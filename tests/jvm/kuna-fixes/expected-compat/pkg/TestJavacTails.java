package pkg;

public class TestJavacTails {
   private TestJavacTails.Node first;
   private TestJavacTails.Node second;
   private int total;
   private String name;
   private TestJavacTails.Res lock;

   static void decodeAll(TestJavacTails.Source d, StringBuilder out) {
      int el = d.next();

      while (true) {
         int sub = d.next();
         if (sub == 0) {
            d.close(el);
            return;
         }

         out.append(sub);
      }
   }

   static int decodePieces(TestJavacTails.Source d) {
      int logicalSize = 0;
      int sizeAccum = 0;

      while (true) {
         int attribId = d.next();
         if (attribId == 0) {
            return logicalSize != 0 ? logicalSize : sizeAccum;
         }

         if (attribId == 1) {
            logicalSize = d.next();
         }

         sizeAccum += attribId;
      }
   }

   static String squeeze(char[] chars, int pos) {
      int spread = 1;

      label21:
      while (true) {
         pos++;

         while (pos != chars.length) {
            if (chars[pos] == ' ') {
               spread++;
               continue label21;
            }

            chars[pos - spread] = chars[pos];
            pos++;
         }

         return new String(chars, 0, pos - spread);
      }
   }

   int collect() {
      int n = 0;

      for (TestJavacTails.Node node = this.first; node != null; node = node.next) {
         n += node.value;
      }

      for (TestJavacTails.Node other = this.second; other != null; other = other.next) {
         n += other.value;
      }

      return n;
   }

   static int skipSpaces(String s, int i) {
      int n = s.length();

      while (i < n && s.charAt(i) == ' ') {
         i++;
      }

      return i;
   }

   static StringBuilder create(String text) {
      try {
         StringBuilder builder = new StringBuilder(text);
         builder.append(Integer.parseInt(text));
         return builder;
      } catch (NumberFormatException e) {
         throw new IllegalStateException("not a number: " + text, e);
      }
   }

   int drain(int max) {
      int taken = 0;
      synchronized (this) {
         while (this.total > 0 && taken < max) {
            this.total--;
            taken++;
         }

         return taken;
      }
   }

   static int count(TestJavacTails.Cursor words) {
      int n = 0;

      while (words.more()) {
         String w = words.next();
         if (!w.isEmpty() && !w.startsWith("#")) {
            n++;
         }
      }

      return n;
   }

   void rename(String newName) {
      try (this.lock) {
         if (!this.name.equals(newName)) {
            this.name = newName;
            this.total++;
         }
      } catch (IllegalStateException e) {
         this.total = -1;
      }
   }

   static void report(boolean debug, Object client, StringBuilder out) {
      try {
         if (!debug) {
            return;
         }

         if (client != null) {
            out.append(client);
            return;
         }
      } finally {
         out.append('.');
      }
   }

   interface Cursor {
      boolean more();

      String next();
   }

   static final class Node {
      TestJavacTails.Node next;
      int value;
   }

   interface Res extends AutoCloseable {
      void close();
   }

   interface Source {
      int next();

      void close(int var1);
   }
}
