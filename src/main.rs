mod lexer;

fn main() {
    let input = String::from("div>ul>li{item}*333");
    let mut l = lexer::Lexer::new(input.chars().collect());
    l.read_char();

    println!("input string: \"{}\"\n", input);
    
    loop {
        let token = l.next_token();
        // se estivermos no fim do arquivo, sair do loop
        if token == lexer::tokens::Token::EOF {
            break;
        }
        // se o token for ilegal, falha
        else if token == lexer::tokens::Token::ILLEGAL {
            panic!("ILLEGAL TOKEN");
        }
        else {
            println!("{:?}", token);
        }
    }
}