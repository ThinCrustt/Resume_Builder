/*

Author: Rajat Kulkarni

References:

[1] https://www.linode.com/docs/guides/build-a-website-using-rust-and-the-rocket-web-framework/
[2] https://medium.com/@james_32022/rocket-frontend-templates-and-static-assets-5b6d04243a08
[3] https://blog.logrocket.com/template-rendering-in-rust/
[4] https://github.com/SergioBenitez/Rocket/tree/v0.4

*/

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

extern crate harmandeep;
use harmandeep::Resume;
use harmandeep::Task;
//use diesel::sql_types::Bool;
//use rocket::response::Redirect;
use std::collections::HashMap;

use crate::handlebars::{
    Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext, RenderError,
};
use rocket::request::Form;
use rocket::Request;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::{handlebars, Template};

struct ResumeCollection {
    list_of_resumes: Vec<Resume>,
}

mod other {
    use rocket_contrib::templates::Template;
    use std::collections::HashMap;

    #[derive(serde::Serialize)]
    struct CurrentInfo {
        date: &'static str,
    }

    #[derive(serde::Serialize)]

    struct userInfo {
        first_name_a: &'static str,
        last_name_a: &'static str,
    }

    #[get("/dashboard")]
    pub fn dashboard() -> Template {
        Template::render(
            "dashboard",
            &userInfo {
                first_name_a: "john",
                last_name_a: "Marty",
            },
        )
    }
    #[get("/resume")]
    pub fn resume() -> Template {
        Template::render(
            "resume",
            &userInfo {
                first_name_a: "john",
                last_name_a: "Marty",
            },
        )
    }
    #[get("/error_page")]
    pub fn error_page() -> Template {
        Template::render(
            "error_page",
            &userInfo {
                first_name_a: "john",
                last_name_a: "Marty",
            },
        )
    }
    #[get("/about")]
    pub fn about() -> Template {
        Template::render(
            "about",
            &userInfo {
                first_name_a: "john",
                last_name_a: "Marty",
            },
        )
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
#[post("/dashboard", data = "<task>")]
fn publish_to_resume(task: Form<Task>) -> Template {
    print!("HIT");
    print!("{}", task.first_name);
    print!("{}", task.last_name);
    print!("{}", task.phone_number);
    print!("{}", task.email);
    print!("{}", task.linkedin);
    print!("{}", task.city);
    print!("{}", task.state);
    print!("{}", task.zip_code);
    print!("{}", task.job_title);
    print!("{}", task.employer);
    print!("{}", task.start_date);
    print!("{}", task.end_date);
    print!("{}", task.description);
    print!("{}", task.school);
    print!("{}", task.gpa);
    print!("{}", task.field_of_study);
    print!("{}", task.graduation_date);
    print!("{}", task.languages);
    print!("{}", task.softwares);
    print!("{}", task.other_tools);

    let mut context = HashMap::new();
    let first_name = task.first_name.clone();
    let last_name = task.last_name.clone();
    let phone_number = task.phone_number.clone();
    let email = task.email.clone();
    let linkedin = task.linkedin.clone();
    let city = task.city.clone();
    let state = task.state.clone();
    let zip_code = task.zip_code.clone();
    let job_title = task.job_title.clone();
    let employer = task.employer.clone();
    let start_date = task.start_date.clone();
    let end_date = task.end_date.clone();
    let description = task.description.clone();
    let school = task.school.clone();
    let gpa = task.gpa.clone();
    let field_of_study = task.field_of_study.clone();
    let graduation_date = task.graduation_date.clone();
    let languages = task.languages.clone();
    let softwares = task.softwares.clone();
    let other_tools = task.other_tools.clone();

    if harmandeep::validate_user_input(task) {
        context.insert("First_name".to_string(), first_name);

        context.insert("Last_name".to_string(), last_name);

        context.insert("Phone_number".to_string(), phone_number);

        context.insert("Email".to_string(), email);

        context.insert("Linkedin".to_string(), linkedin);
        context.insert("City".to_string(), city);
        context.insert("State".to_string(), state);
        context.insert("Zip_code".to_string(), zip_code);
        context.insert("Job_title".to_string(), job_title);
        context.insert("Employer".to_string(), employer);
        context.insert("Start_date".to_string(), start_date);
        context.insert("End_date".to_string(), end_date);
        context.insert("Description".to_string(), description);
        context.insert("School".to_string(), school);
        context.insert("Gpa".to_string(), gpa);
        context.insert("Field_of_study".to_string(), field_of_study);
        context.insert("Graduation_date".to_string(), graduation_date);
        context.insert("Languages".to_string(), languages);
        context.insert("Softwares".to_string(), softwares);
        context.insert("Other_tools".to_string(), other_tools);
        Template::render("resume", &context)
    } else {
        Template::render("error_page", &context)
    }
}

#[get("/")]
fn index() -> Template {
    let current_time = chrono::offset::Local::now();
    let a = current_time.to_string();
    let s_slice = &a[..];
    let sb: Vec<&str> = s_slice.split(' ').collect();
    let date = sb.get(0).unwrap();
    //let date2 = date.clone();
    let context: HashMap<&str, &str> = [("first name", "Jonathan") /*, ("date", date2)*/]
        .iter()
        .cloned()
        .collect();

    Template::render("index", &context)
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
        .mount(
            "/",
            routes![
                index,
                other::dashboard,
                other::resume,
                other::error_page,
                other::about
            ],
        )
        .register(catchers![not_found])
        .manage(ResumeCollection {
            list_of_resumes: Vec::<Resume>::new(),
        })
        .attach(Template::custom(|engines| {
            engines
                .handlebars
                .register_helper("wow", Box::new(harmandeep::wow_helper));
        }))
}

fn main() {
    let helper_needed = 0;

    if helper_needed == 0 {
        rocket::ignite()
            .mount(
                "/",
                routes![
                    index,
                    publish_to_resume,
                    other::dashboard,
                    other::resume,
                    other::error_page,
                    other::about
                ],
            )
            .mount("/static", StaticFiles::from("static"))
            .attach(Template::fairing())
            .launch();
    } else {
        rocket().launch();
    }
}

// --------------------- TESTING -----------------------

macro_rules! dispatch {
    ($method:expr, $path:expr, $test_fn:expr) => {{
        let client = Client::new(rocket()).unwrap();
        $test_fn(&client, client.req($method, $path).dispatch());
    }};
}

// Imports specific to testing:

//use super::{rocket, TemplateContext};

use rocket::http::Method::*;
use rocket::http::Status;
use rocket::local::{Client, LocalResponse};

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
        dispatch!(
            *method,
            "/",
            |client: &Client, mut response: LocalResponse| {
                let mut map = ::std::collections::HashMap::new();
                map.insert("path", "/");
                let expected = Template::show(client.rocket(), "error/404", &map).unwrap();

                assert_eq!(response.status(), Status::NotFound);
                assert_eq!(response.body_string(), Some(expected));
            }
        );
    }
}

/*
Test 4: Tests the error catcher
*/

#[test]
fn test_404() {
    dispatch!(
        Get,
        "/hello/",
        |client: &Client, mut response: LocalResponse| {
            let mut map = ::std::collections::HashMap::new();
            map.insert("path", "/hello/");

            let expected = Template::show(client.rocket(), "error/404", &map).unwrap();
            assert_eq!(response.status(), Status::NotFound);
            assert_eq!(response.body_string(), Some(expected));
        }
    );
}
