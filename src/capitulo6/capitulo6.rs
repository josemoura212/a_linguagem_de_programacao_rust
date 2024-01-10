use super::option::option;

#[allow(unused)]
// enum VersaoIp {
//     V4,
//     V6,
// }
#[derive(Debug)]
enum EnderecoIp {
    // ?V4(String),
    // ?V6(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

// !struct EnderecoIp {
// !    versao: VersaoIp,
// !    endereco: String,
// !}

// !fn rotear(versao_ip: &VersaoIp) {}

pub fn capitulo6() {
    // !let quatro = VersaoIp::V4;
    // !let seis = VersaoIp::V6;

    // !rotear(&quatro);
    // !rotear(&seis);

    // !let local = EnderecoIp {
    // !    versao: quatro,
    // !    endereco: String::from("127.0.0.1"),
    // !};

    // !let loopback = EnderecoIp {
    // !    versao: seis,
    // !    endereco: String::from("::1"),
    // !};

    // ?let local = EnderecoIp::V4(String::from("127.0.0.1"));
    // ?let loopback = EnderecoIp::V6(String::from("::1"));
    let local = EnderecoIp::V4(127, 0, 0, 1);
    let _loopback = EnderecoIp::V6(String::from("::1"));

    local.invocar();
    option();
}

impl EnderecoIp {
    fn invocar(&self) {
        println!("{:?}", &self);
    }
}
