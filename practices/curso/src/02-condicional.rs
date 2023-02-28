fn main() {

    println!("por favor int tu edad:");
    let mut year:String = String::new();
    std::io::stdin().read_line(&mut year).unwrap();
    let year_int:u8 = year.trim().parse().unwrap();

    if year_int >= 18 {
        println!("eres mayor de edad");
    } else {
        println!("eres menor de edad");
    }
}
