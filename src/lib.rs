use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    bare_bones();
    using_a_macro();
    //using_web_sys();
}

// First up let's take a look of binding `console.log` manually, without the
// help of `web_sys`. Here we're writing the `#[wasm_bindgen]` annotations
// manually ourselves, and the correctness of our program relies on the
// correctness of these annotations!

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen(js_namespace = console)]
pub fn bare_bones() {
    log("Hello from Rust!!!");
    log_u32(42);
    log_many("Logging", "many values!");
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn using_a_macro() {
    console_log!("Hello {}!", "world");
    console_log!("Let's print some numbers...");
    console_log!("1 + 3 = {}", 1 + 3);
}


#[wasm_bindgen(js_namespace = console)]
pub fn primes(mut number:i64) -> String {
    //use web-sys::console;
    // A counter variable
    let mut n = 2;
    let mut output = Vec::new();
    let mut pstring: String = "".to_owned();
    let sepstr: String = ",".to_owned();
    let limit: i64 = (((number as f64) * 0.5) as i64) + 1;
    println!("{}", limit as u8);
    while n < limit {
        if number % n == 0 {
            output.push(n);
            //println!("{}", n as u8);
            //primes(number/n);
            number = (number/n) as i64;
        }
        // Increment counter
        else {    
        n += 1;
         }
        }
    
    if output.len() == 0 {
        //println!("It's prime");
        //console_log!("It's prime");
        pstring.push_str("It's prime");
        //console::log_1(&"Hello using web-sys".into());
    }

    else {
        for x in output.iter() {
            //println!("{}", x);
            console_log!("{}", x);
            pstring.push_str(&(x.to_string() + &sepstr));
            //console::log_1(&"Hello using web-sys".into());
        //println!("{}", String::from_utf8(facs).unwrap());
            }
        //return ouput.to_string();
        
    }
    return pstring;
}


#[wasm_bindgen]
pub fn main() {
//facs = Vec::new();
//primes(24507708);
}