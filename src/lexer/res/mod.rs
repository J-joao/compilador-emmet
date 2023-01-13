// verificar se o caracter é uma letra (a..zA..Z)
pub fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

// verificar se o caracter é um digito (0..9)
pub fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}
