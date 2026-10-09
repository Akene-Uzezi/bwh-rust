use bwh_rust::{lexer::Lexer, token};

fn main() {
    let input = "
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            }
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            10 == 10;
            10 != 9;
        ";
    let mut lexer = Lexer::new(input.as_bytes().to_vec());

    loop {
        let tok = lexer.next_token();
        if tok.token_type == token::EOF {
            break;
        }
        println!("{:?}\t{}", tok.token_type, tok.token_literal);
    }
}
