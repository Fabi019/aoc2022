use std::collections::HashMap;

static INPUT: &str = include_str!("../../assets/day07.txt");

#[derive(Debug, Clone)]
struct FileSystem {
    root: Folder,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            root: Folder {
                name: "/".to_string(),
                files: HashMap::new(),
                folder: HashMap::new(),
            },
        }
    }

    fn put_folder(&mut self, path: &[String], folder: Folder) -> Option<Folder> {
        let mut current = &mut self.root;
        for name in path {
            let new = current.folder.get_mut(name).unwrap();
            current = new;
        }
        current.folder.insert(folder.name.clone(), folder)
    }

    #[allow(dead_code)]
    fn get_folder(&self, path: &[String]) -> Option<&Folder> {
        let mut current = &self.root;
        for name in path {
            current = &current.folder[name];
        }
        Some(current)
    }

    fn get_all_folders(&self) -> Vec<&Folder> {
        self.get_all_folders_recursive(&self.root)
    }

    fn get_all_folders_recursive<'a>(&self, folder: &'a Folder) -> Vec<&'a Folder> {
        let mut folders = Vec::new();
        folders.push(folder);
        for (_, current) in &folder.folder {
            folders.extend(self.get_all_folders_recursive(current));
        }
        folders
    }

    fn put_file(&mut self, path: &[String], file: File) -> Option<File> {
        let mut current = &mut self.root;
        for name in path {
            current = current.folder.get_mut(name).unwrap();
        }
        current.files.insert(file.name.clone(), file)
    }

    fn print_tree(&self) {
        self.print_folder_tree(&self.root, 0);
    }

    fn print_folder_tree(&self, folder: &Folder, depth: usize) {
        println!(
            "{:>width$} (dir, {})",
            folder.name,
            self.directory_size(folder),
            width = depth + folder.name.len()
        );

        // Print all files
        for (_, file) in &folder.files {
            println!(
                "{:>width$} (file, {})",
                file.name,
                file.size,
                width = depth + file.name.len() + 2
            );
        }

        // Print all folders
        for (_, folder) in &folder.folder {
            self.print_folder_tree(folder, depth + 2);
        }
    }

    fn directory_size(&self, folder: &Folder) -> u64 {
        let file_size = folder.files.values().fold(0, |acc, file| acc + file.size);
        folder
            .folder
            .values()
            .fold(file_size, |acc, folder| acc + self.directory_size(folder))
    }
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: u64,
}

#[derive(Debug, Clone)]
struct Folder {
    name: String,
    files: HashMap<String, File>,
    folder: HashMap<String, Folder>,
}

fn main() {
    let mut file_system = FileSystem::new();
    let mut current_path: Vec<String> = Vec::new();

    for line in INPUT.lines() {
        let mut split = line.split(' ');

        match split.next() {
            Some("$") => match split.next() {
                Some("cd") => {
                    // Update current path
                    match split.next().unwrap() {
                        "/" => current_path = Vec::new(),
                        ".." => {
                            current_path.pop();
                        }
                        path @ _ => current_path.push(path.to_string()),
                    }

                    println!("Change directory: {:?}", current_path);
                }
                Some("ls") => println!("Listing folder"),
                _ => panic!("Unknown command"),
            },
            data @ _ => match data {
                Some("dir") => {
                    let name = split.next().unwrap();

                    println!("Creating folder: {}", name);

                    let folder = Folder {
                        name: name.to_string(),
                        files: HashMap::new(),
                        folder: HashMap::new(),
                    };

                    file_system.put_folder(&current_path, folder);
                }
                size @ _ => {
                    let name = split.next().unwrap();
                    let size = size.unwrap().parse::<u64>().unwrap();

                    println!("Creating file {} with size: {}", name, size);

                    let file = File {
                        name: name.to_string(),
                        size: size,
                    };

                    file_system.put_file(&current_path, file);
                }
            },
        }
    }

    file_system.print_tree();

    // Part one: All folders with at most 100000 size
    let sum = file_system
        .get_all_folders()
        .iter()
        .map(|folder| file_system.directory_size(folder))
        .filter(|&size| size <= 100000)
        .sum::<u64>();
    println!("Folder Size <=100000: {}", sum);

    // Part two: Smalles folder to delete to get 30000000 free space
    let total_space = 70000000;
    let space_needed = 30000000;
    let used_space = file_system.directory_size(&file_system.root);
    let available_space = total_space - used_space;
    let space_to_delete = space_needed - available_space;

    let mut smallest_folder_size = u64::MAX;
    for folder in file_system.get_all_folders() {
        let size = file_system.directory_size(folder);
        if size >= space_to_delete && size < smallest_folder_size {
            smallest_folder_size = size;
        }
    }

    println!("Smallest folder to delete: {}", smallest_folder_size);
}
