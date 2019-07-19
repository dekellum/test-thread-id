use std::thread;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};

use lazy_static::lazy_static;

fn main() {
    println!("zero seed out: {}", sample(0));
    println!("one  seed out: {}", sample(1));

    let tid = thread::current().id();
    let ms = sample(prng_seed());
    println!("main  thread id: {:?}, seed: {:08x}, out: {}",
             tid, prng_seed(), ms);
    (1..9)
        .map(|l| thread::spawn(move || {
            let tid = thread::current().id();
            let ts = sample(prng_seed());
            println!("alt {} thread id: {:?}, seed: {:08x}, out: {}",
                     l, tid, prng_seed(), ts);
        }))
        .for_each(|t| t.join().unwrap());
}

// Return a thread-specific, 32-bit, non-zero seed value suitable for a 32-bit
// PRNG. This uses one libstd RandomState for a default hasher and hashes on
// the current thread ID to obtain an unpredictable, collision resistant seed.
fn prng_seed() -> u32 {
    // This obtains a small number of random bytes from the host system (for
    // example, on unix via getrandom(2)) in order to seed an unpredictable and
    // HashDoS resistant 64-bit hash function (currently: `SipHasher13` with
    // 128-bit state). We only need one of these, to make the seeds for all
    // process threads different via hashed IDs, collision resistant, and
    // unpredictable.
    lazy_static! {
        static ref RND_STATE: RandomState = RandomState::new();
    }

    // Hash the current thread ID to produce a u32 value
    let mut hasher = RND_STATE.build_hasher();
    thread::current().id().hash(&mut hasher);
    let hash: u64 = hasher.finish();
    let seed = (hash as u32) ^ ((hash >> 32) as u32);

    // Ensure non-zero seed (Xorshift yields only zeroes for that seed)
    if seed == 0 {
        0x9b4e_6d25 // misc bits, could be any non-zero
    } else {
        seed
    }
}

fn sample(mut x: u32) -> String {
    (0..10)
        .map(|_| { x = xorshift(x); x })
        .map(|v| format!("{:08x}", v))
        .collect::<Vec<String>>()
        .join(" ")
}

fn xorshift(mut x: u32) -> u32 {
    x ^= x << 13;
    x ^= x >> 17;
    x ^ x << 5
}
