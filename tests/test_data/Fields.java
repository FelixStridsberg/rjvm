package test_data;

public class Fields {
    public static boolean s_boolean = false;
    public static int s_int = 100;
    public static long s_long = 200L;
    public static float s_float = 300.0f;
    public static double s_double = 400.0d;

    public boolean t_boolean = true;
    public int t_int = 0;
    public long t_long = 100L;
    public float t_float = 1.0f;
    public double t_double = 2.0d;

    public static int other() {
        return Other.static_field;
    }

    public static boolean s_boolean() {
        return Fields.s_boolean;
    }

    public static int s_int() {
        return Fields.s_int;
    }

    public static long s_long() {
        return Fields.s_long;
    }

    public static float s_float() {
        return Fields.s_float;
    }

    public static double s_double() {
        return Fields.s_double;
    }

    public static boolean t_boolean() {
        Fields a = new Fields();
        return a.t_boolean;
    }

    public static int t_int() {
        Fields a = new Fields();
        return a.t_int;
    }

    public static long t_long() {
        Fields a = new Fields();
        return a.t_long;
    }

    public static float t_float() {
        Fields a = new Fields();
        return a.t_float;
    }

    public static double t_double() {
        Fields a = new Fields();
        return a.t_double;
    }
}
