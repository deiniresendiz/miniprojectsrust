fn main() {

    let numero_1 = 123;
    let numero_2 = 123;

    let suma = numero_1 + numero_2;

    loop{
        println!("CuaL ES LA SUMA de estos numeros {} y {}", numero_1, numero_2);
        let mut suma_user:String = String::new();
        std::io::stdin().read_line(&mut suma_user).unwrap();
        let suma_user_int:u32 = suma_user.trim().parse().unwrap();

        if suma_user_int == suma {
            println!("La suma es correcta");
            break;
        } else {
            println!("la suma es incorrecta");
        }
    }
    
}
