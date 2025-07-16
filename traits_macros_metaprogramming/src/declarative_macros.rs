// implementing own vec like macro
macro_rules!  mera_vector{
    ($($x: expr),*) => {{
        let mut v = ::std::vec::Vec::new();
        $(
            v.push($x);
        )*
        v
    }};

    ($element : expr ; $count: expr) => {{
        let mut v = ::std::vec::Vec::new();
        // evaluate only once
        let count = $count;
        // store it so that it doesnt gets moved
        let tmp = $element;
        for _ in 0..count {
            v.push(tmp.clone())
        }
        v
    }}
}

macro_rules! logln {
    // $($arg:tt) is a pattern that matches any number of arguments
    // format_args! is a macro that formats the arguments
    // file!() is a macro that returns the file name
    // line!() is a macro that returns the line number
    ($($arg:tt)*) => {
        println!("[{}:{}] {}", file!(), line!(), format_args!($($arg)*));
    };
}

macro_rules! make_fields {
    ( $( $field:ident ),+ ) => {
        struct MyStruct {
            $( $field: i32 ),+
        }

        impl MyStruct {
            fn new( $( $field: i32 ),+ ) -> Self {
                Self { $( $field ),+ }
            }
        }
    };
}

macro_rules! hashmap {
    ($($key:expr => $val:expr);*) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(map.insert($key, $val);)*
            map
        }
    };
}

pub fn own_vector() {
    // all brackets are supported
    let my_vec = mera_vector!(1, 2, 3);
    let n = mera_vector![4;6];
    println!("{:?}", my_vec);
    println!("{:?}", n);
}

pub fn own_logger() {
    //write a logln! macro that prints file + line: logln!("hi") → [src/declarative_macros.rs:33] hi.
    logln!("hi");
}

pub fn make_fields() {
    //write a make_fields!(A, B, C) macro that makes struct fields + a new() fn.
    make_fields!(x, y, z);

    let obj = MyStruct::new(10, 20, 30);
    println!("x={}, y={}, z={}", obj.x, obj.y, obj.z);
    //println!("{:?}", s);
}

pub fn own_hashmap() {
    // write a hashmap!{ key => val, ... } macro that returns a HashMap.
    let map = hashmap! { "a" => 1; "b" => 2;"c" => 3 };
    println!("{:?}", map);
}
