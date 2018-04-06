mod user;
use class;

pub struct User {
    name : String,
    class: class::Class
}

impl User
{
    pub fn new(name: &str) -> Self
    {
        User{
            name: String::from(name),
            class: class::Class{ id: 0, name: "Default" }
        }
    }

    pub fn SetClass(&mut self, class : class::Class)
    {
        self.class = class;
    }
}


fn createUser() {

}
