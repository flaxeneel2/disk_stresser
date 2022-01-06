use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::time::UNIX_EPOCH;
use clap::Parser;

#[derive(Parser,Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    threads: i64,
    #[clap(short, long)]
    bytes: String,
    #[clap(short, long)]
    delete_file: bool,
    #[clap(short, long)]
    chunk_size: i64
}

fn main() {
    let args = Args::parse();
    let start = std::time::SystemTime::now();
    let file_writer = std::fs::File::create("test").unwrap();
    let (sender, receiver) = std::sync::mpsc::channel();
    let byte_size = convert_to_bytes(args.bytes);
    let chunk_size = convert_to_bytes(args.chunk_size);
    let runs_per_thread: i64 = (byte_size /chunk_size)/args.threads;
    for _ in 0..args.threads {
        let buf = std::io::BufWriter::new(file_writer.try_clone().unwrap());
        let sen_clone = sender.clone();
        let data = String::from_utf8(vec![b'a'; chunk_size.try_into().unwrap()]).unwrap();
        let _ = std::thread::spawn(move || thread(buf, data, runs_per_thread, sen_clone));
    }
    let mut counter = args.threads.clone();
    let _ = std::thread::spawn(move || {
        for i in receiver {
            counter -= 1;
            println!("{}! Threads left: {}", i, counter);
            if counter == 0 {
                let end = std::time::SystemTime::now();
                println!("Time in MS: {}", (end.duration_since(UNIX_EPOCH).unwrap().as_millis() - start.duration_since(UNIX_EPOCH).unwrap().as_millis()));
                if args.delete_file {
                    println!("{}", "Deleting the file");
                    let _ = std::fs::remove_file("test");
                }
                exit(0)
            }
        }
    }).join().unwrap();
}

fn thread(mut file_buffer: std::io::BufWriter<File>, data: String, runs: i64, sender: std::sync::mpsc::Sender<String>) {
    for _ in 0..runs {
        let _ = file_buffer.write(data.as_bytes());
    }
    sender.send(String::from("Done")).unwrap();
}

fn convert_to_bytes(unconverted: String) -> i64 {
    let unit = unconverted.chars().last().unwrap().to_uppercase().last().unwrap();
    let returner: i64 =
        match unit {
            'K' => unconverted[0..unconverted.len()-1].parse::<i64>().unwrap()*1024,
            'M' => unconverted[0..unconverted.len()-1].parse::<i64>().unwrap()*1024*1024,
            'G' => unconverted[0..unconverted.len()-1].parse::<i64>().unwrap()*1024*1024*1024,
            'T' => unconverted[0..unconverted.len()-1].parse::<i64>().unwrap()*1024*1024*1024*1024,
            _ => { unconverted.parse::<i64>().unwrap() }
        };
    returner
}