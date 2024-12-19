fn main() {
    
    // ints
    let _unsigned_int: u32 = "42".parse().expect("Not a number!");
    let _u8int_direct = 42u8;
    let _pretty = 1_000_000_245;

    // floats
    let _x = 2.0; // default f64
    let _y: f32 = 3.0; // I like this better tha putting the type after the number

    let int_1 = 5;
    let int_2 = 2;

    let inaccurate_division = int_1/int_2;
    println!("Wrong result is {}", inaccurate_division);

    let accurate_division: f32 = int_1 as f32 / int_2 as f32;
    println!("Right result is {}", accurate_division);

    // bool
    let _t = true;
    let _f: bool = false;

    //char. This one is interesting. Single quotes!
    let _c = 'z';
    let _z: char = 'Ö';
    let _emoji = '😻';

    // compound
    let tup: (u32, f64, bool) = (_unsigned_int, _x, _t);
    println!("Tuple is ({}, {}, {})", tup.0, tup.1, tup.2);

    let arr = [1, 2, 3, 4, 5];
    println!("Array is {:?}", arr); // This one is interesting too. Must do {:?}

    let _typed_arr: [u32; 3] = [1,2,3];

    let packed_array = ["value"; 5];
    println!("Array is {:?}", packed_array);
    println!("First value is {}", _typed_arr[0]);

}
