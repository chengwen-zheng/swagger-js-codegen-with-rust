use std::{
    fs::File,
    io::{BufReader, Read},
};

pub struct ApiStruct {
    api_version: String,
    swagger_version: String,
    base_path: String,
}

pub struct Template {
    class: String,
    method: String,
    custom_type: String,
}

pub struct Swagger {
    swagger: String,
    paths: String,
    template: Template,
}

pub fn get_code(_opts: ApiStruct) -> String {
    let code = String::from("");

    code
}

fn get_view_for_swagger2(_swagger: Swagger) -> std::io::Result<()> {
    let file = File::open("foo.txt")?;
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);
    Ok(())
}
