struct MyWrapper(*mut f32);

unsafe impl Send for MyWrapper {}
unsafe impl Sync for MyWrapper {}

impl MyWrapper {
    fn get_slice(&self, len: usize) -> &mut [f32] {
        unsafe { std::slice::from_raw_parts_mut(self.0, len) }
    }
}

fn main() {
    const CHUNK_SIZE: usize = 0x100000;
    const NUM_THREADS: usize = 4;
    let mut v = vec![0.0_f32; CHUNK_SIZE * NUM_THREADS];

    let thread_pool = (0..NUM_THREADS).map(|i| {
        let w = MyWrapper(v[(CHUNK_SIZE * i)..].as_mut_ptr());
        std::thread::spawn(move || {
            let subslice = w.get_slice(CHUNK_SIZE);
            for j in 0..CHUNK_SIZE {
                subslice[j] = 1.0;
            }
        })
    });

    thread_pool.for_each(|t| t.join().unwrap());

    assert!(v.iter().all(|x| *x == 1.0));
}
