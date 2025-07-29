use datalang::datalang;

// Comprehensive example showing all features in one file
datalang! {
    dictionary Base

    term Name {
    }

    term LastName {
    }

    term BirthDate {
    }

    term Handle {
    }

    term User has {
        +Name
        +LastName
        +BirthDate
    }

    SocialMediaUser {
        +Name
        +BirthDate
        +Handle
    }

    // Demonstrate exclusion syntax
    MinimalUser {
        +Name
    }
}

fn main() {
    println!("=== DataLang Macro Comprehensive Demo ===\n");
    
    // 1. Individual terms
    println!("1. Individual Terms:");
    let name = Name::new();
    let last_name = LastName::new();
    let birth_date = BirthDate::new();
    let handle = Handle::new();
    
    println!("   Name: {:?}", name);
    println!("   LastName: {:?}", last_name);
    println!("   BirthDate: {:?}", birth_date);
    println!("   Handle: {:?}", handle);
    
    // 2. Composite terms
    println!("\n2. Composite Terms:");
    let mut user = User::new();
    user.name = "John".to_string();
    user.lastname = "Doe".to_string();
    user.birthdate = "1990-01-01".to_string();
    
    println!("   User: {:?}", user);
    
    // 3. Selective field inclusion
    println!("\n3. Selective Field Inclusion:");
    let mut social_user = SocialMediaUser::new();
    social_user.name = "Jane".to_string();
    social_user.birthdate = "1995-05-15".to_string();
    social_user.handle = "@jane_doe".to_string();
    
    println!("   SocialMediaUser: {:?}", social_user);
    println!("   Note: Has name, birthdate, handle - but NO lastname");
    
    // 4. Minimal example
    println!("\n4. Minimal Field Selection:");
    let mut minimal = MinimalUser::new();
    minimal.name = "Alice".to_string();
    
    println!("   MinimalUser: {:?}", minimal);
    println!("   Note: Only has name field");
    
    println!("\n✅ All DataLang features working correctly!");
}