package test_data;

public class Instance {

    private int i = 3;

    public static int main() {
        Instance a = new Instance();
        return a.method();
    }

    private int method() {
        return i;
    }
}
