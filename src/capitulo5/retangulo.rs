pub fn calcular_area() {
    let rect1 = Retangulo {
        length: 50,
        width: 30,
    };

    println!("A area do retangulo e de {} pixels", area(&rect1));
}

fn area(retangulo: &Retangulo) -> u32 {
    retangulo.width * retangulo.length
}

struct Retangulo {
    length: u32,
    width: u32,
}
