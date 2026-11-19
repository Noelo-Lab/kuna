// Source of tries.dex (with Base.java), the DEX fixture of
// tests/stages/kuna-dex-javatry.xml and kuna-dex-javactor.xml.
//
//   javac --release 8 -d classes Tries.java Base.java
//   java -cp r8-8.5.35.jar com.android.tools.r8.D8 --release --min-api 21 \
//       --output out classes/demo/*.class
//
// r8-8.5.35.jar is com.android.tools:r8:8.5.35 from Google's Maven repository.
package demo;

public class Base {
    int b;
    String name;

    public Base(int b, String name) {
        this.b = b;
        this.name = name;
    }

    public Base() {
        this(1, "base");
    }
}
