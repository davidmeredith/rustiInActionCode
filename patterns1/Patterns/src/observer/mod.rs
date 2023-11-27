
// Observers are referended by a Subject for notification via the update method
pub trait Observer {
    fn update(&self);
}

// 'Subject' trait has methods to register, remove, and notify its referenced Observers.
// Observers are provided as references, hence the ref lifetimes. 
// Subject does not own observers. 
pub trait Subject<'a, T: Observer> {
    fn register(&mut self, observer: &'a T);
    fn remove(&mut self, observer: &'a T);
    fn notify(&self);
    fn count_observers(&self) -> usize;
}

// 'MySubject' struct that holds a vector of Observer references
pub struct MySubject<'a, T: Observer> {
    // private by default, no need to leak the vector
    observers: Vec<&'a T>,
}
impl<'a, T: Observer> MySubject<'a, T> {
    // Note that new() returns a concrete MySubject with bounded type T   
    pub fn new() -> MySubject<'a, T> {
        MySubject {
            // start with an empty vector
            observers: Vec::new(),
        }
    }
}
// Impl Subject for MySubject, 
// We also specify two additional trait bounds for 'T' (Observer + PartialEq) 
// to conditionally implement Subject's methods.
impl<'a, T: Observer + PartialEq> Subject<'a, T> for MySubject<'a, T> {
    fn register(&mut self, observer: &'a T) {
        self.observers.push(observer);
    }
    fn remove(&mut self, observer: &'a T) {
        let index = self.observers.iter().position(|x| *x == observer).unwrap();
        self.observers.remove(index);
    }
    fn notify(&self) {
        for observer in self.observers.iter() {
            observer.update();
        }
    }
    fn count_observers(&self) -> usize {
       self.observers.len()
    }
}

// MyObserver must derive PartialEq to enable 'value equality' which compares 
// all MyObserver instance attributes to determine equivalence, unlike identity equality.
#[derive(PartialEq)]
pub struct MyObserver {
    pub id: u32,
    pub name: String,
}
impl Observer for MyObserver {
    fn update(&self) {
        println!("  **  Observer {}: id = {}  **", self.name, self.id);
    }
}

#[test]
fn test_observers() {

    let mut subject: MySubject<MyObserver> = MySubject::new();

    let observer1 = MyObserver { id: 1, name: "Antivirus".to_string() };
    let observer2 = MyObserver { id: 2, name: "Firewall".to_string() };
    let observer3 = MyObserver { id: 3, name: "Antispyware".to_string() };
    let observer4 = MyObserver { id: 4, name: "CloudSecurity".to_string() };

    // Register the observers to the subject
    subject.register(&observer1);
    subject.register(&observer2);
    subject.register(&observer3);
    subject.register(&observer4);
    assert_eq!(subject.count_observers(), 4);
    // Notify all the observers
    subject.notify();

    // Remove observer2 from the subject
    subject.remove(&observer2);
    assert_eq!(subject.count_observers(), 3);

    // Notify the observers again after removing observer2
    subject.notify();
}
