package java_tests;

import static vadeen.test.Assertion.*;

public class VirtualInvocationTests {

    interface Interface {
        int interfaceMethod();
    }

    static class ParentClass implements Interface {

        @Override
        public int interfaceMethod() {
            return 100;
        }

        public int parentMethod() {
            return 101;
        }
    }

    static class SubClass extends ParentClass {
        public int subMethod() {
            return 10;
        }
    }

    public static void test_direct_invocation() {
        ParentClass p = new ParentClass();
        assertEquals(p.parentMethod(), 101);
    }

    public static void test_parent_invocation() {
        SubClass s = new SubClass();
        assertEquals(s.parentMethod(), 101);
    }

    public static void test_interface_invocation() {
        ParentClass p = new ParentClass();
        assertEquals(p.interfaceMethod(), 100);
    }
}
