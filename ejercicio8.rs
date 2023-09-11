fn main() {
    let mut texto = String::new();
    println!("Ingrese su texto: ");
    std::io::stdin().read_line(&mut texto).unwrap();

    let ultima_letra: Option<char> = texto.chars().last();
    let ultima_letra = ultima_letra.unwrap_or(' ');
    println!("la última letra de su texto es: {}",ultima_letra);
}
