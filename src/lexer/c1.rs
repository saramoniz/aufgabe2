use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // TODO: Define variants and their token/regex
	
	//tokens
	
	#[token("bool")]
	KW_BOOLEAN,
	#[token("do")]
	KW_DO,
	#[token("else")]
	KW_ELSE,
	#[token("float")]
	KW_FLOAT,
	#[token("for")]
	KW_FOR,
	#[token("if")]
	KW_IF,
	#[token("int")]
	KW_INT,
	#[token("printf")]
	KW_PRINTF,
	#[token("return")]
	KW_RETURN,
	#[token("void")]
	KW_VOID,
	#[token("while")]
	KW_WHILE,
	
	#[token("+")]
	PLUS,
	#[token("-")]
	MINUS,
	#[token("*")]
	ASTERISK,
	#[token("/")]
	SLASH,
	#[token("=")]
	ASSIGN,
	#[token("==")]
	EQ,
	#[token("!=")]
	NEQ,
	#[token("<")]
	LSS,
	#[token(">")]
	GRT,
	#[token("<=")]
	LEQ,
	#[token(">=")]
	GEQ,
	#[token("&&")]
	AND,
	#[token("||")]
	OR,
	
	#[token(",")]
	COMMA,
	#[token(";")]
	SEMICOLON,
	#[token("(")]
	LPAREN,
	#[token(")")]
	RPAREN,
	#[token("{")]
	LBRACE,
	#[token("}")]
	RBRACEM,
	
	//regex
	
	#[regex("[0-9]+")]
	CONST_INT,
	#[regex("[0-9]+ "." [0-9]+" | ""." [0-9]+ ( [eE] ([-+])? [0-9]+ )?" | "[0-9]+ [eE] ([-+]) [0-9]+")]
	CONST_FLOAT,
	#[regex("true" | "false")]
	CONST_BOOLEAN,
	#[regex(r#""([^"\\]|\\.)*""#)]
	CONST_STRING,
	#[regex("([a-zA-Z])+ ([0-9] | [a-zA-Z])*")]
	ID,
	
	#[logos(skip r#"// ([^"\\]|\\.)*"#)]
	#[logos(skip r#"/\* ([^"\\]|\\.)* \*/"#]
	#[logos(skip r"[ \n\t\f]")]
	
	
    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}
