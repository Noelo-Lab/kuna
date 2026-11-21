package pkg;

public class TestVarMergeUnrelatedType {
    static Object src(int i) {
        return i;
    }

    static void sink(Object o) {
        System.out.print(o + ";");
    }

    static void conditional(boolean c, int k) {
        {
            Object v = c ? "s" : Integer.valueOf(k);
            sink(v);
        }
        {
            Integer w = (Integer) src(k);
            sink(w + 1);
        }
    }

    static void branches(boolean c, int k) {
        {
            Object v;
            if (c) {
                v = "s";
            } else {
                v = Integer.valueOf(k);
            }
            sink(v);
        }
        {
            Integer w = (Integer) src(k);
            sink(w + 1);
        }
    }

    static void copy(boolean c, int k, String s) {
        {
            Object v = s;
            if (c) {
                v = Integer.valueOf(k);
            }
            sink(v);
        }
        {
            Integer w = (Integer) src(k);
            sink(w + 1);
        }
    }
}
