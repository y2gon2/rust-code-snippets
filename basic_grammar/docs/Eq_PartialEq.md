## PartialEq and Eq Trait

### 1. PartialEq
* 동등 비교 연산자 '== / !=' 를 통한 변수값의 비교를 기능의 method 를 포함 한 trait 
* 해당 trait 내 eq method 구현을 통해 사용자 정의 type 의 동등 연산 또는 다른 type 간 동등 연산이 가능하도록 구현 가능. 
* eq method는  '==' 연산 구현, ne method 는 '!=' 연산을 구현하기 위해 존재하지만 사용자 정의 trait mathod 를 구현할 경우, '!=" 연산을 특별히 지정해야 하는 경우가 아닌 일반적인 경우라면, eq method 만 구현하면 '== / !=' 연산자 모두 사용 가능함.
<br><br>
source code : https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
```rust
enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}

sturct Book {
    isbn: i32,
    format: BookFormat,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}

let b1 = Book{ isbn: 3, format: BookFormat::Paperback };
let b2 = Book{ isbn: 3, format: BookFormat::Ebook };
let b3 = Book{ isbl: 5, format: BookFormat::Paperback };

assert!(b1 == b2);
assert!(b1 != b3);
```
해당 코드에서 Book 구조체에 대한 PartialEq trait 구현을 isbn 만 비교 판단하였으므로, b1 == b2 는 true 이고 b1 == b3 는 false 로 나타난다. 

* type 이 다른 경우의 동등 비교가 가능하게 하고 싶은 경우 다음과 같이 작성할 수 있다. 
```rust
#[derive(PartialEq)]
enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}

struct Book {
    isbn: i32,
    format: BookFormat,
}

impl PartialEq for Book {
    fn eq(&self, other: &BookFormat) -> bool {
        self.format == *other
    }
}

let b1 = Book( isbn: 3, format: BookFormat::Paperback );

assert!(b1 == BookFormat::Paperback);
assert!(b1 != BookFormat::Ebook);
```

### 2. Eq
위에 언급된 바와 같이 동등 비교 연산자를 사용하기 위해서는 PartialEq 에 대한 trait method 정의만 구현해도 작동한다. 그러나 다음의 이유로 보통 Eq trait 구현 또는 trait bounding 작업을 진행한다. 
<br><br>
* 완전 동등성 (Complete Equivalence) 확보의 필요성
1. 엄격한 동등성 강제화 : 'PartialEq' 에서 보장되지 않는 다음에 대한 동치 조건을 만족시킬수 있음.
    - reflexive (반사성)    : a == a
    - symmetric (대칭성)    : a == b 는 b == a 
    - transitive (추이성)   : a == b 이고 b == c 이면 a == c
<br><br>
2. 컴파일러 최적화 : 'Eq' 를 구현한 type 은 완전한 동등성을 가지므로, 컴파일러는 이렇나 성질을 활용하여 코드를 최적화 할 수 있음. 예를 들어, 컴파일러는 동일한 값을 가지는 변수들을 서로 교환할 수 있거나, 동일한 값을 가지는 조건문을 단순화 할 수 있음.
<br><br>

일반적으로 Eq trait 은 PartiaEq 를 포함하고 있다. 따라서 사용자 정의 type 에서 Eq trait 을 구현하는 경우, 일반적으로 PartialEq trait 구현이 선행되므로 Eq trait 의 경우 구현 내용을 필요로 하지 않는다. 
<br><br>

source code: https://doc.rust-lang.org/std/cmp/trait.Eq.html
```rust
enum BookFormat { Paperback, Hardback, Ebook }

struct Book {
    isbn: i32,
    format: BookFormat,
}

impl PartialEq for Book {
    fn PartialEq (&self, other: &Self) -> bool {
        self.format == other.format
    }
}

impl Eq for Book {}
```