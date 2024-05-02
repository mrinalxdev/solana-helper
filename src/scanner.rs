use std::string::String;

pub struct Scanner {
    source : String,
    tokens : Vec<Token>,
    start: usize,
    current: usize,
    line: usize,

}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {

        let mut errors = vec![];

        while !self.is_at_end(){
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
            
        }  

        self.tokens.push(Token {
            token_type: Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
            

        });

        if errors.len() > 0 {
            let mut joined = "".to_string();
            let _ = errors.iter().map(|msg| {
                joined.push_str(&msg);
                joined.push_str("\n")
            });
            return Err(joined);
        }

        Ok(self.tokens.clone())
    }

    fn is_at_end(self: &Self) -> bool {
        self.current >= self.source.len() as usize
    }

    fn scan_token(self: &mut Self) -> Result<(), String>{
        let c = self.advance();

        // macro_rules! add_token {
        //     ($char:expr, $token:ident) => {
        //         $char => self.add_token($token),
        //     };
        // }

        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            _ => return Err(format!("Unrecognized char at line {} : {}", self.line, c))
        }

        todo!()
    }

    fn advance(self: &mut Self) -> char {
        // todo!()
        let c = self.source.as_bytes()[self.current];
        self.current += 1;

        c as char
    }

    fn add_token(self: &mut Self, token_type: TokenType){
        self.add_token_lit(token_type, None);
    }

    fn add_token_lit(self: &mut Self, token_type: TokenType, literal: Option<LiteralValue>){
        let mut text = "".to_string();
        let bytes = self.source.as_bytes();
        for i in self.start..self.current {
            text.push(bytes[i] as char);
        };

        self.tokens.push(Token {
            token_type: token_type,
            lexeme :text,
            literal: literal,
            line_number: self.line,
        }); 
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    SLASH,
    Star,

    //One or two character tokens
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    LESS,
    LessEqual,

    //Literals
    IDENTIFIER,
    STRING,
    NUMBER,

    //Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    Eof,
}

use TokenType::*;

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    IdentifierVal(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralValue>,
    line_number: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        line_number: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }

    pub fn to_string(self: &Self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
