

use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};

use std::{
    future::Future,
    pin::Pin,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

// 1. Task Wakeups with Waker (asynchrnous_programming_future_trait.md 참조)
//    
// https://rust-lang.github.io/async-book/02_execution/03_wakeups.html
pub struct TimerFuture {
    shared_state : Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone()); // (1)
            Poll::Pending
        }
    }
}

// --------------------------------------------------------------------------
// 2. Build an Executor
//
// https://rust-lang.github.io/async-book/02_execution/04_executor.html

// 해당 executor 는 channel 에서 event 를 가져오고 task 이 실행되도록 channel 에 보냄
// task 가 'ready' (깨어진)상태일 때, scheduler 가 poll 한 이후 channel 에 넣음. 

/// channel 에서 받은 task 를 받아 실행
struct Executor {
    ready_queue: Receiver<Arc<Task>>, 
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            
            // if let Some(mut futre) = future_slop {} 와 같이 take() method 을 적용하지 않아도 error 가 발생하지 않는다.
            // 그러나 take() 를 사용하지 않은 경우, MutexGuard 가 if let code 가 완료될 때까지 유지되어야 하므로,
            // shared memory 접근에 불필요한 지연을 발생 시킬 수 있다. 
            // 따라서, future_slot.take() 로 future_slot 의 소유권을 가져오고, MutexGuard 내에는 None 을 남긴 상태로
            // 즉시 MutexGuard 해제, 다른 Thread 의 접근을 허용시켜 원할한 작업이 가능하게 만들어 준다. 
            if let Some(mut future) = future_slot.take() {
                // 받은 task 로 부터 waker reference 로 부터
                let waker = waker_ref(&task);
                // context 생성
                let context = &mut Context::from_waker(&waker);
                
                // 해당 future 를 polling 하여 상태를 확인하고,
                // 아직 pending 상태라면 최초 channel 에서 받은 future_slot 에 해당 future 을 다시 넣음.
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

/// 새로운 future 를 task channel 에 생성
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

/// 주어진 future 로 Task 생성 (binding) 및 send 
impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed(); // Box pointer 사용한 것과 동일 -> Box<dyn Future<Output = T> + Send>
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

/// asynchrnous_programming_future_trait.rs  에서 언급된 바와 같이, 
/// Future trait implementation 된 poll method 의 경우, Waker 가  
struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    // task 자체적으로 task queue 에 send 가능하도록 (?)
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);

    (Executor{ ready_queue }, Spawner { task_sender })
}

// executor 가 polling 시 waker 갱싱 할수 있도록
// 해당 trait implementation method call 로 해당 Task 가 복제 및 send 를 진행함.
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}


fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("howdy!");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    drop(spawner);

    executor.run();
    // howdy!
    // done!
}