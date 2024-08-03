extern crate add_macro_impl_display;
use add_macro_impl_display::Display;

#[derive(Display)]
enum Animals {
    Turtle,
    
    #[display]
    Bird,
    
    #[display = "The Cat"]
    Cat,
    
    #[display("The Dog")]
    Dog,

    Other(&'static str),

    #[display]
    Other2(&'static str),
    
    #[display("{0} {1}, {2} years old")]
    Info(Box<Self>, &'static str, i32),

    #[display("{kind} {name}, {age} years old")] 
    Info2 {
        kind: Box<Self>,
        name: &'static str,
        age: i32,
    },
}

#[test]
fn test_enum() {
    assert_eq!(format!("{}", Animals::Turtle), "Turtle");
    assert_eq!(format!("{}", Animals::Bird), "Bird");
    assert_eq!(format!("{}", Animals::Cat), "The Cat");
    assert_eq!(format!("{}", Animals::Dog), "The Dog");
    assert_eq!(format!("{}", Animals::Other("Tiger")), "Tiger");
    assert_eq!(format!("{}", Animals::Other2("Tiger")), "Tiger");
    assert_eq!(format!("{}", Animals::Info(Box::new(Animals::Cat), "Tomas", 2)), "The Cat Tomas, 2 years old");
    assert_eq!(format!("{}", 
        Animals::Info2 {
            kind: Box::new(Animals::Cat),
            name: "Tomas",
            age: 2,
        }),
        "The Cat Tomas, 2 years old"
    );
}
