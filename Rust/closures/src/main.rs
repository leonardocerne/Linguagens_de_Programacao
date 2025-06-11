fn main() {
    let x = 5;
    // Closure = função abstrata
    let square = || x * x;
    let cube = |x:i32| x * x * x;
    println!("{} {}", square(), cube(5));

    let mut v = vec![1,2,3,4,5];
    v[1] = 3;
    let c = || {println!("{:?}", v)};
    //v[1] = 3;
    c();

    let mut v2 =  vec![1,2,3,4,5];
    let mut c2 = || {v2.push(3)};
    v2[1] = 10;
    let mut c2 = || {v2.push(4)};
    c2();
    println!("{:?}", v2);

    let v3 = vec![1,2,3,4,5,6];
    let mut i = v3.iter();
    let y = i.any(|x| *x == 3);
    println!("{}", y);
    println!("{:?}", i);
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());
    println!("{:?}", i.next());


}
