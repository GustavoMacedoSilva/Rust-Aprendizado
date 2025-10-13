#[derive(Debug)]
pub struct Cliente{
    pub nome: String,
    pub saldo: f32,
    pub id: u32,
}
impl Cliente{
    pub fn printar_informacoes(&self){
        println!("
        Nome: {},
        Saldo: {},
        id: {}
        ", self.nome, self.saldo, self.id);
    }
}