package com.ex.fail;

import java.util.function.IntSupplier;
import java.util.function.Supplier;

public class Broken {
    static final Supplier<Runnable> HEADER = () -> new Runnable() {
        @Mark('空')
        public void run() {
        }
    };

    static IntSupplier lambda() {
        IntSupplier s = () -> 0x7b7b;
        return s;
    }

    static Runnable anonymous() {
        return new Runnable() {
            public void run() {
                System.out.println(0x7b7b);
            }
        };
    }

    static int plain(int x) {
        return x * 2;
    }
}
