fn main() {
    let nombre: &str = "deini";
    let edad: u8 = 18;

    println!("Hola soy {} y tengo {} años", nombre, edad);

    println!("por favor int tu nombre:");
    let mut name:String = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    println!("por favor int tu edad:");
    let mut year:String = String::new();
    std::io::stdin().read_line(&mut year).unwrap();
    let year_int:u8 = year.trim().parse().unwrap();


    println!("Hola {} y edad {}", name, year_int);
}
