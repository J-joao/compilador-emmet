// verifica se o caracter é uma letra (a..zA..Z)
pub fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

// verifica se o caracter é um digito (0..9)
pub fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

// verifica se o caracter é [
pub fn is_left_square_bracket (ch: char) -> bool {
    ch == '['
}

// verifica se o caracter é ]
pub fn is_right_square_bracket (ch: char) -> bool {
    ch == ']'
}