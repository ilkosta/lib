// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use arch_fns::{memchr};
use str_one::{ToCStr, CStr, AsByteStr};
use cty::{
    BYTES_PER_SHORT, AF_UNIX, sa_family_t, UNIX_PATH_MAX,
};
use base::{error};
use fmt::{Debug, Write};

use addr::{SockAddr};

pub fn validate(data: &[u8]) -> Result<usize> {
    if data.len() < BYTES_PER_SHORT { return Err(error::InvalidArgument); }

    let mut ty: sa_family_t = 0;
    mem::copy(ty.as_mut(), data);
    if ty != AF_UNIX as sa_family_t { return Err(error::InvalidArgument); }

    // unnamed / autobind
    if data.len() == BYTES_PER_SHORT { return Ok(BYTES_PER_SHORT); }

    // abstract
    if data[BYTES_PER_SHORT] == 0 { return Ok(data.len()); }

    // path
    match memchr(&data[BYTES_PER_SHORT..], 0) {
        Some(p) => Ok(BYTES_PER_SHORT + p + 1),
        _ => Err(error::InvalidArgument),
    }
}

/// A Unix socket address.
pub struct UnixSockAddr { data: [u8] }

/// The type of a unix socket address.
///
/// = See also
///
/// * link:man:unix(7)
pub enum UnixAddrType {
    /// A socket address represented by a filesystem path.
    Path,
    /// The unnamed socket address.
    Unnamed,
    /// An abstract socket address.
    Abstract,
}

impl UnixSockAddr {
    /// Creates a Unix socket address from given bytes.
    ///
    /// [argument, bytes]
    /// The bytes that contain the socket address.
    pub fn from_bytes(bytes: &[u8]) -> Result<&UnixSockAddr> {
        validate(bytes).map(|l| unsafe { mem::cast(&bytes[..l]) })
    }

    /// Creates a mutable Unix socket address from given bytes.
    ///
    /// [argument, bytes]
    /// The bytes that contain the socket address.
    pub fn from_mut_bytes(bytes: &mut [u8]) -> Result<&mut UnixSockAddr> {
        validate(bytes).map(|l| unsafe { mem::cast(&mut bytes[..l]) })
    }

    /// Creates a Unix socket address from given bytes without validation.
    ///
    /// [argument, bytes]
    /// The bytes that contain the socket address.
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &UnixSockAddr {
        mem::cast(bytes)
    }

    /// Creates a mutable Unix socket address from given bytes without validation.
    ///
    /// [argument, bytes]
    /// The bytes that contain the socket address.
    pub unsafe fn from_mut_bytes_unchecked(bytes: &mut [u8]) -> &mut UnixSockAddr {
        mem::cast(bytes)
    }

    /// Creates a new unnamed Unix socket address.
    ///
    /// [argument, bytes]
    /// The buffer in which the address will be stored.
    pub fn from_unnamed(buf: &mut [u8]) -> Result<&mut UnixSockAddr> {
        if buf.len() >= BYTES_PER_SHORT {
            mem::copy(buf, (AF_UNIX as sa_family_t).as_ref());
            Ok(unsafe { mem::cast(&mut buf[..BYTES_PER_SHORT]) })
        } else {
            Err(error::NoMemory)
        }
    }

    /// Creates a new abstract Unix socket address.
    ///
    /// [argument, bytes]
    /// The buffer in which the address will be stored.
    ///
    /// [argument, name]
    /// The address of the socket address.
    pub fn from_abstract<'a, T: ?Sized>(buf: &'a mut [u8],
                                        name: &T) -> Result<&'a mut UnixSockAddr>
        where T: AsRef<[u8]>
    {
        let name = name.as_ref();
        let len = BYTES_PER_SHORT + 1 + name.len();
        if len > BYTES_PER_SHORT + UNIX_PATH_MAX {
            Err(error::InvalidArgument)
        } else if len > buf.len() {
            Err(error::NoMemory)
        } else {
            mem::copy(buf, (AF_UNIX as sa_family_t).as_ref());
            buf[BYTES_PER_SHORT] = 0;
            mem::copy(&mut buf[BYTES_PER_SHORT + 1 ..], name);
            Ok(unsafe { mem::cast(&mut buf[..len]) })
        }
    }

    /// Creates a new path Unix socket address.
    ///
    /// [argument, buf]
    /// The buffer in which the address will be stored.
    ///
    /// [argument, path]
    /// The path of the socket address.
    pub fn from_path<T>(buf: &mut [u8], path: T) -> Result<&mut UnixSockAddr>
        where T: ToCStr,
    {
        if buf.len() < BYTES_PER_SHORT {
            return Err(error::NoMemory);
        }
        mem::copy(buf, (AF_UNIX as sa_family_t).as_ref());
        let len = BYTES_PER_SHORT + 1 + {
            let buf = &mut buf[BYTES_PER_SHORT..];
            let buf_addr = buf.as_ptr() as usize;
            let cstr = try!(path.to_cstr(buf));
            if cstr.as_ptr() as usize != buf_addr {
                return Err(error::InvalidArgument);
            }
            cstr.len()
        };
        if len == BYTES_PER_SHORT + 1 || len > BYTES_PER_SHORT + UNIX_PATH_MAX {
            Err(error::InvalidArgument)
        } else {
            Ok(unsafe { mem::cast(&mut buf[..len]) })
        }
    }

    /// Returns the address type of the socket address.
    pub fn addr_type(&self) -> UnixAddrType {
        if self.data.len() == BYTES_PER_SHORT { return UnixAddrType::Unnamed; }
        if self.data[BYTES_PER_SHORT] == 0 { return UnixAddrType::Abstract; }
        UnixAddrType::Path
    }

    /// Returns the path of a path Unix socket address.
    pub fn as_path(&self) -> Result<&CStr> {
        match self.addr_type() {
            UnixAddrType::Path => Ok(unsafe {
                CStr::from_bytes_unchecked(&self.data[BYTES_PER_SHORT..])
            }),
            _ => Err(error::InvalidArgument),
        }
    }

    /// Returns the mutable path of a path Unix socket address.
    pub fn as_mut_path(&mut self) -> Result<&mut CStr> {
        match self.addr_type() {
            UnixAddrType::Path => Ok(unsafe {
                CStr::from_mut_bytes_unchecked(&mut self.data[BYTES_PER_SHORT..])
            }),
            _ => Err(error::InvalidArgument),
        }
    }

    /// Returns the abstract address of an abstract Unix socket address.
    pub fn as_abstract(&self) -> Result<&[u8]> {
        match self.addr_type() {
            UnixAddrType::Abstract => Ok(&self.data[BYTES_PER_SHORT + 1..]),
            _ => Err(error::InvalidArgument),
        }
    }

    /// Returns the mutable abstract address of an abstract Unix socket address.
    pub fn as_mut_abstract(&mut self) -> Result<&mut [u8]> {
        match self.addr_type() {
            UnixAddrType::Abstract => Ok(&mut self.data[BYTES_PER_SHORT + 1..]),
            _ => Err(error::InvalidArgument),
        }
    }
}

impl AsRef<[u8]> for UnixSockAddr {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl AsRef<SockAddr> for UnixSockAddr {
    fn as_ref(&self) -> &SockAddr {
        unsafe { mem::cast(self) }
    }
}

impl AsMut<SockAddr> for UnixSockAddr {
    fn as_mut(&mut self) -> &mut SockAddr {
        unsafe { mem::cast(self) }
    }
}

impl Debug for UnixSockAddr {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        match self.addr_type() {
            UnixAddrType::Path => {
                write!(w, "UnixSockAddr {{ path: {:?} }}", self.as_path().unwrap())
            },
            UnixAddrType::Unnamed => w.write(b"UnixSockAddr { unnamed }").ignore_ok(),
            UnixAddrType::Abstract => {
                write!(w, "UnixSockAddr {{ abstract: {:?} }}",
                       self.as_abstract().unwrap().as_byte_str())
            },
        }
    }
}
