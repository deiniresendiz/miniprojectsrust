use regex::Regex;

fn main() {

    print!("Calculdora")

    //regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();

    // taer datos user
    print!("Ingrese una operacion: ");
    let mut expression = String::new();
    io::stdin().read_line(&mut expression).unwrap();

    // applid operations
    let caps = re_add.captures(&expression.as_str()).unwrap();
    let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

    let addition = left_value + right_value;
    
}
