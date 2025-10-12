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

    //----------------------------------------------------------------------
    let s1 = String::from("hello"); //s1 entra em escopo
    takes_ownership(s1); //s1 é movida para a função, e não é mais válida aqui

    let x = 5; //x entra em escopo
    makes_copy(x); //x é copiada para a função, e ainda é válida aqui
    println!("{}", x); //imprime o valor de x
//----------------------------------------------------------------------

    let st1 = gives_ownership(); //gives_ownership da o valor do return da função para st1

    let st2 = String::from("hello");

    let st3 = takes_and_gives_back(st2); //st2 é movido para a função takes_and_gives_back, que tambem move o return value para
                                                          //st3

} // aqui st3 sai do escopo e é droppado, st2 foi movido entao nada acontece, s1 sai do escopo e é droppado tambem

fn takes_ownership(some_string: String) { //some_string entra em escopo
    println!("{}", some_string);
} //some_string sai de escopo e a memória é liberada

fn makes_copy(some_integer: i32) { //some_integer entra em escopo
    println!("{}", some_integer);
} //some_integer sai de escopo, mas nada acontece pois é um tipo primitivo

fn gives_ownership() -> String{
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String{
    //a_string entra no escopo
    a_string //a_string é rotarnada e é movida para o chamador da função
}