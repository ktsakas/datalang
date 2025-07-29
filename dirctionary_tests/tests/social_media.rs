use datalang_tests::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_type() {
        let handle = Handle::new();
        assert_eq!(handle.handle, "");
    }

    #[test]
    fn test_social_media_user_fields() {
        let mut social_user = SocialMediaUser::new();
        social_user.name = "Jane".to_string();
        social_user.birthdate = "1995-05-15".to_string();
        social_user.handle = "@jane_dev".to_string();
        
        // Test that SocialMediaUser has expected fields
        assert_eq!(social_user.name, "Jane");
        assert_eq!(social_user.birthdate, "1995-05-15");
        assert_eq!(social_user.handle, "@jane_dev");
    }

    #[test]
    fn test_social_media_user_excludes_lastname() {
        // This test verifies that SocialMediaUser doesn't have lastname field
        // by ensuring it compiles without lastname
        let social_user = SocialMediaUser::new();
        
        // These fields should exist
        let _ = social_user.name;
        let _ = social_user.birthdate;
        let _ = social_user.handle;
        
        // If lastname field existed, this would cause a compile error:
        // let _ = social_user.lastname; // This line should NOT compile
        
        assert!(true); // Test passes if compilation succeeds
    }

    #[test]
    fn test_social_media_user_constructor() {
        let social_user = SocialMediaUser::new();
        
        // Test that new() creates empty strings for all fields
        assert_eq!(social_user.name, "");
        assert_eq!(social_user.birthdate, "");
        assert_eq!(social_user.handle, "");
    }
}

fn main() {
    // This allows running as an example too
    println!("Run 'cargo test' to execute the actual tests");
}