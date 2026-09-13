mod storage;

fn main() {

    let data_base = storage::CacheDatabase::new();
    let chave: String = "teste".to_string();
    let valor: String = "esse é o valor do cpf".to_string();
    
    data_base.set(chave.clone(), valor);
    let retorno: Option<String> = data_base.get(&chave);

    if let Some(retorno_teste) = retorno {
    println!("Usuário encontrado: {}", retorno_teste);
    }
}
