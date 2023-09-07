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
    let texto = String::from("SaLuDOs");
    quitar_vocales(texto);
}
