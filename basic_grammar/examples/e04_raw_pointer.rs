fn print_vec(v: &Vec<&dyn ToString>) {
    for i in v {
        println!("{}", i.to_string());
    }
}

fn main() {
    let v1 = vec![&"hello", &"world"];
    print_vec(&v1);

    let v2: Vec<&str> = vec!["hello", "world"];
    print_vec(&v2);
}