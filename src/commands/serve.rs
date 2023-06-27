use crate::commons::{exit_error, Lexer};

use rusqlite::Connection;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;
use std::str;
use tiny_http::{Header, Method, Request, Response, Server};

use super::index::Document;

fn serve_404(req: Request) {
	let page = File::open("./static/404.html").unwrap();
	let mut response = Response::from_file(page);

	response = response.with_status_code(404);
	req.respond(response).unwrap();
}

fn serve_index(req: Request) {
	let mut response = Response::from_file(File::open("./static/index.html").unwrap());
	response.add_header(
		Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap(),
	);

	req.respond(response).unwrap();
}

struct DbDocument {
	id: i32,
	path: String,
	count_words: i32,
}

struct DbDocWord {
	word: String,
	apparition: i32,
	tf: f32,
}

fn serve_file(req: Request) {
	let uri = req.url().to_string();
	let mut uri: VecDeque<&str> = uri.split("/").into_iter().skip(1).collect();

	let file: File;
	let content_type: &[u8];

	let file_type = uri.pop_front().unwrap();
	let file_type = file_type.to_lowercase();

	// TODO change routes for works on all platforms
	match file_type.as_str() {
		"css" => {
			let path = format!(
				"./static/css/{}",
				uri.into_iter().collect::<Vec<_>>().join("/")
			);

			match File::open(&path) {
				Ok(f) => {
					content_type = &b"text/css"[..];
					file = f;
				}
				Err(_) => {
					println!("Can't Open file {path}");
					return serve_404(req);
				}
			};
		}
		"js" => {
			let path = format!(
				"./static/js/{}",
				uri.into_iter().collect::<Vec<_>>().join("/")
			);

			if let Ok(f) = File::open(&path) {
				content_type = &b"application/javascript"[..];
				file = f;
			} else {
				println!("Can't Open file {path}");
				return serve_404(req);
			}
		}
		_ => return serve_404(req),
	}

	let mut res = Response::from_file(file);
	res.add_header(Header::from_bytes(&b"Content-Type"[..], content_type).unwrap());

	req.respond(res).unwrap();
}

fn handle_search(mut req: Request, db: &Connection, docs: &Vec<Document>) {
	let mut buf: Vec<u8> = Vec::new();
	req
		.as_reader()
		.read_to_end(&mut buf)
		.expect(format!("Error trying to parse body {}", req.url()).as_str());

	let body =
		str::from_utf8(&buf).expect(format!("Error trying to parse body {}", req.url()).as_str());

	// let lexer = Lexer::new(&body.chars().collect::<Vec<char>>());

	let total_documents = docs.len();
	let mut results: HashMap<&str, f32> = HashMap::new();
	let q_total_apparitions = "
        SELECT apparitions FROM T_WORD WHERE word=?1
    ";

	for doc in docs {
		for q in Lexer::new(&body.chars().collect::<Vec<char>>()) {
			let number_apparitions: i32 = db
				.query_row(q_total_apparitions, [&q], |r| r.get(0))
				.unwrap_or(0);

			if let Some(term) = doc.term_freq.get(q.as_str()) {
				let tf_idf = term.1 * (total_documents as f32 / number_apparitions as f32).log10();
				results.insert(doc.path.to_str().unwrap(), tf_idf);
			}
		}
	}

	let mut res_vec: Vec<String> = Vec::new();
	for (k, tf_idf) in results.iter() {
		let mut res = "".to_owned();
		res.push_str("{");
		res.push_str(&format!("\"doc\": \"{k}\", \"tf_idf\": \"{tf_idf}\""));
		res.push_str("}");
		res_vec.push(res);
		//println!("K: {k} => tf_idf: {tf_idf}");
	}

	let res_txt = format!("[{}]", res_vec.join(","));

	let res = Response::from_string(res_txt).with_status_code(200);
	req.respond(res).unwrap();
}

fn start_server(address: SocketAddrV4) {
	let server = Server::http(address);
	let db = Connection::open("indexed.db").unwrap();
	let mut docs: Vec<Document> = Vec::new();
	let mut stmt = db
		.prepare("SELECT id, path, total_terms FROM T_DOCUMENT")
		.unwrap();

	let rows = stmt
		.query_map([], |row| {
			Ok(DbDocument {
				id: row.get(0)?,
				path: row.get(1)?,
				count_words: row.get(2)?,
			})
		})
		.unwrap();

	for row in rows {
		let mut stmt = db
			.prepare("SELECT tw.word, tdw.apparition, tdw.tf FROM T_WORD tw, T_DOC_WORD tdw WHERE tdw.id_word = tw.id AND tdw.id_doc = ?1")
			.unwrap();

		let row = row.unwrap();

		let mut doc = Document {
			path: PathBuf::from(row.path),
			count_words: row.count_words,
			term_freq: HashMap::new(),
		};

		let rows = stmt
			.query_map([row.id], |row| {
				Ok(DbDocWord {
					word: row.get(0)?,
					apparition: row.get(1)?,
					tf: row.get(2)?,
				})
			})
			.unwrap();

		for row in rows {
			let row = row.unwrap();
			doc.term_freq.insert(row.word, (row.apparition, row.tf));
		}

		docs.push(doc);
	}

	match server {
		Ok(server) => {
			for req in server.incoming_requests() {
				println!("{}", req.url());
				match (req.method(), req.url()) {
					(Method::Get, "/") => serve_index(req),
					(Method::Get, ref r) if r.starts_with("/css/") && r.len() > 5 => serve_file(req),
					(Method::Get, ref r) if r.starts_with("/js/") && r.len() > 4 => serve_file(req),
					(Method::Post, "/search") => handle_search(req, &db, &docs),
					_ => serve_404(req),
				}
			}
		}
		Err(e) => exit_error(&format!("Can't start server: {}", e)),
	}
}

pub fn serve(mut args: VecDeque<String>) {
	let mut port: u16 = 8000;
	let mut expose: bool = false;

	while !args.is_empty() {
		if let Some(arg) = args.pop_front() {
			match arg.as_str() {
				"-p" | "--port" => {
					if let Some(custom_port) = args.pop_front() {
						match custom_port.parse::<u16>() {
							Ok(custom_port) => port = custom_port,
							Err(_) => exit_error("Port must be a number"),
						}
					} else {
						exit_error("No port number provided");
					}
				}
				"--host" => {
					expose = true;
				}
				_ => exit_error("Unknown argument"),
			}
		}
	}

	let ip_v4 = if expose {
		Ipv4Addr::new(0, 0, 0, 0)
	} else {
		Ipv4Addr::new(127, 0, 0, 1)
	};

	let address = SocketAddrV4::new(ip_v4, port);
	println!("Starting server on http://{}/", address);
	start_server(address)
}
