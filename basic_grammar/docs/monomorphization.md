## Monomorphization (단일화) 

Genric Trait 은 Compile Time 에 구체적 type 으로 instance 화 됨.

#### Generic Trait 의 작동 방법 
```rust
fn print_item<T: Display>(item: T) {
    println!("{}", item);
}
```
해당 함수에서 Display trait 을 구현한 모든 type 의 'T' 는 수많은 type 이 implement 되 어 있다. 이를 모두 고려하여 runtime 시 결정된다면 그리 효율적 구조라고 볼수 없을 것이다. rust 에서는 이 경우, 실제 해당 code 에서 사용된 type 에 대해서만 complie time 에 각각 해당 함수를 생성한다. <br>
예를 들어,
```rust
print_item(5);         // T = i32
print_item("hello");   // T = &str
```
여기서 T는 두 가지 다른 타입(i32와 &str)으로 사용되었다. 그러므로 컴파일러는 print_item<i32>와 print_item<&str>라는 두 가지 버전의 print_item 함수를 생성하여 가지고 있음으로써, runtime 에 type 을 결정해야 하는 상황을 제거 시킬수 있다. 이렇게 각 type에 대해 별도의 함수 버전을 생성하는 것이 Monomorphization 이다.<br><br>

이렇게 하면, 각 type별로 최적화된 코드를 생성할 수 있다. 그러나 이 과정은 프로그램의 크기를 증가시킬 수 있으므로, 이를 "코드 블로팅(code bloat)"이라고 부르기도 한다. 이 문제는 Generic 관련 코드를 너무 많이 사용하거나 너무 많은 type에 대해 사용하는 경우에만 발생하게 된다.




