use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::{anyhow, Result};

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}

const INITIAL_SIZE: usize = 32;

impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(INITIAL_SIZE)),
            available: Condvar::new(),
            senders: AtomicUsize::new(1),
            receivers: AtomicUsize::new(1),
        }
    }
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    // 优化
    cache: VecDeque<T>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, msg: T) -> Result<()> {
        if self.total_receivers() == 0 {
            return Err(anyhow!("No available receiver."));
        }

        let was_empty = {
            let mut queue = self.shared.queue.lock().unwrap();
            let empty = queue.is_empty();
            queue.push_back(msg);
            empty
        }; // 锁在这里被释放

        // 唤醒挂起的消费者线程
        if was_empty {
            self.shared.available.notify_one();
        }

        Ok(())
    }
    pub fn total_receivers(&self) -> usize {
        self.shared.receivers.load(Ordering::SeqCst)
    }

    pub fn total_queued_items(&self) -> usize {
        let queue = self.shared.queue.lock().unwrap();
        queue.len()
    }
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T> {
        // 优化：缓存不为空则取缓存。
        if let Some(x) = self.cache.pop_front() {
            return Ok(x);
        }

        let mut queue = self.shared.queue.lock().unwrap();
        loop {
            match queue.pop_front() {
                Some(x) => {
                    // 优化：若队列不为空，将队列与消费者内部的缓存交换。
                    if !queue.is_empty() {
                        std::mem::swap(&mut self.cache, &mut queue);
                    }
                    return Ok(x);
                }
                None if self.total_senders() == 0 => {
                    return Err(anyhow!("No available sender."));
                }
                None => {
                    // 在没数据的时候挂起线程
                    // wait 接收 queue的 MutexGuard，然后释放该 Mutex，挂起线程
                    // 等到其他线程唤醒消费者线程时，会重新拿到 queue 的 MutexGuard
                    queue = self.shared.available
                        .wait(queue)
                        .map_err(|_| {
                            anyhow!("Lock poisoned.")
                        })?
                }
            }
        }
    }

    pub fn total_senders(&self) -> usize {
        // 这里必须使用 SeqCst，阻塞其他线程对这个值的操作
        self.shared.senders.load(Ordering::SeqCst)
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.shared.senders.fetch_add(1, Ordering::AcqRel);
        Self {
            shared: self.shared.clone(),
        }
    }
}


impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let old = self.shared.senders.fetch_sub(1, Ordering::AcqRel);
        if old <= 1 {
            self.shared.available.notify_one();
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.receivers.fetch_sub(1, Ordering::AcqRel);
    }
}

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared::default());
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared,
            cache: VecDeque::with_capacity(INITIAL_SIZE),
        }
    )
}


#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use super::*;

    #[test]
    fn channel_should_work() {
        let (mut s, mut r) = unbounded();
        s.send(String::from("Hello world!")).unwrap();
        let msg = r.recv().unwrap();
        assert_eq!("Hello world!", msg);
    }

    #[test]
    fn multiple_senders_should_work() {
        let (mut s, r) = unbounded();
        let mut s1 = s.clone();
        let mut s2 = s.clone();
        let t1 = thread::spawn(move || {
            s.send(1).unwrap();
        });
        let t2 = thread::spawn(move || {
            s1.send(2).unwrap();
        });
        let t3 = thread::spawn(move || {
            s2.send(3).unwrap();
        });

        for handle in [t1, t2, t3] {
            handle.join().unwrap();
        }

        let mut result: Vec<i32> = r.into_iter().collect();

        // 接收数据的顺序可能不一样
        result.sort();

        assert_eq!(vec![1, 2, 3], result);
    }

    #[test]
    fn receiver_should_be_blocked_when_nothing_to_receive() {
        let (mut s, r) = unbounded();

        let mut s1 = s.clone();

        thread::spawn(move || {
            for (idx, i) in r.into_iter().enumerate() {
                // 验证读到的数据跟发送的一致
                assert_eq!(idx, i);
            }

            // 当队列中没数据时，消费者线程应被阻塞
            unreachable!("Receiver thread should be blocked.");
        });

        thread::spawn(move || {
            for i in 0..100usize {
                s.send(i).unwrap();
            }
        });


        // 等待两个线程跑完
        thread::sleep(Duration::from_millis(100));

        // 再次发送数据
        for i in 100..200usize {
            s1.send(i).unwrap();
        }

        // 等待消费者重新接受完所有数据
        thread::sleep(Duration::from_millis(100));

        // 测试队列中的数据是否被接受
        assert_eq!(0, s1.total_queued_items());
    }

    #[test]
    fn receiver_should_error_when_all_senders_are_dropped() {
        let (s, mut r) = unbounded();
        let senders = [s.clone(), s];
        let total = senders.len();

        for mut sender in senders {
            thread::spawn(move || {
                sender.send("hello").unwrap();
                // sender 在此被丢弃
            })
                .join()
                .unwrap();
        }

        for _ in 0..total {
            r.recv().unwrap();
        }

        assert!(r.recv().is_err());
    }

    #[test]
    fn sender_should_error_when_all_receivers_are_dropped() {
        let (mut s1, mut s2) = {
            let (s, _) = unbounded();
            (s.clone(), s.clone())
        };

        assert!(s1.send(1).is_err());
        assert!(s2.send(1).is_err());
    }

    #[test]
    fn receiver_should_be_notified_when_all_senders_are_dropped() {
        let (s, mut r) = unbounded::<u32>();

        let t1 = thread::spawn(move || {
            assert!(r.recv().is_err());
        });

        // 等待 t1 阻塞
        thread::sleep(Duration::from_secs(1));

        drop(s);

        t1.join().unwrap();
    }

    #[test]
    fn receiver_cache_should_work() {
        let (mut s, mut r) = unbounded();

        for msg in 0..10 {
            s.send(msg).unwrap();
        }

        // 未读取数据时缓存为空
        assert!(r.cache.is_empty());
        // 读取一个数据
        assert_eq!(0, r.recv().unwrap());
        assert_eq!(9, r.cache.len());
        assert_eq!(0, s.total_queued_items());

        // 读取剩余数据
        for (idx, msg) in r.into_iter().take(9).enumerate() {
            assert_eq!(idx + 1, msg);
        }
    }
}