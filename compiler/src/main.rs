#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "lang.pest"] // Path to your .pest file
struct MyParser;

fn main() {
    // let input = "6 * 4 + 4 * ( 2 * 4 );"; // This works
    let input = "int x = 4;";
    let parse_result = MyParser::parse(Rule::program, input);

    match parse_result {
        Ok(parsed) => println!("Parse successful: {:?}", parsed),
        Err(error) => println!("Parse error: {:?}", error),
    }
}
