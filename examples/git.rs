use git2::Repository;

fn main() {
    let repo = match Repository::init("examples/demo") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
    println!("{:?}", repo.add_ignore_rule("bin"));
}
