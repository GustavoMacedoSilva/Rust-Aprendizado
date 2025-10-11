fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    }
    else{
        println!("condition was false");
    }

    // -------------------------------------------------------------------
    let condition = true;
    let number2 = if condition {5} else {6};

    println!("The value of number is : {number2}");
    //---------------------------------------------------------------------
    loop_funct();
    println!();
    //--------------------------
    loop_funct_label();
    println!();
    //---------------------------
    loop_funct_while();
    println!();
    //-----------------------------
    loop_funct_for();
    println!();
}

fn loop_funct(){ // se usar loop ele automaticamente cria uma função de loop xD
    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn loop_funct_label(){
    let mut count = 0;

    'couting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'couting_up;
            }
            remaining -= 1;
        };

        count += 1;
    };
    println!("End count = {count}");
}

fn loop_funct_while(){
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn loop_funct_for(){
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}