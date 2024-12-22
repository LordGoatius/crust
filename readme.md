Syntax:
C but with rust enums and pattern matching. Example:
```c,crust
// Hybrid C/Rust style structs
struct structype {
    my_int i32;
    my_uint u32;
    my_fl f64;
};

// Enums are tagged unions, and like in rust can be represented using both
// C style or tagged union style
enum int_option {
    some i32,
    none,
};

// Union type can be defined once a ffi is necessary for C lol.
// For now, built in it is.

// Tuple
typedef int_flt (u32, f64);

// Static variables, tuple
static int_flt my_tuple = (15, 34);
(i32, u32) tuple;
tuple = (-12, 14);

// All formats can assume C style layout without Rust style
// struct layout optimization. 

// functions are more C style than rust style.
// This is both for simplicity and because the constructs that make
// the arrow better for rust (dyn, generics, etc) do not exist in 
// this language. Is that better? No, but that's not the point.
fn thing() -> int_option {
    u32 my_var;
    my_var = 14;
    int_option my_opt = none;

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

// We also get a match expression
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
    } // match needs no semicolon, just like `if` and `for`
    return;
}

```
