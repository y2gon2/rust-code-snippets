// p170 재귀를 이용한 피보나치 수열


fn fib(n: i32) -> i32 {
    if n == 1 {
        return 1;
    } else if n == 2 {
        return 1;
    }
    
    return fib(n - 1) + fib(n - 2);
} 



fn main() {
    for i in 2..10 {
        println!("{} : {}", i, fib(i));
    }
}