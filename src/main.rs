use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> Default for OneOrMany<T> {
    fn default() -> Self {
        OneOrMany::Many(Vec::new())
    }
}

impl<T> OneOrMany<T> {
    pub fn into_vec(self) -> Vec<T> {
        match self {
            OneOrMany::One(item) => vec![item],
            OneOrMany::Many(items) => items,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(default)]
    pub count: Option<u64>,
    #[serde(default)]
    pub results: OneOrMany<Results>,
}

#[derive(Deserialize, Debug)]
pub struct Results {
    #[serde(default)]
    pub big_number: Option<f64>,
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_success() {
        let single = r#"
        {
            "count": 1,
            "results": {
                "big_number": 2922044998640
            }
        }"#;

        let result: Result<Response, _> = serde_json::from_str(single);

        match result {
            Ok(response) => {
                dbg!(response);
            },
            Err(e) => eprintln!("Error parsing Ticker response: {e}"),
        }
    }

    #[test]
    fn test_fail() {
        let single = r#"
        {
            "count": 1,
            "results": {
                "big_number": 2.92204499864e12
            }
        }"#;

        let result: Result<Response, _> = serde_json::from_str(single);

        match result {
            Ok(response) => {
                dbg!(response);
            },
            Err(e) => eprintln!("Error parsing Ticker response: {e}"),
        }
    }
}