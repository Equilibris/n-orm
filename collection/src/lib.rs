#![feature(async_fn_in_trait)]
pub use collection_macro::*;

pub trait Document {
    type Collection: Collection<Document = Self>;
}

pub trait Collection {
    type Internal;
    type Document: Document;

    async fn fetch<F: Fetcher<Self::Document, Self::Internal>>(
        &self,
        f: F,
    ) -> Result<F::Output, F::Error>;
}

pub trait Fetcher<Doc, Internal> {
    type Output;
    type Error;

    async fn fetch(self, surface: &Internal) -> Result<Self::Output, Self::Error>;
}

#[cfg(test)]
mod tests {
    use mongodb::bson::{doc, oid::ObjectId};

    use crate::{Collection, Document, Fetcher};

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Doc {
        id: ObjectId,
    }

    pub struct Coll(mongodb::Collection<Doc>);

    impl Collection for Coll {
        type Internal = mongodb::Collection<Self::Document>;
        type Document = Doc;

        async fn fetch<F: Fetcher<Self::Document, Self::Internal>>(
            &self,
            f: F,
        ) -> Result<F::Output, F::Error> {
            f.fetch(&self.0).await
        }
    }

    pub struct ObjectIdFetcher(ObjectId);

    impl<Doc: serde::de::DeserializeOwned> Fetcher<Doc, mongodb::Collection<Doc>> for ObjectIdFetcher {
        type Output = Option<Doc>;
        type Error = mongodb::error::Error;

        async fn fetch(
            self,
            surface: &mongodb::Collection<Doc>,
        ) -> Result<Self::Output, Self::Error> {
            let mut cursor = surface.find(Some(doc! { "_id":self.0 }), None).await?;

            Ok(if cursor.advance().await? {
                let doc = cursor.deserialize_current()?;

                Some(doc)
            } else {
                None
            })
        }
    }

    impl Document for Doc {
        type Collection = Coll;
    }

    #[test]
    pub fn it_fetches_by_id() {}
}
