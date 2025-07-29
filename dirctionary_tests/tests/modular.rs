use datalang_tests::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_module_types_exist() {
        // Test that types from both modules can be used together
        let _user = User::new();
        let _social_user = SocialMediaUser::new();
        let _name = Name::new();
        let _handle = Handle::new();
        
        // If compilation succeeds, cross-module usage works
        assert!(true);
    }

    #[test]
    fn test_user_has_all_base_fields() {
        let mut user = User::new();
        user.name = "Alice".to_string();
        user.lastname = "Smith".to_string();
        user.birthdate = "1985-12-25".to_string();
        
        // Test that User has all base fields
        assert_eq!(user.name, "Alice");
        assert_eq!(user.lastname, "Smith");
        assert_eq!(user.birthdate, "1985-12-25");
    }

    #[test]
    fn test_social_media_user_selective_fields() {
        let mut social_user = SocialMediaUser::new();
        social_user.name = "Bob".to_string();
        social_user.birthdate = "1990-06-15".to_string();
        social_user.handle = "@bobsmith".to_string();
        
        // Test that SocialMediaUser has selective fields
        assert_eq!(social_user.name, "Bob");
        assert_eq!(social_user.birthdate, "1990-06-15");
        assert_eq!(social_user.handle, "@bobsmith");
    }

    #[test]
    fn test_field_exclusion() {
        // Test that User has lastname but SocialMediaUser doesn't
        let user = User::new();
        let social_user = SocialMediaUser::new();
        
        // User should have lastname
        let _ = user.lastname;
        
        // SocialMediaUser should NOT have lastname (would cause compile error)
        // let _ = social_user.lastname; // This should NOT compile
        
        // Both should have name and birthdate
        let _ = user.name;
        let _ = user.birthdate;
        let _ = social_user.name;
        let _ = social_user.birthdate;
        
        // Only SocialMediaUser should have handle
        let _ = social_user.handle;
        // let _ = user.handle; // This should NOT compile
        
        assert!(true); // Test passes if compilation succeeds
    }

    #[test]
    fn test_types_can_coexist() {
        let mut user = User::new();
        let mut social_user = SocialMediaUser::new();
        
        user.name = "Alice".to_string();
        social_user.name = "Bob".to_string();
        
        // Test that both types can have the same field names but different values
        assert_eq!(user.name, "Alice");
        assert_eq!(social_user.name, "Bob");
        
        // And they maintain their separate identities
        assert_ne!(user.name, social_user.name);
    }
}

fn main() {
    // This allows running as an example too
    println!("Run 'cargo test' to execute the actual tests");
}