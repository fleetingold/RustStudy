fn collections_main() {
    let mut vector = vec![1, 2, 4, 8];
    vector.push(16);
    vector.push(32);
    vector.push(64);
    println!("{:?}", vector);

    let mut v1: Vec<i32> = vec![1, 2, 4, 8];
    let mut v2: Vec<i32> = vec![16, 32, 64];
    v1.append(&mut v2);
    println!("{:?}", v1);

    let v = vec![1, 2, 4, 8];
    println!("{}", match v.get(0) {
        Some(value) => value.to_string(),
        None => "None".to_string()
    });
    println!("{}", v[1]);

    let mut w = vec![100, 32, 57];
    for i in &w {
        println!("{}", i);
    }

    for i in &mut w {
        *i += 50;
    }
    for i in &w {
        println!("{}", i);
    }
}