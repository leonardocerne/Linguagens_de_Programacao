
fn invertestring(s:String) -> String {
    let mut vet:Vec<char> = Vec::new();
    for c in s.chars() {
        vet.push(c);
    }
    let mut inv = String::new();
    while let Some(c) = vet.pop() {
        inv.push(c);
    }
    inv
}

fn main() {
    let oss = String::from("Leonardo");
    let invertida = invertestring(oss);
    println!("{invertida}");
}
