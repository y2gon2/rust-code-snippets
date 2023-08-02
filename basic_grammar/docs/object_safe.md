## Object Safe (객체 안전) 및 관련 내용들

Rust에서, 특정 트레잇이 "객체 안전(object safe)"하다는 것은 해당 트레잇이 동적 디스패치(dynamic dispatch)를 허용한다는 것을 의미<br>
즉, Box<dyn Trait>나 &dyn Trait와 같은 방식으로 트레잇 객체를 사용할 수 있다는 뜻<br>
<br>

### Rust는 트레잇이 객체 안전한지를 결정하기 위한 규칙

### 1. Generic Trait 이 아니여야 한다. 
```rust
pub trait MyTrait<T> {
    fn do_something(&self, value: T);
}
```  
위의 예시와 같이 Generic Type 을 매개변수로 가지고 있는 Trait 은 dynamic dispacth 문법을 사용할 수 없음

### Why?
Genric Trait 은 Compile Time 에 구체적 type 으로 instance 화 된다. (Monomorphization)<br>

그러므로 runtime 에 dynamic dispatch 를 적용하는 방법과는 양립할 수 없게 된다. 다르게 표현하면, Generic Trait 은 compile 시점에 구체화가 진행되므로 runtime 에 구체화 되는 dynamin dispach 를 적요할 수 없는 존재가 된다. 

### . Trait 내에 method 중에서 argument 또는 return type 이 Self 일 수 없다. 
dynamic dispatch 를 사용하여 runtime 에 method 를 호출해야 하므로, trait 에 구현된 parameter 및 return type 은 complie time 에 확정 되는 type 들로만 구성되어야 한다. 그런데 Self type 의 경우 확정 시킬 수 없으므로 object safe 에 위배된다. 

### How to solve it?
Self type 을 Box<Self> type 으로 수정하여 compile time 에 Self 의 실체 크기가 확정되지 않아도 Sized 로 적용됨으로써 object safe 를 만족한다.