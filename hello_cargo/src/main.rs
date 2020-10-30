fn main() {
    println!("Hello world, learning rust here!\n");

    let thing = ("hi", 2, 'c', 2.2);

    let (w, x, y, z) = thing;

    println!("thing.0 = {}, thing.1 = {}, thing.2 = {}, thing.3 = {}", thing.0, thing.1, thing.2, thing.3);
    println!("w = {}, x = {}, y = {}, z = {}", w, x, y, z);
    println!("other(x) = {}", other(x));
    loop_test();
    for_in_array();
    ownership();
    let (s, two) = returns_tup();
    println!("rts = {}, rttwo = {}", s, two);
    string_slice();
    struct_test();
    enum_test();
    test_match_test();
}

fn for_in_array() {
    let thing = ["hi", "hello", "hey"];
    for i in thing.iter() {
        println!("i in thing = {}", i);
    }
}

fn ownership() {
    let thing = String::from("test");
    let other = thing;
    println!("{}", other);
}

fn string_slice() {
    let string = String::from("that string you see");
    println!("string[5..11] = \"{}\", string[11..] = \"{}\", string[..5] = \"{}\"", &string[5..11], &string[11..], &string[..5]);
}

fn returns_tup() -> (String, i32) {
    (String::from("thing"), 2)
}

fn loop_test() {
    let mut i = 0;
    let test = loop {
        if i >= 10 {
            break i;
        }
        i += 1;
    };
    println!("loop = {}", test)
}

fn other(number: i32) -> String {
    number.to_string()
}

fn struct_test() {
    let what = String::from("test");

    let mut test = Learn {
        time: 10,
        what,
        new: true
    };
    test.time = 11;
    let test2 =  Learn {
        new: false,
        ..test
    };
    let test3 = Learn {
        what: "t".to_string(),
        time: 4,
        new: false
    };
    println!("test2 is {:?}, test2.repeat() = {}, test2.longer_than(test3) = {}", test2, test2.repeat(), test2.longer_than(&test3));
}

impl Learn {
    fn repeat(&self) -> String {
        let mut result = String::new();
        for _i in 0..self.time {
            result.push_str(self.what.as_str());
        };
        result
    }
    
    fn longer_than(&self, other: &Learn) -> bool {
        self.time > other.time
    }
}

#[derive(Debug)]
struct Learn {
    what: String,
    time: i64,
    new: bool
}

enum Things {
    Number(i64),
    Name(String)
}

fn enum_test() {
    let thing1 = Things::Number(1);
    let thing2 = Things::Name(String::from("john"));
    let mut val1: i64 = 0;
    if let Things::Number(n) = thing1 {
        val1 = n;
    }
    let mut val2: String = "".to_owned();
    if let Things::Name(s) = thing2 {
        val2 = s;
    }
    println!("val1 = {}, val2 = {}", val1, val2);
}

fn test_match_test() {
    println!("match_test(Unit::Bit) = {}, match_test(Unit::Nibble) = {}, match_test(Unit::Byte) = {}", match_test(Unit::Bit), match_test(Unit::Nibble), match_test(Unit::Byte))
}

fn match_test(unit: Unit) -> u8 {
    match unit {
        Unit::Bit => 1,
        Unit::Nibble => 4,
        Unit::Byte => 8
    }
}

enum Unit {
    Bit,
    Nibble,
    Byte,
}