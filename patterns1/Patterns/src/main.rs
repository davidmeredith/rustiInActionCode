mod builder;
use crate::builder::PersonBuilder; 

mod adapter;
use crate::adapter::{Dog, DogToCatAdapter, Cat};

mod decorator;
use crate::decorator::{*}; 

mod observer;
use crate::observer::{*};


fn main() {
  // Builder Demo
  let homer_result =  PersonBuilder::new()
    .name("Homer".to_string()) 
    .likes("Pie".to_string())
    .age(30)
    .build();
  let homer = match homer_result {
    Ok(p) => p,
    Err(err) => panic!("Invalid person {}", err)
  };
  println!("{} likes {} and is {}", &homer.name, &homer.likes, &homer.age);


   // Decorator demo 
   let pizza = BasePizza { cost: 5.0 };
   let pizza = ToppingDecorator {
     base_pizza: &pizza,
     topping_cost: 2.0,
   };
   let total = pizza.get_cost();
   println!("He orders a pizza-pie costing: £{}", total);


   // Adapter demo
   let dog = Dog { name: "Poppy".to_string() };
   let adapted_dog = DogToCatAdapter { dog }; 
   let cats = vec![adapted_dog];
   cats[0].roar();


   // Observer demo
    let mut subject: MySubject<MyObserver> = MySubject::new();
    let observer1 = MyObserver { id: 1, name: "Antivirus".to_string() };
    let observer2 = MyObserver { id: 2, name: "Firewall".to_string() };
    let observer3 = MyObserver { id: 3, name: "Antispyware".to_string() };
    let observer4 = MyObserver { id: 4, name: "CloudSecurity".to_string() };
    // Register the observer references to the subject
    subject.register(&observer1);
    subject.register(&observer2);
    subject.register(&observer3);
    subject.register(&observer4);
    // Notify all the observers
    println!("Calling all observers:");
    subject.notify();
    // Remove observer2 from the subject
    println!("Removing an observer:");
    subject.remove(&observer2);
    println!("Calling all observers:");
    // Notify the observers again after removing observer2
    subject.notify();

}
