use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "lang.pest"] // Path to .pest file
struct MyParser;

#[derive(Debug)]
pub enum Expr {
    AssgmtExpr(Box<AssgmtExpr>),
    BinaryExpr(Box<BinaryExpr>),
    UnaryExpr(Box<UnaryExpr>),
    Terms(Term),
}

#[derive(Debug)]
pub struct AssgmtExpr {
    pub ident: String,
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: String,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub unary_op: String,
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub enum Term {
    Integer(i64), 
    Decimal(f64), 
    Ident(String), 
    Expr(Box<Expr>),
}

#[derive(Debug)]
pub struct Declaration {
    pub type_spec: String,
    pub ident: String,
    pub expr: Option<Box<Expr>>,
}

#[derive(Debug)]
pub enum Stmt {
    Declaration(Declaration),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

impl FromPairs for Program {
    fn from_pairs(pairs: pest::iterators::Pairs<Rule>) -> Self {
        // AST Implementation...
    }

}


fn main() {
    let input = "6 * 4 + 4 * ( 2 * 4 );"; // This works
    // let input = "int x = 4;";
    // let parse_result = MyParser::parse(Rule::program, input);

    // match parse_result {
    //     Ok(parsed) => println!("Parse successful: {:?}", parsed),
    //     Err(error) => println!("Parse error: {:?}", error),
    // }

    let pairs = MyParser::parse(Rule::program, input).unwrap(); 
    let program = Program::from_pairs(pairs); 
    
    println!("{:#?}", program);
}
