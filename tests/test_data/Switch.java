package test_data;

class Switch {
    public static int simple() {
        int i = 1;

        switch (i) {
            case 1:
                return 2;
        }

        return 0;
    }

    public static int no_match() {
        int i = 1;

        switch (i) {
            case 2:
                return 2;
        }

        return 0;
    }

    public static int default_case() {
        int i = 5;
        i++;

        switch (i) {
            case 2:
                return 2;
            default:
                return 1;
        }
    }
}
