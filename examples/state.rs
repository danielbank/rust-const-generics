#![feature(const_generics)]

struct State<const S: &'static str> {
    total: u32,
}

impl State<"init"> {
    pub fn new() -> Self {
        Self { total: 0 }
    }

    pub fn accumulate(self) -> State<"accumulate"> {
        State { total: self.total }
    }
}

impl State<"accumulate"> {
    pub fn add(&mut self, add: u32) {
        self.total += add
    }

    pub fn freeze(self) -> State<"freeze"> {
        State { total: self.total }
    }
}

impl State<"freeze"> {
    pub fn unwrap(self) -> u32 {
        self.total
    }
}

fn main() {
    let init = State::new();

    let mut acc = init.accumulate();
    acc.add(10);
    acc.add(20);

    let frozen = acc.freeze();
    // frozen.add(120); // nope
    let val = frozen.unwrap();

    println!("{}", val);
}
