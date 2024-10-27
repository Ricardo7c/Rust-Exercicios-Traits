trait Sensor{
    fn ler_dado(&self) -> String;
}

struct SensorTemperatura{
    temperatura: f64
}
struct SensorUmidade{
    umidade: &'static str
}

impl Sensor for SensorTemperatura {
    fn ler_dado(&self) -> String {
        format!("Temperatura: {}", self.temperatura)
    }
}

impl Sensor for SensorUmidade {
    fn ler_dado(&self) -> String {
        format!("Umidade: {}", self.umidade)
    }
}

fn coletar_dados(sensores: Vec<&dyn Sensor>){
    for sensor in sensores{
        println!("{}", sensor.ler_dado())
    }
}


fn main(){
    let t1 = SensorTemperatura{
        temperatura: 23.6
    };
    let t2 = SensorTemperatura{
        temperatura: 32.0
    };
    let u1 = SensorUmidade{
        umidade: "66%"
    };
    let u2 = SensorUmidade{
        umidade: "70%"
    };

    let sensores: Vec<&dyn Sensor> = vec![&t1, &u1, &t2, &u2];

    coletar_dados(sensores);

}