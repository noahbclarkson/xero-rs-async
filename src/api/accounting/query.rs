use std::fmt::Display;

#[derive(Debug, Default)]
pub(crate) struct QueryParams {
    params: Vec<(String, String)>,
}

impl QueryParams {
    pub(crate) fn push<T>(&mut self, key: &str, value: T)
    where
        T: Display,
    {
        self.params.push((key.to_string(), value.to_string()));
    }

    pub(crate) fn push_string(&mut self, key: &str, value: String) {
        self.params.push((key.to_string(), value));
    }

    pub(crate) fn push_opt<T>(&mut self, key: &str, value: Option<T>)
    where
        T: Display,
    {
        if let Some(v) = value {
            self.push(key, v);
        }
    }

    pub(crate) fn push_opt_string(&mut self, key: &str, value: Option<String>) {
        if let Some(v) = value {
            self.push_string(key, v);
        }
    }

    pub(crate) fn push_opt_csv<T, I>(&mut self, key: &str, values: Option<I>)
    where
        T: Display,
        I: IntoIterator<Item = T>,
    {
        if let Some(v) = values {
            self.push_string(key, join_csv(v));
        }
    }

    pub(crate) fn as_slice(&self) -> Option<&[(String, String)]> {
        if self.params.is_empty() {
            None
        } else {
            Some(&self.params)
        }
    }
}

pub(crate) fn join_csv<T, I>(values: I) -> String
where
    T: Display,
    I: IntoIterator<Item = T>,
{
    values
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
