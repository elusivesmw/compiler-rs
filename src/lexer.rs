pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    //char: char,
    //token: Vec<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut out = Vec::<Token>::new();

        loop {
            let t = self.next_token();
            if let Token::Eof = t { break; }
            out.push(t);
        }

        out
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(c) = self.next() {
            return match c {
                '(' => { Token::OpenParen }
                ')' => { Token::CloseParen }
                '{' => { Token::OpenBrace }
                '}' => { Token::CloseBrace }
                '[' => { Token::OpenBracket }
                ']' => { Token::CloseBracket }

                ',' => { Token::Comma }
                '>' => {
                    if is_expected(self.peek(), '=') {
                        Token::GreaterThanOrEquals
                    } else if is_expected(self.peek(), '>') {
                        Token::BitShiftRight
                    } else {
                        Token::GreaterThan
                    }
                }
                '<' => {
                    if is_expected(self.peek(), '=') {
                        Token::LessThanOrEquals
                    } else if is_expected(self.peek(), '<') {
                        Token::BitShiftLeft
                    } else {
                        Token::LessThan
                    }
                }

                '+' => {
                    if is_expected(self.peek(), '+') {
                        Token::Increment
                    } else {
                        Token::Plus
                    }
                }
                '-' => {
                    if is_expected(self.peek(), '-') {
                        Token::Decrement
                    } else {
                        Token::Dash
                    }
                }

                '*' => { Token::Star }
                '/' => {
                    if is_expected(self.peek(), '/') {
                        self.next();
                        self.skip_whitespace();
                        let mut comment = Vec::new();
                        while let Some(c) = self.peek() {
                            if c == '\n' {
                                break;
                            } else {
                                self.next();
                                comment.push(c);
                            }
                        }
                        Token::Comment(comment.iter().collect())
                    } else if is_expected(self.peek(), '*') {
                        self.next();
                        self.skip_whitespace();
                        let mut comment = Vec::new();
                        while let Some(c) = self.peek() {
                            if c == '*' {
                                if let Some(c2) = self.peek_at(1) {
                                    if c2 == '/' {
                                        self.next();
                                        self.next();
                                        break;
                                    }
                                }
                            } else {
                                self.next();
                                comment.push(c);
                            }
                        }
                        Token::Comment(comment.iter().collect())
                    } else {
                        Token::Slash
                    }
                }

                ':' => { Token::Colon }
                ';' => { Token::Semicolon }
                '!' => {
                    if is_expected(self.peek(), '=') {
                        self.next();
                        Token::NotEquals
                    } else {
                        Token::Bang
                    }
                }
                '=' => {
                    if let Some(_) = self.peek() {
                        self.next();
                        Token::Equals
                    } else {
                        Token::Assign
                    }
                }

                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut ident = vec![c];
                    while let Some(n) = self.peek() {
                        if !n.is_alphanumeric() && n != '_' { break; }
                        self.next();
                        ident.push(n);
                    }
                    let ident: String = ident.iter().collect();
                    match ident.as_str() {
                        "main" => { Token::Main }
                        "fn" => { Token::Function }
                        "if" => { Token::If }
                        "else" => { Token::Else }
                        "return" => { Token::Return }
                        "true" => { Token::True }
                        "false" => { Token::False }
                        _ => Token::Identifier(ident)
                    }
                }

                '0'..='9' => {
                    let mut num = vec![c];
                    while let Some(n) = self.peek() {
                        if !n.is_numeric() { break; }
                        self.next();
                        num.push(n);
                    }
                    let num: String = num.iter().collect();
                    if let Ok(num) = num.parse::<u8>() {
                        Token::Uint8(num)
                    } else if let Ok(num) = num.parse::<u16>() {
                        Token::Uint16(num)
                    } else {
                        Token::Invalid(c.to_string())
                    }
                }

                _ => {
                    Token::Invalid(c.to_string())
                }
            };
        } else {
            Token::Eof
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            if let Some(c) = self.peek() {
                match c {
                    ' ' | '\t' | '\n' => { self.pos += 1; }
                    _ => { break; }
                }
            } else { break; }
        }
    }

    fn next(&mut self) -> Option<char> {
        let c = self.char_at(self.pos);
        self.pos += 1;
        c
    }

    fn peek(&self) -> Option<char> {
        self.char_at(self.pos)
    }

    fn peek_at(&self, offset: usize) -> Option<char> {
        self.char_at(self.pos + offset)
    }

    fn char_at(&self, pos: usize) -> Option<char> {
        if pos < self.input.len() {
            Some(self.input[pos])
        } else {
            None
        }
    }

}


fn is_expected(current: Option<char>, expect: char) -> bool {
    if let Some(c) = current {
        if c == expect {
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[derive(Debug)]
pub enum Token {
    Invalid(String),

    Identifier(String),

    // nums - idk how i wanna do this yet
    Int8(i8),
    Uint8(u8),
    Int16(i16),
    Uint16(u16),

    Comment(String),

    // Keyword
    Main,
    Function,
    If,
    Else,
    Return,
    True,
    False,

    // single char
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,

    Bang,
    Plus,
    Dash,
    Star,
    Slash,

    Increment,
    Decrement,

    BitShiftLeft,
    BitShiftRight,
    BitRotateLeft,
    BitRotateRight,

    Comma,
    Colon,
    Semicolon,

    // single or double char
    Assign,
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEquals,
    LessThan,
    LessThanOrEquals,

    //
    Eof,
}


