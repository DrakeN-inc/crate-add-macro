extern crate add_macro_impl_into;
use add_macro_impl_into::Into;

#[derive(Debug, PartialEq)]
struct Cat;

#[derive(Debug, PartialEq)]
struct Dog;

#[derive(Debug, PartialEq)]
struct Bird;

#[derive(Debug, PartialEq)]
struct Python;

#[derive(Debug, PartialEq, Into)]
#[into("String" = "format!(\"Animal::{self:?}\")")]
#[into("Option<Cat>" = "if let Self::Cat(v) = self { Some(v) }else{ None }")]
enum Animal {
    Cat(Cat),
    
    #[into]
    Dog(Dog),   // Option<Dog>
    
    #[into = "if let Self::Bird(value) = self { value }else{ panic!(\"It's not a dog.\") }"]
    // #[into("if let Self::Bird(value) = self { value }else{ panic!(\"It's not a dog.\") }")]
    Bird(Bird),
    
    #[into("Option<Python>" = "if let Self::Python(v) = self { Some(v) }else{ None }")]
    #[into("Python" = "Into::<Option<Python>>::into(self).expect(\"It's not a Python\")")]
    Python(Python),
}

#[test]
fn test_enum() {
    let cat_str: String = Animal::Cat(Cat {}).into();
    assert_eq!(cat_str, "Animal::Cat(Cat)");
    
    let cat: Option<Cat> = Animal::Cat(Cat {}).into();
    assert_eq!(cat, Some(Cat {}));
    
    let dog: Option<Dog> = Animal::Dog(Dog {}).into();
    assert_eq!(dog, Some(Dog {}));

    let bird: Bird = Animal::Bird(Bird {}).into();
    assert_eq!(bird, Bird {});

    let python: Option<Python> = Animal::Python(Python {}).into();
    assert_eq!(python, Some(Python {}));

    let python: Python = Animal::Python(Python {}).into();
    assert_eq!(python, Python {});
}
