#![allow(warnings)]

extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate error_chain;

use structopt::StructOpt;
use errors::*;
use std::path::{PathBuf, Path};
use std::env;

mod errors { error_chain! { } }

#[derive(StructOpt, Debug)]
#[structopt(name = "", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short = "r", long = "revdeps", help = "List crates by reverse deps")]
    revdeps: bool,

    #[structopt(short = "t", long = "trevdeps", help = "List crates by transitive reverse deps")]
    trevdeps: bool,

    #[structopt(short = "o", long = "one-point-oh", help = "List crates by 1.0 date")]
    one_point_oh: bool,

    /// An optional parameter, will be `None` if not present on the
    /// command line.
    #[structopt(help = "The local crates.io index, from ~/.cargo if omitted")]
    index: Option<String>,
}

quick_main!(run);

fn run() -> Result<()> {
    let opt = Opt::from_args();

    let index: Option<PathBuf> = opt.index.map(PathBuf::from);
    let index: Option<&Path> = index.as_ref().map(|p|&**p);
    
    if opt.revdeps {
        print_revdeps(index)?;
    } else if opt.trevdeps {
        print_trevdeps(index)?;
    } else if opt.one_point_oh {
        print_one_point_oh_crates(index)?;
    }
    
    Ok(())
}

// The name of the cargo index directory
const INDEX_DIR: &str = "github.com-1ecc6299db9ec823";

fn default_repo() -> Option<PathBuf> {
    cargo_home().map(|h| h.join("registry/index").join(INDEX_DIR))
}

// On windows, unlike std and cargo, rustup does *not* consider the
// HOME variable. If it did then the install dir would change
// depending on whether you happened to install under msys.
#[cfg(windows)]
pub fn home_dir() -> Option<PathBuf> {
    use std::ptr;
    use kernel32::{GetCurrentProcess, GetLastError, CloseHandle};
    use advapi32::OpenProcessToken;
    use userenv::GetUserProfileDirectoryW;
    use winapi::ERROR_INSUFFICIENT_BUFFER;
    use winapi::winnt::TOKEN_READ;
    use scopeguard;

    ::std::env::var_os("USERPROFILE").map(PathBuf::from).or_else(|| unsafe {
        let me = GetCurrentProcess();
        let mut token = ptr::null_mut();
        if OpenProcessToken(me, TOKEN_READ, &mut token) == 0 {
            return None;
        }
        let _g = scopeguard::guard(token, |h| { let _ = CloseHandle(*h); });
        fill_utf16_buf(|buf, mut sz| {
            match GetUserProfileDirectoryW(token, buf, &mut sz) {
                0 if GetLastError() != ERROR_INSUFFICIENT_BUFFER => 0,
                0 => sz,
                _ => sz - 1, // sz includes the null terminator
            }
        }, os2path).ok()
    })
}

#[cfg(windows)]
fn os2path(s: &[u16]) -> PathBuf {
    use std::os::windows::ffi::OsStringExt;
    PathBuf::from(OsString::from_wide(s))
}

#[cfg(windows)]
fn fill_utf16_buf<F1, F2, T>(mut f1: F1, f2: F2) -> io::Result<T>
    where F1: FnMut(*mut u16, DWORD) -> DWORD,
          F2: FnOnce(&[u16]) -> T
{
    use kernel32::{GetLastError, SetLastError};
    use winapi::{ERROR_INSUFFICIENT_BUFFER};

    // Start off with a stack buf but then spill over to the heap if we end up
    // needing more space.
    let mut stack_buf = [0u16; 512];
    let mut heap_buf = Vec::new();
    unsafe {
        let mut n = stack_buf.len();
        loop {
            let buf = if n <= stack_buf.len() {
                &mut stack_buf[..]
            } else {
                let extra = n - heap_buf.len();
                heap_buf.reserve(extra);
                heap_buf.set_len(n);
                &mut heap_buf[..]
            };

            // This function is typically called on windows API functions which
            // will return the correct length of the string, but these functions
            // also return the `0` on error. In some cases, however, the
            // returned "correct length" may actually be 0!
            //
            // To handle this case we call `SetLastError` to reset it to 0 and
            // then check it again if we get the "0 error value". If the "last
            // error" is still 0 then we interpret it as a 0 length buffer and
            // not an actual error.
            SetLastError(0);
            let k = match f1(buf.as_mut_ptr(), n as DWORD) {
                0 if GetLastError() == 0 => 0,
                0 => return Err(io::Error::last_os_error()),
                n => n,
            } as usize;
            if k == n && GetLastError() == ERROR_INSUFFICIENT_BUFFER {
                n *= 2;
            } else if k >= n {
                n = k;
            } else {
                return Ok(f2(&buf[..k]))
            }
        }
    }
}

#[cfg(unix)]
pub fn home_dir() -> Option<PathBuf> {
    ::std::env::home_dir()
}

pub fn cargo_home() -> Option<PathBuf> {
    let env_var = env::var_os("CARGO_HOME");

    // NB: During the multirust-rs -> rustup transition the install
    // dir changed from ~/.multirust/bin to ~/.cargo/bin. Because
    // multirust used to explicitly set CARGO_HOME it's possible to
    // get here when e.g. installing under `cargo run` and decide to
    // install to the wrong place. This check is to make the
    // multirust-rs to rustup upgrade seamless.
    let env_var = if let Some(v) = env_var {
       let vv = v.to_string_lossy().to_string();
       if vv.contains(".multirust/cargo") ||
            vv.contains(r".multirust\cargo") ||
            vv.trim().is_empty() {
           None
       } else {
           Some(v)
       }
    } else {
        None
    };

    let cwd = if let Some(cwd) = env::current_dir().ok() {
        cwd
    } else {
        return None;
    };
    let cargo_home = env_var.map(|home| {
        cwd.join(home)
    });
    let user_home = home_dir().map(|p| p.join(".cargo"));
    cargo_home.or(user_home)
}

fn print_revdeps(index: Option<&Path>) -> Result<()> {
    panic!()
}

fn print_trevdeps(index: Option<&Path>) -> Result<()> {
    panic!()
}

fn print_one_point_oh_crates(index: Option<&Path>) -> Result<()> {
    panic!()
}

fn load_registry_json(index: Option<&Path>) -> Result<()> {
    let index = index.map(PathBuf::from).or(default_repo())
        .ok_or(Error::from("no crate index repo specified and unable to locate it ~/.cargo"))?;

    panic!()
}
