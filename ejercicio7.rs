fn main() {
    let mut texto = String::new();
    println!("Ingrese su texto:");
    std::io::stdin().read_line(&mut texto).unwrap();
    
    let primera_letra = texto.chars().next().unwrap();
    println!("la primera letra de su texto es: {}", primera_letra);
  }
