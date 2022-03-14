# Rust_Final_Project: _Resume Builder_

Final project for Programming in Rust

Authors:
Harmandeep Singh
Rajat Kulkarni


## Description

The Resume Builder web application allows users to create a personalized resume. The user can enter in their personal information, experience, education and skills, after which a resume containing that information will be generated.

The project uses the Rocket framework, which is a web framework for Rust. It also utilizes templating through Handlebars, as well as a lot of CSS to style the site. 

## How to use _Resume Builder_

Upon successful launch, the web app will display the Home page. On the toolbar at the top of the page, the user will be able to navigate to the "Dashboard" and "About" page. Clicking on "Getting Started" will take the user to resume building page; after successful completion of all the required fields, a new resume will be generated.

## Issues

In our project proposal, we outlined that if possible, we would like to save every resume that was created through either Rocket State or Diesel, and maintain that state so that anytime the user navigates to "localhost:[port_number]/dashboard", the entire list of all the created resumes would be displayed. However, main issue we encountered was getting Diesel set up on our machines, which proved to be extremely buggy and slow. An attempt was made at utilizing Rocket State, and is not removed from the main.rs file (it is placed in comments). Regardless of this, we were able to get most of the features implemented.


## Control Flow

 give a general overview of how the project is laid out

 In _main.rs_, the Rocket serve
