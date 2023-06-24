## New Type

## 1. Tuple Struct
Rust 에서 구조체를 선언할 때, Field 이름 없이 선언 할 수 있으며 이를 Tuple Struct 라고 한다. 

```rust
struct Color(u8, u8, u8);

fn main() {
    let red = (255, 0, 0);

    assert!(red.0 == 255);
    assert!(red.1 == 0);
}
```
<br><br>
## 2. New Type

위와 같이 Field 이름이 없으므로 Field 에 접근하기 위해서는 기존 tuple 의 원소에 접근하듯이 인스턴스명.0 과 같이 접근한다. 

이중 Field 를 하나만 같은 Tuple Struct Type 을 New Type 이라고 한다. 

```rust
struct Age(u8);

fn main() {
    let mut age = Age(30);
    let Age(mut value) = age;

    age.0 *= 2;
    value += 1;

    println!("Age is {}.", value);
    println!("Age is {}.", age.0);
}
```
```
Age is 31.
Age is 60.
```
앞에서 사용한 Tuple Sturct 의 접근 방법으로 작업이 가능하다. 
추가적으로 Field 에 변수명을 지정하여 접근이 가능하며, 해당 변수의 ownership 을 기존 instance 와 분리된다. 
<br><br>

## 3. New Type  사용 이유

* struct Age(u8); 로 선언하여 사용되는 instance 는 일반 변수 u8 type 과 다른 type 으로 인식되므로, 의미적으로는 다르지만 동일 data type 을 사용하는 변수들간의 연산 방지 할 수 있다. 

예를 들면,

```rust
struct Age(u8);

fn main() {
    let age = Age(30);
    let height: u8 = 180;
    let result = age + height; // 오류 발생!!
}
```
이와 같이 복잡한 코드 작성 중 실수 할 수 있는 부분을 구조적으로 차단시키므로써, type 안정성을 향상 시킬 수 있다. 
또한 'Age' 라는 이름의 instance 사용으로 코드의 의도를 더욱 명확하게 할 수 있다.

source code: https://www.worthe-it.co.za/blog/2020-10-31-newtype-pattern-in-rust.html
```rust
pub struct Name(String);
pub struct PhoneNumber(String);
pub struct Years(u32);

pub struct Person {
    pub name: Name,
    pub phone_number: PhoneNumber,
    pub age: Years
}

impl Person {
    pub fn time_to_retirement(current_age: Years) -> Years {
        ...
    }
}
```
위와 같이, 각 Field 의 type 명이 Field Name 과 일치하며, 관련 method 의 내 사용되는 type 에 대헤서도 type 명이 명시적으로 Field 를 가리키므로서 좀더 명료한 코드 작성이 가능해진다. 


추가적으로 New Type 을 선언하고 이에 대한 다양한 trait 을 사용자 정의로 구현함으로써, 작성자의 의도에 맞게 기능을 변경 할 수 있다. 

예를 들면, 만약 본인이 사용하는 instance 에 대해서 임의의 console 출력을 막고자 하는경우 다음과 같이 작성하여 보안을 높일 수 있다. (private 설정으로 해당 instance value 에 대한 접근을 근본적으로 차단하는 것도 가능)

source code : https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html
```rust
use std::fmt::Display;

struct Password(String);

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "********")
    }
}

fn main() {
    let secured_pw: Password = Password("123456789".to_string());

    println!("{}", secured_pw);
}
```
```
********
```
위 와 같이, Display Trait method 를 구현함으로써, 사용자가 원하는 console 출력을 발생 시킬 수 있다. 
<br><br>

## 4. 외부에서 정의된 New Type 이 사용 예시

New Type Field type 에 대해 public 인지 private 인지에 대한 설정이 불가능하며, rust 에서 생략했을 때 기본 설정 규칙에 따라 private 을 설정 된다. 따라서, 만약 New Type 을 외부 module 에서 선언한 경우 기본 Field 값 설정 instance 생성 방식이 사용 불가하다. 
또한, tuple 에 접근하는 방법으로 직접 field 값에 접근하는 것도 불가능하다. 
<br><br>
source code : https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html
```rust
mod some_module {
    pub struct PhoneNumber(String);
}

fn main() {
    let num = some_module::PhoneNumber("1234-5678".to_string());
    println!("{}", num.0);
}
```
```
...
9 |     let num = some_module::PhoneNumber("555-12345".to_string());
  |                            ^^^^^^^^^^^ private tuple struct constructor
  ....
   |
10 |     println!("{}", num.0)
   |                        ^ private field
```

위 문제를 해결하기 위해 생성자와 출력이 가능한 (&str) type 으로 return 해주는 method 정의가 필요하다.
<br><br>
source code : https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html
```rust
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(s: String) -> &self {
        PhoneNumber(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn main() {
    let num = PhoneNumber::new("1234-5678".to_string());
    println!("{}", num.as_str());
}

```
<br><br>
## 5. New Type 사용 시, 유용한 Trait 구현 pattern

* Display Trait
  
  (4) 번의 예제에서 as_str() method 가 아닌 (3) 번 예제에서 정의한 Display Trait 을 사용하여 일반 변수와 같이 console 출력이 가능하게 정의할 수 있다.

```rust
pub struct PhoneNumber(String);

impl std::fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0);
    }
}
```

* FromStr Trait
  
  해당 trait 구현으로 &str 을 사용자 정의 type 으로 생성할 수 있다. 
  아울러 FromStr Trait bound 된 type 에 대해 std::str::parse method 를 사용 가능하므로 해당 정의 구현을 통해 사용자 정의 type 으로 parse method 를 사용 가능해 진다.

```rust
use std::str::FromStr;
use std::error::Error;

pub struct PhoneNumber(String);

impl FromStr for PhoneNumber {
    type Err = Box<dyn Error>;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PhoneNumber(s.to_string()))
    }
}

fn main() {
    let num1 = PhoneNumber::from_str("1234-5678").unwrap();
    assert_eq!(num1.0, "1234-5678".to_string());
    
    let num2: PhoneNumber = "9876-4321".parse().unwrap();
    assert_eq!(num2.0, "9876-4321".to_string());
}
```
* Deref

  만약 New Type Field 의 type 그대로 reference 를 가져 오고 싶다면, Deref Trait 구현을 통해 가능하다. 

```rust
use std::ops::Deref;

pub struct PhoneNumber(String);

impl PhoneNumber {
    fn new(s: String) -> Self {
        PhoneNumber(s)
    }
}

impl Deref for PhoneNumber {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let num = PhoneNumber::new("111-222".to_string());

    assert!(&(*num) == "111-222");
}
```
