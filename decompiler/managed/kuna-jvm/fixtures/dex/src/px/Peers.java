package px;

// A method reference whose d8 lambda class lands in another DEX file of the input
// (regen.sh splits the d8 output into one DEX per class).
public class Peers {
    interface Op {
        long apply(long a, String b);
    }

    static class Impl {
        static long twice(long a, CharSequence b) {
            return a * 2 + b.length();
        }
    }

    static int width(int x) {
        return x + 1;
    }

    static Op get() {
        return Impl::twice;
    }

    public static void main(String[] args) {
        System.out.println(get().apply(width(2), "ab"));
    }
}
