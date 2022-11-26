use php_discovery;

fn main() {
    let builds = php_discovery::discover();

    for build in builds.unwrap() {
        println!(
            "found build version '{}' => {}",
            build.version,
            build.binary.to_string_lossy()
        );
    }
}
