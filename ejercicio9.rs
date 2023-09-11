use std::io;

fn main() {
    println!("Ingrese Hora actual en formato 12Hrs: ");
    let mut hora_actual = String::new();
    io::stdin().read_line(&mut hora_actual).unwrap();
    let hora_actual: u16 = hora_actual.trim().parse().unwrap();

    println!("Cantidad de horas que van a transcurrir:");
    let mut cantidad_horas = String::new();
    io::stdin().read_line(&mut cantidad_horas).unwrap();
    let cantidad_horas: u16 = cantidad_horas.trim().parse().unwrap();

    let hora_final = (hora_actual + cantidad_horas) % 12;
    println!("Hora actual: {}", hora_actual);
    println!("Cantidad de horas a transcurrir: {}", cantidad_horas);
    println!("En {} hora/s, el reloj marcará las {}", cantidad_horas, hora_final);
}
