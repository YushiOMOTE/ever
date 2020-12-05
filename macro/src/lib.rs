#![recursion_limit = "128"]

extern crate proc_macro;

mod vars;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro_hack]
pub fn ever(input: TokenStream) -> TokenStream {
    let version_env = if input.is_empty() {
        LitStr::new("EVER", Span::call_site())
    } else {
        parse_macro_input!(input as LitStr)
    };

    let name = vars::package_name();
    let version = vars::package_version();
    let about = vars::package_description();
    let date = vars::build_date();
    let commit = vars::build_commit_hash();
    let user = vars::build_username();
    let host = vars::build_hostname();
    let builddir = vars::build_dir();
    let rustc = vars::rustc_version();
    let mode = vars::build_mode();
    let lock = vars::lock_file();

    let s = quote! {
        match std::env::var(#version_env).as_deref() {
            Ok("1") | Ok("true") => {

                println!(
                r#"{name} {version} ({mode}): {about}

    date:     {date}
    commit:   {commit}
    user:     {user}
    host:     {host}
    builddir: {builddir}
    rustc:    {rustc}"#,
                    name = #name,
                    version = #version,
                    about = #about,
                    date = #date,
                    commit = #commit,
                    user = #user,
                    host = #host,
                    builddir = #builddir,
                    rustc = #rustc,
                    mode = #mode,
                );

                std::process::exit(1);
            }
            Ok("dump_lock") => {
                println!("{}", #lock);
                std::process::exit(1);
            }
            _ => {}
        }
    };

    s.into()
}

#[proc_macro_hack]
pub fn package_name(_: TokenStream) -> TokenStream {
    vars::package_name().into()
}

#[proc_macro_hack]
pub fn package_version(_: TokenStream) -> TokenStream {
    vars::package_version().into()
}

#[proc_macro_hack]
pub fn package_description(_: TokenStream) -> TokenStream {
    vars::package_description().into()
}

#[proc_macro_hack]
pub fn build_date(_: TokenStream) -> TokenStream {
    vars::build_date().into()
}

#[proc_macro_hack]
pub fn build_mode(_: TokenStream) -> TokenStream {
    vars::build_mode().into()
}

#[proc_macro_hack]
pub fn build_commit_hash(_: TokenStream) -> TokenStream {
    vars::build_commit_hash().into()
}

#[proc_macro_hack]
pub fn build_username(_: TokenStream) -> TokenStream {
    vars::build_username().into()
}

#[proc_macro_hack]
pub fn build_hostname(_: TokenStream) -> TokenStream {
    vars::build_hostname().into()
}

#[proc_macro_hack]
pub fn build_dir(_: TokenStream) -> TokenStream {
    vars::build_dir().into()
}

#[proc_macro_hack]
pub fn rustc_version(_: TokenStream) -> TokenStream {
    vars::rustc_version().into()
}

#[proc_macro_hack]
pub fn lock_file(_: TokenStream) -> TokenStream {
    vars::lock_file().into()
}
