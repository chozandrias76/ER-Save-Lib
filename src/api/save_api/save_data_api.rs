mod save_data {
    use crate::SaveApi;
    use crate::SaveApiError;
    use crate::SaveType;
    use std::
        path::Path;

    impl SaveApi {
        /// Converts the save data to a vector of bytes.
        ///
        /// # Example
        /// ```rust
        /// use er_save_lib::SaveApi;
        /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
        /// let bytes = save_api.to_vec().unwrap();
        /// ```
        pub fn to_vec(&self) -> Result<Vec<u8>, SaveApiError> {
            let bytes = self.raw.write_to_vec()?;
            Ok(bytes)
        }

        /// Writes the save data to the specified path.
        ///
        /// # Example
        /// ```rust
        /// use er_save_lib::SaveApi;
        /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
        /// save_api.write_to_path("./test/null.sl2").unwrap();
        /// ```
        pub fn write_to_path(&self, path: impl AsRef<Path>) -> Result<(), SaveApiError> {
            Ok(self.raw.write_to_path(path)?)
        }

        /// Returns the platform type of the save file.
        ///
        /// # Example
        /// ```rust
        /// use er_save_lib::{SaveApi, SaveType};
        /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
        /// let platform = save_api.platform();
        /// assert_eq!(platform, SaveType::PC);
        /// ```
        pub fn platform(&self) -> SaveType {
            if self.raw.header.len() == 0x6c {
                SaveType::Playstation
            } else {
                SaveType::PC
            }
        }

        /// Returns the Steam ID associated with the save file.
        ///
        /// # Example
        /// ```rust
        /// use er_save_lib::SaveApi;
        /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
        /// let steam_id = save_api.steam_id();
        /// ```
        pub fn steam_id(&self) -> u64 {
            self.raw.user_data_10.steam_id
        }

        /// Sets the Steam ID associated with the save file.
        ///
        /// # Example
        /// ```rust
        /// use er_save_lib::SaveApi;
        /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
        /// save_api.set_steam_id(1234567890).unwrap();
        /// ```
        pub fn set_steam_id(&mut self, steam_id: u64) -> Result<(), SaveApiError> {
            self.raw.user_data_10.steam_id = steam_id;
            Ok(())
        }
    }
}
