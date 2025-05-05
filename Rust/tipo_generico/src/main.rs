use std::mem;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::fmt::Display;

// TIPOS GENERICOS

fn bubble_generico<T:PartialOrd>(mut list: Vec<T>) -> Vec<T> {
    if list.len() < 2{
        return list;
    }
    let mut trocado = true;
        while trocado {
        trocado = false;
        for i in 0..(list.len() - 1) {
            if list[i] > list[i+1] {
                list.swap(i,i + 1);
                trocado = true;
            }
        }
    }
    list
}

// TRAITS
// <T:std::cmp::PartialOrd> Trait
fn largest<T:PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/*
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
*/

pub trait Sorted {
    fn sorted(self: &Self) -> bool; // Metodo sempre tem self: &Self como primeiro parametro
}

impl<T> Sorted for [T] where T:PartialOrd + Debug {
    fn sorted(&self) -> bool {
        if self.is_empty() {
            return true;
        } else {
            self.windows(2).all(|w| w[0] <= w[1])
        }
    }
}

//exemplo
fn f(x: &impl Sorted, y: &impl Sorted) {
}

struct Student {
    name: String,
    id: u32
}

impl<'a> Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Student id: {}, name: {}", self.id, self.name)
    }
}


// LIFETIME

fn teste<'a>(x: &'a str, y: & 'a str) -> &'a str {
    if x.len() > y.len(){
        x
    } else {
        y
    }
}

struct Pessoa<'a> {
    nome: &'a str,
    idade: u32,
}

fn main() {
    let lista= ['a', 'g', 'x'];
    let lista2 = [1, 2, 45, 32, 789, 23, 0, 4];
    println!("{0} {1}", largest(&lista), largest(&lista2));
    let leo = Student {
        name: String::from("Leonardo"),
        id: 54349795
    };
    println!("{}", leo);
    let leonardo = Pessoa {nome:"Leonardo", idade: 20};
    println!("{} {}", leonardo.idade, leonardo.nome);
    let lista3 = vec!["Leonardo", "Vicente", "Victor", "Armando"];
    let lista_resp = bubble_generico(lista3);
    println!("{:?}", lista_resp);
}
