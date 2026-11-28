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

   static String show(TestGenericArgumentOverload.Item i) {
      return "i";
   }

   String element(TestGenericArgumentOverload.Seq<String> seq, TestGenericArgumentOverload.Seq<TestGenericArgumentOverload.Item> items) {
      return show((Object)seq.get(0)) + show(seq.get(1)) + show((Object)items.get(2)) + show(items.get(3));
   }

   String function(TestGenericArgumentOverload.Fn<String, char[]> g, String x) {
      return show((Object)g.apply(x)) + show(g.apply(x));
   }

   String field(TestGenericArgumentOverload.Box<String> box, TestGenericArgumentOverload.Box<char[]> chars) {
      return show((Object)box.value) + show(box.value) + show((Object)chars.value) + show(chars.value);
   }

   String wildcard(
      TestGenericArgumentOverload.Seq<? extends String> seq,
      TestGenericArgumentOverload.Fn<TestGenericArgumentOverload.Item, ? extends char[]> f,
      TestGenericArgumentOverload.Seq<? super String> sink,
      TestGenericArgumentOverload.Seq<?> any
   ) {
      return show((Object)seq.get(0)) + show(seq.get(0)) + show((Object)f.apply(null)) + show(f.apply(null)) + show(sink.get(0)) + show(any.get(0));
   }

   <T extends TestGenericArgumentOverload.Item> String variable(
      TestGenericArgumentOverload.Seq<T> seq, TestGenericArgumentOverload.Box<T> box, TestGenericArgumentOverload.Seq raw
   ) {
      return show(seq.get(0)) + show((Object)seq.get(0)) + show(box.value) + show(raw.get(0)) + show((String)raw.get(1));
   }

   static class Box<T> {
      T value;
   }

   interface Fn<A, B> {
      B apply(A var1);
   }

   static class Item {
   }

   interface Seq<T> {
      T get(int var1);
   }
}
