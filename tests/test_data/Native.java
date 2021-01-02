package test_data;

public class Native {

    public static native int native_method();

    public static int call_native() {
        return native_method();
    }
}
