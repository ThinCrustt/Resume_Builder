#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use crate::handlebars::{
    Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext,
};
use rocket::request::Form;
use rocket_contrib::templates::handlebars;

pub struct Resume {
    first_name: String,
    last_name: String,
    phone_number: String,
    email: String,
    linkedin: String,
    city: String,
    state: String,
    zip_code: String,
    job_title: String,
    employer: String,
    start_date: String,
    end_date: String,
    description: String,
    school: String,
    gpa: String,
    field_of_study: String,
    graduation_date: String,
    languages: String,
    softwares: String,
    other_tools: String,
}
#[derive(FromForm)]
pub struct Task {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub email: String,
    pub linkedin: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub job_title: String,
    pub employer: String,
    pub start_date: String,
    pub end_date: String,
    pub description: String,
    pub school: String,
    pub gpa: String,
    pub field_of_study: String,
    pub graduation_date: String,
    pub languages: String,
    pub softwares: String,
    pub other_tools: String,
}

pub fn validate_user_input(task: Form<Task>) -> bool {
    let _first_name = task.first_name.clone();
    let _last_name = task.last_name.clone();
    let _phone_number = task.phone_number.clone();
    let _email = task.email.clone();
    let _linkedin = task.linkedin.clone();
    let _city = task.city.clone();
    let _state = task.state.clone();
    let _zip_code = task.zip_code.clone();
    let _job_title = task.job_title.clone();
    let _employer = task.employer.clone();
    let _start_date = task.start_date.clone();
    let _end_date = task.end_date.clone();
    let _description = task.description.clone();
    let _school = task.school.clone();
    let _gpa = task.gpa.clone();
    let _field_of_study = task.field_of_study.clone();
    let _graduation_date = task.graduation_date.clone();
    let _languages = task.languages.clone();
    let _softwares = task.softwares.clone();
    let _other_tools = task.other_tools.clone();

    _first_name != "First Name"
        && !_first_name.is_empty()
        && _last_name != "Last Name"
        && !_phone_number.is_empty()
        && _email != "email"
        && _email != "something@example.com"
        && !_email.is_empty()
        && _linkedin != "Linked In"
        && _city != "Example:San Francisco"
        && !_city.is_empty()
        && _state != "Example:CA"
        && !_state.is_empty()
        && _zip_code.parse::<i32>().unwrap() > 10000
        && !_zip_code.is_empty()
        && _job_title != "Example:Software Engineer"
        && _employer != "Example:Intel"
        && _description != "Tell us about your responsibilities"
        && _school != "Example:University of California"
        && !_school.is_empty()
        && _gpa.parse::<f32>().unwrap() < 4.1
        && !_gpa.is_empty()
        && _field_of_study != "Exampel:Bachelor of Technology"
        && !_field_of_study.is_empty()
        && !_graduation_date.is_empty()
        && _languages != "Example:C#, Python"
        && _softwares != "Examples:Unity"
}
pub fn wow_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
    }

    Ok(())
}
