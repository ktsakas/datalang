use datalang_tests::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_individual_base_terms() {
        let name = Name::new();
        let last_name = LastName::new();
        let birth_date = BirthDate::new();
        
        // Test that types are created correctly
        assert_eq!(name.name, "");
        assert_eq!(last_name.lastname, "");
        assert_eq!(birth_date.birthdate, "");
    }

    #[test]
    fn test_composite_user_type() {
        let mut user = User::new();
        user.name = "John".to_string();
        user.lastname = "Doe".to_string();
        user.birthdate = "1990-01-01".to_string();
        
        // Test that User has all expected fields
        assert_eq!(user.name, "John");
        assert_eq!(user.lastname, "Doe");
        assert_eq!(user.birthdate, "1990-01-01");
    }

    #[test]
    fn test_user_constructor() {
        let user = User::new();
        
        // Test that new() creates empty strings
        assert_eq!(user.name, "");
        assert_eq!(user.lastname, "");
        assert_eq!(user.birthdate, "");
    }
}

fn main() {
    // This allows running as an example too
    println!("Run 'cargo test' to execute the actual tests");
}