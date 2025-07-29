# Macro syntax should support the following

In base.rs we want to have:
```datalang
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
```

In social_media.rs we want:
```datalang
import Base

term Handle {
}

SocialMediaUser {
    +Base::Name
    +Base::BirthDate
    +Handle
}
```

And this should be equivalent to:
```datalang
import Base

term Handle {
}

SocialMediaUser {
    +Base::User
    -Base::LastName
    +Handle
}
```