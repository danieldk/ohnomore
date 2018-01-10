use std::io;

use fst;
use hdf5;

error_chain! {
    foreign_links {
        Fst(fst::Error);
        HDF5(hdf5::Error);
        Io(io::Error);
    }
}
