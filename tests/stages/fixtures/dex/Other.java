// Source of the second DEX file in two.apk, the fixture of
// tests/stages/kuna-dex-apk.xml. Built as Hello.java describes, against
// Hello's classes (javac -cp classes); two.apk is a zip of hello.dex as
// classes.dex (deflated) and this file's DEX as classes2.dex (stored).
package demo;

public class Other {
    public static int twice(int x) {
        return Hello.add(x, x);
    }
    public static String hi() {
        return Hello.greet("other");
    }
}
