// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use linux_core::errno::{Errno};
pub use linux_core::errno::{ToCInt};

pub use linux_core::errno::{AccessDenied};
pub use linux_core::errno::{AddrFamilyNotSupported};
pub use linux_core::errno::{AddressInUse};
pub use linux_core::errno::{AddressNotAvailable};
pub use linux_core::errno::{AdvertiseError};
pub use linux_core::errno::{AlreadyInProgress};
pub use linux_core::errno::{BadFileDesc};
pub use linux_core::errno::{BadFileDescState};
pub use linux_core::errno::{BadFontFileFormat};
pub use linux_core::errno::{BrokenPipe};
pub use linux_core::errno::{ChannelOutOfRange};
pub use linux_core::errno::{CommunitacionError};
pub use linux_core::errno::{ConnectionAborted};
pub use linux_core::errno::{ConnectionRefused};
pub use linux_core::errno::{ConnectionReset};
pub use linux_core::errno::{CrossFileSystemLink};
pub use linux_core::errno::{DeadlockAvoided};
pub use linux_core::errno::{DeviceFull};
pub use linux_core::errno::{DiskQuota};
pub use linux_core::errno::{DoesNotExist};
pub use linux_core::errno::{DomainError};
pub use linux_core::errno::{ExchangeFull};
pub use linux_core::errno::{ExecutableBusy};
pub use linux_core::errno::{FileExists};
pub use linux_core::errno::{FileTooBig};
pub use linux_core::errno::{HardwarePoison};
pub use linux_core::errno::{HostCrashed};
pub use linux_core::errno::{HostDown};
pub use linux_core::errno::{HostUnreachable};
pub use linux_core::errno::{IdentifierRemoved};
pub use linux_core::errno::{InputOutput};
pub use linux_core::errno::{Interrupted};
pub use linux_core::errno::{InvalidArgument};
pub use linux_core::errno::{InvalidExchange};
pub use linux_core::errno::{InvalidExecutable};
pub use linux_core::errno::{InvalidPointer};
pub use linux_core::errno::{InvalidReqDesc};
pub use linux_core::errno::{InvalidRequestCode};
pub use linux_core::errno::{InvalidSeek};
pub use linux_core::errno::{InvalidSequence};
pub use linux_core::errno::{InvalidSlot};
pub use linux_core::errno::{IrrecoverableState};
pub use linux_core::errno::{IsADirectory};
pub use linux_core::errno::{KernelBuffersBusy};
pub use linux_core::errno::{KeyExpired};
pub use linux_core::errno::{KeyNotAvailable};
pub use linux_core::errno::{KeyRejected};
pub use linux_core::errno::{KeyRevoked};
pub use linux_core::errno::{Level2Halted};
pub use linux_core::errno::{Level2NotSync};
pub use linux_core::errno::{Level3Halted};
pub use linux_core::errno::{Level3Reset};
pub use linux_core::errno::{LibSectionCorrupted};
pub use linux_core::errno::{LinkNumberOutOfRange};
pub use linux_core::errno::{LinkSevered};
pub use linux_core::errno::{MessageSize};
pub use linux_core::errno::{MultihopAttempted};
pub use linux_core::errno::{NamedTypeFile};
pub use linux_core::errno::{NeedsCleaning};
pub use linux_core::errno::{NetworkDown};
pub use linux_core::errno::{NetworkUnreachable};
pub use linux_core::errno::{NoAnode};
pub use linux_core::errno::{NoBlockSpecialFile};
pub use linux_core::errno::{NoCSIStructAvailable};
pub use linux_core::errno::{NoChildProcesses};
pub use linux_core::errno::{NoDataAvailable};
pub use linux_core::errno::{NoDefaultDestination};
pub use linux_core::errno::{NoLocksAvailable};
pub use linux_core::errno::{NoMedium};
pub use linux_core::errno::{NoMemory};
pub use linux_core::errno::{NoMessageOfType};
pub use linux_core::errno::{NoSuchDevice};
pub use linux_core::errno::{NoSuchProcess};
pub use linux_core::errno::{NoXENIXSemaphores};
pub use linux_core::errno::{NotADataMessage};
pub use linux_core::errno::{NotADirectory};
pub use linux_core::errno::{NotASocket};
pub use linux_core::errno::{NotAStream};
pub use linux_core::errno::{NotATerminal};
pub use linux_core::errno::{NotEmpty};
pub use linux_core::errno::{NotImplemented};
pub use linux_core::errno::{NotOnNetwork};
pub use linux_core::errno::{NotPermitted};
pub use linux_core::errno::{NotSupported};
pub use linux_core::errno::{NotUnique};
pub use linux_core::errno::{NotXENIX};
pub use linux_core::errno::{ObjectIsRemote};
pub use linux_core::errno::{OpNotSupported};
pub use linux_core::errno::{OperationCanceled};
pub use linux_core::errno::{OperationInitiated};
pub use linux_core::errno::{OutOfStreamsResources};
pub use linux_core::errno::{Overflow};
pub use linux_core::errno::{OwnerDied};
pub use linux_core::errno::{PackageNotInstalled};
pub use linux_core::errno::{PathTooLong};
pub use linux_core::errno::{ProcessFileLimit};
pub use linux_core::errno::{ProtoDriverNotAttached};
pub use linux_core::errno::{ProtoFamilyNotSupported};
pub use linux_core::errno::{ProtoNotSupported};
pub use linux_core::errno::{ProtoNotSupported2};
pub use linux_core::errno::{ProtocolError};
pub use linux_core::errno::{RFKill};
pub use linux_core::errno::{RFSError};
pub use linux_core::errno::{RangeError};
pub use linux_core::errno::{ReadOnlyFileSystem};
pub use linux_core::errno::{RemoteAddrChanged};
pub use linux_core::errno::{RemoteIOError};
pub use linux_core::errno::{ResourceBusy};
pub use linux_core::errno::{Restart};
pub use linux_core::errno::{RustError};
pub use linux_core::errno::{SharedLibCorrupted};
pub use linux_core::errno::{SharedLibExec};
pub use linux_core::errno::{SharedLibInaccessible};
pub use linux_core::errno::{SocketConnected};
pub use linux_core::errno::{SocketNotConnected};
pub use linux_core::errno::{SocketShutDown};
pub use linux_core::errno::{SocketTimedOut};
pub use linux_core::errno::{SocketTypeNotSupported};
pub use linux_core::errno::{SrmountError};
pub use linux_core::errno::{StaleFileHandle};
pub use linux_core::errno::{StreamPipeError};
pub use linux_core::errno::{SystemFileLimit};
pub use linux_core::errno::{TimerExpired};
pub use linux_core::errno::{TooManyArguemnts};
pub use linux_core::errno::{TooManyLinks};
pub use linux_core::errno::{TooManyReferences};
pub use linux_core::errno::{TooManySharedLibs};
pub use linux_core::errno::{TooManySymlinks};
pub use linux_core::errno::{TooManyUsers};
pub use linux_core::errno::{WouldBlock};
pub use linux_core::errno::{WrongDeviceType};
pub use linux_core::errno::{WrongMediumType};
