// Source of hello.dex, the DEX fixture of tests/stages/kuna-dex.xml.
//
//   javac --release 8 -d classes Hello.java
//   java -cp r8-8.5.35.jar com.android.tools.r8.D8 --debug --min-api 21 \
//       --output out classes/demo/Hello.class
//
// r8-8.5.35.jar is com.android.tools:r8:8.5.35 from Google's Maven repository.
// `--debug` keeps the sparse switch a sparse-switch instruction.
package demo;

public class Hello {
    static int counter;
    int value;
    String name;
    long total;

    public Hello(String name, int value) {
        this.name = name;
        this.value = value;
    }

    public static int add(int a, int b) {
        return a + b;
    }

    public static long mulWide(long a, int b) {
        return a * b + 7L;
    }

    public int getValue() {
        return value + counter;
    }

    public static String greet(String who) {
        return "Hello, " + who;
    }

    public static int classify(int x) {
        switch (x) {
            case 0: return 3;
            case 1: return 10;
            case 2: return 20;
            case 3: return 33;
            case 4: return 47;
            case 5: return 51;
            case 6: return 64;
            case 7: return 70;
            case 8: return 85;
            case 9: return 99;
            default: return -1;
        }
    }

    public static int sparse(int x) {
        switch (x) {
            case 7: return 1;
            case 100: return 2;
            case 1000: return 3;
            case 54321: return 4;
            case -5: return 5;
            case 77777: return 6;
            case 123456: return 7;
            case 999999: return 8;
            default: return 0;
        }
    }

    public static int sum(int[] arr) {
        int s = 0;
        for (int i = 0; i < arr.length; i++) {
            s += arr[i];
        }
        return s;
    }

    public static int[] makeArray(int n) {
        int[] a = new int[n];
        a[0] = n;
        return a;
    }

    public static boolean isHello(Object o) {
        return o instanceof Hello;
    }

    public static String nameOf(Object o) {
        Hello h = (Hello) o;
        return h.name;
    }

    public void bump(int by) {
        counter += by;
        value = value * 2 + by;
        total += by;
    }

    public static int many(int a, int b, int c, int d, int e, int f) {
        return add(a, b) + add(c, d) + add(e, f);
    }

    public static int callMany(int x) {
        return many(x, 2, 3, 4, 5, 6) + many(x, x, x, x, x, x);
    }

    public int shifts(int x) {
        return (x >>> 3) + (x >> 2) + (value << 1);
    }

    public static void main(String[] args) {
        Hello h = new Hello("x", 3);
        System.out.println(greet(h.name) + h.getValue());
    }
}
