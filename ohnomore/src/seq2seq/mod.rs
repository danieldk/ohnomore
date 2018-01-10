mod writer;
pub use self::writer::{Collector, HDF5Collector};

mod batch;
pub use self::batch::{Batch, InputVector, InputVectorizer};
