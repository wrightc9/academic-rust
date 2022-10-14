#[derive(Debug)]
#[derive(PartialEq)]

pub enum Token
{
	ID(String), 
	NUM(i32), 
	SEMICOLON, 
	COLON,
	COMMA, 
	PERIOD, 
	LPAREN, 
	RPAREN, 
	ASSIGN,
	DEFINITIONS, 
	OPERATIONS, 
	POINT, 
	CIRCLE,
	SQUARE, 
	PRINT, 
	CONTAINED, 
	INTERSECTS,
	END,
	NONE
}

