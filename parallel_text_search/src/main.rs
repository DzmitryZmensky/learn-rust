use anyhow::{self, Context};
use enum_iterator::{self, cardinality, Sequence};
use std::{
    fs,
    io::{self, BufRead},
    thread, env, process,
};

struct SearchParams {
    file_path: String,
    search_text: String,
    thread_count: usize,
}

fn main() -> anyhow::Result<()> {
    let search_params = populate_search_params();
    let (sender, receiver) = crossbeam_channel::bounded::<String>(100);
    send_file_content(&search_params.file_path, sender)?;
    search_file_content(&search_params.search_text, receiver, search_params.thread_count);
    Ok(())
}

fn populate_search_params() -> SearchParams {
    #[derive(Sequence)]
    enum CmdlineArgPos {
        _ExeFileName,
        InputFilePath,
        SearchText,
    }
    let args: Vec<String> = env::args().collect();
    if args.len() != cardinality::<CmdlineArgPos>() {
        show_help();
        process::exit(-1);
    }
    SearchParams {
        file_path: args[CmdlineArgPos::InputFilePath as usize].clone(),
        search_text: args[CmdlineArgPos::SearchText as usize].clone(),
        thread_count: num_cpus::get(),
    }
}

fn show_help() {
    println!("usage: <input file path> <text to search>");
}

fn search_file_content(search_text: &str, receiver: crossbeam_channel::Receiver<String>, thread_count: usize) {
    thread::scope(|scope| {
        let mut join_handles = vec![];
        for _ in 0..thread_count {
            let cloned_receiver = receiver.clone();
            let join_handle = scope.spawn(move || {
                for line in cloned_receiver {
                    if line.contains(search_text) {
                        println!("{}", line);
                    }
                }
            });
            join_handles.push(join_handle);
        }
        for join_handle in join_handles {
            join_handle.join().unwrap();
        }
    });
}

fn send_file_content(file_path: &str, sender: crossbeam_channel::Sender<String>) -> anyhow::Result<()> {
    let f = fs::File::open(file_path).context(format!("Failed to open {}", file_path))?;
    let reader = io::BufReader::new(f);
    
    for line in reader.lines() {
        sender.send(line?)?;
    }

    drop(sender);
    Ok(())
}
