fn cuenta_vocales(texto: &str, letra: char) -> i32{
    let mut contador = 0;

    for caracter in texto.chars(){
        if caracter == letra{
            contador = contador + 1
        }
    }

    return contador
}

fn main(){

    let cadena_texto = String::from("Hola Mundo cruel");
    let letra = 'a' && 'e' && 'i' && 'o' && 'u';
    let numero = cuenta_vocales(&cadena_texto, letra);
    println!("{} tiene {} vocales", cadena_texto, numero);

}
