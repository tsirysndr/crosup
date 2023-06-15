use anyhow::Error;

pub fn add_package(file: &str, pkg: &str) -> Result<String, Error> {
    let mut packages = nix_editor::read::readvalue(file, "home.packages")?;
    let pkg = match pkg.starts_with("pkgs.") {
        true => pkg.to_string(),
        false => format!("pkgs.{}", pkg),
    };

    if packages.contains(&pkg) {
        return Ok(packages);
    }

    let entry = format!("\n  {}\n]", pkg);
    packages = packages.replace("\n]", &entry);

    // replace all \n with \n  to keep the formatting
    packages = packages.replace("\n", "\n  ");

    let output = nix_editor::write::write(file, "home.packages", &packages)?;
    Ok(output)
}

pub fn add_packages(file: &str, pkgs: Vec<String>) -> Result<String, Error> {
    let mut packages = nix_editor::read::readvalue(file, "home.packages")?;
    let mut entry = String::new();
    for pkg in pkgs {
        let pkg = match pkg.starts_with("pkgs.") {
            true => pkg.to_string(),
            false => format!("pkgs.{}", pkg),
        };
        if packages.contains(&pkg) {
            continue;
        }
        entry.push_str(&format!("\n  {}", pkg));
    }
    entry.push_str("\n]");

    packages = packages.replace("\n]", &entry);

    // replace all \n with \n  to keep the formatting
    packages = packages.replace("\n", "\n  ");

    let output = nix_editor::write::write(file, "home.packages", &packages)?;
    Ok(output)
}

pub fn remove_package(file: &str, pkg: &str) -> Result<String, Error> {
    let mut packages = nix_editor::read::readvalue(file, "home.packages")?;
    let pkg = match pkg.starts_with("pkgs.") {
        true => pkg.to_string(),
        false => format!("pkgs.{}", pkg),
    };
    let entry = format!("  {}\n", pkg);
    packages = packages.replace(&entry, "");

    // replace all \n with \n  to keep the formatting
    packages = packages.replace("\n", "\n  ");

    let output = nix_editor::write::write(file, "home.packages", &packages)?;
    Ok(output)
}

pub fn remove_packages(file: &str, pkgs: Vec<String>) -> Result<String, Error> {
    let mut packages = nix_editor::read::readvalue(file, "home.packages")?;
    let mut entry = String::new();
    for pkg in pkgs {
        let pkg = match pkg.starts_with("pkgs.") {
            true => pkg.to_string(),
            false => format!("pkgs.{}", pkg),
        };
        entry.push_str(&format!("  {}\n", pkg));
    }
    packages = packages.replace(&entry, "");

    // replace all \n with \n  to keep the formatting
    packages = packages.replace("\n", "\n  ");

    let output = nix_editor::write::write(file, "home.packages", &packages)?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_add_package() {
        // open file tests/home.nix
        let home_nix = fs::read_to_string("tests/home.nix").unwrap();
        let output = add_package(&home_nix, "vim").unwrap();
        let expected = fs::read_to_string("tests/home-with-vim.nix").unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_add_package_with_pkgs() {
        // open file tests/home.nix
        let home_nix = fs::read_to_string("tests/home.nix").unwrap();
        let output = add_package(&home_nix, "pkgs.vim").unwrap();
        let expected = fs::read_to_string("tests/home-with-vim.nix").unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_remove_package() {
        // open file tests/home-with-vim.nix
        let home_nix = fs::read_to_string("tests/home-with-vim.nix").unwrap();
        let output = remove_package(&home_nix, "vim").unwrap();
        assert!(!output.contains("vim"));
    }

    #[test]
    fn test_remove_package_with_pkgs() {
        // open file tests/home-with-vim.nix
        let home_nix = fs::read_to_string("tests/home-with-vim.nix").unwrap();
        let output = remove_package(&home_nix, "pkgs.vim").unwrap();
        assert!(!output.contains("vim"));
    }

    #[test]
    fn test_add_packages() {
        // open file tests/home.nix
        let home_nix = fs::read_to_string("tests/home.nix").unwrap();
        let output = add_packages(&home_nix, vec!["vim".into(), "git".into()]).unwrap();
        let expected = fs::read_to_string("tests/home-with-vim-git.nix").unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_add_packages_with_pkgs() {
        // open file tests/home.nix
        let home_nix = fs::read_to_string("tests/home.nix").unwrap();
        let output = add_packages(&home_nix, vec!["pkgs.vim".into(), "pkgs.git".into()]).unwrap();
        let expected = fs::read_to_string("tests/home-with-vim-git.nix").unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_remove_packages() {
        // open file tests/home-with-vim-git.nix
        let home_nix = fs::read_to_string("tests/home-with-vim-git.nix").unwrap();
        let output = remove_packages(&home_nix, vec!["vim".into(), "git".into()]).unwrap();
        assert!(!output.contains("vim"));
        assert!(!output.contains("git"));
    }
}
