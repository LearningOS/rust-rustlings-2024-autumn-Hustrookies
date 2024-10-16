// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    // "blue" 是一个字符串字面量，类型是 &str，因此使用 string_slice 函数。

    string("red".to_string());
    // "red".to_string() 将字符串字面量转换为 String 类型，因此使用 string 函数。

    string(String::from("hi"));
    // String::from("hi") 创建一个 String 类型的字符串，因此使用 string 函数。

    string("rust is fun!".to_owned());
    // "rust is fun!".to_owned() 将字符串字面量转换为 String 类型，因此使用 string 函数。

    string_slice("nice weather".into());
    // "nice weather".into() 将字符串字面量转换为 String 类型，但 string_slice 函数接受 &str 类型。
    // 由于 &String 可以自动转换为 &str，因此可以使用 string_slice 函数。

    string(format!("Interpolation {}", "Station"));
    // format! 宏返回一个 String 类型的字符串，因此使用 string 函数。

    string_slice(&String::from("abc")[0..1]);
    // String::from("abc") 创建一个 String 类型的字符串，[0..1] 是一个字符串切片，类型是 &str，因此使用 string_slice 函数。

    string_slice("  hello there ".trim());
    // "  hello there ".trim() 返回一个 &str 类型的字符串切片，因此使用 string_slice 函数。

    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    // "Happy Monday!".to_string() 将字符串字面量转换为 String 类型，replace 方法返回一个新的 String 类型的字符串，因此使用 string 函数。

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
    // to_lowercase 方法返回一个新的 String 类型的字符串，因此使用 string 函数。
}
