# What
This allows people to create a csv in their favorite editor, drag it into the app, and submit each row separately
to an API endpoint. It would be trivial to modify to send all relevant columns in a single request. It offers 
fields that can be grabbed via a GET request to an API endpoint for normalizing data which is particularly useful 
for columns where joins or unions take place and typos cannot happen. 

Also included is validator to ensure proper standardization and sanitization. Albiet it is still in elementary form here.

It would be trivial to modify this to add PUT or DELETE functionality to be a fully featured CRUD app. It
wasn't necessary for the organization I was working with so it isn't included. However, the reqwest crate used natively
supports this functionality.


# Why

You need a simple solution for nontechical persons in your org to send bulk amounts of data to an API and even 
walkthroughs of Postman have proven too difficult for other team members to grasp.

Additionally, this was a project I undertook to better understand Rust. This will not be the same quality as emilk's 
repo which this is built upon. He has spent a lot of time to create tons of tools that can lead
to rapid prototyping. Definitely reference his work when making this your own. 

Even though this was a learning project it was also forked to a production for my company. It is safe to use
out of the box at your organization. It has been used in production environments numerous times without any incidents.



# How

[eframe](https://github.com/emilk/egui/tree/master/crates/eframe)

The official egui docs are at <https://docs.rs/egui>

Web deploy has not been used and has been removed as the intended use was always as a Desktop app. Eframe & EGUI has
support for web deployment.  


# Customizing This for Your Org

Change the name of the crate: Chose a good name for your project, and change the name to it in:
* `Cargo.toml`
  * Change the `package.name` from `eframe_egui_CRUD` to `your_crate`.
  * Change the `package.authors`
* `main.rs`
  * Change `eframe_egui_CRUD::TemplateApp` to `your_crate::TemplateApp`
  * Change your api endpoints. Currently they are set to `https://sdfsfsdfsd.free.beeceptor.com/todos`
