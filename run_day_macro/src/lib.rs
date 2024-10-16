use proc_macro::TokenStream;
use quote::quote;
use syn::{ExprArray, Expr, ExprMatch};
use std::{fs, path::Path};

#[proc_macro]
pub fn run_day(macro_input: TokenStream) -> TokenStream {
    let ast = syn::parse::<ExprArray>(macro_input)/*.or_else(syn::parse2::<Ident>)*/.unwrap();
    let _year = ast.elems.first().expect("Year/day required");//.into();
    let _day = ast.elems.last().expect("Year/day required");//.into();
    let year = "2023"; // TODO try to not hardcode it

    let mut code = "match day.as_str() {".to_string();
    let dir_path_str = format!("./src/{}", year);
    let dir_path = Path::new(&dir_path_str);
    let days_folder = fs::read_dir(dir_path).expect("Year folder not found");
    for df in days_folder {
        match df {
            Ok(df) => { 
                let fname = df.file_name().to_str().unwrap().to_string();
                let fname = fname.split('.').collect::<Vec<_>>()[0].to_string();
                let fname = match fname.strip_prefix('_') {
                    Some(name) => name,
                    None => fname.as_str()
                };
                code += format!("\"{0}\" => {{#[path=\"{year}/{0}.rs\"]mod _{0};use _{0}::run; return run(input)}},", fname).as_str()
            },
            Err(_) => continue
        }
    }
    code += "_ => Err(\"Day not found or doesn't implement the run function\".to_string())}";
    let parsed_code = syn::parse_str::<ExprMatch>(code.as_str());//.expect("Error generating macro code");
    let parsed_code = match parsed_code {
        Ok(o) => o.into(),
        Err(e) => {println!("ParsingError: {e}"); syn::parse_str::<Expr>("{\"Error parsing generated macro code\"}").unwrap()}
    };
    quote! { #parsed_code }.into()
}
