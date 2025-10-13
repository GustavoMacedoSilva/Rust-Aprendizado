//Não é possivel fazer operações com um i32 e um Option<i32>
//Vamos fazer uma função que receba um Option<i32> e, caso tenha algo dentro, some mais 1
//caso tenha None, ele retorna None e nao tenta fazer nenhuma operação
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none:Option<i32> = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i + 1),
    }
}

//matches sao exaustivas, voce tem que cobrir TODOS os casos, se nao, rust cospe um erro