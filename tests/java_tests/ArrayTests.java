package java_tests;

import java.lang.Object;

import static vadeen.test.Assertion.*;

public class ArrayTests {

    public static void test_short_array() {
        short[] b = new short[10];
        b[1] = 2;
        b[2] = (short)(b[1] + 1);

        assertEquals(b[2], 3);
    }

    public static void test_int_array() {
        int[] i = new int[10];
        i[0] = 2;
        i[1] = i[0] + 1;

        assertEquals(i[1], 3);
    }

    public static void test_long_array() {
        long[] i = new long[10];
        i[0] = 2;
        i[1] = i[0] + 1;

        assertEquals(i[1], 3);
    }

    public static void test_byte_array() {
        byte[] b = new byte[10];
        b[0] = 0x02;
        b[1] = (byte)(b[0] + 1);

        assertEquals(b[1], 3);
    }

    public static void test_char_array() {
        char[] b = new char[10];
        b[1] = 'a';
        b[2] = (char)(b[1] + 1);

        assertEquals(b[2], 'b');
    }

    public static void test_float_array() {
        float[] b = new float[10];
        b[1] = 1.4f;
        b[2] = b[1] + 2.22f;

        assertEquals(b[2], 3.62f);
    }

    public static void test_double_array() {
        double[] b = new double[10];
        b[1] = 100.4;
        b[2] = b[1] + 1.22;

        assertEquals(b[2], 101.62);
    }

    public static void test_reference_array() {
        Object[] o = new Object[10];
        o[0] = new Object();
        o[1] = o[0];

        assertEquals(o[0], o[1]);
    }

    public static void test_array_length() {
        int[] f = new int[100];

        assertEquals(f.length, 100);
    }

}
