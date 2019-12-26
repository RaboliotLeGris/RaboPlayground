mod displays {
    pub mod core;
    pub mod fs;
//    mod live;
}

use displays::fs::Fs;
use displays::core::Display;

fn main() {
    Fs::show();
}

