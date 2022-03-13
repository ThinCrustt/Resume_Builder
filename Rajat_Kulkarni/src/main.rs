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

extern crate harmandeep;

//use diesel::sql_types::Bool;
//use rocket::response::Redirect;
use std::collections::HashMap;


use rocket::Request;
use rocket_contrib :: templates :: {Template, handlebars};
use crate::handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, RenderError, JsonRender};
use rocket_contrib::serve::StaticFiles;
use rocket::request::Form;

struct Resume {
    name: String,
    lastname: String,
    phone_number: String,
    email: String,
    company: String,
}

struct ResumeCollection {
    list_of_resumes: Vec<Resume>
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


/*#[get("/count")]
fn count(resume_collection: State<ResumeCollection>) -> ResumeCollection {
    let mut retrieved_resume_collection = Vec::<ResumeCollection>::new();
    let mut temp_resume: Resume;

    for i in (0, resume_collection.list_of_resumes.len()) {

    }
    temp_resume.name = resume_collection.list_of_resumes.to_string();
    temp_resume.lastname = "h".to_string();
    temp_resume.phone_number = "h".to_string();
    temp_resume.email = "h".to_string();
    temp_resume.company = "h".to_string();
    
}*/

#[derive(FromForm)]
struct Task {
    name: String,
    lastname: String,
    phone_number: String,
    email: String,
    company: String,
}


fn validate_user_input(task: Form<Task>) -> bool {
    let name = task.name.clone();
    let lastname = task.lastname.clone();
    let phone_number = task.phone_number.clone();
    let email = task.email.clone();
    let company = task.company.clone();  

    name != "name" && lastname != "lastname" && "phone_number" != phone_number
       && email != "email" && company != "company"
}


#[post("/dashboard", data = "<task>")]
fn publish_to_dashboard(task: Form<Task>) -> Template{ 
    print!("HIT");
    print!("{}", task.name);
    print!("{}", task.lastname);
    print!("{}", task.phone_number);
    print!("{}", task.email);
    print!("{}", task.company);
    
    let mut context = HashMap::new();
    let name = task.name.clone();
    let lastname = task.lastname.clone();
    let phone_number = task.phone_number.clone();
    let email = task.email.clone();
    let company = task.company.clone();

    if validate_user_input(task) {
        context.insert(
            "Name".to_string(),
            name,
        );
    
        context.insert(
            "LastName".to_string(),
            lastname,
        );
    
        context.insert(
            "Phone_Number".to_string(),
            phone_number,
        );
    
        context.insert(
            "Email".to_string(),
            email,
        );
    
        context.insert(
            "Company".to_string(),
            company,
        );
    }

    Template::render("dashboard", &context)
}


#[get("/")]
fn index() -> Template {
    let current_time = chrono::offset::Local::now();
    let a = current_time.to_string(); 
    let s_slice = &a[..];
    let sb: Vec<&str>  = s_slice.split(' ').collect();
    let date = sb.get(0).unwrap();
    //let date2 = date.clone();
    let context: HashMap<&str, &str> = [("name", "Jonathan")/*, ("date", date2)*/].iter().cloned().collect();
    
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


#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

 
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index, other::dashboard])
        .register(catchers![not_found])
        .manage(ResumeCollection { list_of_resumes: Vec::<Resume>::new() })
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("wow", Box::new(wow_helper));
       
        }))       
}


fn main() {
    let helper_needed = 0;

    if helper_needed == 0 {
        rocket::ignite()
        .mount("/", routes![index, publish_to_dashboard, other::dashboard])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();    
    }
    else {
        rocket().launch();
    }
}


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
