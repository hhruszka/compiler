use std::fmt;

#[derive(Debug)]
pub enum Node {
    Program(Vec<Node>),
    FunctionDefinition{name: Box<Node>, body: Box<Node>},
    Statement(String,Vec<Node>),
    Identifier(String),
    Exp(String,Box<Node>),
    IntLiteral(i32)
    // Unknown
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = f.width().unwrap_or(0);
        let indent_str = "  ".repeat(indent);
        let next_indent = indent + 1;

        match self {
            Node::Program(nodes) => {
                writeln!(f, "{}Program(", indent_str)?;
                for node in nodes {
                    write!(f, "{:width$}", node, width = next_indent)?;
                }
                writeln!(f, "{})", indent_str)
            },
            Node::FunctionDefinition { name, body } => {
                writeln!(f, "{}Function(", indent_str)?;
                write!(f, "{:width$}", name, width = next_indent)?;
                write!(f, "{:width$}", body, width = next_indent)?;
                writeln!(f, "{})", indent_str)
            },
            Node::Statement(statement, nodes) => {
                writeln!(f, "{}{}(", indent_str, statement)?;
                for node in nodes {
                    write!(f, "{:width$}", node, width = next_indent)?;
                }
                writeln!(f, "{})", indent_str)
            },
            Node::Identifier(identifier) => {
                writeln!(f, "{}Identifier({})", indent_str, identifier)
            },
            Node::Exp(exp_type, exp) => {
                writeln!(f, "{}{}(", indent_str, exp_type)?;
                write!(f, "{:width$}", exp, width = next_indent)?;
                writeln!(f, "{})", indent_str)
            },
            Node::IntLiteral(i) => {
                writeln!(f, "{}IntLiteral({})", indent_str, i)
            },
        }
    }
}
