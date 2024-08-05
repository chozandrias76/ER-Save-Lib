pub mod user_data_10_api {
    use crate::SaveApi;
    impl SaveApi {
        /// Returns the index of the character with the given name.
        ///
        /// # Example
        /// ```rust
        /// use er_save_lib::SaveApi;
        /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
        /// let index = save_api.character_index_from_name("CharacterName");
        /// ```
        pub fn character_index_from_name(&self, name: &str) -> Option<usize> {
            self.raw
                .user_data_10
                .profile_summary
                .profiles
                .iter()
                .position(|profile| profile.character_name.contains(name))
        }

        /// Returns an array indicating which characters are active.
        ///
        /// # Example
        /// ```rust
        /// use er_save_lib::SaveApi;
        /// let save_api = SaveApi::from_path("./test/ER0000.sl2").unwrap();
        /// let active_characters = save_api.active_characters();
        /// ```
        pub fn active_characters(&self) -> [bool; 10] {
            self.raw.user_data_10.profile_summary.active_profiles
        }
    }
}
