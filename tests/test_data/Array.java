package test_data;


class Array {

    public static int create_int_array() {
        int[] i = new int[10];
        i[0] = 1;
        i[1] = i[0] + 1;

        return i[0] + i[1];
    }
}
