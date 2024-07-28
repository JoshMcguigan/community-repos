use std::{
    io::BufRead,
    path::{Path, PathBuf}, process::Stdio,
};

struct Package {
    name: &'static str,
    url: &'static str,
    tag: &'static str,
    version: &'static str,
}

struct PackageInRepo {
    name: String,
    version: String,
}

fn main() {
    let package_definitions = vec![Package {
        name: "zola",
        url: "https://github.com/getzola/zola.git",
        tag: "v0.19.0",
        version: "0.19.0-1",
    }];
    let packages_in_repo = get_packages_in_repo();

    for package in package_definitions {
        if !in_repo(&package, &packages_in_repo) {
            println!(
                "Package {} {} not found in repo",
                package.name, package.version
            );
            let path_to_deb = build(&package);
            publish(&path_to_deb);
            println!("Package {} {} added to repo", package.name, package.version);
        }
    }
}

fn get_packages_in_repo() -> Vec<PackageInRepo> {
    let mut packages_in_repo = vec![];

    let output = std::process::Command::new("reprepro")
        .args(&["-b", "../debpkgs", "list", "bookworm"])
        .output()
        .unwrap();
    for line in output.stdout.lines() {
        let line = line.unwrap();
        let mut split_line = line.split_whitespace().skip(1);
        let name = split_line.next().unwrap().to_string();
        let version = split_line.next().unwrap().to_string();

        packages_in_repo.push(PackageInRepo { name, version });
    }

    packages_in_repo
}

fn in_repo(package: &Package, packages_in_repo: &[PackageInRepo]) -> bool {
    for package_in_repo in packages_in_repo {
        if package_in_repo.name == package.name && package_in_repo.version == package.version {
            return true;
        }
    }

    false
}

// Builds this package, returning the path to the deb.
fn build(package: &Package) -> PathBuf {
    let path = PathBuf::from("/tmp").join(PathBuf::from(package.name));
    if !path.exists() {
        std::fs::create_dir(&path).unwrap();

        std::process::Command::new("git")
            .arg("clone")
            .arg(&package.url)
            .arg(&path)
            .status()
            .unwrap();
    }
    std::process::Command::new("git")
        .arg("checkout")
        .arg(&package.tag)
        .current_dir(&path)
        .status()
        .unwrap();

    // TODO specify name and version here
    let cargo_deb_output = std::process::Command::new("cargo")
        .arg("deb")
        .current_dir(&path)
        // Print stderr so we can identify issues
        //
        // TODO if we pass in an explicit path for the deb then we can
        // inheret stdout too.
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    PathBuf::from(String::from_utf8(cargo_deb_output.stdout).unwrap().trim())
}

fn publish(path_to_deb: &Path) {
    std::process::Command::new("reprepro")
        .args(&[
            "-b",
            "../debpkgs",
            "-S",
            "unknown",
            "includedeb",
            "bookworm",
        ])
        .arg(path_to_deb)
        .status()
        .unwrap();
}
