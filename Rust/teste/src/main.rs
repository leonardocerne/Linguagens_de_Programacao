use std::fmt;

// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct named `Structure` that contains an `i32`.
struct Structure(i32);

#[derive(Debug)]
struct Estrutura(i16, f32);

struct Person<'a> {
    name: &'a str,
    age: u8
}

struct Lista(Vec<i32>);

impl fmt::Display for Lista{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        for (c,v) in vec.iter().enumerate() {
            if c == 0 {write!(f, "Lista = {}", v)?}
            else {write!(f, " {}", v)?;}
        }
        write!(f, ".")
    }
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {    
        write!(f, "Seu numero = {}", self.0)
    }
}

impl fmt::Display for Person<'_>{
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {    
        write!(f, "Nome = {0}, Idade = {1}", self.name, self.age)
    }
}



fn main() {
    let x = Structure(15);
    let name = "leo";
    let age = 19;
    let teste = Estrutura(30, 10.25);
    let leonardo = Person {name, age};
    let lista = Lista(vec![1, 2, 3, 4, 5]);
    println!("{}", x);
    println!("{}", leonardo);
    println!("{:?} e {:?}", teste.0, teste.1);
    println!("{}", lista);
}
