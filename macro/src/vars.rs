use proc_macro2::TokenStream;
use quote::quote;

fn rustc() -> Option<String> {
    let meta = rustc_version::version_meta().ok()?;

    Some(format!(
        "{} ({} {})",
        meta.semver,
        meta.commit_hash
            .as_deref()
            .and_then(|s| s.get(..9))
            .unwrap_or("<unknown>"),
        meta.commit_date.as_deref().unwrap_or("<unknown>")
    ))
}

fn last_commit() -> Option<String> {
    let repo = git2::Repository::open_from_env().ok()?;
    let head = repo
        .branches(None)
        .ok()?
        .filter_map(|b| b.ok())
        .find(|b| b.0.is_head())?;
    let name = head.0.name().ok()??;
    let object = repo.revparse_single(&name).ok()?;
    let commit = object.peel_to_commit().ok()?;

    let dirty = if repo
        .diff_index_to_workdir(None, None)
        .ok()?
        .deltas()
        .count()
        != 0
    {
        "-dirty"
    } else {
        ""
    };

    Some(format!("{}{}", commit.id(), dirty))
}

fn lock() -> Option<String> {
    let meta = cargo_metadata::MetadataCommand::new().exec().ok()?;

    let mut p = meta.workspace_root;
    p.push("Cargo.lock");

    Some(String::from_utf8_lossy(&std::fs::read(p).ok()?).to_string())
}

pub fn package_name() -> TokenStream {
    quote! {
        option_env!("CARGO_PKG_NAME").unwrap_or("<unknown_package>")
    }
}

pub fn package_version() -> TokenStream {
    quote! {
        option_env!("CARGO_PKG_VERSION").unwrap_or("<unknown_package>")
    }
}

pub fn package_description() -> TokenStream {
    quote! {
        option_env!("CARGO_PKG_DESCRIPTION").unwrap_or("")
    }
}

pub fn build_date() -> TokenStream {
    let date = chrono::Local::now().format("%c %z").to_string();
    quote! { #date }
}

pub fn build_mode() -> TokenStream {
    #[cfg(debug_assertions)]
    let mode = "debug";
    #[cfg(not(debug_assertions))]
    let mode = "release";
    quote! { #mode }
}

pub fn build_commit_hash() -> TokenStream {
    let hash = last_commit().unwrap_or_else(|| "<unknown>".into());
    quote! { #hash }
}

pub fn build_username() -> TokenStream {
    let user = whoami::username();
    quote! { #user }
}

pub fn build_hostname() -> TokenStream {
    let host = whoami::hostname();
    quote! { #host }
}

pub fn build_dir() -> TokenStream {
    let dir = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "<unknown>".into());
    quote! { #dir }
}

pub fn rustc_version() -> TokenStream {
    let rustc = rustc().unwrap_or_else(|| "<unknown>".into());
    quote! { #rustc }
}

pub fn lock_file() -> TokenStream {
    let lock = lock().unwrap_or_else(|| "<unknown>".into());
    quote! { #lock }
}
