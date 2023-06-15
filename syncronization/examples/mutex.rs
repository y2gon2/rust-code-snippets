


// 1. Arc::new(Mutex::new())
/*
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;

const N: usize = 10;

fn main() {
    let data = Arc::new(Mutex::new(0));

    let (tx, rx) = channel();

    for _ in 0..N {
        let (data, tx) = (Arc::clone(&data), tx.clone());
        thread::spawn(move|| {
            let mut data = data.lock().unwrap();
            *data += 1;
            println!("{}", *data);
            if *data == N {
                tx.send(()).unwrap();
            }
        });
    }
    let ans = rx.recv().unwrap();

    println!("{:?} : {}", &ans, *data.lock().unwrap());
}
 */


// 2. To recover from a poisoned mutex
/*
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let lock = Arc::new(Mutex::new(0_u32));
    let lock2 = Arc::clone(&lock);

    let _ = thread::spawn(move || -> () {
        let _guard = lock2.lock().unwrap();
        panic!();
    }).join();

    let mut guard = match lock.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    *guard += 1;

    println!("*guard: {}", *guard);  // 1
}
 */


// 3. working the each thread with the same mutex guarded data
/*
use std::sync::{Arc, Mutex};
use std::thread;

const N: usize = 3;

fn main() {
    let data_mutex = Arc::new(Mutex::new(vec![1,2,3,4]));
    let res_mutex = Arc::new(Mutex::new(0));

    let mut threads = Vec::with_capacity(N);
    (0..N).for_each(|i| {
        let data_mutex_clone = Arc::clone(&data_mutex);
        let res_mutex_clone = Arc::clone(&res_mutex);

        threads.push(thread::spawn(move ||{
            let result = {
                let mut data = data_mutex_clone.lock().unwrap();
                let result = data.iter().fold(0, |acc, x| acc + x * 2);
                data.push(result);
                // println!("{}", result);
                result
            };

            *res_mutex_clone.lock().unwrap() += result;

            println!("{}th thread: {}", i, *res_mutex_clone.lock().unwrap());
        }));
    });

    let mut data = data_mutex.lock().unwrap();
    let result = data.iter().fold(0, |acc, x| acc + x * 2);
    println!("data ?? : {}", result);
    data.push(result);

    drop(data);

    *res_mutex.lock().unwrap()  += result;

    threads.into_iter().for_each(|thread| {
        thread.join().expect("The thread creating or execution failed !!")
    });

    println!("{}", *res_mutex.lock().unwrap());
}
 */

// 4. a multithreading (mutex) accumulator
use std::sync::{Arc, Mutex};
use std::thread::{self};

const THREAD_NUM: usize = 4;

fn accumulator(i: usize, input: Arc<Mutex<usize>>) {
    for _ in 0..1000 {
        let mut data = input.lock().unwrap();
        *data += 1;
    }
    println!("{}th current value : {}", i, *input.lock().unwrap());
}

fn main() {
    let cnt = Arc::new(Mutex::new(0_usize));
    let mut threads = Vec::with_capacity(THREAD_NUM);

    (0..THREAD_NUM).for_each(|i| {
        let input = Arc::clone(&cnt);
        threads.push(thread::spawn(move || {
            println!("{}th thread is started!!", i);
            accumulator(i, input);
            println!("{}th thread is colsed!!", i);
        }));
    });

    threads.into_iter().for_each(|thread|{
        thread.join().expect("The thread creating or execution failed");
    });

    println!("final value : {}", *cnt.lock().unwrap());
}
