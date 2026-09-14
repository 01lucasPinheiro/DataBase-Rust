use std::io::{self, Write};
use crate::parser::{self, Command};
use crate::storage::CacheDatabase;
use crate::lua::LuaEngine; 

pub fn run_loop(db: &CacheDatabase, lua_engine: &LuaEngine) {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, 
            Ok(_) => {
                match parser::parse_line(&input) {
                    Command::Exit => break,
                    Command::Empty => continue,
                    Command::Unknown(err) => println!("ERRO: {}", err),
                    Command::Get { key } => {
                        match db.get(&key) {
                            Some(val) => {
                                // Antes de exibir, passa pela extensão Lua
                                match lua_engine.process_get(&key, &val) {
                                    Ok(formatted) => println!("{}", formatted),
                                    Err(e) => println!("ERRO: {}", e),
                                }
                            },
                            None => println!("ERRO: chave inexistente"),
                        }
                    }
                    Command::Add { key, value } => {
                        // Antes de salvar, a extensão Lua intercepta e valida
                        match lua_engine.process_add(db, &key, &value) {
                            Ok(final_value) => {
                                db.set(key, final_value);
                                println!("OK");
                            }
                            Err(err_msg) => println!("ERRO: {}", err_msg),
                        }
                    }
                }
            }
            Err(_) => {
                println!("ERRO: falha ao ler entrada");
                break;
            }
        }
    }
}