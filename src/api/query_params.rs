use std::borrow::Borrow;

use serde::Serialize;
use url::Url;

use super::error::BodyError;

pub struct QueryParams<'a>(form_urlencoded::Serializer<'a, String>);

impl QueryParams<'_> {
    pub(crate) fn new() -> Self {
        Self(form_urlencoded::Serializer::new(String::new()))
    }

    #[allow(dead_code)]
    pub(crate) fn clear(&mut self) -> &mut Self {
        self.0.clear();
        self
    }

    #[allow(dead_code)]
    pub(crate) fn append_pair(
        &mut self,
        key: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> &mut Self {
        self.0.append_pair(key.as_ref(), value.as_ref());
        self
    }

    #[allow(dead_code)]
    pub(crate) fn extend_pairs<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.0.extend_pairs(iter);
        self
    }

    pub(crate) fn extend_from(&mut self, value: &impl Serialize) -> Result<&mut Self, BodyError> {
        value.serialize(self.serializer())?;
        Ok(self)
    }

    #[allow(dead_code)]
    pub(crate) fn with(value: &impl Serialize) -> Result<Self, BodyError> {
        let mut out = Self::new();
        out.extend_from(value)?;
        Ok(out)
    }

    pub(crate) fn apply_to(self, url: &mut Url) {
        let query = &self.finish();
        if !query.is_empty() {
            url.set_query(Some(query))
        }
    }

    pub(crate) fn finish(mut self) -> String {
        self.0.finish()
    }
}

impl<'a> QueryParams<'a> {
    pub(crate) fn serializer<'b>(&'b mut self) -> serde_urlencoded::Serializer<'a, 'b, String> {
        serde_urlencoded::Serializer::new(&mut self.0)
    }
}

impl Default for QueryParams<'_> {
    fn default() -> Self {
        Self::new()
    }
}
