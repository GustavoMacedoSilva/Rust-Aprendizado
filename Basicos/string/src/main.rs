fn main() {
    let mut _s = String::new();

    let data = "initial contents";

    let _s = data.to_string();

    //este metodo tambem funciona em uma literal diretamente
    let _s = "initial contents".to_string();

    //Updatiando uma String
    let mut s1 = String::from("foo");
    s1.push_str("bar");

    let mut s2 = String::from("foo");
    let s3 = "bar";
    s2.push_str(s3);
    println!("s2 is {s2}");

    //usando o medoto push apenas
    let mut s4 = String::from("lo");
    s4.push('l');

    let s5 = String::from("Hello, ");
    let s6 = String::from("World!");
    let s7 = s5 + &s6;

    //outro jeito de concatenar é:
    let f1 = String::from("tic");
    let f2 = String::from("tac");
    let f3 = String::from("toe");

    let f = format!("{f1}-{f2}-{f3}");

    //Slicing Strings
    let hello = "slaaa";
    let x = &hello[0..4];

    //metodos para iterar sobre Strings
    //usando chars
    for c in "hello".chars() {
        println!("{c}");
    }

    //usando bytes
    for b in "hello".bytes() {
        println!("{b}");
    }
}
