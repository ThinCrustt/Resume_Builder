# Rust_Final_Project: _Resume Builder_

Note: there is an "unused" Cargo Clippy warning on line 360 that is not going away. It is related to one of our testing functions. I think it may be a bug; we got everything else passed for Cargo Clippy and Cargo fmt.


Final project for Programming in Rust

Authors:
Harmandeep Singh
Rajat Kulkarni


## Description

BUILT WITH cargo 1.61.0-nightly (65c826642 2022-03-09)

The Resume Builder web application allows users to create a personalized resume. The user can enter in their personal information, experience, education and skills, after which a resume containing that information will be generated.

The project uses the Rocket framework, which is a web framework for Rust. It also utilizes templating through Handlebars, as well as a lot of CSS to style the site. 

## How to use _Resume Builder_

Upon successful launch, the web app will display the Home page. On the toolbar at the top of the page, the user will be able to navigate to the "Dashboard" and "About" page. Clicking on "Getting Started" will take the user to resume building page; after successful completion of all the required fields, a new resume will be generated.

## How it works

_main.rs_ and _lib.rs_ act as the control centers. The Rocket app is launched here, and all of the routing is handled here depending on where the user navigates in the app. If the user submits any forms, the POST request generated is also handled here. The functions that handle routing and requests make use of Handlebar templates, passing in the specific context structs necessary to render that page. There are also Handlebars helpers that were defined and used in Rust.

## Issues

In our project proposal, we outlined that if possible, we would like to save every resume that was created through either Rocket State or Diesel, and maintain that state so that anytime the user navigates to "localhost:[port_number]/dashboard", the entire list of all the created resumes would be displayed. However, main issue we encountered was getting Diesel set up on our machines, which proved to be extremely buggy and slow. An attempt was made at utilizing Rocket State, and is not removed from the main.rs file (it is placed in comments). Regardless of this, we were able to get most of the features that we wanted implemented.

## What Worked/ Lessons Learned

The majority of the project was spent learning and working with Rocket. This was a challenge, because it was something neither group member had seen before, and the concept of web development in Rust was new. However, at the end, we both feel like we have a solid command of Rocket, which will be useful for developing web applications in the future. 

## What each of us worked on

Harmandeep Singh: _lib.rs_, all _.hbs_ files, all _.css_ files
Rajat Kulkarni: _main.rs_, started on _.hbs_ files


