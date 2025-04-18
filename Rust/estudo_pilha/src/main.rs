struct Pilha{
    size: i32,
    elements: Vec<i32>,
}

fn nova_pilha() -> Pilha{
    Pilha{
        size: 0,
        elements: Vec::new(),
    }
}

fn pilha_push(pilha: &mut Pilha, x:i32){
    pilha.size += 1;
    pilha.elements.push(x);
}

fn pilha_pop(pilha: &mut Pilha) -> i32{
    if pilha.size > 0 {
        pilha.size -= 1;
        let x = pilha.elements.pop().unwrap();
        x
    } else{
        panic!("Pilha vazia");
    }
}

fn pilha_len(pilha : &mut Pilha) -> i32{
    pilha.size
}

fn pilha_imprime(pilha: &mut Pilha){
    let mut aux = nova_pilha();
    let mut saida = String::from("Pilha = {");
    let mut x:i32;
    let tmp = pilha.size;
    for i in 0..pilha.size{
        x = pilha_pop(pilha);
        let mut a = x.to_string();
        if i != tmp - 1{
            a.push_str(", ");
        }
        saida.push_str(&a);
        pilha_push(&mut aux, x);
    }
    for j in 0..aux.size{
        x = pilha_pop(&mut aux);
        pilha_push(pilha, x);
    }
    saida.push_str("}");
    println!("{}", saida);
}

fn main(){
    let mut pilha = nova_pilha();
    let mut entrada = String::new();
    println!("Digite os números para a pilha (digite 'sair' para encerrar):");
    loop {
        entrada.clear();
        std::io::stdin().read_line(&mut entrada).expect("Erro ao ler a entrada");
        let entrada = entrada.trim();
        if entrada == "sair" {
            break;
        }
        let numero:i32 = entrada.trim().parse().expect("Erro ao converter para inteiro");
        pilha_push(&mut pilha, numero);
    }
    pilha_imprime(&mut pilha);
    println!("Tamanho da pilha: {}", pilha_len(&mut pilha));
}