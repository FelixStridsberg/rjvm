package java_tests;

import static vadeen.test.Assertion.*;

public class InvocationTests {

    private static class ParentClass {
        protected static int static_parent_method() {
            return 2;
        }

        protected int instance_method_parent() {
            return 10;
        }
    }

    private static class ChildClass extends ParentClass {
        private static int static_method() {
            return 1;
        }

        private int instance_method(int a, int b, boolean sub) {
            if (sub) {
                return a - b;
            }

            return a + b;
        }
    }

    public static void test_static_invocation() {
        assertEquals(ChildClass.static_method(), 1);
    }

    public static void test_static_invocation_parent() {
        assertEquals(ChildClass.static_parent_method(), 2);
    }

    public static void test_instance_invocation() {
        ChildClass i = new ChildClass();
        assertEquals(i.instance_method(1, 2, false), 3);
    }

    public static void test_instance_invocation_parent() {
        ChildClass i = new ChildClass();
        assertEquals(i.instance_method_parent(), 10);
    }
}
