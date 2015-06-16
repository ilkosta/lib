// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use {Hash, Hasher};

impl Hash for char {
    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        h.write_u8(*self as u8);
    }

    fn stateful_hash_slice<H: Hasher>(val: &[Self], h: &mut H) {
        h.write_bytes(val.as_ref());
    }

    fn hash<H: Hasher>(&self, seed: H::Seed) -> H::Digest {
        H::hash_u8(*self as u8, seed)
    }

    fn hash_slice<H: Hasher>(val: &[Self], seed: H::Seed) -> H::Digest {
        H::hash_bytes(val.as_ref(), seed)
    }
}
