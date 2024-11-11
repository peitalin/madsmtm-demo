use std::cell::RefCell;

#[repr(align(16))]
#[derive(Debug)]
struct MockedBlockchain {
    _size: [u8; 2512],
}

thread_local! {
    static BLOCKCHAIN_INTERFACE: RefCell<MockedBlockchain> = RefCell::new(MockedBlockchain {
        _size: [1; 2512],
    });
}

fn main() {
    BLOCKCHAIN_INTERFACE.with(|b| println!("{b:?}"));
}
