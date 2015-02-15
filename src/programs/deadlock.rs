/// A simple program that produces a deadlock.
#[allow(unused_must_use)]
pub fn produce_deadlock() {
    use std::thread::Thread;
    use std::sync::mpsc::channel;

    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    Thread::spawn(move ||{
        rx1.recv();
        tx2.send(());
    });

    rx2.recv();
    tx1.send(());
}
