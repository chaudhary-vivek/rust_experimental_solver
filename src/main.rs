fn add_numbers(x:i32, y:i32) -> i32{
    x + y
}

fn nice(x:i32){
    for _ in 0..x{
        println!("nice!");
    }
}

fn nice_plus(x:i32){
    for x in 1..x{
        println!("{}", x);
    }
}

fn main() {
    let result = add_numbers(340, 9);
    println!("Hello, world!");
    println!("The sum is {}", result);
    nice(3);
    nice_plus(3);
}
