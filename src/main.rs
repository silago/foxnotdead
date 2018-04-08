pub mod input;
pub mod user;
pub mod class;

extern crate rusqlite;


use std::io;
use rusqlite::Connection;



struct ParamsHolder {
    id  : i32,
    str : i32,
    dex : i32,
    lck : i32
}

struct ClassLevelParams {
    class_id : i32,
    params_id: i32

}



struct Location {
    id : i32,

}

struct LocationAction {
    id : i32,
    location_id : i32,
    requirement_param_id : i32,
    reward_param_id : i32,
    result_action_id : i32
}



fn main() {
    let conn = Connection::open_in_memory().unwrap();
    let thief_class = user::Class{ id: 0, name: "" };
    let fighter_class = user::Class{ id: 0, name: "" };

    /* prepare end */


    let mut u : user::User = user::User::new(
            input::getInput("Enter tour name").trim()
    );
    let class_input = input::getInput("Choose your class:");


    let mut bot : user::User = user::User::CreateBot();
    //let u2 = &mut &u;
    //let b2 = &mut &bot;
    //if (class_input == "1") {
    //    u.SetClass(thief_class);
    //}

    //if (class_input == "2") {
    //    u.SetClass(fighter_class);
    //}




    battle(&mut u, &mut bot);

}


fn kick(auser : &user::User, aname : &str, buser: &mut user::User, bname : &str) {
    let damage = auser.GetDamage();
    buser.RecieveDamage(damage);

    println!("{} damaged: {}, {} health left: {}", aname, damage,bname, buser.GetHealth());
    //(attacker,defender);
}

fn battle(user : &mut user::User, bot : &mut user::User) {
    let damage : i32;
    while (true) {
        kick(user, "you", bot, "enemy");
        if (bot.GetHealth()<=0) {
            println!("you win");
            return;
        }


        kick(bot, "enemy", user,"you");

        if (user.GetHealth()<=0) {
            println!("you win");
            return;
        }

        input::getInput("").trim();
    }
}

fn walk() {
    let bot  = user::User::CreateBot();
    print!("you have walked down the steer and meet {}! Fight him: " , bot.ToString());
}