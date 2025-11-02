use uuid::{Builder, Uuid};

pub fn new_v4() -> Uuid {
    let mut random_bytes: [u8; 16] = [0; 16];
    getrandom::fill(&mut random_bytes).unwrap();
    Builder::from_random_bytes(random_bytes).into_uuid()
}
