// fn main2() {
//     let x: i32 = 1; // i32 is a 32-bit signed integer type (i means signed)
//     println!("x is {}", x);
// }

/*
    // Integer types
    let a: i32 = -10;         // Signed 32-bit integer
    let b: u64 = 100;         // Unsigned 64-bit integer (can't be negative)

    // Floating-point types
    let c: f32 = 3.14;        // 32-bit float
    let d: f64 = -2.71828;    // 64-bit float (more precise)

    // Boolean type
    let is_active: bool = true; // Can be `true` or `false`

    // Character type
    let initial: char = 'T';    // A single Unicode character
*/



// fn main3(){
//     let male: bool = true;
//     let age: i8 = 22;

//     if age > 18 && male == true {
//         println!("You are eligible for the job.");

//     }

// }



fn get_first_word(sentence: String) -> String{
    let mut ans = String::new();
    for c in sentence.chars(){
        if c == ' '{
            break;  
        }
        ans.push(c);
    }
    return ans;
}


fn main(){
    let sentence = String::from("Hello World");
    let first_word = get_first_word(sentence);
    println!("The first word is: {}", first_word);
}