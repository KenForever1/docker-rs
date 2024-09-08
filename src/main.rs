use clap::{Parser, Subcommand};
use nix::mount::{mount, MsFlags};
use nix::sched::{unshare, CloneFlags};
use nix::unistd::execv;
use std::ffi::{CStr, CString};
use std::io;
use std::process;
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(name = "demo_runner", version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: DockerCommand,
}
#[derive(Debug, Subcommand)]
enum DockerCommand {
    /// Help message for read.
    Init {
        /// An example option
        #[arg(index = 1, required = true)]
        cmd_args: Vec<String>,
    },
    /// Help message for read.
    Run {
        #[arg(index = 1, required = true)]
        cmds: Vec<String>,
        #[arg(long = "it", default_value = "false")]
        /// Print page index
        it: bool,
        #[arg(long = "mem", required = false)]
        /// Print page index
        mem: Option<i64>,
        #[arg(long = "cpu", required = false)]
        /// Print page index
        cpu: Option<f32>,
    },
}

fn run_container_init_process(command: &str, args: &[&str]) -> io::Result<()> {
    println!("command: {}, args: {}", command, args.join(" "));

    mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_PRIVATE | MsFlags::MS_REC,
        None::<&str>,
    )
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    println!("Mount proc filesystem...");
    // 设置默认的挂载标志
    let default_mount_flags = MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV;

    // 挂载proc文件系统
    mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        default_mount_flags,
        None::<&str>,
    )
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // 将命令和参数转换为CString
    let command_cstr = CString::new(command).unwrap();
    let args_cstr: Vec<CString> = args
        .iter()
        .map(|&arg| {
            println!("arg cstr: {}", arg);
            CString::new(arg).unwrap()
        })
        .collect();

    let args_cstr: Vec<&CStr> = args_cstr.iter().map(|cstr| cstr.as_c_str()).collect();
    // 创建一个新的向量来包含命令本身和参数
    let mut exec_args: Vec<&CStr> = vec![command_cstr.as_c_str()];
    exec_args.extend(&args_cstr);
    println!("Exec command...");
    // 执行命令
    execv(&command_cstr, &exec_args).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(())
}

fn new_parent_process(tty: bool, command: &str) -> std::io::Result<Command> {
    // split by first black space
    let (cmd, arg_str) = match command.split_once(' ') {
        Some((cmd, arg_str)) => (cmd, arg_str),
        None => (command, ""),
    };

    let args = vec!["init", cmd, "--", arg_str];

    // 设置命名空间隔离
    unshare(
        CloneFlags::CLONE_NEWUTS
            | CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWNET
            | CloneFlags::CLONE_NEWIPC,
    )
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut cmd = Command::new("/proc/self/exe");
    cmd.args(&args);

    if tty {
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
    }

    Ok(cmd)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        DockerCommand::Init { cmd_args } => {
            println!("call Init subcommand, args : {}", cmd_args.join(" "));
            // let command = "/bin/bash";
            let command = cmd_args[0].as_str();

            let args = cmd_args[1..].iter().map(|s| s.as_str()).collect::<Vec<_>>();

            if let Err(e) = run_container_init_process(command, &args) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        DockerCommand::Run { cmds, it, mem, cpu } => {
            println!("call Run subcommand , it : {}", it);
            if let Some(m) = mem {
                println!("mem : {}", m);
            }

            if let Some(c) = cpu {
                println!("cpu : {}", c);
            }

            let tty = it;
            let command = cmds.join(" ");

            match new_parent_process(tty, &command) {
                Ok(mut cmd) => {
                    if let Err(e) = cmd.spawn() {
                        eprintln!("Failed to start new process: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }

    Ok(())
}