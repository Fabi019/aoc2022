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

    fn get_all_folders(&self) -> Vec<&Folder> {
        FileSystem::get_all_folders_recursive(&self.root)
    }

    fn get_all_folders_recursive(folder: &Folder) -> Vec<&Folder> {
        let mut folders = Vec::from_iter(folder.folder.values());
        for current in folder.folder.values() {
            folders.extend(FileSystem::get_all_folders_recursive(current));
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

impl Folder {
    fn new(name: String) -> Folder {
        Folder {
            name,
            files: HashMap::new(),
            folder: HashMap::new(),
        }
    }

    fn size(&self) -> u64 {
        self.files.values().fold(0, |a, f| a + f.size)
            + self.folder.values().fold(0, |a, f| a + f.size())
    }
}

fn main() {
    let mut file_system = FileSystem::new();
    let mut current_path: Vec<String> = Vec::new();

    for line in INPUT.lines() {
        let mut split = line.split(' ');

        match (split.next(), split.next()) {
            (Some("$"), Some("cd")) => match split.next().unwrap() {
                "/" => current_path = Vec::new(),
                ".." => _ = current_path.pop(),
                path => current_path.push(path.to_string()),
            },
            (Some("$"), Some("ls")) => {}
            (Some("dir"), Some(name)) => {
                file_system.put_folder(&current_path, Folder::new(name.to_string()));
            }
            (Some(size), Some(name)) => {
                file_system.put_file(
                    &current_path,
                    File {
                        name: name.to_string(),
                        size: size.parse().unwrap(),
                    },
                );
            }
            _ => panic!("Invalid input"),
        }
    }

    let folders = file_system.get_all_folders();

    // Part one: All folders with at most 100000 size
    let sum = folders
        .iter()
        .map(|folder| folder.size())
        .filter(|&size| size <= 100000)
        .sum::<u64>();
    println!("Folder Size <=100000: {}", sum);

    // Part two: Smallest folder to delete to get 30000000 free space
    let used_space = file_system.root.size();
    let available_space = 70000000 - used_space;
    let space_to_delete = 30000000 - available_space;

    let min = folders
        .iter()
        .map(|folder| folder.size())
        .filter(|&size| size >= space_to_delete)
        .min()
        .unwrap();

    println!("Smallest folder to delete: {}", min);
}
