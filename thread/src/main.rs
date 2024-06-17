use std::{
    thread::{self, spawn},
    time::Duration,
};
use thread_amount::thread_amount;

fn main() {
    // start_one_thread();
    // start_one_thread_result()
    // start_two_threads();
    // start_n_threads();
    // start_one_thread_by_builder();
    // current_thread();
    // unpark_thread();
    // get_parallelism_count();
    // get_current_thread_count();
    // start_thread_with_yield_now();
    // thread_park2();
}

fn thread_park2() {
    let handle = spawn(|| {
        println!("睡眠前...");
        thread::sleep(Duration::from_secs(10));
        println!("线程醒来并准备挂起...");
        thread::park();
        println!("来自挂起线程的问候！");
    });
    // 主线程睡眠，确保新线程在挂起前已经睡眠完成
    handle.thread().unpark();
    handle.join().unwrap();
}

fn start_thread_with_yield_now() {
    let handle1 = spawn(|| {
        thread::yield_now();
        println!("yield_now!");
    });
    let handle2 = spawn(|| {
        thread::yield_now();
        println!("yield_now in another thread!");
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn get_current_thread_count() {
    let num = thread_amount();
    println!("thread count: {:?}", num)
}

/// 获取可用的并发数量（CPU核心数量）
fn get_parallelism_count() {
    let count = thread::available_parallelism().unwrap();
    println!("Avaliable parallelism: {}", count);
}

/// 通过park()让线程暂停，通过unpark()让线程唤醒
fn unpark_thread() {
    let parked_thread = thread::Builder::new()
        .spawn(|| {
            println!("Parking thread");
            thread::park();
            println!("Thread unparked");
        })
        .unwrap();
    thread::sleep(Duration::from_secs(10));
    println!("Unpark the thread");
    parked_thread.thread().unpark(); // thread() 获取线程句柄
    parked_thread.join().unwrap();
}

fn current_thread() {
    let current_thread = thread::current();
    println!(
        "current thread: {:?} {:?}",
        current_thread.id(),
        current_thread.name()
    );
    let builder = thread::Builder::new()
        .name("foo".into())
        .stack_size(32 * 1024);
    let handler = builder
        .spawn(|| {
            let current_thread = thread::current();
            println!(
                "child thread: {:?} {:?}",
                current_thread.id(),
                current_thread.name()
            )
        })
        .unwrap();
    handler.join().unwrap();
}

fn start_one_thread_by_builder() {
    let builder = thread::Builder::new()
        .name("foo".into())
        .stack_size(32 * 1024);
    let handle = builder
        .spawn(|| {
            println!("Hello from a thread!");
        })
        .unwrap();
    handle.join().unwrap();
}

/// 创建N个线程
fn start_n_threads() {
    const N: isize = 10;
    let handles: Vec<_> = (0..N)
        .map(|i| {
            thread::spawn(move || {
                println!("Hello from a thread {}!", i);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

fn start_two_threads() {
    let handle1 = thread::spawn(|| {
        println!("Hello from a thread1!");
    });
    let handle2 = thread::spawn(|| {
        println!("Hello from a thread2!");
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}

/// 可以通过join拿到线程的返回值
fn start_one_thread_result() {
    let handle = thread::spawn(|| {
        println!("Hello from start_one_thread_result!");
        panic!("oh error");
        200
    });
    match handle.join() {
        Ok(v) => println!("thread result: {}", v),
        Err(e) => println!("error: {:?}", e),
    };
}

/// 创建一个线程
fn start_one_thread() {
    let handle = thread::spawn(|| println!("Hello from start_one_thread!"));
    // 如果不使用join，则主线程退出这个线程也会跟着退出
    // 如果使用join，则主线程会等待这个线程结束后再退出
    handle.join().unwrap();
}
