//ennums ou ennumerations, te permitem definir um tipo enumerando suas variantes.
//vamos pegar o exemplo de classes de um jogo
//o player pode escolher 3 classes: Cavaleiro, Mago ou Bandido
//tambem existe um item chamado livro, esse livro pode ter alguns tipos

enum Livro{
    Magico{tipo: String, descricao: String},
    Normal{tipo: String, xp: u8},
}
#[derive(Debug)]
enum Espada{
    Espada1,
    Espada2,
    Espada3,
}
#[derive(Debug)]
enum Cajado{
    Cajado1,
    Cajado2,
    Cajado3,
}
#[derive(Debug)]
enum Adaga{
    Adaga1,
    Adaga2,
    Adaga3,
}
enum Classe{
    Cavaleiro(Espada),
    Mago(Cajado),
    Bandido(Adaga),
}
impl Classe{
    fn falar(&self){
        println!("Ola eu sou um player!");
    }
}
fn main() {
    let player1 = Classe::Cavaleiro(Espada::Espada1);
    let player2: Classe = Classe::Bandido(Adaga::Adaga2);
    let player3: Classe = Classe::Mago(Cajado::Cajado3);

    player1.falar();
    player2.falar();
    player3.falar();

    let some_number = Some(5);
    let some_char = Some("e");

    let absent_number: Option<i32> = None;
    
    classe_escolhida(player1);
    classe_escolhida(player2);
    classe_escolhida(player3);
}
//outro tipo importante é o Option<T>
//ele pode ser algo, ou nada


fn classe_escolhida(classe: Classe){
    match classe{
        Classe::Bandido(adaga) =>{
            println!("Sou um Bandido e uso {adaga:?}");
        },
        Classe::Cavaleiro(espada) =>{
            println!("Sou um Cavaleiro e uso {espada:?}");
        },
        Classe::Mago(cajado) => {
            println!("Sou um Mago e uso {cajado:?}");
        }
    }
}