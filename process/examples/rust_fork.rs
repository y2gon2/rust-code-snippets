// refrence : https://github.com/y2gon2/self-learning-cs/blob/main/process/process_cplusplus.md

// 1. Checking PID
// This is a nightly-only experimental API. (unix_socket_ancillary_data #76915)
// Available on (Android or Linux) and Unix only.
/*
use libc::{getpid};

fn main() {
    let pid = unsafe { getpid() };
    println!("Process ID :{}", pid); 
}
// Process ID :1768
*/

// 2.1 fork(1)
/*
use libc::{getpid, fork};

fn main() {
    let pid = unsafe { getpid() };
    println!("Parent PID : {}", pid);                    // Parent PID : 1962

    if unsafe { fork() } == 0 {
        println!("Child PID  : {}", unsafe{ getpid() }); // Child PID  : 1984 
    }
}
 */
// unsafe { fork() } == -1  : failed to fork call
// unsafe { fork() } == 0   : child process
// unsafe { fork() } == +   : parent process


// 2.2 fork(2)
/*
use libc::{getpid, fork};

fn main() {
    let result = unsafe { fork() };

    if result == -1 {
        eprintln!("Failde to fork process");
    } else if result == 0 {
        println!("I'm the child process. My PID is {}", unsafe{ getpid() }); 
        println!("I'm the parent process. My PID is {}", unsafe{ getpid() }); 
    }
}
// I'm the parent process. My PID is 2087
// I'm the child process. My PID is 2089
*/

// 3. create child process working the same thing with parent process.

use libc::{getpid, fork};

fn foo(cnt: i32) -> i32 {
    println!("{} : execute foo!!", cnt);
    return cnt + 1;
}

fn main() {
    let mut cnt = 1;

    if unsafe{ fork() } == 0{
        println!("The child process PID : {}", unsafe{ getpid() });
        cnt = foo(cnt);
    } else {
        println!("The parent process PID : {}", unsafe{ getpid()});
        cnt = foo(cnt);
    }

    println!{"Final cnt : {}", cnt};
}
// The parent process PID : 2280
// 1 : execute foo!!
// The child process PID : 2299
// 1 : execute foo!!
// Final cnt : 2  
// -> the last output value is only affected on parent process. 
// -> if we add any different function in the child process block, it can execute the other work.