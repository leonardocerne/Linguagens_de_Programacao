// Função para retornar a prioridade das operações em valor numérico
fn prioridade(s:&String) -> i32 {
    if s == "(" {0}
    else if s == "+" || s == "-" {1}
    else if s == "*" || s == "/" {2}
    else {3}
}

// Função que transforma expressão infixada em pós fixada
fn transforma(s:String) -> String {
    let p1 = vec!["+", "-"];
    let p2 = vec!["*", "/"];
    let p3 = "^";
    let mut pilhaaux:Vec<String> = Vec::new();
    let mut expressao = Vec::new();
    let mut resposta = String::new();
    for c in s.split_whitespace(){
        // Se é um operando
        if let Ok(num) = c.parse::<i32>(){
            expressao.push(String::from(c));
        }
        // Se não é um operando
        else {
            if c == "(" {
                pilhaaux.push(String::from(c));
            }
            else if c == p3{
                while let Some(topo) = pilhaaux.last(){
                    let topo = topo.clone();
                    if topo == p3 {
                        expressao.push(pilhaaux.pop().unwrap());
                    }
                    else {break}
                }
                pilhaaux.push(String::from(c));
            }
            else if p2.contains(&c) {
                while let Some(op) = pilhaaux.last() {
                    if prioridade(op) >= 2 {
                        expressao.push(pilhaaux.pop().unwrap());
                    }
                    else {
                        break;
                    }
                }
                pilhaaux.push(String::from(c));
            }
            else if p1.contains(&c) {
                while let Some(op) = pilhaaux.last() {
                    if prioridade(op) >= 1 {
                        expressao.push(pilhaaux.pop().unwrap());
                    }
                    else {
                        break;
                    }
                }
                pilhaaux.push(String::from(c));
            }
            else if c == ")" {
                while let Some(op) = pilhaaux.pop() {
                    if op != "(" {
                        expressao.push(op);
                    } else {
                        break;
                    }
                }
            }
        }
    }
    // Esvaziando a pilha
    while let Some(op) = pilhaaux.pop() {
        expressao.push(op);
    }
    // Montando a string
    for i in expressao {
        resposta.push_str(i.as_str());
        resposta.push(' ');
    }
    resposta
}

fn calcula(s:String) -> i32 {
    let mut pilha:Vec<i32> = Vec::new();
    let mut a:i32;
    let mut b:i32;
    for c in s.split_whitespace() {
        // Movendo operandos
        if let Ok(num) = c.parse::<i32>() {
            pilha.push(num);
        }
        // Fazendo operações
        else {
            a = match pilha.pop() {
                Some(valor) => valor,
                None => {0} 
            };
            b = match pilha.pop() {
                Some(valor) => valor,
                None => {0} 
            };
            if c == "+" {
                pilha.push(a + b);
            }
            else if c == "-" {
                pilha.push(b - a);
            }
            else if c == "*" {
                pilha.push(b * a);
            }
            else if c == "/" {
                pilha.push(b / a);
            }
            else if c == "^" {
                pilha.push(b.pow(a as u32));
            }
        }
    }
    // Resgatando resultados
    let x = match pilha.pop() {
        Some(valor) => valor,
        None => {0}
    };
    x
}

fn main() {
    println!("_______________________________________________________________________________________________________________________
    \nInsira a expressão que quer transformar em pós fixada, separada por espaços em branco (ex: ( 22 + 33 ) * 5
    \n_______________________________________________________________________________________________________________________");
    let mut entrada = String::new();
    std::io::stdin().read_line(&mut entrada).expect("Erro");
    let resp = transforma(entrada);
    let resp_clone = resp.clone();
    let x = calcula(resp);
    println!("Expressão = [{}], Resultado = {}", resp_clone, x);
}
