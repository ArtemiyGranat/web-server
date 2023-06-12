use crate::request::HttpRequest;

pub struct Path {
    target: String,
    query_idx: Option<usize>,
}

impl Path {
    pub fn new(req: HttpRequest) -> Self {
        let mut query_idx = None;
        for (idx, c) in req.target().chars().enumerate() {
            if c == '?' {
                query_idx = Some(idx);
                break;
            }
        }

        Self {
            target: req.target().to_string(),
            query_idx,
        }
    }

    pub fn uri(&self) -> &str {
        if let Some(i) = self.query_idx {
            &self.target[..i]
        } else {
            &self.target
        }
    }

    pub fn query(&self) -> Option<&str> {
        if let Some(i) = self.query_idx {
            Some(&self.target[i + 1..])
        } else {
            None
        }
    }
}
