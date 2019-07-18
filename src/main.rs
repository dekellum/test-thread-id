use std::thread;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};

use lazy_static::lazy_static;

fn main() {
    let tid = thread::current().id();
    println!("main thread id: {:?}, seed: {:08x} {:08x}",
             tid, prng_seed(), prng_seed());

    for _ in 0..8 {
        thread::spawn(|| {
            let tid = thread::current().id();
            println!("alt  thread id: {:?}, seed: {:08x} {:08x}",
                     tid, prng_seed(), prng_seed());
        }).join().unwrap();
    }
}


// Piggyback on libstd's mechanism for obtaining system randomness for default
// hasher, and hash on current thread id for a seed value.
fn prng_seed() -> u32 {
    lazy_static! {
        static ref RND_STATE: RandomState = RandomState::new();
    }

    let mut hasher = RND_STATE.build_hasher();
    thread::current().id().hash(&mut hasher);
    let hash: u64 = hasher.finish();
    let seed = (hash as u32) ^ ((hash >> 32) as u32);

    // ensure non-zero
    if seed == 0 {
        0x9b4e_6d25 // misc bits
    } else {
        seed
    }
}
