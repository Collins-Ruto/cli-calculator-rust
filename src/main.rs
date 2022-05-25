use std::env::args;

fn main() {
    println!("Welcome to Rust CLI calculator");
    println!("Use x, X for multiplication ");

    let mut args = args().skip(1);

    let first_val =  args.next().unwrap();
    let operand =  args.next().unwrap().chars().next().unwrap();
    let second_val =  args.next().unwrap();

    let first = first_val.parse::<f32>().unwrap();
    let second = second_val.parse::<f32>().unwrap();
    
    print!("computation of: {} {} {} gives: ", first_val, operand, second_val);

    fn compute(first: f32, second: f32, operand: char) -> f32 {
        if operand == '+' {
            first + second
        } else if operand == '-' {
            first - second
        } else if operand == '/' {
            first / second
        } else if operand == '*' {
            first * second
        } else {
            0.0
        }
    }

    fn compute_match(first: f32, second: f32, operand: char) -> f32 {
        match operand {
            '+' => first + second,
            '-' => first - second,
            '/' => first / second,
            '*' | 'x' | 'X' => first * second,
            _ => panic!("Invalid operand")
        }
        
    }

    let calc = compute(first, second, operand);
    let calc2 = compute_match(first, second , operand);

    println!("{} or {} for match", calc, calc2);

}
