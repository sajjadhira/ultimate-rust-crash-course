pub fn forloop() {
    for i in 1..101{
        println!("The number is {}", i);
    }
}

pub fn ifelse(n : i32) {
    let mesg = if n == 7 {
        "seven is the number!"
    } else if n == 13 {
        "thirteen is the number!"
    } else {
        "This is not the number."
    };

    println!("{}", mesg);

}