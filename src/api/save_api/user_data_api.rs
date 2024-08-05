pub mod user_data_api {
    use crate::SaveApiError;
    impl crate::SaveApi {
        /// Sets the archetype of the character at the specified index.
        ///
        /// # Example
        /// ```rust
        /// use er_save_lib::SaveApi;
        /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
        /// let (index, archetype_id) = (0, 1);
        /// save_api.set_archetype(index, archetype_id);
        /// ```
        pub fn set_archetype(&mut self, index: usize, archetype: u8) -> Result<(), SaveApiError> {
            self.raw.user_data_x[index].player_game_data.archetype = archetype;
            self.raw.user_data_10.profile_summary.profiles[index].archetype = archetype;
            Ok(())
        }

        /// Sets the level of the character at the specified index.
        ///
        /// # Example
        /// ```rust
        /// use er_save_lib::SaveApi;
        /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
        /// save_api.set_level(0, 1);
        /// ```
        pub fn set_level(&mut self, index: usize, level: u32) -> Result<(), SaveApiError> {
            self.raw.user_data_x[index].player_game_data.level = level;
            self.raw.user_data_10.profile_summary.profiles[index].level = level;
            Ok(())
        }

        /// Sets the rune memory of the character at the specified index.
        ///
        /// # Example
        /// ```rust
        /// use er_save_lib::SaveApi;
        /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
        /// save_api.set_runes_memory(0, 1_000);
        /// ```
        pub fn set_runes_memory(
            &mut self,
            index: usize,
            runes_memory: u32,
        ) -> Result<(), SaveApiError> {
            self.raw.user_data_x[index].player_game_data.runes_memory = runes_memory;
            self.raw.user_data_10.profile_summary.profiles[index].runes_memory = runes_memory;
            Ok(())
        }

        /// Sets the name of the character at the specified index.
        ///
        /// # Example
        /// ```rust
        /// use er_save_lib::SaveApi;
        /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
        /// save_api.set_character_name(0, "NewName").unwrap();
        /// ```
        pub fn set_character_name(
            &mut self,
            index: usize,
            new_name: &str,
        ) -> Result<(), SaveApiError> {
            self.raw.user_data_x[index].player_game_data.character_name = new_name.to_string();
            self.raw.user_data_10.profile_summary.profiles[index].character_name =
                new_name.to_string();
            Ok(())
        }

        /// Sets the gender of the character at the specified index.
        ///
        /// # Example
        /// ```rust
        /// use er_save_lib::SaveApi;
        /// let mut save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
        /// let (index, gender_id) = (0, 1);
        /// save_api.set_gender(index, gender_id);
        /// ```
        pub fn set_gender(&mut self, index: usize, gender: u8) -> Result<(), SaveApiError> {
            self.raw.user_data_x[index].player_game_data.gender = gender;
            self.raw.user_data_10.profile_summary.profiles[index].gender = gender;
            Ok(())
        }
    }
}
