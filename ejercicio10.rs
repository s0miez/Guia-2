use::std::stdin;

fn anio_ingresado() {
    let mut anio:String = String::new();
    println!("Ingrese un año: ");
    let stdin = io::stdin();
    stdin.read_line(&mut anio).unwrap();
    let anio: u8 = anio.trim().parse().unwrap();
}

fn bisiesto_antiguo() {
    let ab1: &str = "es bisiesto";
    let nb1: &str = "no es bisiesto";
    if anio_antiguo % 40 = 0 {
        return ab1
    } else {
        return nb1
    }
}

fn busiesto_actual() {
    let ab2: &str = "es bisiesto";
    let nb2: &str = "no es bisiesto";
    if anio_actual % 4 == 0 && (anio_actual % 100 != 0 || anio_actual % 400 == 0) {
        return ab2
    } else {
        return nb2
    }
}

fn main() {

}

//ta super malo y nose pq 
