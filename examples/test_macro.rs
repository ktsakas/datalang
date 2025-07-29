use datalang::datalang;

// Define a User struct using the custom syntax with attributes
datalang! {
    User {
        #[length < 10]
        name

        last_name

        birthdate
    }
}

// Define SocialMediaUser that inherits from User
datalang! {
    SocialMediaUser {
        trait User

        handle
    }
}

fn main() {
    // Test User with validation
    let mut user = User::new();
    user.name = "John".to_string();
    user.last_name = "Doe".to_string();
    user.birthdate = "1990-01-01".to_string();
    
    println!("User: {:?}", user);
    
    // Test validation
    match user.validate_name() {
        Ok(_) => println!("Name validation passed"),
        Err(e) => println!("Name validation failed: {}", e),
    }
    
    // Test with invalid name (too long)
    user.name = "ThisNameIsTooLong".to_string();
    match user.validate_name() {
        Ok(_) => println!("Long name validation passed"),
        Err(e) => println!("Long name validation failed: {}", e),
    }
    
    // Test SocialMediaUser
    let mut social_user = SocialMediaUser::new();
    social_user.handle = "@johndoe".to_string();
    
    println!("Social Media User: {:?}", social_user);
    println!("Handle: {}", social_user.handle);
}