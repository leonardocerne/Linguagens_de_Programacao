// Comentarios igual C e C++
/*exemplo*/
fn main() {
    println!("Hello, world!"); // Print com quebra de linha
    let mut x:u32 = 10; // Inicializando variável x com o valor 10 em Unsigned Integer, e usa o "mut" para poder alterar o valor dessa variavel posteriormente
    let mut y:f64 = 52.5; // Inicializando variável y com o valor 52.5 em Float 64
    println!("{} e {}", x, x as f64 + y); // É preciso fazer um casting ao x para poder somá-los
    x = 15;
    println!("{}", x);
    y = 0.123123123123;
    // let mut z = 4; criando uma variavel e nao utilizando-a gera um warning
    println!("Agora o valor y é = {:.3}", y);
    // Tipos escalares
    // inteiro com sinal
    let int1:i8 = 1;
    let int2:i16 = 2;
    let int3:i32 = 3; // int default
    let int4:i64 = 4;
    println!("Inteiros = {0}, {1}, {2}, {3}.", int1, int2, int3, int4);
    // inteiro sem sinal
    let uint1:u8 = 1;
    let uint2:u16 = 2;
    let uint3:u32 = 3;
    let uint4:u64 = 4;
    println!("Inteiros sem sinal = {0}, {1}, {2}, {3}.", uint1, uint2, uint3, uint4);
    // ponto flutuante
    let flo1:f32 = 1.1;
    let flo2:f64 = 2.2; // float default
    println!("Floats = {0}, {1}", flo1, flo2);
    // máximos e mínimos
    println!("f32 MAX {}", std::f32::MAX); //funciona para outros tipos de f e i
    println!("f32 MIN {}", std::f32::MIN);

    // Mudando tipos
    let k1 = 10;
    let k2 = 15.0;
    // println!("k1 + k2 = {}", k1 + k2); assim dá erro
    println!("k1 + k2 = {}", k1 + k2 as i32); //assim nao

    // Printando inteiros decimais em outras bases
    let test = 10;
    println!("Em decimal {}, em octal {:o}, em hexadecimal {:X}, em binário {:b}.", test, test, test, test);
    
    // Shadowing
    // let s = 0; se essa linha nao tivesse sido comentada, o valor 123 seria printado da mesma forma
    let s = 123;
    println!("Esse é o valor de s = {}", s);
}
