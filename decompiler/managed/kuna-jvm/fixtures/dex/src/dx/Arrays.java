package dx;

public class Arrays {
    static int sum(int... v) {
        int s = 0;
        for (int x : v) {
            s += x;
        }
        return s;
    }

    static String join(Object... parts) {
        StringBuilder sb = new StringBuilder();
        for (Object p : parts) {
            sb.append(p).append('|');
        }
        return sb.toString();
    }

    public static void main(String[] args) {
        boolean[] z = {true, false, true};
        byte[] b = {1, -2, 127, -128};
        char[] c = {'x', 'y', 'ሴ'};
        short[] s = {1000, -1000, 32767};
        int[] i = {1, 2, 3, 100000, -5};
        long[] l = {1L, -1L, Long.MAX_VALUE};
        float[] f = {1.5f, -0f, Float.NaN};
        double[] d = {2.5, 1e100, -3};
        String[] str = {"a", null, "c"};
        int[][] grid = new int[3][4];
        for (int r = 0; r < 3; r++) {
            for (int k = 0; k < 4; k++) {
                grid[r][k] = r * 10 + k;
            }
        }
        long[][][] cube = new long[2][2][];
        cube[1][0] = new long[] {7L, 8L};
        z[1] = !z[0];
        b[0] += b[1];
        c[0]++;
        s[2] -= 7;
        i[4] *= i[3];
        l[2] += l[1];
        f[1] -= 2f;
        d[0] /= 2;
        str[1] = str[0] + str[2];
        System.out.println(z[0] + " " + z[1] + " " + b[0] + " " + c[0] + " " + s[2] + " " + i[4] + " " + l[2] + " " + f[1]
                + " " + d[0] + " " + str[1] + " " + grid[2][3] + " " + cube[1][0][1] + " " + (cube[0][1] == null));
        System.out.println(z.length + b.length + c.length + s.length + i.length + l.length + f.length + d.length + str.length);
        System.out.println(sum() + " " + sum(1) + " " + sum(1, 2, 3, 4, 5, 6, 7) + " " + join("p", 1, 2L, 'c', null));
        Object[] objs = new Integer[2];
        try {
            objs[0] = "no";
        } catch (ArrayStoreException e) {
            System.out.println("ase");
        }
        try {
            System.out.println(i[9]);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("aioobe");
        }
        int[] none = null;
        try {
            System.out.println(none.length);
        } catch (NullPointerException e) {
            System.out.println("npe");
        }
        int[] filled = new int[] {sum(1, 2), i[0], 9};
        String[] names = new String[] {"q", join(), String.valueOf(filled[0])};
        System.out.println(filled[0] + filled[1] + filled[2] + names[2] + names.length);
        char[] chars = "hello".toCharArray();
        java.util.Arrays.sort(chars);
        System.out.println(new String(chars) + java.util.Arrays.toString(l) + java.util.Arrays.toString(d));
    }
}
