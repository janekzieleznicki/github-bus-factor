use serde::Deserialize;

#[derive(Deserialize)]
pub struct Contributor {
    pub login: String,
    pub contributions: usize,
    #[serde(skip)]
    pub bus_factor: f32,
}

#[derive(Deserialize)]
pub struct Repository {
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub contributors_url: String,
    #[serde(skip)]
    pub contributors: Vec<Contributor>,
}

impl Repository {
    pub fn update_bus_factors(&mut self) {
        let contributions_sum: usize = self
            .contributors
            .iter()
            .map(|user| user.contributions)
            .sum();
        self.contributors.iter_mut().for_each(|user| {
            user.bus_factor = user.contributions as f32 / contributions_sum as f32
        });
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoriesResponse {
    #[serde(rename = "total_count")]
    pub total_count: u64,
    #[serde(rename = "items")]
    pub repos: Vec<Repository>,
}

#[cfg(test)]
mod tests {
    use crate::models::{Contributor, Repository};

    #[test]
    fn calculate_bus_factor() {
        let mut repository = Repository {
            id: 0,
            node_id: "".to_string(),
            name: "repo-name".to_string(),
            contributors_url: "".to_string(),
            contributors: vec![
                Contributor {
                    login: "workaholic".to_string(),
                    contributions: 1000,
                    bus_factor: 0.0,
                },
                Contributor {
                    login: "beginner".to_string(),
                    contributions: 1,
                    bus_factor: 0.0,
                },
            ],
        };
        repository.update_bus_factors();
        assert!(repository.contributors.first().unwrap().bus_factor > 0.75);
        assert!(repository.contributors.last().unwrap().bus_factor < 0.75);
    }
}
