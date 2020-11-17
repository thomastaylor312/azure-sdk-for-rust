use azure_core::errors::AzureError;
use http::HeaderMap;
use url::Url;

const HEADER_NEXTPARTITIONKEY: &str = "x-ms-continuation-NextPartitionKey";
const HEADER_NEXTROWKEY: &str = "x-ms-continuation-NextRowKey";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuationToken {
    pub(crate) previous_url: Url,
    pub(crate) new_url: Url,
}

impl ContinuationToken {
    pub fn new(previous_url: Url, next_partition_key: &str, next_row_key: &str) -> Self {
        let mut partition_key_replaced = false;
        let mut row_key_replaced = false;

        let v: Vec<(String, String)> = previous_url
            .query_pairs()
            .map(|(k, v)| {
                let new_v = match k.as_ref() {
                    HEADER_NEXTPARTITIONKEY => {
                        partition_key_replaced = true;
                        next_partition_key.to_string()
                    }
                    HEADER_NEXTROWKEY => {
                        row_key_replaced = true;
                        next_row_key.to_string()
                    }
                    _ => v.into_owned(),
                };

                (k.into_owned(), new_v)
            })
            .collect();

        let mut new_url = previous_url.clone();
        new_url.query_pairs_mut().clear().extend_pairs(v);
        if !partition_key_replaced {
            new_url
                .query_pairs_mut()
                .append_pair(HEADER_NEXTPARTITIONKEY, &next_partition_key);
        }
        if !row_key_replaced {
            new_url
                .query_pairs_mut()
                .append_pair(HEADER_NEXTROWKEY, &next_row_key);
        }

        Self {
            previous_url,
            new_url,
        }
    }

    pub fn previous_url(&self) -> &Url {
        &self.previous_url
    }
    pub fn new_url(&self) -> &Url {
        &self.new_url
    }

    pub fn previous_partition_key(&self) -> Option<String> {
        self.new_url.query_pairs().find_map(|(k, v)| {
            if k == HEADER_NEXTPARTITIONKEY {
                Some(v.into_owned())
            } else {
                None
            }
        })
    }

    pub fn previous_row_key(&self) -> Option<String> {
        self.new_url.query_pairs().find_map(|(k, v)| {
            if k == HEADER_NEXTROWKEY {
                Some(v.into_owned())
            } else {
                None
            }
        })
    }

    pub fn next_partition_key(&self) -> String {
        self.new_url
            .query_pairs()
            .find_map(|(k, v)| {
                if k == HEADER_NEXTPARTITIONKEY {
                    Some(v)
                } else {
                    None
                }
            })
            .unwrap()
            .into_owned()
    }

    pub fn next_row_key(&self) -> String {
        self.new_url
            .query_pairs()
            .find_map(|(k, v)| {
                if k == HEADER_NEXTROWKEY {
                    Some(v)
                } else {
                    None
                }
            })
            .unwrap()
            .into_owned()
    }

    pub(crate) fn parse_from_headers_optional(
        previous_url: Url,
        headers: &HeaderMap,
    ) -> Result<Option<Self>, AzureError> {
        let result = if let (Some(partition_key), Some(row_key)) = (
            headers.get(HEADER_NEXTPARTITIONKEY),
            headers.get(HEADER_NEXTROWKEY),
        ) {
            println!("partition_key == {:?}", partition_key.to_str());
            println!("row_key == {:?}", row_key.to_str());

            Some(Self::new(
                previous_url,
                partition_key.to_str()?,
                row_key.to_str()?,
            ))
        } else {
            None
        };

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use url::{Position, Url};

    #[test]
    fn parse() {
        let u =
            Url::parse("http://www.microsoft.com/?some=value&x-ms-continuation-NextPartitionKey=p1&x-ms-continuation-NextRowKey=r1&someother=cc")
                .unwrap();
        let c: ContinuationToken = ContinuationToken::new(u, "new_pp", "new_rk");
        assert_eq!(&format!("{}", c.new_url()),
            "http://www.microsoft.com/?some=value&x-ms-continuation-NextPartitionKey=new_pp&x-ms-continuation-NextRowKey=new_rk&someother=cc");

        let u = Url::parse("https://myaccount.table.core.windows.net/mytable()?$filter=query-expression&$select=comma-separated-property-names").unwrap();
        let c: ContinuationToken = ContinuationToken::new(u, "new_pp", "new_rk");
        assert_eq!(&format!("{}", c.new_url()),
            "https://myaccount.table.core.windows.net/mytable()?%24filter=query-expression&%24select=comma-separated-property-names&x-ms-continuation-NextPartitionKey=new_pp&x-ms-continuation-NextRowKey=new_rk");

        assert_eq!("/mytable()?%24filter=query-expression&%24select=comma-separated-property-names&x-ms-continuation-NextPartitionKey=new_pp&x-ms-continuation-NextRowKey=new_rk",
        &c.new_url[Position::BeforePath..]);
    }
}
