use datalang::datalang;

datalang! {
    import Base

    term Handle {
    }

    SocialMediaUser {
        +Base::Name
        +Base::BirthDate

        +Handle
    }
}