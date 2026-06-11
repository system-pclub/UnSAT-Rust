#![feature(test)]
extern crate test;


//test bench_sync_vec_push ... bench:           15 ns/iter (+/- 1)
#[bench]
fn bench_queue_push(b: &mut test::Bencher) {
    let rw  = crossbeam::queue::ArrayQueue::new(40000_0000);
    let mut i = 0;
    b.iter(|| {
        let _r = rw.push(i);
        i += 1;
        if i == 40000_0000 - 2{
            return;
        }
    });
}

// test bench_queue2_push   ... bench:          27 ns/iter (+/- 2)
#[bench]
fn bench_queue2_push(b: &mut test::Bencher) {
    let rw  = crossbeam::queue::SegQueue::new();
    let mut i = 0;
    b.iter(|| {
        let _r = rw.push(i);
        i += 1;
    });
}

// test bench_queue_push    ... bench:          13 ns/iter (+/- 1)
#[bench]
fn bench_channel_send(b: &mut test::Bencher) {
    let (send, rev)  = crossbeam::channel::unbounded();
    let mut i = 0;
    b.iter(|| {
        let _r = send.send(i).unwrap();
        i += 1;
    });
}