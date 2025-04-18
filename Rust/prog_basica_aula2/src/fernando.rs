use std::io::stdin;

pub fn fernandin() {
    println!("Insira numeros inteiros separados por espaço:");
    let mut entrada = String::new();
    stdin().read_line(&mut entrada).unwrap();
    let vetor: Vec<i32> = entrada
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
    let mut mult= 1;
    for(i, n) in vetor.iter().enumerate() {
        mult = mult * (n + i as i32);
    }
    println!("Multiplicação dos itens do vetor = {}", mult);
}

pub fn illana(a:i32, b:i32) -> i32 {
    let mut resp = 0;
    for i in 1..=a {
        resp = resp + (b * i);
    }
    resp
}