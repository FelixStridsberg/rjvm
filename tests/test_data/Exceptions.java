package test_data;

import vadeen.java.lang.Exception;

public class Exceptions {

    static int value = 0;

    public static int simple() {
        try {
            test();
        } catch (Exception e) {
            value++;
        }
        return value;
    }

    public static int with_finally() {
        try {
            test();
        } catch (Exception e) {
            value++;
        } finally {
            value++;
        }
        return value;
    }

    static void test() throws Exception {
        throw new Exception();
    }
}
