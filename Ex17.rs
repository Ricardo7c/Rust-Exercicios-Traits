trait Pagamento {
    fn processar(&self);
}

struct CartaoCredito{}
struct Boleto{}

impl Pagamento for CartaoCredito {
    fn processar(&self) {
        println!("Processando pagamento por cartão de crédito.");
    }
}

impl Pagamento for Boleto {
    fn processar(&self) {
        println!("Procesando pagamento por boleto bancário.");
    }
}

fn executar_pagamentos(pagamentos: Vec<&dyn Pagamento>){
    for pagamento in pagamentos{
        pagamento.processar();
    }
}


fn main(){
    let cartao = CartaoCredito{};
    let boleto = Boleto{};
    let pagamentos: Vec<&dyn Pagamento> = vec![&cartao, &boleto];
    executar_pagamentos(pagamentos);
}