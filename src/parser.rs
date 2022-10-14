use crate::tokens::Token;
use std::collections::HashMap;

// creates a symbol struct
// Purpose stores info about variables proved. 
// ex symbol will have a type of circle, square, or point depending on what user chooses
// lhs and rhs will be what the user put between paren seperated by comma
pub struct Symbol
{
    pub _type: String,
    pub _lhs: String,
    pub _rhs: String
}

impl Symbol
{
    pub fn new() -> Symbol
    {
        // constructor
        // initializes variables
        let f: Symbol = Symbol
        {
            _type: "".to_string(),
            _lhs: "".to_string(),
            _rhs: "".to_string()
        };

        return f;
    }
}

pub struct Parser <'a>
{
    _next: usize,
	_tokens: &'a Vec<Token>,
    _s: bool,
    _token_table: HashMap<String, Symbol>,
    _output_string: String
}

impl Parser <'_>
{
    // constructor
	pub fn new(tokens: &Vec<Token>, s: bool) -> Parser
    {
        // creates the struct and sets default values to
		// variables: _next and _tokens
        let p = Parser
        {
            _next: 0,
            _tokens: tokens,
            _s: s,
            _token_table: HashMap::new(),
            _output_string: String::from("")
        };

        return p;
    }

    // starts the syntax analysis using recursive descent
    pub fn start_parser(&mut self)
    {
        self.program();
    }

    /*  
    starts to run through the vector of tokens and determines if the order is correct
    uses self._next += 1; to iterate to the next token in the vector
    calls check_next function and passes in the token it expects it to be, and gets a bool
    back. True if it is - false if it isn't
    */
    fn program(&mut self)
    {
        // checks for definitions keyword and colon then goes into def function
        if !self.check_next(Token::DEFINITIONS)
        {
            panic!("Syntax Error: Expected DEFINITIONS, Got {:?}", self._tokens.get(self._next).unwrap());
        }
        self._next += 1;

        if !self.check_next(Token::COLON)
        {
            panic!("Syntax Error: Expected COLON, Got {:?}", self._tokens.get(self._next).unwrap());
        }
        self._next += 1;

        self.defs();

        // checks for operations keyword and colon then goes into operations function
        if !self.check_next(Token::OPERATIONS)
        {
            panic!("Syntax Error: Expected OPERATIONS, Got {:?}", self._tokens.get(self._next).unwrap());
        }
        self._next += 1;

        if !self.check_next(Token::COLON)
        {
            panic!("Syntax Error: Expected COLON, Got {:?}", self._tokens.get(self._next).unwrap());
        }
        self._next += 1;

        self.operations();

        // checks if the program is ended with the correct grammer
        // expected: end.
        if !self.check_next(Token::END)
        {
            panic!("Syntax Error: ");
        }
        self._next += 1;

        if !self.check_next(Token::PERIOD)
        {
            panic!("Syntax Error: ")
        }

        // print schema or prolog outut
        // _output_string is filled during operations function
        if self._s
        {
            println!("; Lexical and Syntax analysis passed");
        }
        else 
        {
            println!("   Lexical and Syntax analysis passed */");
        }
        print!("{}", self._output_string);
        
        if !self._s
        {
            print!("writeln(T) :- write(T), nl.\
                \nmain:- forall(query(Q), Q-> (writeln(‘yes’)) ; (writeln(‘no’))),\
                \n      halt.\n");
        }

    }

    // calls def for each definition seperated by semicolon
    fn defs(&mut self)
    {
        self.def();

        // if there is a semicolon, runs defs again
        if self.check_next(Token::SEMICOLON)
        {
            self._next += 1;
            self.defs();
        }
    }

    /* 
    checks to see if everything in the definitions section is expected.
    this section consists of assignment statements like:
        a = point(3,3);
        d = point(9,4);
        x = circle(a,1);
        y = square(d,2)

    also saves id and the lhs and rhs of what the id is storing into and object 
    and inserts it into a hashmap
    */
    fn def(&mut self)
    {
        // Creates symbol object for token table
        let mut symbol: Symbol = Symbol::new();
        let mut key: &String = &String::from("");

        // if statements to check if order is correct: ID -> '=' -> POINT || CIRCLE || SQUARE
        if !self.check_next(Token::ID("".to_string()))
        {
            panic!("Syntax Error: Expected ID, Got {:?}", self._tokens.get(self._next).unwrap());
        }
        

        // get value from token and put it in symbol obj
        if let Token::ID(value) = self._tokens.get(self._next).unwrap() {
            key = value;
        }


        self._next += 1;

        if !self.check_next(Token::ASSIGN)
        {
            panic!("Syntax Error: Expected ASSIGN, Got {:?}", self._tokens.get(self._next).unwrap());
        }
        self._next += 1;

        if !self.check_next(Token::POINT) && !self.check_next(Token::CIRCLE) && !self.check_next(Token::SQUARE)
        {
            panic!("Syntax Error: Expected POINT, Got {:?}", self._tokens.get(self._next).unwrap());
        }

        // checks if the token at _next is a point, if it is it sets x to bool
        let x: bool = self.check_next(Token::POINT);
        let y: bool = self.check_next(Token::CIRCLE);

        // sets _type of symbol based on what is created
        if x
        {
            symbol._type = String::from("point");
        }
        else if y
        {
            symbol._type = String::from("circle");
        }
        else
        {
            symbol._type = String::from("square");
        }
        self._next += 1;

        // checks for left parenthesis
        if !self.check_next(Token::LPAREN)
        {
            panic!("Syntax Error: Expected LPAREN, Got {:?}", self._tokens.get(self._next).unwrap());
        }
        self._next += 1;

        // if token at _next is a POINT then it will expect format to be (NUM, NUM)
        // if it is a CIRCLE or a SQUARE it will expect format to be (ID, NUM)
        if x
        {
            if !self.check_next(Token::NUM(0))
            {
                panic!("Syntax Error: Expected NUM, Got {:?}", self._tokens.get(self._next).unwrap());
            }

            // gets value from num enum and stores it in lhs
            if let Token::NUM(value) = self._tokens.get(self._next).unwrap() {
                let lhs = &value.to_string();
                symbol._lhs = lhs.to_string();
            }
        }
        else
        {
            if !self.check_next(Token::ID("".to_string()))
            {
                panic!("Syntax Error: Expected ID, Got {:?}", self._tokens.get(self._next).unwrap());
            }

            // gets value from ID emum and stores it in lhs
            if let Token::ID(value) = self._tokens.get(self._next).unwrap() {
                let lhs = value;
                if self._token_table.contains_key(&lhs.to_string())
                {
                    // gets type from symbol inside of hashmap token table
                    let symbol_type = String::from(self._token_table.get_key_value(lhs).unwrap().1._type.as_str());
                    // gets left side lhs from symbol inside of hashmap token table
                    let left = String::from(self._token_table.get_key_value(lhs).unwrap().1._lhs.as_str());
                    // gets right side rhs from symbol inside of hashmap token table
                    let right = String::from(self._token_table.get_key_value(lhs).unwrap().1._rhs.as_str());

                    // if schema format
                    if self._s
                    {
                        if symbol_type == "point"
                        {
                            symbol._lhs.push_str(format!("makepoint {left} {right}").as_str());
                        }
                        else if symbol_type == "circle"
                        {
                            symbol._lhs.push_str(format!("circle ({left}) {right}").as_str());
                        }
                        else // square
                        {
                            symbol._lhs.push_str(format!("circle ({left}) {right}").as_str()); 
                        }
                    }
                    else // prolog format
                    {
                        if symbol_type == "point"
                        {
                            symbol._lhs.push_str(format!("point2d({left},{right})").as_str());
                        }
                        else if symbol_type == "circle"
                        {
                            symbol._lhs.push_str(format!("circle({left}, {right})").as_str());
                        }
                        else // square
                        {
                            symbol._lhs.push_str(format!("square({left}, {right})").as_str()); 
                        }
                    }
                }
                else 
                {
                    panic!("Error: Point does not exist");
                }
                
            }
            
        }
        self._next += 1;


        if !self.check_next(Token::COMMA)
        {
            panic!("Syntax Error: Expected COMMA, Got {:?}", self._tokens.get(self._next).unwrap());
        }
        self._next += 1;

        if !self.check_next(Token::NUM(0))
        {
            panic!("Syntax Error: Expected NUM, Got {:?}", self._tokens.get(self._next).unwrap());
        }

        // gets value from num enum and stores it in rhs
        if let Token::NUM(value) = self._tokens.get(self._next).unwrap() {
            let rhs = &value.to_string();
            symbol._rhs = rhs.to_string();
        }

        self._next += 1;

        if !self.check_next(Token::RPAREN)
        {
            panic!("Syntax Error: Expected RPAREN, Got {:?}", self._tokens.get(self._next).unwrap());
        }
        self._next += 1;

        // insert in token table
        self._token_table.insert(key.to_string(), symbol);

    }

    // calls operation on each operation seperated by a semicolon
    fn operations(&mut self)
    {
        self.operation();

        // if there is a semicolon, runs operation again
        if self.check_next(Token::SEMICOLON)
        {
            self._next += 1;
            self.operations();
        }
    }

    /* 
    checks to see if everything in the operations section is expected.
    this section consists of operational statements using print, contained
    and intersects. It should look like this:
        print(x);
        print(y);
        intersects(x,y)
    */
    fn operation(&mut self)
    {
        // checks if the token is one of the possible operations tokens
        if !self.check_next(Token::PRINT) && !self.check_next(Token::CONTAINED) && !self.check_next(Token::INTERSECTS)
        {
            panic!("Syntax Error: Expected PRINT, CONTAINED, OR INTERSECT Got {:?}", self._tokens.get(self._next).unwrap());
        }

        // checks if print
        let print: bool = self.check_next(Token::PRINT);
        let contained: bool = self.check_next(Token::CONTAINED);

        self._next += 1;

        // checks for left parenthesis then id
        if !self.check_next(Token::LPAREN)
        {
            panic!("Syntax Error: Expected LPAREN, Got {:?}", self._tokens.get(self._next).unwrap());
        }
        self._next += 1;

        if !self.check_next(Token::ID("".to_string()))
        {
            panic!("Syntax Error: Expected ID, Got {:?}", self._tokens.get(self._next).unwrap());
        }

        // type of left and right
        // right is optional so must be initialized
        let left_type;
        let mut right_type = String::from("");
        // bool to say if right exists
        let mut right_exist: bool = false;
        // left and right id
        let mut left = String::from("");
        let mut right = String::from("");

        // gets value of ID and stores it in left
        if let Token::ID(value) = self._tokens.get(self._next).unwrap() {
            left = value.to_string();
        }
        self._next += 1;

        // if operation is print, then it just checks the right parenthesis and thats it.
        // if it is either contained or intersects then it checks for a comma, second id,
        // and then right parenthesis
        if !print
        {
            if !self.check_next(Token::COMMA)
            {
                panic!("Syntax Error: Expected COMMA, Got {:?}", self._tokens.get(self._next).unwrap());
            }
            self._next += 1;

            if !self.check_next(Token::ID("".to_string()))
            {
                panic!("Syntax Error: Expected ID, Got {:?}", self._tokens.get(self._next).unwrap());
            }

            // gets value of ID and stores it in right
            if let Token::ID(value) = self._tokens.get(self._next).unwrap() {
                right = value.to_string();
            }
            right_exist = true;
            self._next += 1;
        }

        if !self.check_next(Token::RPAREN)
        {
            panic!("Syntax Error: Expected RPAREN, Got {:?}", self._tokens.get(self._next).unwrap());
        }
        self._next += 1;

        // print formatting sequel or prolog
        // else is intersect

        // gets left type from token table 
        left_type = String::from(self._token_table.get_key_value(&left).unwrap().1._type.as_str());

        // the left side lhs and right side rhs from what is stored in the symbol at left side key in token table
        let left_left = String::from(self._token_table.get_key_value(&left).unwrap().1._lhs.as_str());
        let left_right = String::from(self._token_table.get_key_value(&left).unwrap().1._rhs.as_str());

        // the left side lhs and right side rhs from what is stored in the symbol at right side key in token table
        let mut right_left = "".to_string();
        let mut right_right = "".to_string();

        // checks if we need a right hand side
        if right_exist
        {
            right_type = String::from(self._token_table.get_key_value(&right).unwrap().1._type.as_str());

            // the left side lhs and right side rhs from what is stored in the symbol at right side key in token table
            right_left = String::from(self._token_table.get_key_value(&right).unwrap().1._lhs.as_str());
            right_right = String::from(self._token_table.get_key_value(&right).unwrap().1._rhs.as_str());
        }
        

        // if it is scheme
        if self._s
        {
            if print
            {
                self._output_string.push_str(format!("(print-{left_type} ({left_left}) {left_right})\n").as_str())
            }
            else if contained
            {
                self._output_string.push_str(format!("(contained-{left_type}-{right_type} \
                    ({left_left}) {left_right} ({right_left}) {right_right})\n").as_str())
            }
            else // intersect
            {
                self._output_string.push_str(format!("(intersects-{left_type}-{right_type} \
                    ({left_left}) {left_right} ({right_left}) {right_right})\n").as_str())
            }
        }
        else // if it is prolog
        {
            if print
            {
                self._output_string.push_str(format!("query({left_left}, {left_right})\n").as_str())
            }
            else if contained
            {
                self._output_string.push_str(format!("query(contained({left_type}({left_left}, {left_right}), \
                {right_type}({right_left}, {right_right})))\n").as_str())
            }
            else //intersect
            {
                self._output_string.push_str(format!("query(intersects({left_type}({left_left}, {left_right}), \
                {right_type}({right_left}, {right_right})))\n").as_str())
            }
        }

    }

    /*
    PARAMETERS: exp_token (a token that is expected to be at _next)

    checks if the token at _next in the vector _tokens is the same as the token passed in (expected token)

    RETURNS:
        - if token at _next in the vector _tokens is same as expected token
    */
    fn check_next(&mut self, exp_token: Token) -> bool
    {


        // makes sure vector at _next is not empty
        if self._tokens.get(self._next).is_none()
        {
            println!("NO MORE TOKENS");
            return false;
        }

        if std::mem::discriminant(self._tokens.get(self._next).unwrap()) 
            == std::mem::discriminant(&exp_token)
        {
            return true;
        }

        return false;
    }
}