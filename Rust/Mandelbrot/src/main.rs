mod displays {
    pub mod core;
    pub mod fs;
    pub mod live;
}

use displays::fs::Fs;
use displays::live::Live;
use displays::core::Display;

fn main() {
//    Fs::show();
    Live::show();
}

