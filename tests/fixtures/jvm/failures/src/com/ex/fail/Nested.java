package com.ex.fail;

import java.util.function.IntFunction;

public class Nested {
    static final IntFunction<Runnable> QUOTE = x -> new Runnable() {
        @Mark('空')
        int mark;

        public void run() {
        }
    };

    static Runnable escape(int c) {
        IntFunction<Runnable> make = x -> new Runnable() {
            @Mark('空')
            public void run() {
            }
        };
        return make.apply(c);
    }

    static Runnable both(int c) {
        IntFunction<Runnable> one = x -> new Runnable() {
            @Mark('空')
            public void run() {
            }
        };
        IntFunction<Runnable> two = x -> new Runnable() {
            @Mark('空')
            public void run() {
            }
        };
        return c > 0 ? one.apply(c) : two.apply(c);
    }

    static Runnable later(StringBuilder b) {
        return new Runnable() {
            @Mark('空')
            public void run() {
                b.append('"');
            }
        };
    }

    static int plain(int x) {
        return x + 1;
    }
}
