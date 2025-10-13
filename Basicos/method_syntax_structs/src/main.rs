//metodos é a mesma coisa que metodos em poo so que para structs
//seu primeiro parametro sempre deve ser self, que representa a instancia que esta chamando o metodo
// struct Cliente{
//     nome: String,
//     saldo: f32,
//     id: u32,
// }
// impl Cliente{
//     fn printar_informacoes(&self){
//         println!("
//         Nome: {},
//         Saldo: {},
//         id: {}
//         ", self.nome, self.saldo, self.id);
//     }
// }
// struct Banco{
//     nome: String,
//     clientes: [Cliente; 10],
// }
// impl Banco{
//     fn adicionar_fundos(&mut self, id: usize, qtd: f32){
//         self.clientes[id].saldo = qtd;
//         println!("Foi adicionado ao cliente {} uma quantia de {}", self.clientes[id].nome, qtd);
//         println!("Saldo final: {}", self.clientes[id].saldo);
//     }
//     fn remover_fundos(&mut self, id: usize, qtd: f32){
//         self.clientes[id].saldo = qtd;
//         println!("Foi removido do {} uma quantidade {}", self.clientes[id].nome, qtd);
//         println!("Saldo final: {}", self.clientes[id].saldo);
//     }
//     fn transferencia(&mut self, id_transferidor: usize, id_recebidor: usize, qtd: f32){
//         self.clientes[id_recebidor].saldo += qtd;
//         self.clientes[id_transferidor].saldo -= qtd;

//         println!("Saldo final do Cliente {}: {}", self.clientes[id_recebidor].nome, self.clientes[id_recebidor].saldo);
//         println!("Saldo final do Cliente {}: {}", self.clientes[id_transferidor].nome, self.clientes[id_transferidor].saldo);
//     }
// }
mod banco;
mod cliente;

use cliente::Cliente;
use banco::Banco;
fn main() {
    
    let cliente1: Cliente = Cliente{
        nome: String::from("Cliente1"),
        saldo: 100.0,
        id: 1,
    };
    let cliente2: Cliente = Cliente{
        nome: String::from("Cliente2"),
        saldo: 100.0,
        id: 2,
    };
    
    let mut banco1: Banco = Banco{
        nome: String::from("Banco 1"),
        clientes: [cliente1, cliente2],
    };

    banco1.clientes[0].printar_informacoes();
    println!();
    banco1.clientes[1].printar_informacoes();
    println!();

    banco1.adicionar_fundos(0, 1000.0);
    println!();
    banco1.remover_fundos(1, 50.0);
    println!();

    banco1.transferencia(0, 1, 400.0);
    println!();

    banco1.printar_informacoes();

}
