package java_tests;

import static vadeen.test.Assertion.*;

public class ControlFlowTests {

    public static void test_switch_break() {
        int i = 1;

        switch (i) {
            case 1:
                i = 2;
                break;
            case 200:
                i = 3;
        }

        assertEquals(i, 2);
    }

    public static void test_switch_fall_through() {
        int i = 1;

        switch (i) {
            case 1:
                i = 2;
            case 200:
                i = 3;
        }

        assertEquals(i, 3);
    }

    public static void test_switch_default() {
        int i = 1;

        switch (i) {
            case 100:
                i = 2;
                break;
            case 200:
                i = 3;
                break;
            default:
                i = 4;
        }

        assertEquals(i, 4);
    }

    public static void test_switch_no_match() {
        int i = 1;

        switch (i) {
            case 100:
                i = 2;
                break;
            case 200:
                i = 3;
        }

        assertEquals(i, 1);
    }

    public static void test_table_switch() {
        int i = 2;

        switch (i) {
            case 1:
                i = 11;
                break;
            case 2:
                i = 22;
                break;
            case 3:
                i = 33;
                break;
            default:
                i = 44;
        }

        assertEquals(i, 22);
    }
}
