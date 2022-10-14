use std::fs;
use regex::Regex;
use crate::tokens::Token;

pub struct Lexer 
{
	_filename: String,
	_file: String,
	_tokens: Vec<Token>
}

impl Lexer
{
	// constructor
	pub fn new(filename: String) -> Lexer
	{
		// creates the struct and sets default values to
		// variables: _filename and _file
		let mut l = Lexer
		{
			_filename: filename,
			_file: String::from(""),
			_tokens: Vec::new()
		};
		
		// calls read_to_string
		l.read_to_string();
		
		return l;
	}
	
	// returns token vector
	pub fn get_tokens(&self) -> &Vec<Token>
	{
		return &self._tokens;
	}

	// analyzes the string and creates tokens
	pub fn start_lexer(& mut self)
	{
		// sets empty string temp_lex
		let temp_lex: &mut String = &mut String::from("");
	
		// range based look for each char in string _files
		for s in self._file.chars()
		{
			// removes whitespace
			if s.is_whitespace() {
				continue;
			}
			// if s is a letter or a number then it puts it into the temp_lex string 
			// and goes to the next iteration of the loop
			if s.is_alphanumeric()
			{
				temp_lex.push(s.clone());
				continue;
			}
			
			if temp_lex != ""
			{
				let tk : Token = self.tokenizer(temp_lex);
				self._tokens.push(tk);
			}
			
			// gets token and pushes it into the vector
			let tk : Token = self.tokenizer(&s.to_string());
			self._tokens.push(tk);
			temp_lex.clear();
		}
	}
	
	// reads file and puts it into a string
	// variable: _file
	fn read_to_string(& mut self)
	{
		self._file = fs::read_to_string(self._filename.clone()).expect("Failed to open file.");
	}

	// 
	fn tokenizer(&self, temp: &String) -> Token
	{
		// checks for keywords in the string temp, creates a token for it and returns if it finds one
		let temp_token = match temp.as_str()
		{
			"definitions" => Token::DEFINITIONS,
			"operations" => Token::OPERATIONS,
			"point" => Token::POINT,
			"circle" => Token::CIRCLE,
			"square" => Token::SQUARE,
			"print" => Token::PRINT,
			"contained" => Token::CONTAINED,
			"intersects" => Token::INTERSECTS,
			"end" => Token::END,
			"=" => Token::ASSIGN,
			")" => Token::RPAREN,
			"(" => Token::LPAREN,
			"." => Token::PERIOD,
			"," => Token::COMMA,
			":" => Token::COLON,
			";" => Token::SEMICOLON,
			_=> Token::NONE
		};

		// returns the token if a keyword was found and a token was created
		if temp_token != Token::NONE
		{
			return temp_token;
		}

		/*
		Creates 3 Regex for checking the string to determine if it is a valid ID or NUM or neither
		1. checks for any caps in the string
		2. checks if the whole string is lowercase letters
		3. checks if the whole string is numbers
		NOTE: 
			this makes sure it fits in our grammer because we do not accept mixed letters and numbers
		*/
		let caps = Regex::new(r"[A-Z]").unwrap();
		let alpha = Regex::new(r"^[a-z]+$").unwrap();
		let number = Regex::new(r"^[0-9]+$").unwrap();

		if caps.is_match(&temp)
		{
			panic!("Lexical Error: {}", temp);
		}

		// performs regex checks
		if alpha.is_match(&temp) 
		{
			return Token::ID(String::from(temp));
		}
		else if number.is_match(&temp)
		{
			return Token::NUM(temp.parse().unwrap());
		}

		panic!("Lexical Error: {}", temp);
	}
	
}