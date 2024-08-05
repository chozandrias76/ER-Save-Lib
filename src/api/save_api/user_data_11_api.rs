pub mod user_data_api {
    use crate::SaveApiError;
    use std::{
        collections::{BTreeMap, HashMap},
    };
    pub struct Param<P: crate::param_trait::Param> {
        pub rows: HashMap<i32, P::ParamType>,
    }
    impl crate::SaveApi {
        /// Returns parameters by type
        ///
        /// # Example
        /// ```rust
        /// use er_save_lib::SaveApi;
        /// use er_save_lib::TalkParam::TalkParam;
        /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
        /// let param = save_api.get_param::<TalkParam>().unwrap();
        /// ```
        pub fn get_param<P: crate::param_trait::Param>(&self) -> Result<Param<P>, SaveApiError> {
            let rows = self.raw.user_data_11.regulation.get_param::<P>()?;
            Ok(Param::<P> { rows })
        }

        /// Returns a map of parameter bytes.
        ///
        /// # Example
        /// ```rust
        /// use er_save_lib::SaveApi;
        /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
        /// let param_bytes_map = save_api.get_param_bytes_map().unwrap();
        /// ```
        pub fn get_param_bytes_map(&self) -> Result<&BTreeMap<String, Vec<u8>>, SaveApiError> {
            Ok(&self
                .raw
                .user_data_11
                .regulation
                .content
                .data
                .file_data
                .param_files)
        }
    }
}
