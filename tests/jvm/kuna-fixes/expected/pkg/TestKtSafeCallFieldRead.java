package pkg;

import kotlin.Metadata;
import kotlin.jvm.JvmField;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

@Metadata(
   mv = {2, 0, 0},
   k = 1,
   xi = 48,
   d1 = "\u0000$\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\n\u0002\u0018\u0002\n\u0002\b\u0003\n\u0002\u0010\b\n\u0002\b\u0005\n\u0002\u0010\u0002\n\u0002\b\u0004\u0018\u00002\u00020\u0001:\u0001\u0013B\u0007¢\u0006\u0004\b\u0002\u0010\u0003J\u0006\u0010\u000e\u001a\u00020\u000fJ\u0006\u0010\u0010\u001a\u00020\tJ\u0006\u0010\u0011\u001a\u00020\u0005J\u0006\u0010\u0012\u001a\u00020\tR\u0014\u0010\u0004\u001a\u0004\u0018\u00010\u00058\u0006@\u0006X\u0087\u000e¢\u0006\u0002\n\u0000R\u0014\u0010\u0006\u001a\u0004\u0018\u00010\u00058\u0006@\u0006X\u0087\u000e¢\u0006\u0002\n\u0000R\u0010\u0010\u0007\u001a\u0004\u0018\u00010\u0005X\u0082\u0004¢\u0006\u0002\n\u0000R\u001a\u0010\b\u001a\u00020\tX\u0086\u000e¢\u0006\u000e\n\u0000\u001a\u0004\b\n\u0010\u000b\"\u0004\b\f\u0010\r",
   d2 = {
         "Lpkg/TestKtSafeCallFieldRead;",
         "",
         "<init>",
         "()V",
         "stream",
         "Lpkg/TestKtSafeCallFieldRead$Node;",
         "plain",
         "fixed",
         "closed",
         "",
         "getClosed",
         "()I",
         "setClosed",
         "(I)V",
         "cancel",
         "",
         "valueOr",
         "orDefault",
         "finalField",
         "Node"
   }
)
public final class TestKtSafeCallFieldRead {
   @JvmField
   @Nullable
   public volatile TestKtSafeCallFieldRead.Node stream;
   @JvmField
   @Nullable
   public TestKtSafeCallFieldRead.Node plain;
   @Nullable
   private final TestKtSafeCallFieldRead.Node fixed = new TestKtSafeCallFieldRead.Node(3);
   private int closed;

   public final int getClosed() {
      return this.closed;
   }

   public final void setClosed(int var1) {
      this.closed = var1/* $VF was: <set-?> */;
   }

   public final void cancel() {
      TestKtSafeCallFieldRead.Node var10001 = this.stream;
      this.closed = var10001 != null ? var10001.close() : -1;
   }

   public final int valueOr() {
      TestKtSafeCallFieldRead.Node var10000 = this.plain;
      return var10000 != null ? var10000.value : -1;
   }

   @NotNull
   public final TestKtSafeCallFieldRead.Node orDefault() {
      TestKtSafeCallFieldRead.Node var10000 = this.stream;
      if (var10000 == null) {
         var10000 = new TestKtSafeCallFieldRead.Node(0);
      }

      return var10000;
   }

   public final int finalField() {
      return this.fixed != null ? this.fixed.value : -1;
   }

   @Metadata(
      mv = {2, 0, 0},
      k = 1,
      xi = 48,
      d1 = "\u0000\u0012\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0010\b\n\u0002\b\u0003\u0018\u00002\u00020\u0001B\u000f\u0012\u0006\u0010\u0002\u001a\u00020\u0003¢\u0006\u0004\b\u0004\u0010\u0005J\u0006\u0010\u0006\u001a\u00020\u0003R\u0010\u0010\u0002\u001a\u00020\u00038\u0006X\u0087\u0004¢\u0006\u0002\n\u0000",
      d2 = {"Lpkg/TestKtSafeCallFieldRead$Node;", "", "value", "", "<init>", "(I)V", "close"}
   )
   public static final class Node {
      @JvmField
      public final int value;

      public Node(int value) {
         this.value = value;
      }

      public final int close() {
         return this.value;
      }
   }
}
