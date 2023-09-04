use regex::Regex;
// use std::env;
use std::fs;
use std::path::Path;
// use std::process;
use colored::*;

pub fn find<P: AsRef<Path>>(roots: Vec<P>, regexes: Vec<Regex>, verbose: &bool) 
    -> Result<Vec<String>, Box<dyn std::error::Error>>{
        let mut match_files = Vec::new();
        for root in roots{
            travel_dir(root.as_ref(), &regexes, &mut match_files, verbose)?;
        }
        
        Ok(match_files)
    }

pub fn travel_dir(dir: &Path, regexes: &Vec<Regex>, match_files: &mut Vec<String>, verbose: &bool)
    -> Result<(), Box<dyn std::error::Error>>{
        if dir.is_dir(){
            for in_file in fs::read_dir(dir)?{
                let in_file = in_file?;//从read_dir中获得的是Result,通过这个转换为fs::DirEntry
                let cur_path = in_file.path();
                if cur_path.is_dir(){
                    travel_dir(&cur_path, regexes, match_files, verbose)?;
                }else if let Some(filename)=cur_path.file_name().and_then(|name| name.to_str()){
                    if *verbose{
                        //print the travel progress if verbose
                        println!("{}", cur_path.to_string_lossy().to_string().green());
                    }
                    for regex in regexes{
                        if regex.is_match(filename){
                            match_files.push(cur_path.to_string_lossy().to_string());
                            break;
                        }
                    }
                    
                }
            }
        }else if let Some(filename)=dir.file_name().and_then(|name| name.to_str()){
                    if *verbose{
                        //print the travel progress if verbose
                        println!("{}", dir.to_string_lossy().to_string().green());
                    }
                    for regex in regexes{
                        if regex.is_match(filename){
                            match_files.push(dir.to_string_lossy().to_string());
                        }
                    }
                }
        Ok(()) 
    }