## RUST 비동기 프로그래밍 정리 (1) 
 - 원문 : https://rust-lang.github.io/async-book/02_execution/02_future.html

 ### 1. The Futere Trait
rust future trait 의 기본 동작원리는 다음과 같다

```rust

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>; 
}

enum Poll<T> {
    Ready(T),              
    Pending, 
}

pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {                 
            Poll::Ready(self.socket.read_buf())
        } else {                                            
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    } 
}

```
<br>
time      　  　>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>><br>
future    　　**************************************************************|<-Done!<br>
Poll　　　　　^   　　　　　　　 ^ 　  　　　     　　　   ^ 　　　　      　　    　^       
Ready  　　　　　　　　　　　　　　　　　　　　　　　　　　　 　　O <br>
Pending 　　　O 　　　　　　　O  　　　　　　　　O         <br>
　　　　　　　-> Pending  　  　 -> Pending 　　　　-> Pending 　　　-> Ready(result)<br><br>

    - Future 작업이 발생한 이후 특정 방법에 의해 주기적으로 위와 같이 Future Trait 에 정의된 fn poll 을 실행한다.
    - 아직 Future 가 완료되지 않았다면, wake fn() 을 준비하여 Future 가 완료됐을 때, 작업가능하도록 한다. 그리고 Poll::Pending 을 return 한다.
    - 만약 polling 시점에 Future 가 완료 되었다면, 정의된 이후 작업을 진행하여 결과값과 함께 Poll::Ready(result) 를 return 한다.
<br>
* 어떻게 특정 방법에 의해 주기적으로 poll 이 진행 가능 한가?

    - 거대한 프로그램을 구현하다보면 다양한 상황에서 수많은 Future 가 발생할 수 있다. 
    - 따라서 OS thread 를 관리하는 scheduler 처럼 누군가 이렇게 '나중에 언젠가는 처리되어야 할 일' 들을 관리하는 존재가 필요해 진다. 
    - rust 에서 해당 주체를 'executor' 라고 하며, '처리되어야 할 일' 에 대해한 추상적 단위을 'task' 라고 명명한다. 
    - 따라서 Future 는 곧 task 의 구체적 대상으로 executor 가 관리해야 할 존재이며, Pending, ready, wake 등의 상태 및 조건을 갖춘 이후 실제 실행, 작업이 완료기 전까지 해당 구조 안에서 존재한다. 
    - 이를 정리하면 "Future 가 executor의 관리 구조안에서 lifecycle 을 가지게 되며, 이를 Future 가 task 에 binding  된다." 라고 표현 가능하다. 
<br>

* Future 와 task 의 관계2 (with Context)
    - 다시 언급하면, task 는 실행해야 하는 어떤 작업의 단위를 나타내는데, 이 작업은 어떤 Future 에 의해 정의된다. 
    - 그리고 이는 곧 task 는 Future 를 실행하기 위한 추상화된 context 이다. 여기서 context 란 무엇일까?
    - "Future 를 실행한다" 는 말은 현재 pending 상태의 Future 가 관련 작업이 가능한 상태가 되었을 때, 작업을 재개 할수 있는 상태로 만들어 주어야 한다는 의미이다. 
    - 위 말을 구조적으로 다시 말하면, task binding 된 Future 가 executor 에 의해 특정 시간마다 poll method 가 호출 될때, ready 상태라면 작업을 재개할 수 있는 Waker 를 제공하여야 한다. 그리고 해당 Waker 는 task 의 Context 내 정의되어 있다. 
    - 위 내용을 요약하면 "executor 가 각 task 를 관리하고, 그 대상이 곧 Context 이며, Ready 상태엘 경우 Context 내 Waker 를 실행한다." 라고 표현 가능하다. 
<br><br>

위의 내용을 바탕으로 사용자 정의 Future 을 정의해 보자. 

아래 코드는 일정 시간 경과 후 재실행되는 'timer future' 이다.<br>
(sorce code : https://rust-lang.github.io/async-book/02_execution/03_wakeups.html)
```rust
pub struct TimerFuture {
    shared_state : Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}
```
우선 Future instance 사용환경은 multithreads 임을 감안 공유 가능 Arc-Mutex 구조이여야 하며,
하위 field 들은 completed - 시간 경과 여부(pending/ready 기준) 와 waker - ready 상태에서 실행될 waker 에 대한 정의로 구성되어 있다. 

해당 type 에 대한 Future trait implementation 은 다음과 같다. 
```rust
impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.sharded_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone()); // (1)
            Poll::Pending
        }
    }
}
```
<br>
위 Future trait implemenation 에서 (1) 부분을 보면, Context instance 의 waker field 를 복제하여 state warker 로 설정 하고 있다. 그런데 매번 poll call 일 때마다 복제하여 설정할 필요가 있을까?

- 그 이유는 Future instance 내에 다수의 하위 Future 가 존재하거나 Future 에 대한 재귀적 구조로 인해 ownership 이전이 발생할 수 있다. 
```rust
async fn create_and_run_future() {
    let future = create_future();   // create_future 함수가 Future 생성
    run_future(future).awiat;       // run_future 함수가 Future의 ownership 을 받아서 실행
}
```
- 이와 같이 Top-Level Future 를 제외한 하위 level Future 들의 경우 복잡한 실행 흐름으로 어떤 Future 가 어떤 task 에 binding 되고, 현재의 binding 이 언제 변경될지 예측하기 어려워 진다. 
- 이러한 이유로, Future의 'poll' method 는 매번 호출될 때마다 새로운 'Context' 를 받고, 이에 현재 bindind 된 'Waker'를 포함하게 된다.   