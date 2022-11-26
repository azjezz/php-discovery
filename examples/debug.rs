use php_discovery;

fn main() {
    let builds = php_discovery::discover();

    for build in builds.unwrap() {
        println!("{:#?}", build);
    }
}
