package test_data;

public class Fields {
    public boolean t_boolean = true;
    public int t_int = 0;
    public long t_long = 100L;
    public float t_float = 1.0f;
    public double t_double = 2.0d;

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
