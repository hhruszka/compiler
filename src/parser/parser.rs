use crate::lexer::token_match::TokenMatch;
use crate::lexer::tokens::TokenType;
use crate::parser::ast::Node;

pub struct Parser {
    tokens: Vec<TokenMatch>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenMatch>) -> Self {
        Self {
            tokens: tokens,
            pos: 0,
        }
    }

    pub fn parse_program(&mut self) -> Result<Node, String> {
        let nodes= self.parse_function_definition()?;
        if self.pos < self.tokens.len() {
            return Err(format!("Syntax error: unexpected token {}", self.tokens[self.pos]));
        }
        Ok(Node::Program(nodes))
    }

    pub fn parse_statement(&mut self) -> Result<Node, String> {
        let mut nodes: Vec<Node> = Vec::new();
        self.expect(TokenType::Return,"return")?;
        nodes.push(self.parse_exp()?);
        self.expect(TokenType::Semicolon,";")?;
        Ok(Node::Statement("Return".to_string(),nodes))
    }

    pub fn parse_exp(&mut self) -> Result<Node, String> {
        // let mut nodes: Vec<Node> = Vec::new();
        let node = self.parse_int()?;
        Ok(Node::Exp("Const".to_string(),node.into()))
    }

    pub fn parse_int(&mut self) -> Result<Node, String> {
        match self.take_token()?.to_string().parse::<i32>() {
            Ok(i) =>  Ok(Node::IntLiteral(i)),
            Err(err) =>  Err(format!("Syntax error: {}", err.to_string())),
        }
    }

    pub fn parse_identifier(&mut self) -> Result<Node, String> {
        let token = self.take_token()?;
        if token.token_type() != TokenType::Identifier && token.token_type() != TokenType::Main {
            return Err(format!("Syntax error: expected identifier, got {}", token));
        }
        Ok(Node::Identifier(token.to_string()))
    }

    pub fn parse_function_definition(&mut self) -> Result<Vec<Node>, String> {
        let mut nodes: Vec<Node> = Vec::new();
        self.expect(TokenType::Int,"int")?;
        let name = self.parse_identifier()?;
        self.expect(TokenType::OpenParen,"(")?;
        self.expect(TokenType::Void,"void")?;
        self.expect(TokenType::CloseParen,")")?;
        self.expect(TokenType::OpenBrace,"{")?;
        let body = self.parse_statement()?;
        self.expect(TokenType::CloseBrace,"}")?;
        nodes.push(Node::FunctionDefinition{name: name.into(),body: body.into()});
        Ok(nodes)
    }

    fn expect(&mut self, token_type: TokenType, token: &str) -> Result<(), String> {
        let actual = self.take_token()?;
        if actual.token_type() != token_type || actual.to_string() != token {
            return Err(format!("Syntax error: expected {} {}, got {}", token_type,token, actual));
        }
        Ok(())
    }

    fn take_token(&mut self) -> Result<&TokenMatch,String> {
        if self.pos >= self.tokens.len() {
            return Err(String::from("EOF"))
        }
        let token: &TokenMatch = &self.tokens[self.pos];
        self.pos += 1;
        Ok(token)
    }
}
