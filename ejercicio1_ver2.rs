use std::io::stdin;

fn texto_entrante(texto: String) -> () {
    println!("Ingrese su texto: ");
    let mut texto_usuario: String = String::new();
    stdin().read_line(&mut texto_usuario).expect_err("Porfavor, ingresa Caracteres Válidos.");
}

fn quitar_vocales(texto: String) -> () { 
    let texto: String = texto.to_lowercase();
    let mut texto_sinvocal = String::new(); 
    for i in texto.chars() {
        if i != 'a' && i != 'e' && i != 'i' && i != 'o' && i != 'u' {
            texto_sinvocal.push(i);
        }        
    }
    println!("{}", texto_sinvocal);
}

fn main() {
    let texto = texto_entrante();
    let vocales_eliminadas = quitar_vocales(&texto);
    println!("Su texto sin vocales es: {}", vocales_eliminadas);
}

//ta too malo saludos (mentira)
