# Macro syntax should support the following

in base.rs we want to have:
```
dictionary Base

term Name {
}

term LastName {
}

term BirthDate {
}

term User has {
    +Name

    +LastName

    +BirthDate
}

in social_media.rs we want:
```
import Base

term Handle {

}

SocialMediaUser {
    +Base::Name
    +Base::BirthDate

    +Handle
}
```

and this should be equivalent to:
```
import Base

term Handle {

}

SocialMediaUser {
    +Base::User
    -Base::LastName

    +Handle
}
```