pub mod parser;

use fxhash::FxBuildHasher;
use indexmap::IndexMap;
pub type Map<K, V> = IndexMap<K, V, FxBuildHasher>;