// Golden vectors for rt::jnum, from a JDK 21:  java GenJnumGolden.java > jnum_golden.txt
//
// Lines (parser inputs are "=" + the text when it is printable ASCII, else "x" + hex UTF-16 units):
//   D <bits16> <Double.toString>          F <bits8> <Float.toString>
//   PD <input> <bits16|NFE>               PF <input> <bits8|NFE>
//   PI <input> <int|NFE>                  PL <input> <long|NFE>
//   HI <int> <Integer.toHexString>        HL <long> <Long.toHexString>
//   DB <bits16> <doubleToLongBits> <hashCode>   FB <bits8> <floatToIntBits> <hashCode>
import java.math.BigDecimal;
import java.util.*;

public class GenJnumGolden {
  static final Random R = new Random(0x5EED_F1L);
  static final LinkedHashSet<Long> DOUBLES = new LinkedHashSet<>();
  static final LinkedHashSet<Integer> FLOATS = new LinkedHashSet<>();
  static final LinkedHashSet<String> PARSE = new LinkedHashSet<>();
  static final LinkedHashSet<String> PARSE_INT = new LinkedHashSet<>();

  static void d(double v) { DOUBLES.add(Double.doubleToRawLongBits(v)); }
  static void f(float v) { FLOATS.add(Float.floatToRawIntBits(v)); }

  public static void main(String[] args) {
    doubles();
    floats();
    parseInputs();
    intInputs();
    StringBuilder out = new StringBuilder();
    for (long bits : DOUBLES) {
      out.append("D ").append(String.format("%016x", bits)).append(' ').append(Double.toString(Double.longBitsToDouble(bits))).append('\n');
    }
    for (int bits : FLOATS) {
      out.append("F ").append(String.format("%08x", bits)).append(' ').append(Float.toString(Float.intBitsToFloat(bits))).append('\n');
    }
    for (String s : PARSE) {
      String rd, rf;
      try { rd = String.format("%016x", Double.doubleToRawLongBits(Double.parseDouble(s))); } catch (NumberFormatException e) { rd = "NFE"; }
      try { rf = String.format("%08x", Float.floatToRawIntBits(Float.parseFloat(s))); } catch (NumberFormatException e) { rf = "NFE"; }
      out.append("PD ").append(enc(s)).append(' ').append(rd).append('\n');
      out.append("PF ").append(enc(s)).append(' ').append(rf).append('\n');
    }
    for (String s : PARSE_INT) {
      String ri, rl;
      try { ri = Integer.toString(Integer.parseInt(s)); } catch (NumberFormatException e) { ri = "NFE"; }
      try { rl = Long.toString(Long.parseLong(s)); } catch (NumberFormatException e) { rl = "NFE"; }
      out.append("PI ").append(enc(s)).append(' ').append(ri).append('\n');
      out.append("PL ").append(enc(s)).append(' ').append(rl).append('\n');
    }
    int[] ints = {0, 1, -1, 15, 16, 255, 0x7fffffff, 0x80000000, 0xdeadbeef, 12345, -12345};
    for (int i : ints) out.append("HI ").append(i).append(' ').append(Integer.toHexString(i)).append('\n');
    for (int k = 0; k < 40; k++) { int i = R.nextInt(); out.append("HI ").append(i).append(' ').append(Integer.toHexString(i)).append('\n'); }
    long[] longs = {0L, 1L, -1L, Long.MAX_VALUE, Long.MIN_VALUE, 0xcafebabeL, -4096L};
    for (long l : longs) out.append("HL ").append(l).append(' ').append(Long.toHexString(l)).append('\n');
    for (int k = 0; k < 40; k++) { long l = R.nextLong(); out.append("HL ").append(l).append(' ').append(Long.toHexString(l)).append('\n'); }
    long[] dbits = {0L, 0x8000000000000000L, 0x7ff8000000000000L, 0x7ff0000000000001L, 0xfff8000000000123L, 0x7ff0000000000000L, 0x3ff0000000000000L};
    for (long b : dbits) { double v = Double.longBitsToDouble(b); out.append("DB ").append(String.format("%016x", b)).append(' ').append(Double.doubleToLongBits(v)).append(' ').append(Double.hashCode(v)).append('\n'); }
    for (int k = 0; k < 30; k++) { long b = R.nextLong(); double v = Double.longBitsToDouble(b); out.append("DB ").append(String.format("%016x", b)).append(' ').append(Double.doubleToLongBits(v)).append(' ').append(Double.hashCode(v)).append('\n'); }
    int[] fbits = {0, 0x80000000, 0x7fc00000, 0x7f800001, 0xffc00123, 0x7f800000, 0x3f800000};
    for (int b : fbits) { float v = Float.intBitsToFloat(b); out.append("FB ").append(String.format("%08x", b)).append(' ').append(Float.floatToIntBits(v)).append(' ').append(Float.hashCode(v)).append('\n'); }
    for (int k = 0; k < 30; k++) { int b = R.nextInt(); float v = Float.intBitsToFloat(b); out.append("FB ").append(String.format("%08x", b)).append(' ').append(Float.floatToIntBits(v)).append(' ').append(Float.hashCode(v)).append('\n'); }
    System.out.print(out);
  }

  static String enc(String s) {
    if (!s.isEmpty() && s.chars().allMatch(c -> c > 0x20 && c < 0x7f)) return "=" + s;
    StringBuilder b = new StringBuilder("x");
    for (int i = 0; i < s.length(); i++) b.append(String.format("%04x", (int) s.charAt(i)));
    return b.toString();
  }

  static void doubles() {
    double[] special = {0.0, -0.0, Double.NaN, Double.longBitsToDouble(0x7ff0000000000123L), Double.longBitsToDouble(0xfff8000000000000L),
      Double.POSITIVE_INFINITY, Double.NEGATIVE_INFINITY, Double.MIN_VALUE, Double.MIN_NORMAL, Double.MAX_VALUE,
      Math.nextDown(Double.MIN_NORMAL), Math.PI, Math.E, 0.1, 0.2, 0.3, 1.0 / 3, 2.0 / 3, 100.0, 1e7, 1e-3, 9999999.0,
      9999999.5, 9999999.999999998, 10000000.000000002, 0.001, Math.nextDown(0.001), Math.nextUp(0.001), 0.00099999, 1e23, 2e23, 8.41e21, 5e-324, 1.0E22};
    for (double v : special) { d(v); d(-v); }
    for (int e = -325; e <= 310; e++) {
      double v = Double.parseDouble("1e" + e);
      d(v); d(Math.nextUp(v)); d(Math.nextDown(v));
      d(Double.parseDouble("9.999999999999999e" + e));
      d(Double.parseDouble("5e" + e));
      d(Double.parseDouble("2e" + e));
      d(Double.parseDouble("1.2345678901234567e" + e));
    }
    for (int e = -1074; e <= 1023; e++) d(Math.scalb(1.0, e));
    for (int i = 0; i <= 200; i++) { d(i); d(-i); d(i + 0.5); }
    long p = 1;
    for (int k = 0; k <= 18; k++) { d(p); d(p - 1); d(p + 1); p *= 10; }
    for (int t = 1; t <= 60; t++) d(Double.longBitsToDouble(t));
    for (int k = 0; k < 300; k++) d(Double.longBitsToDouble(R.nextLong() & 0x000fffffffffffffL));
    for (int k = 0; k < 2500; k++) d(Double.longBitsToDouble(R.nextLong()));
    for (int k = 0; k < 1200; k++) d(Double.parseDouble(niceDecimal(17, 25)));
    for (int i = 0; i < 300; i++) d(i / 1000.0);
    for (int k = 0; k < 300; k++) d((double) Float.intBitsToFloat(R.nextInt()));
  }

  static void floats() {
    float[] special = {0f, -0f, Float.NaN, Float.intBitsToFloat(0x7f800123), Float.POSITIVE_INFINITY, Float.NEGATIVE_INFINITY,
      Float.MIN_VALUE, Float.MIN_NORMAL, Float.MAX_VALUE, Math.nextDown(Float.MIN_NORMAL), (float) Math.PI, (float) Math.E,
      0.1f, 0.2f, 0.3f, 1f / 3, 100f, 1e7f, 1e-3f, 9999999f, 1.0E10f, 0.001f, Math.nextDown(0.001f), Math.nextUp(0.001f), 1.17549435E-38f, 3.4028235E38f};
    for (float v : special) { f(v); f(-v); }
    for (int e = -46; e <= 39; e++) {
      float v = Float.parseFloat("1e" + e);
      f(v); f(Math.nextUp(v)); f(Math.nextDown(v));
      f(Float.parseFloat("9.999999e" + e));
      f(Float.parseFloat("5e" + e));
      f(Float.parseFloat("1.2345678e" + e));
    }
    for (int e = -149; e <= 127; e++) f(Math.scalb(1.0f, e));
    for (int i = 0; i <= 200; i++) { f(i); f(-i); f(i + 0.5f); }
    for (int t = 1; t <= 64; t++) f(Float.intBitsToFloat(t));
    for (int k = 0; k < 200; k++) f(Float.intBitsToFloat(R.nextInt() & 0x007fffff));
    for (int k = 0; k < 2500; k++) f(Float.intBitsToFloat(R.nextInt()));
    for (int k = 0; k < 1000; k++) f(Float.parseFloat(niceDecimal(9, 12)));
    for (int i = 0; i < 300; i++) f(i / 1000.0f);
    for (int k = 0; k < 300; k++) f((float) Double.longBitsToDouble(R.nextLong()));
  }

  static String niceDecimal(int maxDigits, int maxExp) {
    int n = 1 + R.nextInt(maxDigits);
    StringBuilder b = new StringBuilder();
    b.append((char) ('1' + R.nextInt(9)));
    for (int i = 1; i < n; i++) b.append((char) ('0' + R.nextInt(10)));
    if (n > 1) b.insert(1, '.');
    b.append('e').append(R.nextInt(2 * maxExp + 1) - maxExp);
    return b.toString();
  }

  static void trimFloatInputs(String value, boolean isFloat) {
    if (value.length() <= 3) return;
    String exp = "";
    int eIdx = value.indexOf('E');
    if (eIdx != -1) { exp = value.substring(eIdx); value = value.substring(0, eIdx); }
    String temp = value;
    int dotIdx = value.indexOf('.');
    do {
      value = temp;
      temp = value.substring(0, value.length() - 1);
      if (!temp.isEmpty() && !"-".equals(temp)) PARSE.add(isFloat ? temp + exp : temp);
    } while (!temp.isEmpty() && !"-".equals(temp) && temp.length() > 1);
    if (dotIdx != -1 && dotIdx < value.length()) {
      String integer = value.substring(0, dotIdx);
      String decimal = value.substring(dotIdx + 1);
      try { PARSE.add((Long.parseLong(integer) + 1) + ".0" + exp); } catch (NumberFormatException e) { }
      long decimalVal = 1;
      for (int i = 0; i < decimal.length() - 1; i++) {
        decimalVal = (decimalVal - 1) * 10 + decimal.charAt(i) - '0' + 1;
        PARSE.add(integer + '.' + decimalVal + exp);
      }
    }
  }

  static void parseInputs() {
    String[] fixed = {"", " ", "0", "-0", "+0", "0.0", "-0.0", ".0", "0.", ".", "-", "+", "1", "1.", ".5", "-.5", "1e5", "1E5", "1e+5", "1e-5",
      "1e", "1e+", "e1", ".e1", "1.e1", " 1.5 ", "\t2\n", "\u00001.5\u0001", "1.5f", "1.5F", "1.5d", "1.5D", "1.5ff", "1.5fd", "f", "1f ", " 1f",
      "1e5f", "1.2.3", "1..2", "1e5e5", "--1", "+-1", "NaN", "-NaN", "+NaN", "NaNf", "nan", "Infinity", "-Infinity", "+Infinity", "Infinityf",
      "infinity", "Inf", "INFINITY", "1_000", "1,5", "１", "١", "0x", "0x1", "0x1p", "0x1p3", "0X1P3", "-0x1p3", "0x1.8p1", "0x.8p1",
      "0x1.p1", "0x.p1", "0X.8P-1f", "0x1p-1074", "0x1p-1075", "0x1.8p-1075", "0x0.0000000000001p-1022", "0x1.fffffffffffffp1023",
      "0x1.fffffffffffff8p1023", "0x1.fffffffffffff7ffffp1023", "0x1p1024", "0x1.0000000000000801p0", "0x1.00000000000008p0",
      "0x1.00000000000018p0", "0x1.000001p0", "0x1.0000018p0", "0x1.00000100000000000001p0", "0x123456789abcdef0123p-20", "0x0p0", "0x00000p99999999999",
      "0x1p+2147483648", "0x1p-2147483649", "0x1.ffffffp127", "0x1.fffffe8p127", "0x1p-149", "0x1p-150", "0x1.000002p-150", "0x1p1g",
      "1e400", "-1e400", "1e-400", "4.9e-324", "2.4703282292062327e-324", "2.4703282292062328e-324", "2.4703282292062327208828439643411e-324",
      "1.7976931348623157e308", "1.7976931348623158e308", "1.7976931348623159e308", "1e2147483647", "1e2147483648", "1e-2147483648",
      "1e-2147483649", "1e99999999999999999999", "0e99999999999", "0.0000000000000000000000000000000000000000000000001e49",
      "3.4028235e38", "3.4028236e38", "3.40282357e38", "1.4e-45", "7.0e-46", "7.1e-46", "1.401298464324817e-45", "0.000001", "123456789012345678901234567890",
      "9007199254740993", "9007199254740993.0000000000000000000000001", "1.00000005960464477539062499", "1.000000059604644775390625",
      "1.00000005960464477539062501", "16777217", "16777217.000000000000001"};
    PARSE.addAll(Arrays.asList(fixed));
    StringBuilder longDigits = new StringBuilder("0.");
    for (int i = 0; i < 1200; i++) longDigits.append(i % 7 == 0 ? '1' : '0');
    PARSE.add(longDigits.toString());
    PARSE.add(longDigits + "e1200");
    StringBuilder longInt = new StringBuilder();
    for (int i = 0; i < 900; i++) longInt.append((char) ('0' + (i * 7) % 10));
    PARSE.add(longInt.toString());
    PARSE.add(longInt + "e-1100");
    PARSE.add(new BigDecimal(Double.MIN_VALUE).divide(BigDecimal.valueOf(2)).toString());
    PARSE.add(new BigDecimal(Double.MIN_VALUE).divide(BigDecimal.valueOf(2)).add(new BigDecimal("1e-1100")).toPlainString());
    for (int k = 0; k < 300; k++) {
      float v = Float.intBitsToFloat(R.nextInt() & 0x7fffffff);
      if (Float.isNaN(v) || Float.isInfinite(v)) continue;
      trimFloatInputs(Float.toString(v), true);
    }
    for (int k = 0; k < 200; k++) {
      double v = Double.longBitsToDouble(R.nextLong() & 0x7fffffffffffffffL);
      if (Double.isNaN(v) || Double.isInfinite(v)) continue;
      trimFloatInputs(Double.toString(v), false);
    }
    for (int k = 0; k < 120; k++) trimFloatInputs(Float.toString(Float.parseFloat(niceDecimal(8, 10))), true);
    for (int k = 0; k < 120; k++) trimFloatInputs(Double.toString(Double.parseDouble(niceDecimal(16, 10))), false);
    for (int k = 0; k < 240; k++) {
      double v = Double.longBitsToDouble(R.nextLong() & 0x7fefffffffffffffL);
      BigDecimal lo = new BigDecimal(v), hi = new BigDecimal(Math.nextUp(v));
      BigDecimal mid = lo.add(hi).divide(BigDecimal.valueOf(2));
      PARSE.add(mid.toString());
      if (k % 4 == 0) PARSE.add(mid.add(mid.ulp()).toString());
      if (k % 4 == 1) PARSE.add(mid.subtract(mid.ulp()).toString());
    }
    for (int k = 0; k < 300; k++) {
      float v = Float.intBitsToFloat(R.nextInt() & 0x7f7fffff);
      BigDecimal lo = new BigDecimal(v), hi = new BigDecimal(Math.nextUp(v));
      BigDecimal mid = lo.add(hi).divide(BigDecimal.valueOf(2));
      PARSE.add(mid.toString());
      if (k % 3 == 0) PARSE.add(mid.add(mid.ulp()).toString());
    }
    String[] suffix = {"", "", "", "f", "D", "e3", "E-7", "e+12"};
    for (int k = 0; k < 600; k++) {
      StringBuilder b = new StringBuilder();
      if (R.nextInt(4) == 0) b.append(R.nextBoolean() ? '-' : '+');
      int lead = R.nextInt(4) == 0 ? R.nextInt(5) : 0;
      for (int i = 0; i < lead; i++) b.append('0');
      int n = R.nextInt(R.nextInt(5) == 0 ? 40 : 12);
      for (int i = 0; i < n; i++) b.append((char) ('0' + R.nextInt(10)));
      if (R.nextInt(3) > 0) {
        b.insert(Math.max(0, b.length() - R.nextInt(n + 1)), '.');
      }
      b.append(suffix[R.nextInt(suffix.length)]);
      if (R.nextInt(20) == 0) b.insert(R.nextInt(b.length() + 1), "xe.-".charAt(R.nextInt(4)));
      PARSE.add(b.toString());
    }
    for (int k = 0; k < 300; k++) PARSE.add(Double.toHexString(Double.longBitsToDouble(R.nextLong())));
    for (int k = 0; k < 200; k++) PARSE.add(Float.toHexString(Float.intBitsToFloat(R.nextInt())));
    for (int k = 0; k < 400; k++) {
      StringBuilder b = new StringBuilder(R.nextBoolean() ? "0x" : "-0X");
      int n = 1 + R.nextInt(R.nextInt(4) == 0 ? 24 : 15);
      for (int i = 0; i < n; i++) b.append("0123456789abcdefABCDEF".charAt(R.nextInt(22)));
      if (R.nextBoolean()) b.insert(2 + R.nextInt(n + 1) + (b.charAt(0) == '-' ? 1 : 0), '.');
      b.append(R.nextBoolean() ? 'p' : 'P').append(R.nextInt(2400) - 1200);
      if (R.nextInt(5) == 0) b.append(R.nextBoolean() ? 'f' : 'd');
      PARSE.add(b.toString());
    }
  }

  static void intInputs() {
    String[] fixed = {"0", "-0", "+0", "+5", "-5", "2147483647", "2147483648", "-2147483648", "-2147483649", "", "-", "+", " 1", "1 ",
      "١٢", "１２", "-३", "12a", "0x10", "00012", "9223372036854775807", "9223372036854775808",
      "-9223372036854775808", "-9223372036854775809", "1.0", "+-1", "--1", "999999999999999999999"};
    PARSE_INT.addAll(Arrays.asList(fixed));
    for (int k = 0; k < 60; k++) PARSE_INT.add(Integer.toString(R.nextInt()));
    for (int k = 0; k < 60; k++) PARSE_INT.add(Long.toString(R.nextLong()));
  }
}
