use std::fs;

pub fn get_packages(root_dir_path: &String, packages_dir: &String) -> Vec<String> {
    let mut package_dirs: Vec<String> = Vec::new();
    
    let mut package_path = String::from(root_dir_path.clone());
    package_path.push_str("/");
    package_path.push_str(&packages_dir);
    
    let dirs = match fs::read_dir(package_path) {
        Ok(dirs) => dirs,
        Err(e) => panic!("Failed to read root dir: {}", e),
    };
    
    for entry in dirs {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue
        };
        
        let path = entry.path();
        
        let metadata = match fs::metadata(&path) {
            Ok(metadata) => metadata,
            Err(e) => panic!("Failed reading metadata from file: {}", e),
        };
        
        if metadata.is_dir() { //only interested in 
            let filename = match path.file_name() {
                Some(filename) => filename.to_str().unwrap(),
                None => panic!("Couldn't get filename"),
            };
            
            let mut package_path = packages_dir.clone();
            package_path.push_str("/");
            package_path.push_str(filename);
            
            package_dirs.push(package_path);
        }
    }
    
    return package_dirs;
}

pub fn filter_files_to_packages(known_packages: Vec<String>, changed_files: Vec<String>) -> Vec<String> {
    let mut changed_packages: Vec<String> = Vec::new();

    for change in changed_files {
        for package in &known_packages {
            if change.starts_with(package) {
                if !changed_packages.contains(&package) {
                    changed_packages.push(package.clone())
                }
            }
        } 
    }

    return changed_packages;
}