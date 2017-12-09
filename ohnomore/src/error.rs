use std::io;

use fst;

error_chain! {
    foreign_links {
        Fst(fst::Error);
        Io(io::Error);
    }
}
