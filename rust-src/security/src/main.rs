use security::bcrypt::hash_context;

fn main() {
    let hash = hash_context("admin".to_string());
    println!("{:?}", hash)
}