// Compiled with ECJ 3.46.100 (org.eclipse.jdt:ecj): java -jar ecj-3.46.100.jar -g -17 -d kuna-fixes/classes
package pkg;

public class TestEcjStringSwitch {
    enum Kind { RED, GREEN, BLUE, NONE }

    static Kind kind(String word) {
        return switch (word) {
            case "green" -> Kind.GREEN;
            case "red" -> Kind.RED;
            case "blue" -> Kind.BLUE;
            default -> Kind.NONE;
        };
    }
}
