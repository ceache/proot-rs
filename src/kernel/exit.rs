use kernel::groups::{SyscallGroup, syscall_group_from_sysnum};
use kernel::execve;
use kernel::heap::*;
use kernel::ptrace::*;
use kernel::socket::*;
use kernel::standard::*;
use errors::Error;
use register::{Word, Current};
use process::tracee::Tracee;

#[allow(dead_code)]
pub enum SyscallExitResult {
    /// Indicates a new value for the syscall result, that is not an error.
    /// The SYS_RESULT register will be poked and changed to the new value.
    Value(Word),
    /// Indicates an error that happened during the translation.
    /// The SYS_RESULT register will be poked and changed to the new value.
    /// More precisely, the new value will be `-errno`.
    Error(Error),
    /// The SYS_RESULT register won't be overwritten.
    None,
}

pub fn translate(tracee: &Tracee) -> SyscallExitResult {
    let syscall_number = tracee.regs.get_sys_num(Current);
    let syscall_group = syscall_group_from_sysnum(syscall_number);

    println!("exit  \t({:?}, \t{:?})", syscall_number, syscall_group);

    match syscall_group {
        SyscallGroup::Brk => brk::exit(),
        SyscallGroup::GetCwd => getcwd::exit(),
        SyscallGroup::Accept => accept::exit(),
        SyscallGroup::GetSockOrPeerName => get_sockorpeer_name::exit(),
        SyscallGroup::SocketCall => socketcall::exit(),
        SyscallGroup::Chdir => chdir::exit(),
        SyscallGroup::Rename => link_rename::exit(),
        SyscallGroup::RenameAt => rename_at::exit(),
        SyscallGroup::ReadLink => readlink_at::exit(),
        SyscallGroup::ReadLinkAt => readlink_at::exit(),
        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        SyscallGroup::Uname => uname::exit(),
        SyscallGroup::Execve => execve::exit(tracee),
        SyscallGroup::Ptrace => ptrace::exit(),
        SyscallGroup::Wait => wait::exit(),
        _ => SyscallExitResult::None,
    }
}
