package pkg;

public class TestJavacTails {
   interface Source {
      int next();

      void close(int el);
   }

   interface Cursor {
      boolean more();

      String next();
   }

   interface Res extends AutoCloseable {
      @Override
      void close();
   }

   static final class Node {
      Node next;
      int value;
   }

   private Node first;
   private Node second;
   private int total;
   private String name;
   private Res lock;

   static void decodeAll(Source d, StringBuilder out) {
      int el = d.next();
      for (;;) {
         int sub = d.next();
         if (sub == 0) {
            break;
         }
         out.append(sub);
      }
      d.close(el);
   }

   static int decodePieces(Source d) {
      int logicalSize = 0;
      int sizeAccum = 0;
      for (;;) {
         int attribId = d.next();
         if (attribId == 0) {
            break;
         } else if (attribId == 1) {
            logicalSize = d.next();
         }
         sizeAccum += attribId;
      }
      int result = logicalSize != 0 ? logicalSize : sizeAccum;
      return result;
   }

   static String squeeze(char[] chars, int pos) {
      int spread = 1;
      OUT:
      while (true) {
         pos++;
         while (true) {
            if (pos == chars.length) {
               break OUT;
            }
            if (chars[pos] == ' ') {
               break;
            }
            chars[pos - spread] = chars[pos];
            pos++;
         }
         spread++;
      }
      return new String(chars, 0, pos - spread);
   }

   int collect() {
      int n = 0;
      Node node = this.first;
      while (node != null) {
         n += node.value;
         node = node.next;
      }
      Node other = this.second;
      while (other != null) {
         n += other.value;
         other = other.next;
      }
      return n;
   }

   static int skipSpaces(String s, int i) {
      int n = s.length();
      for (; i < n; i++) {
         if (s.charAt(i) != ' ') {
            break;
         }
      }
      return i;
   }

   static StringBuilder create(String text) {
      StringBuilder builder;
      try {
         builder = new StringBuilder(text);
         builder.append(Integer.parseInt(text));
      } catch (NumberFormatException e) {
         throw new IllegalStateException("not a number: " + text, e);
      }
      return builder;
   }

   int drain(int max) {
      int taken = 0;
      synchronized (this) {
         while (this.total > 0 && taken < max) {
            this.total--;
            taken++;
         }
      }
      return taken;
   }

   static int count(Cursor words) {
      int n = 0;
      while (words.more()) {
         String w = words.next();
         if (w.isEmpty() || w.startsWith("#")) {
            continue;
         }
         n++;
      }
      return n;
   }

   void rename(String newName) {
      try (Res r = this.lock) {
         if (this.name.equals(newName)) {
            return;
         }
         this.name = newName;
         this.total++;
      } catch (IllegalStateException e) {
         this.total = -1;
      }
   }

   static void report(boolean debug, Object client, StringBuilder out) {
      try {
         if (!debug) {
            return;
         }
         if (client == null) {
            return;
         }
         out.append(client);
      } finally {
         out.append('.');
      }
   }
}
