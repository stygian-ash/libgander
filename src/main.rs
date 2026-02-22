use anyhow::Result;
use nix::sys::ptrace;
use nix::unistd;
use std::env;

struct Config {
	pid: unistd::Pid,
}

impl Config {
	fn build(args: &[String]) -> Result<Self> {
		Ok(Self {
			pid: unistd::Pid::from_raw(args[1].parse()?),
		})
	}
}

fn main() -> Result<()> {
	let args: Vec<_> = env::args().collect();
	let config = Config::build(&args)?;
	ptrace::attach(config.pid)?;
	nix::sys::wait::wait()?;
	let regs = ptrace::getregs(config.pid)?;
	let tib_addr = regs.gs_base;
	println!("tib_addr=0x{tib_addr:X}");
	let peb_addr = ptrace::read(config.pid, (tib_addr + 0x60) as ptrace::AddressType)?;
	println!("peb_addr=0x{peb_addr:X}");
	let ldr_data_addr = ptrace::read(config.pid, (peb_addr + 3 * 8) as ptrace::AddressType)?;
	println!("ldr_data_addr=0x{ldr_data_addr:X}");
	let first_addr = ptrace::read(config.pid, (ldr_data_addr + 0x10) as ptrace::AddressType)?;
	println!("first_addr=0x{first_addr:X}");
	let name_addr = ptrace::read(
		config.pid,
		(first_addr + 0x48 + 0x08) as ptrace::AddressType,
	)?;
	println!("name_addr=0x{name_addr:X}");
	let name = ptrace::read(config.pid, name_addr as ptrace::AddressType)?;
	println!("name=0x{name:X}");

	ptrace::detach(config.pid, None)?;
	Ok(())
}
