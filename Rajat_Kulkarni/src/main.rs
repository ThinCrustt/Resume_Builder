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
use std::{collections::HashMap, thread::current};
use chrono::Utc;
use std::fmt;
use rocket::Request;
//use handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};
use rocket_contrib :: templates :: {Template, handlebars};
use crate::handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, RenderError, JsonRender};
use std::sync::atomic::AtomicUsize;
use rocket::State;
use rocket::request::Form;



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


    #[derive(serde::Serialize)]
    struct CurrentInfo {
        date: &'static str
    }


    #[derive(serde::Serialize)]

    struct userInfo {
        first_name: &'static str,
        last_name: &'static str,
    }


    #[get("/dashboard")]
    pub fn dashboard() -> Template {
        Template::render("dashboard", &userInfo {
            first_name: "john",
            last_name: "Marty",
        })
    }
}


#[derive(FromForm)]
struct Task {
    Name: String,
    Description: String,
}

#[post("/dashboard", data = "<task>")]
fn publish_to_dashboard(task: Form<Task>) -> Template{ 
    print!("HIT");
    print!("{}", task.Name);
    print!("{}", task.Description);
    let mut context = HashMap::new();
    let name = task.Name.clone();
    let description = task.Description.clone();

    context.insert(
        "Name".to_string(),
        name,
    );

    context.insert(
        "Description".to_string(),
        description,
    );

    Template::render("dashboard", &context)
}


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

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}


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
    rocket::ignite()
    .mount("/", routes![index, publish_to_dashboard, other::dashboard])
    .attach(Template::fairing())
    .launch();    

    //rocket().launch();
}


// ------------------------------------------------------------------------
// ------------------------------------------------------------------------
// ------------------------------------------------------------------------
// ------------------------------------------------------------------------
// ------------------------------------------------------------------------
// ------------------------------------------------------------------------
// ------------------------------------------------------------------------
// ------------------------------------------------------------------------

// --------------------- TESTING -----------------------

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
