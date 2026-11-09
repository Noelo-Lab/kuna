package com.ex.fail;

import java.util.function.IntFunction;

public class Nested {
    static final IntFunction<Object> QUOTE = c -> new StringBuilder().append(switch (c) {
        case '"' -> "\\\"";
        default -> (char) c;
    });

    static String escape(CharSequence input) {
        StringBuilder b = new StringBuilder();
        input.chars().forEach(c -> b.append(switch (c) {
            case '"' -> "\\\"";
            default -> (char) c;
        }));
        return b.toString();
    }

    static String both(CharSequence a, CharSequence z) {
        StringBuilder b = new StringBuilder();
        a.chars().forEach(c -> b.append(switch (c) {
            case '"' -> "\\\"";
            default -> (char) c;
        }));
        z.chars().forEach(c -> b.append(switch (c) {
            case '\\' -> "\\\\";
            default -> (char) c;
        }));
        return b.toString();
    }

    static Runnable later(StringBuilder b, int c) {
        return new Runnable() {
            @Override
            public void run() {
                b.append(switch (c) {
                    case '"' -> "\\\"";
                    default -> (char) c;
                });
            }
        };
    }

    static int plain(int x) {
        return x + 1;
    }
}
