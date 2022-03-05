/*

Author: Rajat Kulkarni

References:

[1] https://www.linode.com/docs/guides/build-a-website-using-rust-and-the-rocket-web-framework/
[2] https://medium.com/@james_32022/rocket-frontend-templates-and-static-assets-5b6d04243a08
[3] https://blog.logrocket.com/template-rendering-in-rust/
[4] https://github.com/SergioBenitez/Rocket/tree/v0.4

*/


#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;


use rocket::response::Redirect;
//use rocket_contrib::templates::Template;
use std::{collections::HashMap, thread::current};
use chrono::Utc;
use std::fmt;
use rocket::Request;
//use handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};
use rocket_contrib :: templates :: {Template, handlebars};
use crate::handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, RenderError, JsonRender};
use std::sync::atomic::AtomicUsize;
use rocket::State;


/*
WRITE 
COMMENTS
HERE
*/
struct HitCount {
    count: AtomicUsize
}


mod other {
    use rocket_contrib::templates::Template;
    use std::collections::HashMap;
    //use handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};


    #[derive(serde::Serialize)]
    struct CurrentInfo {
        date: &'static str
    }


    #[derive(serde::Serialize)]

    // WRITE COMMENT HERE
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

/*
WRITE 
COMMENTS
HERE
*/

/*#[get("/count")]
fn count(hit_count: State<HitCount>) -> String {
    let current_count = hit_count.count.load(Ordering::Relaxed);
    format!("Number of visits: {}", current_count)
}*/

 /*
WRITE 
COMMENTS
HERE
*/

#[get("/")]
fn index() -> Template {
    
    let mut current_time = chrono::offset::Local::now();
    let mut a = current_time.to_string(); 
    //print!("ORIGINAL: {}", current_time);
    let mut s_slice: &str = &a[..];
    let mut sb: Vec<&str>  = s_slice.split(' ').collect();
    let mut t = sb.get(0).unwrap();
    let context: HashMap<&str, &str> = [("name", "Jonathan"), ("time", t)].iter().cloned().collect();
    
    Template::render("index", &context)
}


/*
CUSTOM HANDLEBARS HELPERES
    - wow_helper() obtained from Sergio Benitez [4]
    - 
*/

fn wow_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output
) -> HelperResult {
    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
        
    }

    Ok(())
}

/*
WRITE 
COMMENTS
HERE
*/

fn on_create_click(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output
) -> HelperResult {
    if let Some(param) = h.param(0) {
        out.write("</b></i>")?;
    }

    Ok(())
}

/*
WRITE 
COMMENTS// Obtained from Sergio Benitez [4]

Error catcher: _________
HERE
*/

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

/*
WRITE 
COMMENTS
HERE
*/

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, other::dashboard])
        .register(catchers![not_found])
        .manage(HitCount {count: AtomicUsize::new(0) })
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("wow", Box::new(wow_helper));
        }))
}


fn main() {
    /*for s in current_time {
        print!("{}", s);
    }*/
    //print!("{}", current_time);

    //current_time.split_whitespace();

    /*rocket::ignite()
    .mount("/", routes![index, other::dashboard])
    .attach(Template::fairing())
    .launch();    */

    rocket().launch();
}


// TESTING

macro_rules! dispatch {
    ($method:expr, $path:expr, $test_fn:expr) => ({
        let client = Client::new(rocket()).unwrap();
        $test_fn(&client, client.req($method, $path).dispatch());
    })
}


// Imports specific to testing:

//use super::{rocket, TemplateContext};

use rocket::local::{Client, LocalResponse};
use rocket::http::Method::*;
use rocket::http::Status;


/*
TEST 1:
    Test the function to etc etc
*/ 

#[test]
fn test_root() {
    // Check that the redirect works.
    for method in &[Get, Head] {
        dispatch!(*method, "/", |_: &Client, mut response: LocalResponse| {
            assert_eq!(response.status(), Status::SeeOther);
            assert!(response.body().is_none());

            let location: Vec<_> = response.headers().get("Location").collect();
            assert_eq!(location, vec!["/hello/Unknown"]);
        });
    }

    // Check that other request methods are not accepted (and instead caught).
    for method in &[Post, Put, Delete, Options, Trace, Connect, Patch] {
        dispatch!(*method, "/", |client: &Client, mut response: LocalResponse| {
            let mut map = ::std::collections::HashMap::new();
            map.insert("path", "/");
            let expected = Template::show(client.rocket(), "error/404", &map).unwrap();

            assert_eq!(response.status(), Status::NotFound);
            assert_eq!(response.body_string(), Some(expected));
        });
    }
}


/*
Test 4: Tests the error catcher
*/

#[test]
fn test_404() {
    dispatch!(Get, "/hello/", |client: &Client, mut response: LocalResponse| {
        let mut map = ::std::collections::HashMap::new();
        map.insert("path", "/hello/");

        let expected = Template::show(client.rocket(), "error/404", &map).unwrap();
        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(response.body_string(), Some(expected));
    });
}