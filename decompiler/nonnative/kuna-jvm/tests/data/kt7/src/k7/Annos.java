package k7;

import java.util.List;
import kotlin.Metadata;
import kotlin.jvm.JvmStatic;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

@Metadata(k = 2)
public class Annos {
  @NotNull @TU String a;
  @Nullable @TU("v") String b;
  @JvmStatic static int c;
  List<@TU String> d;
  @TU("x") @TU2 int e;

  @NotNull
  @TU String m1() {
    return null;
  }

  @JvmStatic
  static void m2() {
  }

  @Nullable
  @TU2(n = 4) Object m3(@TU String p) {
    return null;
  }
}
