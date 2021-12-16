/*
 *                  Cadence Data Soft Pvt. Ltd.
 *
*/


//pub mod filetype;

use std::convert::TryInto;
use std::error::Error;
use std::fs;
use std::fs::{File};
use std::io::{self, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::result;
use ring::digest::{Context, Digest, SHA256};
use walkdir::{DirEntry, WalkDir};
use bstr::ByteSlice;
use std::collections::HashMap;
use num256::uint256::Uint256;
use std::time::{Instant};

#[path = "./filetype.rs"]
mod filetype;

type Result<T> = result::Result<T, Box<dyn Error>>;

macro_rules! err {
    ($($tt:tt)*) => { Err(From::from(format!($($tt)*))) }
}

#[cfg(target_os = "macos")]
static DEFAULT_EXT: &str = "sh";
#[cfg(target_os = "linux")]
static DEFAULT_EXT: &str = "sh";
#[cfg(target_os = "windows")]
static DEFAULT_EXT: &str = "cmd";

use indicatif::{ProgressBar, ProgressStyle};

// struct Stats { 
//     audio_files: u64,
//     compressed_files: u64,
//     media_files: u64,
//     db_files: u64, 
//     email_files: u64,
//     exec_files: u64, 
//     font_files: u64, 
//     image_files: u64, 
//     internet_files: u64, 
//     presentation_files: u64, 
//     program_files: u64, 
//     spreadsheet_files: u64, 
//     sys_files: u64, 
//     video_files: u64, 
//     wordprocessor_files: u64, 
// }

// impl Stats {
//     pub fn new() -> Self {
//         Stats{
//             audio_files: 0,
//             compressed_files: 0,
//             media_files: 0,
//             db_files: 0, 
//             email_files: 0,
//             exec_files: 0, 
//             font_files: 0, 
//             image_files: 0, 
//             internet_files: 0, 
//             presentation_files: 0, 
//             program_files: 0, 
//             spreadsheet_files: 0, 
//             sys_files: 0, 
//             video_files: 0, 
//             wordprocessor_files: 0,         
//         }
//     }
// }

struct ReclaimStorage {
    unique_files0: HashMap<u64, Vec<DirEntry>>,
    duplicate_files: HashMap<Uint256, Vec<DirEntry>>,
    empty_folders: Vec<PathBuf>,
    files_processed: u64,
    possible_dup_files: u64,
    confirmed_dups: u64,
    total_reclaim: u64,
}

impl ReclaimStorage {

    pub fn new() -> Self {
        ReclaimStorage{
            unique_files0: HashMap::new(),
            duplicate_files: HashMap::new(),
            empty_folders: Vec::new(),
            files_processed: 0,
            //While maintaining stats, possible_dup_files get decremented first, and then it gets incremented. 
            //So let's init this to 1, otherwise, the application may panic. 
            //This issue is for the first decrement only.            
            possible_dup_files: 1,
            confirmed_dups: 0,
            total_reclaim: 0,            
        }
    }

    fn get_df(&mut self) -> &mut HashMap<Uint256, Vec<DirEntry>>{
        &mut self.duplicate_files
    }

    fn show_duplicates(&self) -> () {
        if self.duplicate_files.len() != 0 {
            let itr = self.duplicate_files.iter();
            for x in  itr{
                let vec: Vec<DirEntry> = x.1.to_vec();
                if vec.len() > 1 {
                    writeln!(io::stdout(), "KEY: {}", x.0.to_string()).unwrap();

                    let mut strpaths: String = "File names:\n".to_owned();
                    for ent in vec{
                        strpaths.push_str(ent.path().to_str().unwrap());
                        strpaths.push_str("\n");
                    }
                    writeln!(io::stdout(), " {}", strpaths).unwrap();                
                }
            }
        }else{
            writeln!(io::stdout(), "No duplicate found.").unwrap();
        }
    }

    fn gen_deldup_batch(&mut self, args: &Args) -> (){
        let file = File::create(&format!("deleteduplicates.{}", DEFAULT_EXT));
        match file {
            Err(_e) => (),
            Ok(fh) => {
                if self.get_df().len() != 0 {
                    for x in self.get_df().values_mut(){
                        if x.len() > 1 {
                            //Lets sort this list by access time. The one that is accessed last is seen as commented in the script file.
                            x.sort_by_key(|d| match d.metadata() {
                                Err(_e) => 0,
                                Ok(md) => match md.accessed(){
                                    Err(_e) => 0,
                                    Ok(st) => match st.elapsed() {
                                        Ok(val) => val.as_secs(),
                                        Err(_err) => 0,
                                    },
                                },
                            });
                            let len = x.len();
                            let mut counter = 0;
                            
                            while counter < len {
                                let de = x.get(counter).unwrap();
                                if counter == 0 {
                                    ReclaimStorage::gen_delete_statement(args, de, &fh, true);
                                }else{
                                    ReclaimStorage::gen_delete_statement(args, de, &fh, false);  
                                }
                                counter += 1;
                            }
                        }
                    }
                }      
            }
        }
    }

    fn gen_delempty_batch(&mut self, args: &Args) -> (){
        let file = File::create(&format!("delemptyfolders.{}", DEFAULT_EXT));
        match file {
            Err(_e) => (),
            Ok(fh) => {
                for ed in &self.empty_folders{
                    ReclaimStorage::gen_delete_empty_dir_statement(args, &ed, &fh)
                }                  
            }
        }
    }    

    #[cfg(unix)]
    fn gen_delete_empty_dir_statement(_args: &Args, dir: &PathBuf, mut fh: &File){
        let mut statment: String = String::from("rmdir ");
        statment.push_str("\"");
        statment.push_str(dir.to_str().unwrap());
        statment.push_str("\"");
        statment.push_str("\r\n");
        fh.write_all(statment.as_bytes()).unwrap();             
    }

    #[cfg(windows)]
    fn gen_delete_empty_dir_statement(_args: &Args, dir: &PathBuf, mut fh: &File){
        let mut statment: String = String::from("rmdir ");
        statment.push_str("\"");
        statment.push_str(dir.to_str().unwrap());
        statment.push_str("\"");
        statment.push_str("\r\n");
        fh.write_all(statment.as_bytes()).unwrap();             
    }

    #[cfg(unix)]
    fn gen_delete_statement(_args: &Args, de: &DirEntry, mut fh: &File, cmt: bool){
        let mut statment: String = String::from("");
        if cmt {
            statment.push_str("# ");
        }
        statment.push_str("rm ");
        statment.push_str("\"");
        statment.push_str(de.path().to_str().unwrap());
        statment.push_str("\"");
        statment.push_str("\r\n");
        fh.write_all(statment.as_bytes()).unwrap();   
    }

    #[cfg(windows)]
    fn gen_delete_statement(_args: &Args, de: &DirEntry, mut fh: &File, cmt: bool){
        let mut statment: String = String::from("");
        if cmt {
            statment.push_str("REM ");
        }
        statment.push_str("del ");
        statment.push_str("\"");
        statment.push_str(de.path().to_str().unwrap());
        statment.push_str("\"");
        statment.push_str("\r\n");
        fh.write_all(statment.as_bytes()).unwrap();
    }    

    #[inline]
    fn update_duplicate_list0 (&mut self, size: u64, dent: DirEntry) -> (){
        if self.unique_files0.contains_key(&size){
            self.unique_files0.get_mut(&size).unwrap().push(dent);
        }else{
            let mut newlst: Vec<DirEntry> = Vec::new();
            newlst.push(dent);            
            self.unique_files0.insert(size, newlst);
        }
    }

    fn traverse_path(&mut self, args: &Args, stats: &mut filetype::FileTypes) -> Result<()>
    {
        for dir in &args.dirs {
            self.find_duplicate_files(args, dir, stats).unwrap();
        }
        Ok(())
    }

    fn find_duplicate_files(
        &mut self,
        args: &Args,
        dir: &Path,
        stats: &mut filetype::FileTypes
    ) -> Result<()>
    {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(200);
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("/|\\- ")
                .template("{spinner:.dim.bold} {wide_msg}"),
        );

        let mut minfilesize: u64 = args.dup_minsize.unwrap().try_into().unwrap();
        minfilesize = minfilesize * 1024 * 1024;

        if args.dupfiles{
            //PASS1
            self.find_duplicates(args, dir, minfilesize, &pb);

            //PASS2
            if args.dupfiles {
                let keys = self.unique_files0.keys().cloned().collect::<Vec<_>>();
                for key in keys {
                    let ele = self.unique_files0.get(&key).unwrap();
                    let siz = key;

                    if ele.len() > 1 {
                        let mut vsingles:  Vec<Uint256> = Vec::new();
                        for de in ele.to_vec(){
                            match File::open( de.path(), ) {
                                Err(_err) => {
                                    //eprintln!("{}", err);
                                },
                                Ok(h) =>{
                                    let r = BufReader::new(h);
                                    let d = sha256_digest(r)?;
                                    let ui256_d =  MyUtil::bytes_2uint256(d);
                                    let ext: String = match de.path().extension(){
                                        Some(v) => v.to_str().unwrap().to_string(),
                                        None => String::from("noext"),
                                    };

                                    if self.duplicate_files.contains_key(&ui256_d){
                                        self.duplicate_files.get_mut(&ui256_d).unwrap().push(de);
                                        //We need this only here because that is what can be reclaimed
                                        self.total_reclaim += siz;
                                        self.confirmed_dups += 1;
                                        stats.inc(ext);
                                    }else{
                                        let mut newlst: Vec<DirEntry> = Vec::new();
                                        newlst.push(de);
                                        self.duplicate_files.insert(ui256_d.clone(), newlst);
                                        vsingles.push(ui256_d);
                                    }
                                }
                            };
                        }
                        //Remove single entries, saves memory
                        self.remove_single_entries(vsingles);
                        pb.set_message(format!("Phase II: Confirming duplicates out of {}, found {}, total space that can be reclaimed {}MB", self.possible_dup_files, self.confirmed_dups, self.total_reclaim/(1024*1024)));
                    }
                    
                    //We are done with this element lets remove it
                    self.unique_files0.remove(&siz);
                }
            }            
        } else if args.emptydirs{
            self.find_emptyfolders(args, dir, minfilesize, &pb);
        }

        pb.tick();
        pb.finish_and_clear();
        Ok(())
    }

    fn find_duplicates(&mut self, args: &Args, dir: &Path, minfilesize: u64, pb: &ProgressBar) {
        for result in args.walkdir(dir) {
            let dent = match dir_entry(result) {
                Some(value) => value,
                None => continue,
            };

            if dent.path().is_file() {
                if args.dupfiles {
                    //let input = fs::metadata(dent.path());
                    match fs::metadata(dent.path()) {
                        Err(_err) => {
                            //eprintln!("{}", err);
                        },
                        Ok(meta) =>{
                            let size = meta.len();
                            if minfilesize < size {
                                self.update_duplicate_list0(size, dent);
                                pb.set_message(format!("Phase I: Processing file {}", self.files_processed));
                            }
                        }
                    }
                }
            }
            self.files_processed += 1;
        }
    }

    fn find_emptyfolders(&mut self, args: &Args, dir: &Path, _minfilesize: u64, pb: &ProgressBar) {
        for result in args.walkdir(dir) {
            let dent = match result {
                Ok(dent) => dent,
                Err(_err) => {
                    continue;
                }
            };

            if args.emptydirs {
                if dent.path().is_dir() {
                    match dent.path().read_dir() {
                        Err(_err) => {
                            //eprintln!("{}", err);
                        },
                        Ok(mut v) => {
                            if v.next().is_none() {
                                self.empty_folders.push(dent.path().to_path_buf());
                                self.confirmed_dups += 1;
                            }
                            pb.set_message(format!("Processing folder {}, found {} empty folders", self.files_processed, self.confirmed_dups));
                        }
                    }
                }
            }

            self.files_processed += 1;
        }
    }

    fn remove_single_entries(&mut self, vsingles: Vec<Uint256>) {
        for dig in vsingles{
            let lst = self.duplicate_files.get_mut(&dig).unwrap();
            let lstlen:u64 = lst.len().try_into().unwrap();

            if lstlen == 1 {
                self.possible_dup_files -= 1;
                self.duplicate_files.remove(&dig);
            }
            self.possible_dup_files += lstlen;
        }
    }

    fn show_stats(&self, args: &Args, stats: &mut filetype::FileTypes) -> () {
        if args.dupfiles {
            writeln!(io::stdout(), "Total no. of files processes: {:?}", self.files_processed).unwrap();
            writeln!(io::stdout(), "Total no. of duplicate files: {:?}", self.confirmed_dups).unwrap();
            writeln!(io::stdout(), "Total amount of disk space can be reclaimed: {:?}MB", self.total_reclaim/(1024*1024)).unwrap();
            writeln!(io::stdout(), "To reclaimed the disk space run this generated script file \"deleteduplicates.{}\"\n. \
            You may review the script before before running it", DEFAULT_EXT).unwrap();
            self.show_filetype_stats(args, stats);
        }
        if args.emptydirs {
            writeln!(io::stdout(), "Total no. of folders processes: {:?}", self.files_processed).unwrap();
            writeln!(io::stdout(), "Total no. of empty folders found: {:?}", self.confirmed_dups).unwrap();
            writeln!(io::stdout(), "To remove the empty folders run this generated script file \"delemptyfolders.{}\"\n. \
            You may review the script before before running it", DEFAULT_EXT).unwrap();            
        }
    }

    fn show_filetype_stats(&self, _args: &Args, stats: &filetype::FileTypes){
        write!(io::stdout(), "The distribution of duplicates looks like this: ").unwrap();
        for (k, v) in &stats.known_cat{
            if *v != 0 {
                write!(io::stdout(), " {}: {:.3}%,", k,  (*v as f64)/(self.confirmed_dups as f64) * 100.0).unwrap();
            }
        }

        if stats.others > 0 {
            write!(io::stdout(), " other_files: {:.3}%", (stats.others as f64)/(self.confirmed_dups as f64) * 100.0).unwrap();
        }
    }
}

fn dir_entry (result: std::result::Result<DirEntry, walkdir::Error>) -> Option<DirEntry> {
    let dent: DirEntry = match result {
        Ok(dent1) => dent1,
        Err(_err) => {
            return None;
        }
    };
    Some(dent)
}

struct MyUtil;

impl MyUtil {

    fn is_system_le() -> bool{
        let v: u16 = 0x00ff;
        let first_octet: u8 = unsafe {
            let ptr = &v as *const u16;
            let ptr = ptr as *const u8;
            *ptr
        };
        return first_octet == 0x00ff;
    }

    fn bytes_2uint256(digest: Digest) -> Uint256 {
        let digbytes = digest.as_ref().as_bytes();
        if MyUtil::is_system_le() {
            return Uint256::from_bytes_le(digbytes);
        }else{
            return Uint256::from_bytes_be(digbytes);
        }
    }
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn try_main() -> Result<()> {

    let args = Args::parse()?;
    let mut stderr = io::stderr();
    let start = Instant::now();

    if args.dupfiles && args.emptydirs {
        return err!("Please choose one of the two flags --emptydirs OR --dupfiles {}{}", "", 0);
    }else{
        if args.dupfiles {
            println!("Searching for duplicate files...");
        }
        if args.emptydirs {
            println!("Searching for empty folders...");
        }        
    }


    let mut r = ReclaimStorage::new();
    let s = &mut filetype::FileTypes::new();

    r.traverse_path(&args, s).unwrap();
 
    if args.dispdup {
        r.show_duplicates();
    }

    if args.dupfiles {
        r.gen_deldup_batch(&args);
    }
    
    if args.emptydirs {
        r.gen_delempty_batch(&args)
    }

    r.show_stats(&args, s);

    if args.timeit {
        let since = Instant::now().duration_since(start);
        writeln!(stderr, "duration: {:?}", since)?;
    }

    Ok(())
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<ring::digest::Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

#[derive(Debug)]
struct Args {
    dirs: Vec<PathBuf>,
    timeit: bool,
    emptydirs: bool,
    dupfiles: bool,
    dispdup: bool,
    dup_minsize: Option<usize>,
}

impl Args {
    fn parse() -> Result<Args> {
        use clap::{crate_authors, crate_version, App, Arg};

        let parsed = App::new("Locate and delete empty folders and duplicate files using non-intrusive find-duplicate")
            .author(crate_authors!())
            .version(crate_version!())
            .max_term_width(100)
            .arg(Arg::with_name("dirs").multiple(true))
            .arg(
                Arg::with_name("timeit")
                    .long("timeit")
                    .short("t")
                    .help("Print timing info."),
            )
            .arg(
                Arg::with_name("dispdup")
                    .long("dispdup")
                    .short("i")
                    .help("Display duplicate files on concole."),
            ) 
            .arg(
                Arg::with_name("dupfiles")
                    .long("dupfiles")
                    .short("d")
                    .help("Generate batch file to delete duplicate files."),
            )        
            .arg(
                Arg::with_name("emptydirs")
                    .long("emptydirs")
                    .short("e")
                    .help("Generate batch file to delete empty folders."),
            )
            .arg(
                Arg::with_name("dup_minsize")
                    .long("dup_minsize")
                    .takes_value(true)
                    .default_value("5")
                    .help("Specify min file size above which duplicates should be searched for, in MB."),
            )
            .get_matches();

        let dirs = match parsed.values_of_os("dirs") {
            None => vec![PathBuf::from("./")],
            Some(dirs) => dirs.map(PathBuf::from).collect(),
        };
        Ok(Args {
            dirs: dirs,
            timeit: parsed.is_present("timeit"),
            dupfiles: parsed.is_present("dupfiles"),
            dispdup: parsed.is_present("dispdup"),
            emptydirs: parsed.is_present("emptydirs"),
            dup_minsize: parse_usize(&parsed, "dup_minsize")?,
        })
    }

    fn walkdir(&self, path: &Path) -> WalkDir {
        let walkdir = WalkDir::new(path);
        walkdir
    }
}

fn parse_usize(
    parsed: &clap::ArgMatches,
    flag: &str,
) -> Result<Option<usize>> {
    match parsed.value_of_lossy(flag) {
        None => Ok(None),
        Some(x) => x.parse().map(Some).or_else(|e| {
            err!("failed to parse --{} as a number: {}", flag, e)
        }),
    }
}
