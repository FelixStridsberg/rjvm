package java_tests;

import static vadeen.test.Assertion.*;

public class InvocationTests {

    public static void test_static_invocation() {
        assertEquals(static_invocation(1, 2, true), 3);
    }

    public static void test_instance_invocation() {
        InvocationTests i = new InvocationTests();
        assertEquals(i.instance_invocation(1, 2, false), 3);
    }

    public static int static_invocation(int a, int b, boolean add) {
        if (add) {
            return a + b;
        }

        return a - b;
    }

    public int instance_invocation(int a, int b, boolean sub) {
        if (sub) {
            return a - b;
        }

        return a + b;
    }
}
