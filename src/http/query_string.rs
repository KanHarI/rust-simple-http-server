use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, QueryStringValue<'buf>>
}

#[derive(Debug)]
pub enum QueryStringValue<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&QueryStringValue> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data: HashMap<&'buf str, QueryStringValue<'buf>> = HashMap::new();
        for subquery in s.split('&') {
            let (key, val): (&str, &str) = match subquery.find('=') {
                Some(i) => (&subquery[..i], &subquery[(i+1)..]),
                None => (subquery, "")
            };
            data.entry(key).and_modify(|existing: &mut QueryStringValue| match existing {
                QueryStringValue::Single(prev_val) => {
                    *existing = QueryStringValue::Multiple(vec![prev_val, val]);
                },
                QueryStringValue::Multiple(vec) => {vec.push(val);}
            }).or_insert(QueryStringValue::Single(val));
        }
        Self {
            data
        }
    }
}
