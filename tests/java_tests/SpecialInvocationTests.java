package java_tests;

import static vadeen.test.Assertion.*;

public class SpecialInvocationTests {
    interface Interface {
        int interfaceMethod();
    }

    static class ParentClass implements Interface {

        @Override
        public int interfaceMethod() {
            return 100;
        }

        public int superMethod() {
            return 102;
        }

        public int superOverridden() {
            return interfaceMethod();
        }
    }

    static class SubClass extends ParentClass {
        @Override
        public int interfaceMethod() {
            return 10;
        }

        public int superMethod() {
            assertEquals(1, 2); // Should never be called
            return 0;
        }

        public int callSuper() {
            return super.superMethod();
        }
    }

    public static void test_super_invocation() {
        SubClass s = new SubClass();
        assertEquals(s.callSuper(), 102);
    }

    public static void test_super_overridden() {
        SubClass s = new SubClass();
        assertEquals(s.superOverridden(), 10);
    }
}
