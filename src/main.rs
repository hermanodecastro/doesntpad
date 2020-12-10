#![feature(proc_macro_hygiene, decl_macro)]

mod database;
mod models;

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate rusqlite;

use rocket::{response::Redirect, request::{Form, FromForm}};
use rocket_contrib::{json::Json, templates::Template, serve::StaticFiles};
use rusqlite::params;
use database::{Database, Result};
use models::Notepad;

#[derive(FromForm)]
struct SlugRequest {
	slug: String
}

#[get("/")]
fn index() -> Template {
	Template::render("index", {})
}

#[post("/", data = "<request>")]
fn create(request: Form<SlugRequest>) -> Redirect {
	Redirect::to(format!("/{}", request.slug))
}

#[get("/<slug>")]
fn notepad(slug: String) -> Result<Template> {
	let mut db = Database::new()?;
	let notepad_already_exist = db.exists("SELECT * FROM notepads WHERE slug = ?1", params![slug])?;
	if notepad_already_exist {
		let notepad = db.select("SELECT * FROM notepads WHERE slug = ?1", params![slug])?;
		db.close().unwrap();
		Ok(Template::render("notepad", notepad))
	} else {
		let notepad = Notepad {
			id: None,
			slug,
			content: String::from(" ")
		};
		db.execute("INSERT INTO notepads (slug, content) VALUES (?1, ?2)", params![notepad.slug, notepad.content])?;
		db.close().unwrap();
		Ok(Template::render("notepad", notepad))
	}
}

#[put("/<id>", format = "application/json", data = "<request>")]
fn update(id: i32, request: Json<Notepad>) -> Result<()> {
	let mut db = Database::new()?;
	db.execute("UPDATE notepads set content = ?1 WHERE id = ?2", params![request.content, id])?;
	db.close().unwrap();
	Ok(())
}

fn main() {
    rocket::ignite()
    	.mount("/static", StaticFiles::from("static"))
    	.mount("/", routes![index])
    	.mount("/", routes![create])
    	.mount("/", routes![notepad])
    	.mount("/", routes![update])
    	.attach(Template::fairing())
    	.launch();
}
