

fn main() {
    let x = 5;
    let y = &x;
    let z = &y;

    match y {
        &a => println!("{}", a), // &a : i32
    } 
    // match arm 에서 & 는 해당 변수값에 대한 참조가 아니라.
    // 해당 변수 value 가 이미 참조 형일 때, 그 참조형 value 가 
    // 가리키고 있는 value 를 가리킨다. 
    // 따라서 해당 &a 의 자료형은 &i32 가 아닌  i32 가 되며

    /*
    match x {
        &a => println!("z : {}", a), // 타입 불일치에 따른 에러 발생
    } 
    */
    // 이와같이 참조형 value 를 가지지 않고 실제 value 를 가진 변수에
    // & 를 사용하면 해당 value 에 대한 타입 불일치로 에러가 발생한다. 

    match x {
        ref a => println!("{}", a), // ref a: &i32
    } 
    // 따라서 value 에 대한 참조로 match arm 에서 사용하고 싶다면
    // ref 키워드를 사용해 줘야 한다. 

    match y {
        ref a => println!("{}", a), // ref a: &&i32
    }
}