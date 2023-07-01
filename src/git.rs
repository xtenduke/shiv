use git2::{Repository, BranchType, Branch, Tree, DiffDelta};

/// Get main branch reference, either from local checked out branch, or pull from remote config
pub fn get_main_branch_ref<'a>(repo: &'a Repository, main_branch: &String) -> Branch<'a> {
    let main: Option<Branch> = match repo.find_branch(&main_branch, BranchType::Remote) {
        Ok(main) => Some(main),
        Err(_) => None,
    };

    return if main.is_none() {
        // Fallback to remote
        // find current remote

        // pick first remote.. lol
        let remote_names = match repo.remotes() {
            Ok(remote_names) => remote_names,
            Err(e) => panic!("Failed to get remote names {}", e),
        };

        // stringbuilder i.e. `origin/main`
        let mut remote_name = match remote_names.get(0) {
            Some(remote_name) => remote_name.to_string(),
            None => panic!("Can't find main branch locally, can't see any remotes.")
        };

        remote_name.push_str("/");
        remote_name.push_str(&main_branch);
        let remote_main = match repo.find_branch(&remote_name, BranchType::Remote) {
            Ok(main) => main,
            Err(e) => panic!("Unable to find main branch named {} because {}", main_branch, e),
        };

        remote_main
    } else {
        main.unwrap()
    }
}

pub fn get_changed_files(root_dir: &String, main_branch: &String) -> Vec<String> {
    let repo = match Repository::open(root_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open: {}", e),
    };

    let remote_main = get_main_branch_ref(&repo, &main_branch);
    let main_tree: Tree = match remote_main.into_reference().peel_to_tree() {
        Ok(head_tree) => head_tree,
        Err(e) => panic!("Failed getting tree from main branch ref: {}", e),
    };

    let diff = match repo.diff_tree_to_workdir_with_index(Some(&main_tree), None) {
        Ok(diff) => diff,
        Err(e) => panic!("Diff failed: {}", e),
    };

    let mut changes: Vec<String> = Vec::new();

    let mut fun = |diff: DiffDelta<'_>, _: f32| -> bool {
        let new_file = diff.new_file().path().unwrap().to_str().unwrap();
        let old_file = diff.old_file().path().unwrap().to_str().unwrap();
        
        // rename into diff path potentially
        changes.push(new_file.to_owned());
        if new_file != old_file {
            changes.push(old_file.to_owned());
        }

        return true;
    };

    let _ = match diff.foreach(&mut fun, None, None, None) {
        Ok(result) => result,
        Err(e) => panic!("Looping diff failed: {}", e),
    };
    
    return changes;
}