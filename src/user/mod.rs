extern crate rand;
//use self::rand::Rng;
use self::rand::Rng;




mod user;

pub struct Class  {
    pub id   : i32,
    pub name : &'static str
}

pub struct User {
    health : i32,
    minDamage : i32,
    maxDamage : i32,
    name : String,
    class: Class
}

impl User
{

    pub fn CreateBot() -> Self {
        return User::new("");
    }

    pub fn new(name: &str) -> Self
    {
        User{
            name: String::from(name),
            class: Class{ id: 0, name: "Default" },
            health: 100,
            minDamage: 10,
            maxDamage: 30
        }
    }

    pub fn GetInfo(&self) {

    }

    pub fn SetClass(&mut self, class : Class)
    {
        self.class = class;
    }

    pub fn Getname(&self) -> &String {
        &self.name
    }

    pub fn ToString(self) -> String {
        self.name
    }


    // further mb add dodge chance
    pub fn GetDamage(&self) -> i32 {
        //let rng = rand::thread_rng();
        //let x = rng.get_range(self.minDamage, self.maxDamage);

        let num = rand::thread_rng().gen_range(self.minDamage, self.maxDamage);
        return num;
    }


    // further mb add dodge chance
    pub fn RecieveDamage(&mut self, x : i32) {
        self.health-=x;
    }

    pub fn GetHealth(&self)->i32  {return self.health;}

    pub fn IsAlive(self) -> bool {
        return self.health>0;
    }


}

fn createRandomBot() -> User {
    let user: User = User::new("");
    return user;
}

fn createUser() {

}

