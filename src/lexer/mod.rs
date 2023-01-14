mod res;
pub mod tokens;

pub struct Lexer {
    input:             Vec<char>,   // string contendo todo o código
    pub position:      usize,       // posição de leitura
    pub read_position: usize,       // posição de leitura + 1 (aponta para o proximo lexema)
    pub ch:            char         // caracter a ser lido
}

impl Lexer {
    /* funções do lexer */
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input:         input,   // input = input
            position:      0,       // posição inicial = 0
            read_position: 0,       // posição de leitura = 0
            ch:           '0'       // caracter = '0'
        }
    }

    // ler caracter
    pub fn read_char(&mut self) {
        // se a posição de leitura = tamanho do input, definir que chegamos ao fim
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } 
        // senão, caracter = input[posição_atual];
        else {
            self.ch = self.input[self.read_position];
        }
        
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    // pular espaços em branco entre outros caracteres
    pub fn skip_whitespace(&mut self) {
        let ch = self.ch;
        
        if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            self.read_char();
        }
    }

    // ler proximo token, interpretar seu valor e definir seu tipo
    pub fn next_token(&mut self) -> tokens::Token {
        /* closures para ler tokens complexos (palavras, numerais e conteudos dos atributos */
        // ler palavra
        let read_word = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;

            // enquanto caracter atual é letra
            while l.position < l.input.len() && res::is_letter(l.ch) {
                // prucurar uma não-letra
                l.read_char();
            }
            // concatenar palavra 
            l.input[position..l.position].to_vec()
        };

        // ler os conteudos entre colchetes
        let read_attr_content = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            
            // se caracter atual é '['
            if res::is_left_square_bracket(l.ch) {
                // enquanto caracter atual é diferente de ']'
                while l.position < l.input.len() && res::is_right_square_bracket(l.ch) == false {
                    // prucurar por um ']'
                    l.read_char();
                }
            }
            // inserir o ']'
            l.read_char();
            // concatenar o conteudo 
            l.input[position..l.position].to_vec()
        };

        // ler numero
        let read_number = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            
            // enquanto caracter atual é digito
            while l.position < l.input.len() && res::is_digit(l.ch) {
                // procurar um não-digito
                l.read_char();
            }
            // concatenar numero 
            l.input[position..l.position].to_vec()
        };

        let tok: tokens::Token;
        self.skip_whitespace();

        /* fazer o reconhecimento dos lexemas e rotular os tokens */
        match self.ch {
            // reconhecer simbolos/acentuação
            '{'  => { tok = tokens::Token::T_LBRACE(self.ch); }          
            '}'  => { tok = tokens::Token::T_RBRACE(self.ch); }          
            '('  => { tok = tokens::Token::T_LPAREN(self.ch); }          
            ')'  => { tok = tokens::Token::T_RPAREN(self.ch); }          
            '<'  => { tok = tokens::Token::T_SMALLERTHAN(self.ch); }     
            '>'  => { tok = tokens::Token::T_GREATERTHAN(self.ch); }
            '\'' => { tok = tokens::Token::T_SINGLEQUOTE(self.ch); }     
            '\"' => { tok = tokens::Token::T_DOUBLEQUOTE(self.ch); }     
            '.'  => { tok = tokens::Token::T_DOT(self.ch); }             
            ','  => { tok = tokens::Token::T_COMMA(self.ch); }           
            '='  => { tok = tokens::Token::T_ASSIGN(self.ch); }          
            '+'  => { tok = tokens::Token::T_PLUS(self.ch); }            
            '-'  => { tok = tokens::Token::T_MINUS(self.ch); }           
            '*'  => { tok = tokens::Token::T_ASTERISK(self.ch); }        
            '/'  => { tok = tokens::Token::T_SLASH(self.ch); }           
            '0'  => { tok = tokens::Token::T_EOF; }

            _ => {
                // se uma letra for encontrada, ler a palavra
                if res::is_letter(self.ch) {
                    let word: Vec<char> = read_word(self);
                    
                    // verificar se a palavra é uma palavra chave
                    match tokens::get_keyword(&word) {
                        // se sim, retorna a palavra chave
                        Ok(keywork_token) => {
                            return keywork_token;
                        },
                        // senão, retorna a palavra
                        Err(_err) => {
                            return tokens::Token::T_WORD(word);
                        }
                    }
                }
                // se um digito for encontrado
                else if res::is_digit(self.ch) {
                    // ler o numero completo
                    let number: Vec<char> = read_number(self);
                    // retorna o numero
                    return tokens::Token::T_INTLIT(number);
                } 
                // se o digito for um '['
                else if self.ch == '[' {
                    // ler o conteudo entre colchetes
                    let attr_content: Vec<char> = read_attr_content(self);

                    // converter o vetor de caracteres em string
                    let str: String = attr_content.iter().collect();
                    
                    // retornar o conteudo entre colchetes
                    return tokens::Token::T_CUSTOM_ATTR_CONTENT(str);
                }
                // se o token for irreconhecivel (ilegal)
                else {
                    // debug
                    println!("unrecognized token: {}", self.ch);
                    
                    return tokens::Token::T_ILLEGAL
                }
            }
        }
        // ler o proximo token
        self.read_char();
        // retornar o token
        tok
    }
}