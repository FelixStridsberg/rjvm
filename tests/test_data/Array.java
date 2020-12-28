package test_data;


class Array {

    public static int create_int_array() {
        int[] i = new int[10];
        i[0] = 1;
        i[1] = i[0] + 1;

        return i[0] + i[1];
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
