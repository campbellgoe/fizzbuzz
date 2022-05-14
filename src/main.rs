fn main() {
    // fizzbuzz_a();
    // fizzbuzz_b();
    _fizzbuzz_c();
}

// A way to generate fizzbuzz by concatenating Fizz and Buzz based on divisibility by 3 and 5
fn _fizzbuzz_a() {
    for i in 0..100 {
        let mut out = String::from("");
        let div3 = i % 3 == 0;
        let div5 = i % 5 == 0;
        if div3 {
            out.push_str("Fizz");
        }
        if div5 {
            out.push_str("Buzz");
        }
        if out.len() > 0 {
            println!("{}", out);
        } else {
            println!("{}", i);
        }
    }
}

// Another way to generate fizzbuzz by explicitly handling each kind of output
fn _fizzbuzz_b() {
    for i in 0..100 {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 3 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

// Another way to generate FizzBuzz using match
fn _fizzbuzz_c() {

    for i in 0..100 {
        let num_string;
        let out = match i {
            i if i % 15 == 0 => { "FizzBuzz" }
            i if i % 3 == 0 => { "Fizz" }
            i if i % 5 == 0 => { "Buzz" }
            _ => {
                num_string = i.to_string();
                &num_string
            }
        };
        println!("{}", out);
    };
}