const PI: f64 = 3.14;

trait Forma {
    fn area(&self) -> f64;
}

struct Circulo{
    raio: f64
}

struct Retangulo{
    largura: f64,
    altura: f64
}

impl Forma for Circulo {
    fn area(&self) -> f64 {
        PI*(self.raio*self.raio)
    }
}

impl Forma for Retangulo {
    fn area(&self) -> f64 {
        self.altura*self.largura
    }
}

fn mostrar_areas(formas: Vec<&dyn Forma>){
    for forma in formas{
        println!("{:.2}", forma.area());
    }
}


fn main(){
    let c1 = Circulo{
        raio: 3.56
    };

    let r1 = Retangulo{
        largura: 3.2,
        altura: 2.0
    };

    let c2 = Circulo{
        raio: 2.1
    };

    let r2 = Retangulo{
        largura: 5.0,
        altura: 4.0
    };

    let formas: Vec<&dyn Forma> = vec![&c1, &r1, &c2, &r2];

    mostrar_areas(formas);
}