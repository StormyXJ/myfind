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
    if inargs.len() == 2{
        if inargs[1].contains("-h")||inargs[1].contains("--help"){
            println!("the way to use: {} <target dir> <Regex> \n\
                        Inorder to not be misunderstood by the shell, you are recommended to\n\
                        surround each one with \"\".\n\
                        the \"target dir\" and \"Regex\" can both be one or more \n\
                        multiple targets will increase the finding place, while the \n\
                        multiple regexes will use logic \'||\' to match the file.\n\
                        -h --help         show this help and exit \n\
                        -v --verbose      show the file, in green, traveled during the find",inargs[0]);
            process::exit(0);
        }else{
            eprintln!("the way to use: {} <target dir> <Regex>",inargs[0]);
            process::exit(1);
        }
    }else if inargs.len() >= 4 {
        if inargs[inargs.len()-1].contains("-v")||inargs[inargs.len()-1].contains("--verbose"){
            verbose = true;
        }
    }

    let mut arg_cnt=1;
    let mut root_vec=Vec::new();
    while arg_cnt < inargs.len(){
        let cur_root = Path::new(&inargs[arg_cnt]);
        if cur_root.is_dir(){
            root_vec.push(&inargs[arg_cnt]);
            arg_cnt += 1;
        }else{
            break;
        }
    }
    let mut regex_vec = Vec::new();
    while arg_cnt < inargs.len(){
        match Regex::new(&inargs[arg_cnt]){
            Ok(re) => {
                regex_vec.push(re);
                arg_cnt += 1;
            },
            Err(e) => {//#the invalid one and end can be blurred
                if regex_vec.is_empty() {
                    eprintln!("An {} occurred : {}","error".red(),e);
                    process::exit(1);
                }else{
                    break;
                }
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
                println!("\n{}\n {},{}\n","Those files were found:".yellow(),".rs file is blue".blue(),"others is white");
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