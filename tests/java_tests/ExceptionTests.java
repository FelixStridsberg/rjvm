package java_tests;

import java.lang.Exception;

import static vadeen.test.Assertion.*;


public class ExceptionTests {

    public static void test_try_catch_no_exception() {
        int i = 0;
        try {
            if (false) {
                throw new Exception();
            }
            i = 1;
        } catch (Exception e) {
            assertEquals(1, 2); // Always fail if we reach this
        }

        assertEquals(i, 1);
    }

    public static void test_try_catch() {
        int i = 0;
        try {
            i++;
            throw new Exception();
        } catch (Exception e) {
            i++;
        }

        assertEquals(i, 2);
    }

    public static void test_finally() {
        int i = 0;
        try {
            i++;
        } finally {
            i++;
        }
        assertEquals(i, 2);
    }

    public static void test_try_catch_finally() {
        int i = 0;
        try {
            i++;
            throw new Exception();
        } catch (Exception e) {
            i++;
        } finally {
            i++;
        }
        assertEquals(i, 3);
    }

    public static void test_try_catch_finally_no_exception() {
        int i = 0;
        try {
            i++;
            if (false) {
                throw new Exception();
            }
        } catch (Exception e) {
            i++;
        } finally {
            i++;
        }
        assertEquals(i, 2);
    }

}
