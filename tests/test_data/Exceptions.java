package test_data;

import vadeen.java.lang.Exception;

public class Exceptions {

    static int value = 0;

    public static int main() {
        try {
            test();
        } catch (Exception e) {
            value++;
        }
        return value;
    }

    static void test() throws Exception {
        throw new Exception();
    }
}
