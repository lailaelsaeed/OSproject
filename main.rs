#![allow(unused)]
use std::io::{self, Write};
use procfs::process::Process;
use nix::sys::signal::{Signal, kill, SIGSTOP, SIGCONT};
use nix::unistd::Pid;
use clap::Parser;
use inquire::list_option::ListOption;
use inquire::Select;
use inquire::InquireError;

fn killing(pid: i32){
    
    let signal = Signal::SIGTERM;
    if let Ok(_) = kill(Pid::from_raw(pid), signal) {
        println!("Successfully killed process {}", pid);
    } else {
        println!("Failed to kill process {}", pid);
    }

}
fn pausing (pid: i32){
    let signal = SIGSTOP;
    if let Ok(_) = kill(Pid::from_raw(pid), signal) {
            println!("Successfully paused process {}", pid);
    } else {
            println!("Failed to paused process {}", pid);
    }
}

fn resuming(pid: i32){
    let signal = SIGCONT;
    if let Ok(_) = kill(Pid::from_raw(pid), signal) {
            println!("Successfully resumed process {}", pid);
    } else {
            println!("Failed to resume process {}", pid);
    }
}

fn filtering(filter_state: char){
    let tps = procfs::ticks_per_second();
    let page_size = procfs::page_size();
    let mut total_time = 0 as f32;
    let mut total_mem = 0 as f32;
    for prc in procfs::process::all_processes().unwrap() {
        if let Ok(process) = prc {
            if let Ok(stat) = process.stat() {
                let t = (stat.utime + stat.stime) as f32 / (tps as f32);
                total_time += t as f32;
                if let Ok(mem) = process.statm() {
                    total_mem += mem.resident as f32 * page_size as f32 / 1024.0 / 1024.0; // Convert to MB
                }
            }
        }
    }

    println!("{: >10} {: <8} {: >8} {: >8} {: >8} {}", "PID", "STATE","TIME", "CPU%", "MEM%", "COMMAND");

    for prc in procfs::process::all_processes().unwrap() {
        if let Ok(process) = prc {
            if let Ok(stat) = process.stat() {
                let elapsed_time = process.stat().unwrap().starttime as f32 / tps as f32;
                let total_time_process = (stat.utime + stat.stime) as f32 / (tps as f32);
                let state = stat.state;
                let cpu_usage = 100.0 * total_time_process / total_time;

                if state == filter_state || filter_state == 'A' {
                    if let Ok(mem) = process.statm() {
                        let mem_usage = 100.0 * mem.resident as f32 * page_size as u64 as f32 / 1024.0 / 1024.0 / total_mem;
                        let cmdline = stat.comm;
                        println!("{: >10} {: <8} {: >8} {: >8.2} {: >8.2} {}", stat.pid, state, total_time_process, cpu_usage, mem_usage, cmdline);
                    }
                }
            }
        }
    }
    
}
fn main() {
    let me = Process::myself().unwrap();
    let me_stat = me.stat().unwrap();
    //initialize the program
    let mut program = Program::new();
    //initialising the program
    let mut tui = Tui::new();
    //running the tui
    tui.run();
    
    let options: Vec<&str> = vec!["Filter", "Resume",  "Pause", "Kill"];
 let ans: Result<&str, InquireError> = Select::new("What would you like to do?", options).prompt();

 let mut choice = match ans {
    Ok(choice) => choice,
    Err(_) => return,
};

    //let args = Cli::parse();
    
    if (choice == "Filter") {
        let choosefilter: Vec<&str> = vec!["All", "Interruptable Sleep", "Running/Runnable", "Uninterruptible Sleep", "Stopped", "Zombie"];
        let ans2: Result<&str, InquireError> = Select::new("Which state?", choosefilter).prompt();
       
        let filter_value = match ans2 {
           Ok(filter_value) => filter_value,
           Err(_) => return,
        };
      
        let filter_state =filter_value.trim().chars().next().unwrap_or('*');
        filtering(filter_state);
    }
    if (choice == "Pause") {
        println!("PID: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let pid = input.trim().parse::<i32>().unwrap();
            pausing(pid);
       
    }
   if (choice == "Resume") {
        println!("PID: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let pid = input.trim().parse::<i32>().unwrap();
        resuming(pid);
       
    }
    if (choice == "Kill") {
        println!("PID: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let pid = input.trim().parse::<i32>().unwrap();
        killing(pid);       
    }
}
//lesa
mod tui;

fn main() {
    // Initialize your program
    let mut program = Program::new();

    // Initialize your TUI
    let mut tui = Tui::new();

    // Run the TUI
    tui.run();
}

