use std::{collections::HashMap, fmt::format, fs};
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    //    let filelist: Vec<Vec<FsElement>> = vec![];
    let mut fsmap: HashMap<String, usize> = HashMap::new();
    let mut currentdir: String = "".to_string();
    println!(
        "test path ret {}",
        strip_trailing_folders("/test/dir/1/423", 3)
    );
    for line in input.lines() {
        match &line[..4] {
            "$ ls" => {}
            "$ cd" => {
                match &line[4..6] {
                    " /" => currentdir = "/".to_string(),
                    " ." => {
                        println!("cd {}", currentdir);
                        currentdir = strip_trailing_folders(&currentdir, 1);
                        println!("cd ..  {}", currentdir);
                    }
                    _ => {
                        let mut slash = "";
                        if currentdir != "/" {
                            slash = "/";
                            println!("currentdir {}", currentdir);
                        }
                        let dir = format!("{}{}{}", currentdir, slash, &line[5..]);
                        currentdir = dir;
                    }
                }
                println!("currentdir {}", currentdir);
            }
            "dir " => {
                let mut slash = "";
                if currentdir != "/" {
                    slash = "/";
                    println!("currentdir {}", currentdir);
                }
                let dir = format!("{}{}{}", currentdir, slash, &line[5..]);
                if let Some(old) = fsmap.insert(dir.to_string(), 0) {
                    fsmap.insert(dir.to_string(), old);
                }
            }
            _ => {
                let filesize: usize = line.split(' ').collect::<Vec<&str>>()[0]
                    .parse::<usize>()
                    .unwrap_or(0);
                let folderdepth = {
                    if currentdir != "/" {
                        currentdir.split("/").collect::<Vec<&str>>().len()
                    } else {
                        1
                    }
                };
                for i in 0..folderdepth {
                    let fldr = strip_trailing_folders(&currentdir, i);
                    let currentsize = fsmap.get(&fldr).unwrap_or(&0);
                    println!("adding {} to folder {}", filesize, fldr);
                    fsmap.insert(fldr, filesize + currentsize);
                    println!("{} {}", i, folderdepth);
                }
            }
        }
    }
    //println!("{:?}", fsmap);
    let mut free_space = 0;
    let mut folderlist = vec![];
    for (path, val) in fsmap {
        if path == "/" {
            free_space = 70000000 - val;
        }
        folderlist.push(val);
        //if val < 100000 {
        //sum += val;
        println!("{}  {}", path, val);
        //}
    }
    let needed_space = 30000000 - free_space;
    println!("Free_space: {} needed: {},", free_space, needed_space);
    folderlist.sort();
    for v in folderlist {
        if v > needed_space {
            println!("V is {}", v);
        }
    }
}
fn strip_trailing_folders(fullpath: &str, nums_to_strip: usize) -> String {
    let mut path_ret: String = fullpath.to_string();
    for _ in 0..nums_to_strip {
        let lastfolder = path_ret.split("/").last().unwrap();
        path_ret = path_ret
            .strip_suffix(lastfolder)
            .unwrap()
            .trim_end_matches("/")
            .to_string();
    }
    if path_ret.is_empty() {
        "/".to_string()
    } else {
        path_ret
    }
}

const TESTINPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
