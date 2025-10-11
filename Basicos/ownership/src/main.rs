// ownership é uma característica única do Rust que gerencia a memória de forma segura e eficiente sem um coletor de lixo.
// algumas features são: borrowing, slices e como o Rust coloca variaveis na memoria.

//ownership é um set de regras que o Rust usa para gerenciar a memória. A memória é gerenciada atráves de um set de regras que o compilador checa em tempo de compilação.
//se uma regra é quebrada, o código não compila.

//Stack e Heap
//Stack é uma região de memória que armazena dados de tamanho fixo. É rápido e organizado em uma estrutura LIFO (Last In, First Out).
//Adicionar data é chamado de "push" e remover é "pop". Todos os dados na stack devem ter um tamanho conhecido em tempo de compilação.
//Heap é uma região de memória que armazena dados de tamanho dinâmico. É mais lento que a stack, mas permite alocação e desalocação flexível.
//Quando você cria uma variável, o Rust decide se ela vai para a stack ou heap baseado no tamanho e tipo da variável.
//String é um tipo de dado que armazena uma sequência de caracteres. Em Rust, String é alocada na heap, permitindo que cresça e diminua dinamicamente.
//String literals, por outro lado, são armazenados na stack e têm um tamanho fixo conhecido em tempo de compilação.

//O processo de alocar memoria no heap é chamado de "allocating". O alocador de memória encontra um espaço livre na heap grande o suficiente para armazenar os dados e retorna um ponteiro para esse espaço.
//Já que o ponteiro é conhecido, de tamanho fixo, ele é armazenado na stack. O dado real é armazenado na heap.

//As regras de ownership são:
//1. Cada valor em Rust tem um dono (uma variável que é responsável por ele).
//2. Só pode haver um dono por vez.
//3. Quando o dono sai de escopo, o valor é descartado (a memória é liberada).


fn main() {
    
    { //s nao é válido aqui, pois está fora do escopo
        let s = "hello";
        println!("{}", s); //imprime a string
    }//s não é mais válido aqui, pois saiu do escopo

    let mut s = String::from("hello"); //String é alocada na heap
    s.push_str(", world!"); //adiciona uma string ao final de s
    println!("{}", s); //imprime a string
}