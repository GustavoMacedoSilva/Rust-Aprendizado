fn main() {
    //Vectors permitem que guarde mais de um valor de um mesmo tipo perto de cada um na memoria
    let mut v: Vec<i32> = Vec::new();
    //codigo acima cria um vector do tipo i32 vazio
    //podemos usar o macro vec! para criar um vector autimaticamente, ja inferindo o tipo pelo
    //conteudo inicial
    let v2 = vec![1, 2, 3];
    //para Updatear um Vector nos usamos o metodo push
    v.push(5);
    v.push(6);
    //para ler elementos de um Vector temos 2 meios
    //usando indexing ou com o metodo get
    let mut v3 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    //para iterar sobre um vector imutavel, fazemos:
    for i in &v2 {
        println!("{i}");
    }
    //para iterar sobre um vector mutavel:
    for i in &mut v3 {
        *i += 50;
    }

    //caso precisamos que um vector guarde valores de tipos diferentes, usamos enums
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("Blue")),
    ];
}
