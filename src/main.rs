mod travel;

use regex::Regex;
use std::env;
// use std::fs;
use std::path::Path;
use std::process;
use colored::*;

use travel::*;

fn main(){
    let inargs: Vec<String> = env::args().collect();
    let mut verbose = false;
    if inargs.len() == 2 {
        if inargs[1].contains("-h")||inargs[1].contains("--help"){
            println!("the way to use: {} <target dir> <Regex> -dx -ry (-v)\n\
                        Inorder to not be misunderstood by the shell, you are recommended to\n\
                        surround each one with \"\".\n\
                        the \"target dir\" and \"Regex\" can both be one or more \n\
                        multiple targets will increase the finding place, while the \n\
                        multiple regexes will use logic \'||\' to match the file.\n\
                        -h --help         show this help and exit \n\
                        -dx -ry           {}; x means the number of directories, y means the number of regexes\n\
                        -v                show the file, in green, traveled during the find",inargs[0],"CAN'T BE MISSED".red());
            process::exit(0);
        }else{
            eprintln!("\nthe way to use: {} <target dir> <Regex> -dx -ry (-v)\n\
                        try {} \"-h\"",inargs[0],inargs[0]);
            process::exit(1);
        }
    }else if inargs.len() >= 5 {
        if inargs[inargs.len()-1].contains("-v"){
            verbose = true;
        }
    }else{
        eprintln!("\nthe way to use: {} <target dir> <Regex> -dx -ry (-v)\n\
                        try {} \"-h\"",inargs[0],inargs[0]);
        process::exit(1);
    }
    let mut dir_num;
    let mut reg_num;
    if verbose {
        if !inargs[inargs.len() - 2].contains("-r") || !inargs[inargs.len() - 3].contains("-d") {
            eprintln!("\nthe way to use: {} <target dir> <Regex> -dx -ry (-v)\n\
                        try {} \"-h\"",inargs[0],inargs[0]);
            process::exit(1);
        }
        dir_num = inargs[inargs.len()-3][2..].parse::<u32>().unwrap();
        reg_num = inargs[inargs.len()-2][2..].parse::<u32>().unwrap();
    }else{
        if !inargs[inargs.len() - 1].contains("-r") || !inargs[inargs.len() - 2].contains("-d") {
            eprintln!("\nthe way to use: {} <target dir> <Regex> -dx -ry (-v)\n\
                        try {} \"-h\"",inargs[0],inargs[0]);
            process::exit(1);
        }
        dir_num = inargs[inargs.len()-2][2..].parse::<u32>().unwrap();
        reg_num = inargs[inargs.len()-1][2..].parse::<u32>().unwrap();
    }

    

    let mut arg_cnt=1;
    let mut root_vec=Vec::new();
    while arg_cnt < inargs.len()&&dir_num > 0{
        dir_num -=1;
        let cur_root = Path::new(&inargs[arg_cnt]);
        if cur_root.is_dir(){
            root_vec.push(&inargs[arg_cnt]);
            arg_cnt += 1;
        }else{
            eprintln!("An {}: {}","non-existent directory".red(), inargs[arg_cnt]);
            process::exit(1);
        }
    }
    let mut regex_vec = Vec::new();
    while arg_cnt < inargs.len()&&reg_num > 0{
        reg_num -= 1;
        match Regex::new(&inargs[arg_cnt]){
            Ok(re) => {
                regex_vec.push(re);
                arg_cnt += 1;
            },
            Err(e) => {//
                eprintln!("An {} occurred : {}","error".red(),e);
                process::exit(1);
            }
        }
    }
    // let pattern = &inargs[2];//borrow the Regex pattern
    // let regex = match Regex::new(pattern){
    //     Ok(re) => re,
    //     Err(err) =>{
    //         eprintln!("Invalid regex \"{}\" : {}", pattern, err);
    //         process::exit(1);
    //     }
    // };

    // let root = &inargs[1];
    match find(root_vec, regex_vec, &verbose){
        Ok(files) =>{
            if files.is_empty(){
                println!("\n{}","No files found.".yellow());
            }else{
                println!("\n{}\n {}, others is white\n","Those files were found:".yellow(),".rs file is blue".blue());
                for file in files{
                    if file.contains(".rs"){
                        println!("{}",file.blue());
                    }    
                    else{
                        println!("{}",file);
                    }    
                }
            }
            
        }
        Err(err) => {
            eprintln!("An error occurred: {}",err);
            process::exit(1);
        }
    }

}

// fn find<P: AsRef<Path>>(roots: Vec<P>, regexes: Vec<Regex>, verbose: &bool) 
//     -> Result<Vec<String>, Box<dyn std::error::Error>>{
//         let mut match_files = Vec::new();
//         for root in roots{
//             travel_dir(root.as_ref(), &regexes, &mut match_files, verbose)?;
//         }
        
//         Ok(match_files)
//     }

// fn travel_dir(dir: &Path, regexes: &Vec<Regex>, match_files: &mut Vec<String>, verbose: &bool)
//     -> Result<(), Box<dyn std::error::Error>>{
//         if dir.is_dir(){
//             for in_file in fs::read_dir(dir)?{
//                 let in_file = in_file?;//从read_dir中获得的是Result,通过这个转换为fs::DirEntry
//                 let cur_path = in_file.path();
//                 if cur_path.is_dir(){
//                     travel_dir(&cur_path, regexes, match_files, verbose)?;
//                 }else if let Some(filename)=cur_path.file_name().and_then(|name| name.to_str()){
//                     if *verbose{
//                         //print the travel progress if verbose
//                         println!("{}", cur_path.to_string_lossy().to_string().green());
//                     }
//                     for regex in regexes{
//                         if regex.is_match(filename){
//                             match_files.push(cur_path.to_string_lossy().to_string());
//                             break;
//                         }
//                     }
                    
//                 }
//             }
//         }else if let Some(filename)=dir.file_name().and_then(|name| name.to_str()){
//                     if *verbose{
//                         //print the travel progress if verbose
//                         println!("{}", dir.to_string_lossy().to_string().green());
//                     }
//                     for regex in regexes{
//                         if regex.is_match(filename){
//                             match_files.push(dir.to_string_lossy().to_string());
//                         }
//                     }
//                 }
//         Ok(()) 
//     }