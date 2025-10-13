fn main(){
    //em um programa que quer variaveis relacionadas façam um certo processo
    //por exemplo: calcular a area de um retangulo
    //podemos usar tuples ou struct

    //tuples
    let rect1:(f32, f32) = (30.0, 50.0);

    println!(
        "A area do retangulo é {}",
        area(rect1)
    );

    let rect2: Retangulo = Retangulo{
        largura: dbg!(30.0),
        altura: 50.0,
    };

    println!("A area do retangulo struct eh {}", area_struct(&rect2));

    //caso queiramos(?) printar uma struct inteira com o println!, devemos antes colocar #[derive(Debug)] na nossa struct
    //ai temos 2 jeitos de printar
    println!("Rect1 eh {rect1:?}");
    //ou
    println!("Rect1 eh {rect1#?}");
    //outro jeito é usando dbg!, diferente do println!, o dbg! retorna o ownership da expressao
    dbg!(&rect2);
}

fn area(dimensions: (f32, f32)) -> f32{
    dimensions.0 * dimensions.1
}

fn area_struct(retangulo: &Retangulo) -> f32{
    retangulo.largura * retangulo.altura
}

//é possivel fazer a mesma coisa com structs, fica melhor e mais entendivel
#[derive(Debug)]
struct Retangulo{
    largura: f32,
    altura: f32,
}
