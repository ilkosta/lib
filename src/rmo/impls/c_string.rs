// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use str_one::{CStr, NoNullStr, ByteStr};
use str_two::{CString, NoNullString, ByteString};
use {Rmo, ToRmo};
use alloc::{MemPool};
use vec::{Vec};
use arch_fns::{memchr, all_bytes};
use base::{error};

impl<D, P> ToRmo<D, CStr, CString<P>> for CStr
    where P: MemPool,
{
    fn to_rmo_with<'a>(&'a self, _: D) -> Result<Rmo<'a, CStr, CString<P>>> {
        Ok(Rmo::Ref(self))
    }
}

impl<P> ToRmo<P, CStr, CString<P>> for [u8]
    where P: MemPool,
{
    fn to_rmo_with<'a>(&'a self, pool: P) -> Result<Rmo<'a, CStr, CString<P>>> {
        if let Some(idx) = memchr(self, 0) {
            if idx == self.len() - 1 || all_bytes(&self[idx+1..], 0) {
                Ok(unsafe { Rmo::Ref(CStr::from_bytes_unchecked(&self[..idx+1])) })
            } else {
                Err(error::InvalidArgument)
            }
        } else {
            let mut vec = Vec::with_pool(pool);
            try!(vec.reserve(self.len() + 1));
            vec.push_all(self.as_ref());
            vec.push(0);
            Ok(Rmo::Owned(unsafe { CString::from_bytes_unchecked(vec) }))
        }
    }
}

impl<P> ToRmo<P, CStr, CString<P>> for [i8]
    where P: MemPool,
{
    fn to_rmo_with<'a>(&'a self, pool: P) -> Result<Rmo<'a, CStr, CString<P>>> {
        let bytes: &[u8] = self.as_ref();
        bytes.to_rmo_with(pool)
    }
}

impl<P> ToRmo<P, CStr, CString<P>> for str
    where P: MemPool,
{
    fn to_rmo_with<'a>(&'a self, pool: P) -> Result<Rmo<'a, CStr, CString<P>>> {
        let bytes: &[u8] = self.as_ref();
        bytes.to_rmo_with(pool)
    }
}

impl<P> ToRmo<P, CStr, CString<P>> for NoNullStr
    where P: MemPool,
{
    fn to_rmo_with<'a>(&'a self, pool: P) -> Result<Rmo<'a, CStr, CString<P>>> {
        let mut vec = Vec::with_pool(pool);
        try!(vec.reserve(self.len() + 1));
        vec.push_all(self.as_ref());
        vec.push(0);
        Ok(Rmo::Owned(unsafe { CString::from_bytes_unchecked(vec) }))
    }
}

impl<A, P> ToRmo<P, CStr, CString<P>> for NoNullString<A>
    where A: MemPool,
          P: MemPool,
{
    fn to_rmo_with<'a>(&'a self, pool: P) -> Result<Rmo<'a, CStr, CString<P>>> {
        self.deref().to_rmo_with(pool)
    }
}

impl<P> ToRmo<P, CStr, CString<P>> for ByteStr
    where P: MemPool,
{
    fn to_rmo_with<'a>(&'a self, pool: P) -> Result<Rmo<'a, CStr, CString<P>>> {
        let bytes: &[u8] = self.as_ref();
        bytes.to_rmo_with(pool)
    }
}

impl<A, P> ToRmo<P, CStr, CString<P>> for ByteString<A>
    where A: MemPool,
          P: MemPool,
{
    fn to_rmo_with<'a>(&'a self, pool: P) -> Result<Rmo<'a, CStr, CString<P>>> {
        self.deref().to_rmo_with(pool)
    }
}
