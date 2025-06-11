fn f(s:String) {
    println!("{s}");
}

fn t(x: String) -> String{
    println!("{x}");
    x
}

fn m(m:&String) {
    println!("{m}");
}

fn m2(m:&mut String) {
    m.push_str(" da Gama");
}

fn main() {
    // Ownership
    /*
    Regras:
    -Cada valor tem um dono.
    -Só existe um único dono a cada momento
    -Quando o dono do valor sair de escopo, o valor é liberado
     */

    //exemplo:
    let s = String::from("hello");
    /*
    Quando s sai de escopo, a implementação da função drop é chamada
    automaticamente para desalocar a string "hello" que foi alocada na heap
    por String::from
    */
    
    //exemplo:
    let s1 = String::from("hello");
    let s2 = s1;

    /*
    Isso faria com que a área de memória contendo hello fosse desalocada duas
    vezes, o que pode levar a invalidar a memória (memory corruption) e vulnerabilidade do código
    . Se desalocamos s1, sua área fica livre para uma variavel (s3 por exemplo) ser alocada
    ali, e quando s2 é desalocada, s3 fica como um ponteiro inválido
     */

    // println!("{s1}"); isso dá erro!
    println!("{s2}"); // isso não

    /* 
    A linguagem Rust, ao atribuir s1 para s2, invalida s1, e quando tentamos imprimir o valor
    de s1, é gerado um erro por tentar imprimir um valor que não existe
    */

    f(s);
    // println!("{s}"); isso dá erro!

    /*
    Ao chamar a função f, o valor referenciado por s é pego emprestado, e ao finalizar
    sua execução, s é liberado. Logo, quando o println! tenta pegar s emprestado, um erro
    é gerado, pois s já foi invalidada
     */

    let mut x = String::from("oie oie");
    x = t(x);
    println!("{x}");

    /*
    Na função t, é retornado o valor x, logo ela devolve a propriedade de um valor
    à função chamador, ou seja, não gera erro mesmo usando o mesmo nome de variável
     
    Porem, é uma prática passível de erro, já que fica por conta do programador, logo,
    utilizamos referências
     */

    //exemplo
    let mut m1 = String::from("Vasco");
    m(&m1);
    println!("{m1}");

    /*
    Dessa forma, passamos a referência de m1 ao invés do valor, e a função agora passa uma referência
    e não uma string
     */

    // Dessa forma, conseguimos mudar um valor em uma função e retornar pra main sem problema
    m2(&mut m1);
    println!("{m1}");
    
}
