struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let user1 = User{
        active: true,
        username: String::from("username1"),
        email: String::from("email1@gmail.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.username); //pega o email do user1, nao é possivel alterar, ja que esse user nao é mut
    //nao é possivel criar um user com apenas algumas coisas mut

    //vamos supor que voce quer copiar algumas informações de um outro user pra um novo
    //para nao ter que escrever tudo denovo faça

    let user2:User = User{
        email: String::from("xexo@gmail.com"),
        ..user1
    };

    //tambem é possivel criar tuples structs, sem nome para as variaveis
    let black = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);

    //é possivel tambem criar unit-like structs
    let subject = AlwaysEqual;
}



struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn criar_user(username: String, email: String) -> User{
    User { active: (true), username: (username), email: (email), sign_in_count: (1) }
}
//tambem é possivel fazer assim
fn criar_user_short(username: String, email: String) -> User{
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}//como os nomes dos parametros e das variaveis sao iguais, é possivel fazer desse jeito para evitar escrever a mesma coisa varias vezes