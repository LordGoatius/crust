struct structype {
    my_int i32;
    my_uint u32;
    my_fl f64;
};

enum int_option {
    some i32,
    none,
};

typedef int_flt (u32, f64);

static int_flt my_tuple = (15, 34);
(i32, u32) tuple;
tuple = (-12, 14);

int_option thing() {
    u32 my_var;
    my_var = 14;
    int_option my_opt = none;

    char* thing = "thing";

    if let int_option.some value = my_opt {
        return enum int_option.none;
    } else {
        return enum int_option.some my_var;
    }
}

structype thing_1() {
    struct structype thing = {
        my_int = -255;
        my_uint = 13;
        my_fl = 12.4;
    };
    thing.my_int = 12;
    (&thing)->my_int = 13;
    return struct structype {
        my_int = -255;
        my_uint = 13;
        my_fl = 12.4;
    };

}

void thing_2() {
    u32[4] arr = [1, 2, 3, 4];
    u32* ptr_u32 = (u32*) arr;
}

void match() {
    match enum int_option.none {
        int_option.none: {
            return;
        },
        int_option.some 4: {
            return;
        },
        int_option.some _: {
            return;
        },
    }
    return;
}
