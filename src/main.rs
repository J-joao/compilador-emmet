mod lexer;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // abrir arquivo de teste
    let mut file = File::open("example/sample.emmet").expect("file not found");
    // ler dados do arquivo para "file_data"
    let mut file_data = String::new();
    file.read_to_string(&mut file_data).expect("error while reading file");
    
    // debug
    println!("{}\n\n", file_data);
 
    // criar nova instancia de um lexer para analisar os dados do arquivo
    let mut l = lexer::Lexer::new(file_data.chars().collect());
    l.read_char();
    
    
    loop {
        let token = l.next_token();
        // se estivermos no fim do arquivo, sair do loop
        if token == lexer::tokens::Token::T_EOF {
            break;
        }
        // se o token for ilegal, falha
        else if token == lexer::tokens::Token::T_ILLEGAL {
            panic!("ILLEGAL TOKEN: {}", l.ch);
        }
        else {
            println!("{:?}", token);
        }
    }
    Ok(())
}