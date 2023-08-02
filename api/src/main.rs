mod vm;
use crate::vm::{VM,Language};
use tide::Request;
use tide::prelude::*;
use http_types::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};


#[derive( Deserialize)]
struct Payload {
    language: String,
    code: String,
    input: String,
}


#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    let cors = CorsMiddleware::new()
    .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
    .allow_origin(Origin::from("*"))
    .allow_credentials(false);

    app.with(cors);

    app.at("/api/code/compile").post(compile_code);
    app.at("/api/session/getquestion").get(session_get_questions);
    app.listen("127.0.0.1:6070").await?;
    Ok(())
}

async fn compile_code(mut req: Request<()>) -> tide::Result {
    println!("compile called");
    let Payload { language, code, input} = req.body_json().await?;
    //println!("code: {}\nlang: {}\ninput: {}", code, language, input);
    let virtual_machine: VM = VM::new(&language, &code, &input);
    let res = virtual_machine.execute();
    //println!("{}", res.1);
    Ok(format!("{}", res.1).into())
}


async fn session_get_questions(mut req: Request<()>) -> tide::Result {
    Ok(format!("").into())
}
