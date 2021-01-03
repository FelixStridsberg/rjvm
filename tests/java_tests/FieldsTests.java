package java_tests;

import java_tests.helpers.OtherFields;

import static vadeen.test.Assertion.assertEquals;

public class FieldsTests {
    public static boolean static_boolean = false;
    public static int static_int = 100;
    public static long static_long = 200L;
    public static float static_float = 300.0f;
    public static double static_double = 400.0d;
    public static Object static_reference = new Object();
    public static Object static_null_reference = null;

    public static boolean u_static_boolean;
    public static int u_static_int;
    public static long u_static_long;
    public static float u_static_float;
    public static double u_static_double;
    public static Object u_static_reference;

    public boolean instance_boolean = true;
    public int instance_int = 10;
    public long instance_long = 101L;
    public float instance_float = 1.0f;
    public double instance_double = 2.0d;
    public Object instance_reference = new Object();
    public Object instance_null_reference = null;

    public boolean u_instance_boolean;
    public int u_instance_int;
    public long u_instance_long;
    public float u_instance_float;
    public double u_instance_double;
    public Object u_instance_reference;

    public static void test_static_fields() {
        assertEquals(static_boolean, false);
        assertEquals(static_int, 100);
        assertEquals(static_long, 200L);
        assertEquals(static_float, 300.0f);
        assertEquals(static_double, 400.0d);
        assertEquals(static_reference, static_reference);
        assertEquals(static_null_reference, null);
    }

    /* TODO
    public static void test_static_uninitialized_fields() {
        assertEquals(u_static_boolean, false);
        assertEquals(u_static_int, 0);
        assertEquals(u_static_long, 0);
        assertEquals(u_static_float, 0.0);
        assertEquals(u_static_double, 0.0);
        assertEquals(u_static_reference, null);
    }*/

    public static void test_static_fields_other_class() {
        assertEquals(OtherFields.static_boolean, false);
        assertEquals(OtherFields.static_int, 101);
        assertEquals(OtherFields.static_long, 202L);
        assertEquals(OtherFields.static_float, 303.0f);
        assertEquals(OtherFields.static_double, 404.0d);
        assertEquals(OtherFields.static_reference, OtherFields.static_reference);
        assertEquals(OtherFields.static_null_reference, null);
    }

    /* TODO
    public static void test_static_fields_other_class_super() {
        assertEquals(OtherFields.super_static_boolean, false);
        assertEquals(OtherFields.super_static_int, 101);
        assertEquals(OtherFields.super_static_long, 202L);
        assertEquals(OtherFields.super_static_float, 303.0f);
        assertEquals(OtherFields.super_static_double, 404.0d);
        assertEquals(OtherFields.super_static_reference, OtherFields.static_reference);
        assertEquals(OtherFields.super_static_null_reference, null);
    }*/

    public static void test_instance_fields() {
        FieldsTests i = new FieldsTests();
        assertEquals(i.instance_boolean, true);
        assertEquals(i.instance_int, 10);
        assertEquals(i.instance_long, 101L);
        assertEquals(i.instance_float, 1.0f);
        assertEquals(i.instance_double, 2.0d);
        assertEquals(i.instance_reference, i.instance_reference);
        assertEquals(i.instance_null_reference, null);
    }

    public static void test_instance_uninitialized_fields() {
        FieldsTests i = new FieldsTests();
        assertEquals(i.u_instance_boolean, false);
        assertEquals(i.u_instance_int, 0);
        assertEquals(i.u_instance_long, 0);
        assertEquals(i.u_instance_float, 0.0);
        assertEquals(i.u_instance_double, 0.0);
        assertEquals(i.u_instance_reference, null);
    }

    public static void test_instance_fields_other_class() {
        OtherFields i = new OtherFields();
        assertEquals(i.instance_boolean, true);
        assertEquals(i.instance_int, 1000);
        assertEquals(i.instance_long, 2000L);
        assertEquals(i.instance_float, 3000.0f);
        assertEquals(i.instance_double, 4000.0d);
        assertEquals(i.instance_reference, i.instance_reference);
        assertEquals(i.instance_null_reference, null);
    }

    public static void test_instance_fields_other_class_super() {
        OtherFields i = new OtherFields();
        assertEquals(i.super_instance_boolean, true);
        assertEquals(i.super_instance_int, 1000);
        assertEquals(i.super_instance_long, 2000L);
        assertEquals(i.super_instance_float, 3000.0f);
        assertEquals(i.super_instance_double, 4000.0d);
        assertEquals(i.super_instance_reference, i.super_instance_reference);
        assertEquals(i.super_instance_null_reference, null);
    }
}
