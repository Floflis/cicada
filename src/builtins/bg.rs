use std::io::Write;

use crate::jobc;
use crate::libc;
use crate::shell;
use crate::types;

pub fn run(sh: &mut shell::Shell, tokens: &types::Tokens) -> i32 {
    if sh.jobs.is_empty() {
        println_stderr!("cicada: bg: no job found");
        return 0;
    }

    let mut job_id = -1;
    if tokens.len() == 1 {
        for (gid, _) in sh.jobs.iter() {
            job_id = *gid;
            break;
        }
    }

    if tokens.len() >= 2 {
        let mut job_str = tokens[1].1.clone();
        if job_str.starts_with("%") {
            job_str = job_str.trim_start_matches('%').to_string();
        }

        match job_str.parse::<i32>() {
            Ok(n) => job_id = n,
            Err(_) => {
                println_stderr!("cicada: bg: invalid job id");
                return 1;
            }
        }
    }
    if job_id == -1 {
        println_stderr!("cicada: not job id found");
    }

    let gid: i32;

    {
        let mut result = sh.get_job_by_id(job_id);
        // fall back to find job by using prcess group id
        if let None = result {
            result = sh.get_job_by_gid(job_id);
        }

        match result {
            Some(job) => {
                let cmd = if job.cmd.ends_with(" &") {
                    job.cmd.clone()
                } else {
                    format!("{} &", job.cmd)
                };
                println_stderr!("{}", &cmd);

                unsafe {
                    libc::killpg(job.gid, libc::SIGCONT);
                    gid = job.gid;
                    if job.status == "Running" {
                        println_stderr!("cicada: bg: job {} already in background", job.id);
                        return 0;
                    }
                }
            }
            None => {
                println_stderr!("cicada: bg: no such job");
                return 1;
            }
        }
    }

    jobc::mark_job_as_running(sh, gid, true);
    return 0;
}
