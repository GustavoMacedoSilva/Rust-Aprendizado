// Em Rust temos 2 tipos de tipos de DataTypes, as Scalar e as compound
fn scalar_types(){
    //O tipo scalar representa um valor unico. Rust tem 4 tipo scalar primarios:
    //Integer, floating-point numbers, Booleans e characters.
    
    //Integers sao definidos do seguinte jeito, eles podem ser Unsigned ou Signed:
    //length   ---  Signed ----- Unsigned
    //8-bit         i8           u8
    //16-bit        i16          u16
    //e isso ate 128 bits
    //architecture dependent isize   usize
    

    // Signed e Unsigned dizem se o numero pode ou nao ser negativo, ou seja, se precisam ter sinal
    // Cada Signed pode contar numeros de -(2^(n-1)) ate 2^(n-1)-1, onde n eh o numero de bits que
    // a variante usa , ou seja, o i8 pode ter numeros de -(2^7) ate (2^7 - 1), que seria de -128
    // ate 127
    // Ja Unsigned podem guardar numeros de 0 ate (2^n - 1), ou seja, se usarmos u8, sera de 0 ate
    // 2^8 -1 que seria 0 ate 255
    // Ja o isize e o usize varia de pendendo da arquitetura que voce esta utilizando, ou seja se
    // seu pc eh 64-bits ent usara 64bit
    // Voce pode em Integer separa os numeros decimais desse jeito 1_000, isso significa 1000 mas
    // fica mais facil de ler
    
    //Floating-Point Types possuem f32 e f64, o default do Rust para float eh f64, ja que eh quase
    //a mesma velocidade do f32 soq melhor pq cabe mais
    //let x = 2.0 //f64
    //let y: f32 = 3.0 //f32

    //Booleans todo mundo sabe como funciona
    
    //Character type: sao o mais primitivo do alphabetic types, sao definidos com single quotes
    //let c = 'z'
    //let z: char = 'Z'
}

fn compound_types(){
    //Em rust temos 2 tipos primitivos de compound: tuples e arrays
    //
    //Uma Tuple tem tamanho fixo, ou seja, depois de criadas elas nao podem crescer nem diminuir
    //let tup = (i32, f64, u8) = (500, 6.4, 1);
    //
    //Para pegar valores individuais de uma Tuple fazemos:
    //let tup = (500, 6.4, 1);
    //let (x, y, z) = tup;
    //println!("O valor de y eh {y}")
    //Isso eh chamado de destructuring, porque ele quebra a tuple em 3 partes
    //
    //Nos tambem podemos usar o (.) seguido do index do que voce quer pegar:
    //let x: (i32, f64, u8) = (500, 6.4, 1);
    //let five_hundred = x.0;
    //let six_point_four = x.1;
    //let one = x.2;
    //
    //arrays
    //Diferente das Tuples, arrays devem conter apenas conteudos do mesmo tipo. Elas tambem tem
    //tamano fixo
    //Arrays sao mais uteis que Vectors quando voce sabe que a quantidade de elementos nao vai
    //mudar. Por exemplo, nomes dos meses do ano. nunca vai ser nem mais nem menos que 12
    //Tem alguns jeitos de declarar arrays
    //let a = [1, 2, 3, 4];
    //let b: [i32; 5] = [1, 2, 3, 4, 5]; especificamos o tipo e o tamanho
    //let c = [3; 5]; aqui criamos uma array de tamanho 5 ja colocando 3 em todos os indexes
    //Para acessar valores eh igual C++
    //let a = [1, 2, 3, 4, 5];
    //let first = a[0];
    //let last = b[4];
    //
    //
}
