mod res;
pub mod tokens;

pub struct Lexer {
    input:             Vec<char>,   // string contendo todo o código
    pub position:      usize,       // posição de leitura
    pub read_position: usize,       // posição de leitura + 1 (aponta para o proximo lexema)
    pub ch:            char         // caracter a ser lido
}

impl Lexer {
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
        // ler palavra
        let read_word = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            
            // enquanto caracter atual é letra
            while l.position < l.input.len() && res::is_letter(l.ch) {
                // prucurar uma não-letra
                l.read_char();
            }
            // concatenar letra à palavra 
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
            // concatenar digito ao numero 
            l.input[position..l.position].to_vec()
        };

        let tok: tokens::Token;
        self.skip_whitespace();

        match self.ch {
            // reconhecer simbolos de acentuação
            '*' => {
                tok = tokens::Token::ASTERISK(self.ch);
            },
            '<' => {
                tok = tokens::Token::SMALLERTHAN(self.ch);
            },
            '>' => {
                tok = tokens::Token::GREATERTHAN(self.ch);
            },
            '{' => {
                tok = tokens::Token::LBRACE(self.ch);
            },
            '}' => {
                tok = tokens::Token::RBRACE(self.ch);
            },
            '0' => {
                tok = tokens::Token::EOF;
            }
            // switch "default:"
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
                            return tokens::Token::WORD(word);
                        }
                    }
                }
                // se um digito for encontrado
                else if res::is_digit(self.ch) {
                    // ler o numero completo
                    let word: Vec<char> = read_number(self);
                    // retorna o numero
                    return tokens::Token::INT(word);
                } 
                // token irreconhecivel
                else {
                    return tokens::Token::ILLEGAL
                }
            }
        }
        // ler o proximo token
        self.read_char();
        // retornar o token
        tok
    }
}