package vadeen.test;

import java.lang.Object;

public class Assertion {

    public static native void assertEquals(boolean left, boolean right);

    public static native void assertEquals(short left, short right);

    public static native void assertEquals(int left, int right);

    public static native void assertEquals(long left, long right);

    public static native void assertEquals(float left, float right);

    public static native void assertEquals(double left, double right);

    public static native void assertEquals(Object left, Object right);

    public static native void fail();

}
