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

use harmandeep::Task;
//use diesel::sql_types::Bool;
//use rocket::response::Redirect;
use std::collections::HashMap;

use rocket::request::Form;
use rocket::request::Request;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod other {
    use rocket_contrib::templates::Template;

    #[derive(serde::Serialize)]
    struct CurrentInfo {
        date: &'static str,
    }

    #[derive(serde::Serialize)]

    struct UserInfo {
        first_name_a: &'static str,
        last_name_a: &'static str,
    }

    #[get("/dashboard")]
    pub fn dashboard() -> Template {
        Template::render(
            "dashboard",
            &UserInfo {
                first_name_a: "john",
                last_name_a: "Marty",
            },
        )
    }
    #[get("/resume_page")]
    pub fn resume_page() -> Template {
        Template::render(
            "resume_page",
            &UserInfo {
                first_name_a: "john",
                last_name_a: "Marty",
            },
        )
    }
    #[get("/error_page")]
    pub fn error_page() -> Template {
        Template::render(
            "error_page",
            &UserInfo {
                first_name_a: "john",
                last_name_a: "Marty",
            },
        )
    }
    #[get("/about")]
    pub fn about() -> Template {
        Template::render(
            "about",
            &UserInfo {
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
    print!("{}", task.uschool);
    print!("{}", task.ugpa);
    print!("{}", task.ufield_of_study);
    print!("{}", task.ugraduation_date);
    print!("{}", task.languages);
    print!("{}", task.softwares);
    print!("{}", task.other_tools);
    print!("{}", task.project_title_1);
    print!("{}", task.project_tech_1);
    print!("{}", task.project_desc_1);
    print!("{}", task.project_title_2);
    print!("{}", task.project_tech_2);
    print!("{}", task.project_desc_2);
    print!("{}", task.project_title_3);
    print!("{}", task.project_tech_3);
    print!("{}", task.project_desc_3);

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
    let uschool = task.uschool.clone();
    let ugpa = task.ugpa.clone();
    let ufield_of_study = task.ufield_of_study.clone();
    let ugraduation_date = task.ugraduation_date.clone();
    let languages = task.languages.clone();
    let softwares = task.softwares.clone();
    let other_tools = task.other_tools.clone();
    let project_title_1 = task.project_title_1.clone();
    let project_tech_1 = task.project_tech_1.clone();
    let project_desc_1 = task.project_desc_1.clone();
    let project_title_2 = task.project_title_2.clone();
    let project_tech_2 = task.project_tech_2.clone();
    let project_desc_2 = task.project_desc_2.clone();
    let project_title_3 = task.project_title_3.clone();
    let project_tech_3 = task.project_tech_3.clone();
    let project_desc_3 = task.project_desc_3.clone();

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
        context.insert("uSchool".to_string(), uschool);
        context.insert("uGpa".to_string(), ugpa);
        context.insert("uField_of_study".to_string(), ufield_of_study);
        context.insert("uGraduation_date".to_string(), ugraduation_date);
        context.insert("Languages".to_string(), languages);
        context.insert("Softwares".to_string(), softwares);
        context.insert("Other_tools".to_string(), other_tools);
        context.insert("Project_title_1".to_string(), project_title_1);
        context.insert("Project_tech_1".to_string(), project_tech_1);
        context.insert("Project_desc_1".to_string(), project_desc_1);
        context.insert("Project_title_2".to_string(), project_title_2);
        context.insert("Project_tech_2".to_string(), project_tech_2);
        context.insert("Project_desc_2".to_string(), project_desc_2);
        context.insert("Project_title_3".to_string(), project_title_3);
        context.insert("Project_tech_3".to_string(), project_tech_3);
        context.insert("Project_desc_3".to_string(), project_desc_3);

        Template::render("resume_page", &context)
    } else {
        context.insert("First_name".to_string(), first_name);
        context.insert("Last_name".to_string(), last_name);
        context.insert("Phone_number".to_string(), phone_number);
        context.insert("Email".to_string(), email);
        context.insert("City".to_string(), city);
        context.insert("State".to_string(), state);
        context.insert("Zip_code".to_string(), zip_code);
        context.insert("uSchool".to_string(), uschool);
        context.insert("uGpa".to_string(), ugpa);
        context.insert("uField_of_study".to_string(), ufield_of_study);
        context.insert("uGraduation_date".to_string(), ugraduation_date);
        context.insert("Languages".to_string(), languages);
        context.insert("Softwares".to_string(), softwares);

        Template::render("error_page", &context)
    }
}

#[get("/")]
fn index() -> Template {
    let current_time = chrono::offset::Local::now();
    let a = current_time.to_string();
    let s_slice = &a[..];
    let sb: Vec<&str> = s_slice.split(' ').collect();
    let _date = sb.get(0).unwrap();
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
                other::resume_page,
                other::error_page,
                other::about
            ],
        )
        .register(catchers![not_found])
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
                    other::resume_page,
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

// Imports specific to testing:

//use super::{rocket, TemplateContext};
/*
TEST 1:
    Test the function to etc etc
*/
macro_rules! dispatch {
    ($method:expr, $path:expr, $test_fn:expr) => {{
        let client = rocket::local::Client::new(rocket()).unwrap();
        $test_fn(&client, client.req($method, $path).dispatch());
    }};
}

/*
Test 4: Tests the error catcher
*/

#[test]
fn test_404() {
    dispatch!(
        rocket::http::Method::Get,
        "/hello/",
        |client: &rocket::local::Client, mut response: rocket::local::LocalResponse| {
            let mut map = ::std::collections::HashMap::new();
            map.insert("path", "/hello/");

            let expected = Template::show(client.rocket(), "error/404", &map).unwrap();
            assert_eq!(response.status(), rocket::http::Status::NotFound);
            assert_eq!(response.body_string(), Some(expected));
        }
    );
}
