package test_data;

import java.io.Serializable;

public class Invocation {

    public static int static_no_args() {
        int i = 0;
        i++;
        return i;
    }

    public static int static_int_args(int a, int b) {
        return a + b;
    }

    public static long static_long_args(long a, long b) {
        return a + b;
    }

    public static int static_nested(int a, int b) {
        return static_int_args(a, b);
    }

    public static int instance_invocation_no_args() {
        Invocation a = new Invocation();
        return a.no_args();
    }

    public static int instance_invocation_int_arg() {
        Invocation a = new Invocation();
        return a.int_args(1);
    }

    public static int instance_invocation_reference_arg() {
        Invocation i1 = new Invocation();
        Invocation i2 = new Invocation();
        return i1.reference_arg(i2);
    }

    public static int instance_invocation_null_reference_arg() {
        Invocation i1 = new Invocation();
        return i1.reference_arg(null);
    }

    public static int instance_invocation_different_args() {
        Invocation i1 = new Invocation();
        return i1.different_args(1, 2, 3.0, 4.0f);
    }

    private int no_args() {
        return 3;
    }

    private int int_args(int n) {
        return 3 + n;
    }

    private int reference_arg(Invocation obj) {
        if (obj != null) {
            return obj.no_args();
        }

        return 1;
    }

    private int different_args(int i, long l, double d, float f) {
        return i + (int)l + (int)d + (int)f;
    }
}
