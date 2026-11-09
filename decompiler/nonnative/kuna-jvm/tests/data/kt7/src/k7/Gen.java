package k7;

import java.io.Serializable;
import java.util.List;

public class Gen<T extends Number & Comparable<T>, U, W extends List<? extends U>> {
  <V extends Serializable, X> void g() {
  }
}
