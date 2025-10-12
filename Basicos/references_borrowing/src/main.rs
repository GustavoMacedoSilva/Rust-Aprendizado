fn main() {
    //usando referencia nos nao precisamos mover a variavel
    let mut s1 = String::from("hello");

    let mut len = calcular_len(&mut s1);

    println!("O tamanho de {s1} eh {len}");

    mudar(&mut s1);
    
    len = calcular_len(&mut s1);

    println!("O tamanho de '{s1}' eh {len}");

    //voce nao pode ter mais de 1 variavel borrowing como mut
}

fn calcular_len(s: &String) -> usize{
    s.len() //esse "s" é um ponteiro que aponta para o ponteiro do s1 que aponta para o valor "hello" na heap
}

fn mudar(s: &mut String){
    s.push_str(", world!");
}

//recap:
//voce so pode ter 1 mutable reference ou ou qualquer quantidade de referencias imutaveis
//referencias sempre devem ser válidas