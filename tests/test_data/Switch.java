package test_data;

class Switch {
    public static int simple() {
        int i = 1;

        switch (i) {
            case 1:
                return 2;
            case 200:
                return -1;
        }

        return 0;
    }

    public static int no_match() {
        int i = 1;

        switch (i) {
            case 2:
                return 2;
            case 200:
                return -1;
        }

        return 0;
    }

    public static int default_case() {
        int i = 5;
        i++;

        switch (i) {
            case 2:
                return 2;
            case 200:
                return -1;
            default:
                return 1;
        }
    }

    public static int table_switch() {
        int i = 0;

        switch (i) {
            case 1: return 1;
            case 2: return 2;
            case 3: return 3;
            default: return 4;
        }
    }
}
