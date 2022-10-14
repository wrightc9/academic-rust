mod lexer;
mod tokens;
mod parser;

use std::env;

use crate::lexer::Lexer;
use crate:: parser::Parser;
use crate::tokens::Token;

fn main() 
{

	// bool variable to check if acceptable flag is found
	// Acceptable arguments: '-s', '-p'

	let (mut in_found, mut s_found, mut p_found) = (false, false, false);
	let mut input_file : String = String::from("");

	// parsing and identifying arguments
	for argument in env::args()
	{
		// take a input file
		if argument.contains(".sc")
		{
			in_found = true;
			input_file = argument.clone();
		}

		// check for flags
		if argument == "-s"
		{
			s_found = true;
		}
		else if argument == "-p"
		{
			p_found = true;
		}
	}

	// error out if no valid input file found (must be a .sc file)
	if !in_found
	{
		panic!("No valid input file found.");
	}

	// error out if no appropriate flag is found
	if !s_found && !p_found
	{
		panic!("No valid argument found.");
	}

	// creates new lexer named file
	let mut file = Lexer::new(String::from(&input_file));

	// runs lexer
	file.start_lexer();

	// gets token vector generated from lexer
	let tokens: &Vec<Token> = file.get_tokens();

	// schema output
	if s_found
	{
		println!("; processing input file {}", input_file);
	}
	else // prolog output
	{
		println!("/* processing input file {}", input_file)
	}

	// creates parser and tells it schema or prolog
	let mut parser: Parser = Parser::new(tokens, s_found);
	
	// runs parser
	parser.start_parser();

}
