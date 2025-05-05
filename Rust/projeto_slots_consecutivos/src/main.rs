use std::{io::stdin, fmt::Display};


// Enumerador para os dias da semana
#[derive(Debug, Clone)]
enum DiaDaSemana {
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
    Domingo,
}

// Estrutura de slot, contendo dia da semana, inicio e fim do turno em minutos
#[derive(Debug, Clone)]
struct Slot {
    dia: DiaDaSemana,
    inicio: u32,
    fim: u32,
}

// Estrutura do desenvolvedor, que contem o nome do desenvolvedor e seus slots trabalhados
#[derive(Debug, Clone)]
struct Dev{
    nome: String,
    lista: Vec<Slot>,
}

impl<'a> Display for Dev {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nome do desenvolvedor: {}", self.nome)
    }
}

impl Dev {
    fn novo() -> Dev {
        Dev {
            nome: String::new(),
            lista: Vec::new()
        }
    }
}

impl Slot {
    fn diaparanumero(&self) -> u32 {
        match self.dia {
            DiaDaSemana::Segunda => 0,
            DiaDaSemana::Terca => 1,
            DiaDaSemana::Quarta => 2,
            DiaDaSemana::Quinta => 3,
            DiaDaSemana::Sexta => 4,
            DiaDaSemana::Sabado => 5,
            DiaDaSemana::Domingo => 6,
        }
    }
}


fn retorna_dia(x: u32) -> Option<DiaDaSemana> {
    match x {
        0 => Some(DiaDaSemana::Segunda),
        1 => Some(DiaDaSemana::Terca),
        2 => Some(DiaDaSemana::Quarta),
        3 => Some(DiaDaSemana::Quinta),
        4 => Some(DiaDaSemana::Sexta),
        5 => Some(DiaDaSemana::Sabado),
        6 => Some(DiaDaSemana::Domingo),
        _ => None
    }
}

fn ordena(x: &mut Vec<Dev>) {
    for i in x.iter_mut() {
        i.lista.sort_by_key(|s| (s.diaparanumero(), s.inicio))
    }
}

fn conta_slots_consecutivos(x: &Dev) -> u32 {
    let mut cont = 1;
    let mut fimant = 0;
    let mut max_cont = 0;
    let mut dia_ant = 0;
    for i in &x.lista {
        if fimant == 0 {
            fimant = i.fim;
            dia_ant = i.diaparanumero();
        }
        else {
            if i.inicio == fimant && i.diaparanumero() == dia_ant {
                cont += 1;
                max_cont = max_cont.max(cont);
            }
            else {
                cont = 1;
            }
            fimant = i.fim;
            dia_ant = i.diaparanumero();
        }
    }
    max_cont
}

fn principal(x:&mut Vec<Dev>) -> Dev {
    let mut maior = 0;
    let mut resp: Option<Dev> = None;
    ordena(x);
    for i in x.iter(){
        let k = conta_slots_consecutivos(i);
        if k > maior {
            maior = k;
            resp = Some(i.clone())
        }
    }
    resp.expect("Desenvolvedor não encontrado")
}

fn main() {
    let mut lista:Vec<Dev> = Vec::new();
    let mut x:i32 = 0;
    while x != -1 {
        println!("Digite 1 para adicionar novo desenvolvedor, -1 para sair");
        let mut entrada = String::new();
        stdin().read_line(&mut entrada).expect("Erro");
        let numero:i32 = entrada.trim().parse().expect("Erro");
        entrada.clear();
        if numero == -1 {break;}
        else {
            let mut x = Dev::novo();
            println!("Escreva o nome do desenvolvedor");
            stdin().read_line(&mut entrada).expect("Erro");
            x.nome = entrada.clone();
            entrada.clear();
            println!("Escreva quantos slots o desenvolvedor trabalhou");
            stdin().read_line(&mut entrada).expect("Erro");
            let i = entrada.trim().parse().expect("Erro");
            entrada.clear();
            for j in 0..i {
                println!("Digite o dia da semana(0 para segunda, 1 para terça ... 6 para domingo");
                stdin().read_line(&mut entrada).expect("Erro");
                let data:u32 = entrada.trim().parse().expect("Erro");
                entrada.clear();
                println!("Digite o horário de ínicio e fim em minutos (Ex: 300 360");
                stdin().read_line(&mut entrada).expect("Erro");
                let numeros: Vec<u32> = entrada.trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();
                entrada.clear();
                if numeros.len() != 2 {
                    println!("Voce precisa digitar dois numeros");
                } else if numeros[0] > numeros[1]{
                    println!("O inicio tem que ser menor que o fim");
                } else {
                    let (a,b) = (numeros[0], numeros[1]);
                    match retorna_dia(data) {
                        Some(dia) => {
                            let novo_slot = Slot { dia, inicio: a, fim: b };
                            x.lista.push(novo_slot);
                        }
                        None => println!("Digitou dia errado")
                    }
                    
                }
            }
            lista.push(x);
        }
    }
    let resposta:Dev = principal(&mut lista);
    println!("O desenvolvedor com a maior quantidade de slots consecutivos é o {} com {} slots", resposta, conta_slots_consecutivos(&resposta));
}
