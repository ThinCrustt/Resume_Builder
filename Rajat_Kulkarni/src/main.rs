/*

Author: Rajat Kulkarni

References:

[1] https://www.linode.com/docs/guides/build-a-website-using-rust-and-the-rocket-web-framework/
[2] https://medium.com/@james_32022/rocket-frontend-templates-and-static-assets-5b6d04243a08
[3] https://blog.logrocket.com/template-rendering-in-rust/

*/


#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;


use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use std::{collections::HashMap, thread::current};
use chrono::Utc;
use std::fmt;


mod other {
    use rocket_contrib::templates::Template;
    use std::collections::HashMap;


    #[derive(serde::Serialize)]
    struct CurrentInfo {
        date: &'static str
    }


    #[derive(serde::Serialize)]
    struct userInfo {
        first_name: &'static str,
        last_name: &'static str,
        /*phone_number: u128,
        skills: Option<String>,
        education: Option<String>,
        experience: Option<String>*/
    }


    /*
    -- dashboard() --

    - routes to /dashboard
    - renders the dashboard.hbs handlebars template

    TO DO:
    - have a form in dashboard.hbs that opens when the "create resume" button is pressed.
      The state from that form should be passed as context into the dashboard.hbs template
    */

    #[get("/dashboard")]
    pub fn dashboard() -> Template {
        //let context: HashMap<&str, &str> = [("name", "Jonathan")]
        //.iter().cloned().collect();

        Template::render("dashboard", &userInfo {
            first_name: "john",
            last_name: "Marty",
            /*phone_number: 19999999999,
            skills: Some("F".to_string()),
            education: Some("F".to_string()),
            experience: Some("F".to_string())*/
        })
    }
}

/*#[get("/")]
pub fn index() -> &'static str {
    "MAIN PAGE - 
    Add more description
    to
    increase
    line
    count
    like
    this"
}*/

#[get("/")]
fn index() -> Template {
    
    let mut current_time = chrono::offset::Local::now();
    let mut a = current_time.to_string(); 
    print!("ORIGINAL: {}", current_time);
    let mut s_slice: &str = &a[..];
    let mut sb: Vec<&str>  = s_slice.split(' ').collect();
    let mut t = sb.get(0).unwrap();
    let context: HashMap<&str, &str> = [("name", "Jonathan"), ("time", t)].iter().cloned().collect();
    
    Template::render("index", &context)
}

fn main() {

    let dt = Utc::now();
    let timestamp: i64 = dt.timestamp();

    //println!("Current timestamp is {}", timestamp);
    //println!("{:?}", chrono::offset::Local::now());

    
    print!("****************");
    let mut current_time = chrono::offset::Local::now();

    let mut a = current_time.to_string(); 
    print!("ORIGINAL: {}", current_time);
    let mut s_slice: &str = &a[..];
    let mut sb: Vec<&str>  = s_slice.split(' ').collect();


    for s in sb {
        println!("\n{}\n", s)
    }
    

    let mut strings = "bananas,apples,pear";
    let mut sa = strings.split(",");
    print!("****************");

    /*for s in current_time {
        print!("{}", s);
    }*/
    //print!("{}", current_time);



    //current_time.split_whitespace();
  


    rocket::ignite()
    .mount("/", routes![index, other::dashboard])
    .attach(Template::fairing())
    .launch();    
    
    //rocket::ignite().mount("/", routes![index, other::about, other::dashboard]).launch();
}


// TESTING

/*
TEST 1:
    Test the function to etc etc
*/ 