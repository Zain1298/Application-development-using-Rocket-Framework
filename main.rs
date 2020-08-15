#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::content::Html;
use rocket::request::Form;


fn main() {
    rocket::ignite().mount("/", routes![index, add]).launch();
}

#[get("/")]
fn index() -> Html<&'static str> {
    Html(r#"<!DOCTYPE html>
<html>
    <head> 
        <title>Assignment</title>
    </head>
    <body>
        <h1>Development of application using Rocket Framework</h1>
        <p>If you want to add number then just go to the above address bar and after 8000 write /add/number like /add/20 and enter. You will get the result with addition of 100 like 120</p>
        <h2>Bonus Task</h2>
        <p> Here are the pre-defined examples for better understanding.
        <p>Click <a href="http://localhost:8000/add/20">Here</a> to Send a POST request with html form to the api</p>
        <p>Click <a href="http://localhost:8000/add/50",>Here</a> to Send a POST request with html form to the api</p>
        <i> End</i>
    </body>
</html>"#)
}


#[get("/add/<number>")]
fn add(number: i32) -> String {
    let new_number = number + 100;
    format!("You entered {}, We added 100, new number is {}",number, new_number)
}














