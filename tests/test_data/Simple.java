package test_data;

import java.io.Serializable;

public class Simple {

    public static int no_args() {
        int i = 0;
        i++;
        return i;
    }

    public static int add(int a, int b) {
        return a + b;
    }

    public static long add_long(long a, long b) {
        return a + b;
    }
}
