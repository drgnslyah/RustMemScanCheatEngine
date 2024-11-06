mod processes;
use crate::processes::enum_proc::enum_proc;
use crate::processes::open_proc::Process;

fn main() {
    enum_proc()
        .unwrap()
        .into_iter()
        .for_each(|pid| match Process::open(pid) {
            Ok(proc) => match proc.name() {
                Ok(name) => println!("{}: {}", pid, name),
                Err(e) => println!("{}: (failed to get name: {})", pid, e),
            },
            Err(e) => eprintln!("Failed to open {}: {}", pid, e),
        });
}
