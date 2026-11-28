package pkg;

public class TestGenericArgumentOverload {
   static String show(Object o) {
      return "o";
   }

   static String show(char[] c) {
      return "c";
   }

   static String show(String s) {
      return "s";
   }

   static String show(Item i) {
      return "i";
   }

   String element(Seq<String> seq, Seq<Item> items) {
      return show((Object)seq.get(0)) + show(seq.get(1)) + show((Object)items.get(2)) + show(items.get(3));
   }

   String function(Fn<String, char[]> g, String x) {
      return show((Object)g.apply(x)) + show(g.apply(x));
   }

   String field(Box<String> box, Box<char[]> chars) {
      return show((Object)box.value) + show(box.value) + show((Object)chars.value) + show(chars.value);
   }

   String wildcard(Seq<? extends String> seq, Fn<Item, ? extends char[]> f, Seq<? super String> sink, Seq<?> any) {
      return show((Object)seq.get(0)) + show(seq.get(0)) + show((Object)f.apply(null)) + show(f.apply(null)) + show(sink.get(0)) + show(any.get(0));
   }

   <T extends Item> String variable(Seq<T> seq, Box<T> box, Seq raw) {
      return show(seq.get(0)) + show((Object)seq.get(0)) + show(box.value) + show(raw.get(0)) + show((String)raw.get(1));
   }

   static class Item {
   }

   interface Seq<T> {
      T get(int i);
   }

   interface Fn<A, B> {
      B apply(A a);
   }

   static class Box<T> {
      T value;
   }
}
