pub enum Command {
    Add { key: String, value: String },
    Get { key: String },
    Exit,
    Unknown(String),
    Empty,
}

pub fn parse_line(line: &str) -> Command {
    let mut clean_line = line;
    if let Some(idx) = clean_line.find("->") {
        clean_line = &clean_line[..idx];
    }
    
    clean_line = clean_line.trim();
    if clean_line.is_empty() || clean_line.starts_with('#') {
        return Command::Empty;
    }

    let parts: Vec<&str> = clean_line.split_whitespace().collect();
    if parts.is_empty() {
        return Command::Empty;
    }

    match parts[0] {
        "EXIT" => Command::Exit,
        "GET" => {
            if parts.len() == 2 {
                Command::Get { key: parts[1].to_string() }
            } else {
                Command::Unknown("Comando GET incompleto. Uso: GET chave".to_string())
            }
        }
        "ADD" => {
            if let Some((_, rest)) = clean_line.split_once(' ') {
                let rest = rest.trim_start();
                if let Some((key, value)) = rest.split_once(' ') {
                    let clean_value = value.trim().to_string();
                    if clean_value.is_empty() {
                        return Command::Unknown("valor vazio".to_string());
                    }
                    Command::Add {
                        key: key.to_string(),
                        value: clean_value,
                    }
                } else {
                    Command::Unknown("valor vazio".to_string())
                }
            } else {
                Command::Unknown("comando incompleto".to_string())
            }
        }
        _ => Command::Unknown("comando desconhecido".to_string()),
    }
}



// pub enum Command {
//     Add { key: String, value: String },
//     Get { key: String },
//     Exit,
//     Unknown(String),
//     Empty,
// }

// pub fn parse_line(line: &str) -> Command {
//     let line = line.trim();
//     if line.is_empty() {
//         return Command::Empty;
//     }

//     let parts: Vec<&str> = line.splitn(3, ' ').collect();
    
//     match parts[0] {
//         "EXIT" => Command::Exit,
//         "GET" => {
//             if parts.len() == 2 {
//                 Command::Get { key: parts[1].to_string() }
//             } else {
//                 Command::Unknown("Comando GET incompleto. Uso: GET chave".to_string())
//             }
//         }
//         "ADD" => {
//             if parts.len() >= 3 {
//                 // O valor é tudo depois do primeiro espaço após a chave
//                 let (_, rest) = line.split_once(' ').unwrap();
//                 let (key, value) = rest.split_once(' ').unwrap();
//                 Command::Add {
//                     key: key.to_string(),
//                     value: value.to_string(),
//                 }
//             } else {
//                 Command::Unknown("Comando ADD incompleto. Uso: ADD chave valor".to_string())
//             }
//         }
//         _ => Command::Unknown("Comando inválido. Use ADD, GET ou EXIT".to_string()),
//     }
// }


