use std::collections::HashMap;
fn main() {
    //aqui nos criamos um HashMap vazio
    let mut scores = HashMap::new();
    //aqui inserimos valores no HashMap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //acessando valores no HashMap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    //essa linha de codigo acima faz o seguinte:
    //o 'get' metodo retorna um Option<&V>; se nao tem valor para aquela chave no HashMap
    //retornara um None. O programa lida com o Option chamando o 'copied', para pegar um
    //Option<i32> invés de um Option<&i32>, e o 'unwrap_or' seta o score para zero se nao tem
    //uma entrada para a chave
    //nos podemos iterar por cada chave-valor em um hashmap do mesmo jeito que fazemos em
    //um vector, usando um for loop
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    //falando agora um pouco sobre ownership em HashMaps
    //fazendo o seguinte:
    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //depois disso field_name e field_value são inválidos, se voce tentar usar
    //terá um erro de compilação xD

    //para sobreescrever um valor num hashmap fazemos
    let mut scores1 = HashMap::new();

    scores1.insert("Blue", 20);
    scores1.insert("Blue", 10);

    println!("{scores:?}");
    //o codigo vai printar {"Blue": 25}, o valor original de 20 foi sobreescrito
    //
    //Adicionando uma chave e um valo caso uma chave ja nao exista
    let mut scores2 = HashMap::new();
    scores2.insert("Blue", 20);
    scores2.entry("Yellow").or_insert(50);
    scores2.entry("Blue").or_insert(50);

    println!("{scores2:?}");
    //isso vai printar apenas {"Yellow": 50, "Blue": 20}, já que Blue ja existe no HashMap
    //
    //Updateando um valor baseado em um valor antigo
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
    //isso ira printar {"world": 2, "hello": 1, "wonderful": 1}
}
