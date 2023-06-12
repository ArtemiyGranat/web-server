#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Path {
    query_idx: Option<usize>,
    path: String,
}

impl Path {
    pub fn new(path: String) -> Self {
        let mut query_idx = None;
        for (idx, c) in path.chars().enumerate() {
            if c == '?' {
                query_idx = Some(idx);
                break;
            }
        }

        Self {
            path,
            query_idx,
        }
    }

    pub fn uri(&self) -> &str {
        if let Some(i) = self.query_idx {
            &self.path[..i]
        } else {
            &self.path
        }
    }

    pub fn query(&self) -> Option<&str> {
        if let Some(i) = self.query_idx {
            Some(&self.path[i + 1..])
        } else {
            None
        }
    }
}
