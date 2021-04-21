use std::io::Read;
use std::io::Write;
use std::os::unix::fs::FileTypeExt;
use std::path::PathBuf;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    match args.len() {
        2 => {
            travess_for_ioerrors(args[1].clone().into());
            print!("\r\x1B[0K");
            std::io::stdout().flush().unwrap();
        }
        _ => println!("Usage:\n  {} BTRFS_MOUNT_POINT", args[0]),
    }
}

fn travess_for_ioerrors(path: PathBuf) {
    match path.metadata() {
        Ok(metadata) => {
            let ft = metadata.file_type();
            if ft.is_block_device()
                || ft.is_char_device()
                || ft.is_fifo()
                || ft.is_socket()
                || ft.is_symlink()
            {
                // Ignore non-content files
            } else if ft.is_dir() {
                for subpath in path.read_dir().unwrap() {
                    // handle corrupt file listing instead of blindly unwrapping
                    travess_for_ioerrors(subpath.unwrap().path()); // handle corrupt file entry instead of blindly unwrapping
                }
            } else if ft.is_file() {
                deal_with_file(path);
            } else {
                // Skip unexpected file
            }
        }
        Err(_) => {
            eprintln!("\r{}\x1B[0K", path.to_string_lossy());
            std::io::stdout().flush().unwrap();
            // handle corrupt metadata
        }
    }
}

fn deal_with_file(path: PathBuf) {
    print!("\r{}\x1B[0K", path.to_string_lossy());
    std::io::stdout().flush().unwrap();
    let mut file = std::fs::File::open(&path).unwrap();
    let mut list_of_chunks = Vec::new();

    let chunk_size = 0x4000;

    loop {
        let mut chunk = Vec::with_capacity(chunk_size);
        let rn = std::io::Read::by_ref(&mut file)
            .take(chunk_size as u64)
            .read_to_end(&mut chunk);
        match rn {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                list_of_chunks.push(chunk);
                if n < chunk_size {
                    break;
                }
            }
            Err(e) => {
                if Some(5) == e.raw_os_error() {
                    print!("[x]");
                    std::io::stdout().flush().unwrap();
                    deal_with_bad_files(path);
                    println!("[v]");
                    std::io::stdout().flush().unwrap();
                }
                break;
            }
        }
    }
}

fn deal_with_bad_files(path: PathBuf) {
    if path.is_dir() {
        std::fs::remove_dir_all(path).unwrap_or(());
    } else {
        std::fs::remove_file(path).unwrap_or(());
    }
}
