use std::{fmt::format, io::Read};

struct PilhaString {
    size:i32,
    letras: Vec<char>,
}

fn pilha_string_push(pilha: &mut PilhaString, c:char) {
    pilha.size += 1;
    pilha.letras.push(c);
}

fn pilha_string_pop(pilha: &mut PilhaString) -> char {
    if pilha.size > 0 {
        pilha.size -= 1;
        pilha.letras.pop().unwrap()
    }
    else {'!'}
}

fn nova_pilha_string() -> PilhaString {
    PilhaString { size: 0, letras: Vec::new() }
}

fn montar_pilha_string(s: String) -> PilhaString {
    let mut resp = nova_pilha_string();
    for c in s.chars(){
        pilha_string_push(&mut resp, c);
    }
    resp
}

fn inverte_string_pilha(s: String) -> String {
    let mut pilha = montar_pilha_string(s);
    let mut resp = String::new();
    for i in 0..pilha.size{
        let c = pilha_string_pop(&mut pilha);
        resp.push(c);
    }
    resp
}

fn main() {
    println!("Digite uma String para inverter usando pilha:");
    let mut entrada = String::new();
    std::io::stdin().read_line(&mut entrada).expect("Erro");
    let invertida = inverte_string_pilha(entrada);
    println!("String invertida:{}", invertida);
}
