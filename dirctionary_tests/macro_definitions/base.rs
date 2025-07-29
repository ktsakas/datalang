use datalang::datalang;

datalang! {
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
}