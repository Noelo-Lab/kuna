package dx;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.function.Function;
import java.util.function.IntBinaryOperator;
import java.util.function.Supplier;

public class Objects {
    static int created;
    static final String NAME = "objects";
    static final long BIG = 1L << 40;
    static final double HALF = 0.5;
    static String greeting;
    final int id;
    private final List<String> tags = new ArrayList<>();

    static {
        created = 100;
        greeting = "hi " + NAME;
    }

    {
        tags.add("init");
    }

    Objects(int id) {
        this.id = id;
        created++;
    }

    interface Shape {
        double area();

        default String describe() {
            return getClass().getSimpleName() + "(" + area() + ")";
        }
    }

    enum Color {
        RED, GREEN, BLUE;

        Color next() {
            return values()[(ordinal() + 1) % values().length];
        }
    }

    static class Square implements Shape {
        private final double side;

        Square(double side) {
            this.side = side;
        }

        public double area() {
            return side * side;
        }
    }

    class Inner {
        int twice() {
            return id * 2;
        }
    }

    static abstract class Base {
        abstract String name();

        String hello() {
            return "hello " + name();
        }
    }

    String local(final int k) {
        class Local extends Base {
            String name() {
                return "local" + k + id;
            }
        }
        Base anon = new Base() {
            String name() {
                return "anon" + tags.size();
            }
        };
        return new Local().hello() + "/" + anon.hello();
    }

    static String colors() {
        StringBuilder sb = new StringBuilder();
        for (Color c : Color.values()) {
            switch (c) {
                case RED: sb.append('r'); break;
                case GREEN: sb.append('g'); break;
                default: sb.append('?');
            }
            sb.append(c.next().name().toLowerCase());
        }
        return sb.toString() + Color.valueOf("BLUE").ordinal();
    }

    static String lambdas(List<String> words) {
        Function<String, Integer> len = String::length;
        IntBinaryOperator max = Math::max;
        int best = 0;
        for (String w : words) {
            best = max.applyAsInt(best, len.apply(w));
        }
        Supplier<String> sup = () -> "best=";
        List<String> sorted = new ArrayList<>(words);
        Comparator<String> byLength = (p, q) -> p.length() != q.length() ? p.length() - q.length() : q.compareTo(p);
        sorted.sort(byLength);
        final int offset = best;
        Function<Integer, Integer> add = x -> x + offset;
        return sup.get() + best + sorted + add.apply(1);
    }

    @Override
    public String toString() {
        return "Objects#" + id + tags;
    }

    public static void main(String[] args) {
        Objects o = new Objects(7);
        System.out.println(created + " " + greeting + " " + BIG + " " + HALF + " " + o);
        Shape sq = new Square(1.5);
        Shape lam = () -> 2.0;
        System.out.println(sq.describe() + " " + lam.area());
        System.out.println(o.new Inner().twice() + " " + o.local(3));
        System.out.println(colors());
        List<String> words = new ArrayList<>();
        words.add("pear");
        words.add("fig");
        words.add("banana");
        words.add("kiwi");
        System.out.println(lambdas(words));
        Runnable r = () -> System.out.println("run " + created);
        r.run();
        Object x = o;
        System.out.println(x.hashCode() == o.hashCode());
        System.out.println(String.format("%d-%s-%.2f", 5, "s", 1.25));
    }
}
