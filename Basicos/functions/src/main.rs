fn main() {
    println!("Hello, world!");

    another_function();

    ao_quadrado(2);

    pow_of(2, 3);

    println!("Five is : {}", five())
}

fn another_function(){
    println!("This is another Function!");
}

fn ao_quadrado(x: i32){
    println!("O numero ao quadrado eh: {}", x*x);
}

fn pow_of(x: i32, y: u32){
    println!("{x} elevado a {y} eh: {}", x.pow(y));
}

fn five() -> i32{
    5
}
