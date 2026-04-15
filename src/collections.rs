use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;

pub type HashMap<K, V> = FxHashMap<K, V>;
pub type HashSet<V> = FxHashSet<V>;
pub type List<E> = SmallVec<E>;
pub use smallvec::smallvec as list;

pub const PIDS: usize = 512;
pub const TIDS_FULL: usize = 128;
pub const TIDS_CAPED: usize = 64;
pub const CONSUMER_CPUS: usize = 32;
pub const PENDING: usize = 16;
