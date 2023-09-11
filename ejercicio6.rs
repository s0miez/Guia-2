fn main() {
    let mut num1 = String::new();
    println!("Ingrese el primer número: ");
    std::io::stdin().read_line(&mut num1).unwrap();
    let num1: i16 = num1.trim().parse().unwrap();

    let mut num2 = String::new();
    println!("Ingrese el segundo número: ");
    std::io::stdin().read_line(&mut num2).unwrap();
    let num2: i16 = num2.trim().parse().unwrap();
      
    if num1 > num2 {
        println!("el número mayor es: {}", num1);
    } else if num2 > num1 {
        println!("el número menor es: {}", num2);
    } else {
        println!("los números son iguales");
    }
}
