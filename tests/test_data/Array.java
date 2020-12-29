package test_data;


class Array {

    public static int int_array() {
        int[] i = new int[10];
        i[0] = 1;
        i[1] = i[0] + 1;

        return i[0] + i[1];
    }

    public static byte byte_array() {
        byte[] b = new byte[10];
        b[1] = 0x0f;
        b[2] = (byte)0xf0;
        b[3] = (byte)(b[1] | b[2]);

        return b[3];
    }

    public static char char_array() {
        char[] b = new char[10];
        b[1] = 'a';
        b[2] = (char)(b[1] + 1);

        return b[2];
    }

    public static int array_length() {
        int[] f = new int[100];
        return f.length;
    }

    public static int array_length_npe() {
        int[] f = null;
        return f.length;
    }

    // TODO different types of arrays
}
