use crate::cliente::Cliente;
pub struct Banco{
    pub nome: String,
    pub clientes: [Cliente; 2],
}
impl Banco{
    pub fn adicionar_fundos(&mut self, id: usize, qtd: f32){
        self.clientes[id].saldo += qtd;
        println!("Foi adicionado ao cliente {} uma quantia de {}", self.clientes[id].nome, qtd);
        println!("Saldo final: {}", self.clientes[id].saldo);
    }
    pub fn remover_fundos(&mut self, id: usize, qtd: f32){
        self.clientes[id].saldo -= qtd;
        println!("Foi removido do {} uma quantidade {}", self.clientes[id].nome, qtd);
        println!("Saldo final: {}", self.clientes[id].saldo);
    }
    pub fn transferencia(&mut self, id_transferidor: usize, id_recebidor: usize, qtd: f32){
        self.clientes[id_recebidor].saldo += qtd;
        self.clientes[id_transferidor].saldo -= qtd;
        
        println!("Foi feita uma transferencia de {} do {} para o {}", qtd, self.clientes[id_transferidor].nome, self.clientes[id_recebidor].nome);
        println!("Saldo final do Cliente {}: {}", self.clientes[id_recebidor].nome, self.clientes[id_recebidor].saldo);
        println!("Saldo final do Cliente {}: {}", self.clientes[id_transferidor].nome, self.clientes[id_transferidor].saldo);
    }
    pub fn printar_informacoes(&self){
        println!("Esse banco se chama {}", self.nome);
        println!("Possui {} Clientes", self.clientes.len());
        println!("Lista de clientes cadastrados: {:#?}", self.clientes);
    }
}