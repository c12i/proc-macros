use builder::Builder;

#[derive(Builder, Debug)]
pub struct User {
    id: u32,
    name: String,
    is_admin: bool,
}

#[test]
fn builder_works() {
    let user_a = User::builder()
        .id(1)
        .name("Foo Baz".to_string())
        .is_admin(true)
        .build()
        .unwrap();
    assert_eq!(user_a.id, 1);
    assert_eq!(user_a.name, "Foo Baz".to_string());
    assert!(user_a.is_admin);
}
