use crate::lexer::token_match::TokenMatch;
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
        Ok(Node::Program(nodes))
    }

    pub fn parse_statement(&mut self) -> Result<Node, String> {
        let mut nodes: Vec<Node> = Vec::new();
        self.expect("return")?;
        nodes.push(self.parse_exp()?);
        self.expect(";")?;
        Ok(Node::Statement("Return".to_string(),nodes))
    }

    pub fn parse_exp(&mut self) -> Result<Node, String> {
        // let mut nodes: Vec<Node> = Vec::new();
        let node = self.parse_int()?;
        Ok(Node::Exp("Const".to_string(),node.into()))
    }

    pub fn parse_int(&mut self) -> Result<Node, String> {
        match self.take_token()?.parse::<i32>() {
            Ok(i) =>  Ok(Node::IntLiteral(i)),
            Err(err) =>  Err(format!("Syntax error: {}", err.to_string())),
        }
    }

    pub fn parse_identifier(&mut self) -> Result<Node, String> {
        let token = self.take_token()?;
        Ok(Node::Identifier(token))
    }

    pub fn parse_function_definition(&mut self) -> Result<Vec<Node>, String> {
        let mut nodes: Vec<Node> = Vec::new();
        self.expect("int")?;
        let name = self.parse_identifier()?;
        self.expect("(")?;
        self.expect("void")?;
        self.expect(")")?;
        self.expect("{")?;
        let body = self.parse_statement()?;
        self.expect("}")?;
        nodes.push(Node::FunctionDefinition{name: name.into(),body: body.into()});
        Ok(nodes)
    }

    fn expect(&mut self, token: &str) -> Result<(), String> {
        let actual = self.take_token()?;
        if actual != token {
            return Err(format!("Syntax error: expected {}, got {}", token, actual));
        }
        Ok(())
    }

    fn take_token(&mut self) -> Result<String,String> {
        if self.pos >= self.tokens.len() {
            return Err(String::from("EOF"))
        }
        let token = self.tokens[self.pos].to_string();
        self.pos += 1;
        Ok(token)
    }
}
