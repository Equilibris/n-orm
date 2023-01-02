#![feature(async_fn_in_trait)]

pub mod mongo {
    pub mod err {
        use thiserror::Error;

        #[derive(Debug, Error)]
        pub enum IndexCreationError {}
    }

    pub trait MultiplexedGlobalSharable {
        async fn prepare();
    }

    pub trait ReadableCollection {
        async fn ensure_indicies(&self) -> Result<(), err::IndexCreationError> {
            Ok(())
        }
    }
}
