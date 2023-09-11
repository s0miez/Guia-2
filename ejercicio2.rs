fn coincidencias(texto: &str, letra: char) -> i32{
    let mut contador = 0;

    for caracter in texto.chars(){
        if caracter == letra{
            contador = contador + 1
        }
    }

    return contador
}

fn main(){

    let input = String::from("Tengo Hambre, Buenos días");
    let letra = 'e';
    let numero = coincidencias( &input, letra);
    println!("El texto ingresado tiene {} veces la letra {}", numero, letra);
}
