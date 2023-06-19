use git2::{Repository, BranchType, Branch, Tree, DiffDelta};

pub struct ComparisonBranches<'a> {
    head: Branch<'a>,
    main: Branch<'a>,
}

pub fn get_changed_files(root_dir: &String, main_branch: &String) -> Vec<String> {
    let repo = match Repository::open(root_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open: {}", e),
    };

    let branches = match get_branches(&repo, &main_branch) {
        Ok(references) => references,
        Err(e) => panic!("Failed to get a reference: {}", e),
    };

    let head_tree: Tree = match branches.head.into_reference().peel_to_tree() {
        Ok(head_tree) => head_tree,
        Err(e) => panic!("Failed getting tree from head branch ref: {}", e),
    };

    let main_tree: Tree = match branches.main.into_reference().peel_to_tree() {
        Ok(head_tree) => head_tree,
        Err(e) => panic!("Failed getting tree from main branch ref: {}", e),
    };

    let diff = match repo.diff_tree_to_tree(Some(&main_tree), Some(&head_tree), None) {
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

fn get_branches<'a>(repo: &'a Repository, main_branch: &str) -> Result<ComparisonBranches<'a>, & 'a str> {
    let mut head: Option<Branch> = None;
    let mut main: Option<Branch> = None;

    // list local branches, need full clone
    let branches = match repo.branches(Some(BranchType::Local)) {
        Ok(branches) => branches,
        Err(e) => panic!("Failed to get branches: {}", e),
    };
    for branch in branches {
        let branch = match branch {
            Ok(branch) => branch.0,
            Err(_) => continue,
        };

        let beans = &branch.name();
        let name = match beans {
            Ok(name) => name,
            Err(_) => continue,
        };

        let name = match name {
            Some(name) => name,
            None => continue,
        };

        if branch.is_head() {
            println!("found head branch: {}", &name);
            head = Some(branch);
        } else if name == &main_branch {
            println!("found main branch: {}", &name);
            main = Some(branch);
        }
    }

    if head.is_none() {
        return Err("Unable to find head reference");
    }

    if main.is_none() {
        return Err("Unable to find main reference");
    }

    return Ok(ComparisonBranches {
        head: head.unwrap(),
        main: main.unwrap()
    });
}
