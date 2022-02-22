use busfactorlib::models::{Contributor, RepositoriesResponse};
use std::fs::File;
use std::io::BufReader;

#[test]
fn deserialize_contributors() {
    let data = std::fs::read_to_string("tests/deno-contributors.json").unwrap();
    let contributors: Vec<Contributor> = serde_json::from_str(data.as_str()).unwrap();
    assert_eq!(contributors.len(), 100);
    assert_eq!(contributors.first().unwrap().contributions, 1387);
    assert_eq!(contributors.first().unwrap().login, "ry");
    assert_eq!(contributors.last().unwrap().contributions, 4);
    assert_eq!(contributors.last().unwrap().login, "FSou1");
}
#[test]
fn deserialize_repositories() {
    let reader = BufReader::new(File::open("tests/rust-repos.json").unwrap());
    let repositories: RepositoriesResponse = serde_json::from_reader(reader).unwrap();
    assert_eq!(repositories.repos.len(), 100);
    assert_eq!(repositories.repos.first().unwrap().name, "deno");
    assert_eq!(repositories.repos.last().unwrap().name, "pyo3");
    assert_eq!(
        repositories.repos.last().unwrap().contributors_url,
        "https://api.github.com/repos/PyO3/pyo3/contributors"
    );
}
