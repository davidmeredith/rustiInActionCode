
# Dave's Rust Notes

* Panic==crash
* No null
* binary crate vs library crate (defined in Cargo.toml)
* 'cargo update' ignores the Cargo.lock file on updating
* 'cargo doc --open' to create local doc from proj+deps

* Rust has Generics, e.g. `std::collections::HashMap<String, String>`
* Rust has `Result<T, E>` akin to Optionals. Result is an ENUM type that wraps two variants Ok(T) or Err(E) (aka 'discriminated union')

* Special type called 'Unit' == void
* `std::collections` has many good stuff
* Create imports with 'use' 
* Can't have unused vars
* Forced to handle errors using e.g., `Result<res, error>` (note, can use `.unwrap()` which panics as quick hack to write quick code rather than dealing with the errors). 
* Immutable by default, use `let` to create a immutable var, use `mut` to make mutable
* `::` indicates an 'associated function' implemented on a type, e.g. String::new (akin to a static method)

* Rust allows variable shadowing i.e. redefined let vars shadow vars of the same name, you can also change the type with each re-definition (in contrast, you aren't allowed to mutate a variables type).
* `const` is used for constants, can only be set to a constant expression, not the result of a function call or runtime computation.

* Compound types: 
  * Tuple: holds fixed size multiple types in same tuple. You can _destruct_ a tuple to get at individual elements, and also use indexing (zero offset)

```rust    
    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple
    let (x, y, z) = tup; // destructuring
    println!("The value of y is: {}", y);
    let a: [i32; 5] = [1, 2, 3, 4, 5]; //Array: fixed size single type 
```

* Pattern-match expression (akin to pattern-match for switch in java which returns a value for binding). It matches Result types e.g. Result::Ok or Result:Err. Is an expression, not a statement, so you can bind the result of the match to a var e.g.
 

```rust
fn main () {
    // ...
    // expect causes panic-crash if Database is not returned. 
    let database: Database::myNewDb().expect("Database::new() crashed");
    // expect() v.similar to unwrap()
}

// structs allocated on the stack
struct Database {
    map: Hashmap<String, String>,
}

// Implement our Database to add methods
impl Database {
    // fn returns a Result which is a discrimiated union type that 
    // uses enums under the hood. 
    fn myNewDb() -> Result<Database, std::io::Error> {
        let contents = match std::fs::read_to_string("kv.db") {
            Ok(c) => c,   // if pattern match ==j Ok, 
                          // return c and bind to contents
            Err(error) => {
                return Result::Err(error); // can just use Err(error);
            }
        };
        // Above is so common, can use ? shorthand 
        // ? indicates error will be returned if err occurs e.g.: 
        // let contents = std::fs::read_to_string("kv.db")?;

    lset mut map = HashMap::new();

    for line in contents.lines() { // lines returns an iterator of string slices (&str)
        //let pair = line.split_once('t').expect("Corrupt DB");
        let mut chunks = line.splitn(2, 't');
        let key = chunks.next().expect("No key");
        let value = chunks.next().expect("No Value");
        map.insert(key, value);

        // can deconstruct but not yet stable at time of video:
        //let (key.to_owned(), value.to_owned()) = line.split_once('t').expect("Corrupt DB");
    }

    Ok(Database { map: map })  // statement, no return key needed
}
```

* Need to use 'mut' keyword even when iterating  
* fn's can't take var-args, macros can !
* In function signatures, you must declare the type of each parameter
* Statements perform some actions and do not return a value. Expressions evaluate 'express' a value. Rust has different behaviour from C and other languages w.r.t. assignment expressions: In C you can write `x = y = 6`, can't in Rust, e.g. `let x = (let x = 6);` will panic, the let=6 does not return a value !
  * Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value. Keep this in mind for function return values and expressions.

```
fn four() -> i32 {
    let x = 3;
    x + 1   // if we put ; on the end it becomes a statement which does not 
            // return a value, and code would panic "mismatched types" 
            // because a return type is specified
}
```
### Control flow

* Because 'if' is an expression, we can use it on the right side of a let statement
* use 'break' with a value to return a value, e.g. `break counter;` returns the loop counter
* `for ... in` is the most reliable loop in Rust 

* structs + primitives (e.g. u8 int takes up one byte, ie 8 bits) 
* create structs on the stack using 'new' keyword (this does not mean the struct is allocated on the heap, structs live on stack)
* Can add 'impl' to the struct for adding methods
* Has visibility with 'pub' keyword (members are module-private by default)
* Has no notion of constructors, just convention of using 'new' as fn name
* Tuples allow for different types, arrays don't 

## Pointer/Ref Ownership

### Moving vs Cloning 

* When a var goes out of scope, its _drop_ function is automatically called to 'drop' the var from the current scope.
* _Double free_ errors are handled by Rust by _moves_:

```rust
    let s1 = String::from("hello");
    let s2 = s1;                // s1 moved into s2
    println!("{}, world!", s1); // panic 'borrow of moved value s1' as the
                                // s1 rference is invalidated - it was moved into s2.
```

* Move: The concept of copying the reference (pointer address, length, and capacity) without copying the heap data is like making a shallow copy, but because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a _move_. 
* Moving / transferring ownership of a pointer/ref occurs when:
  * putting a value into a struct
  * returning a value from a function   
  * passing args to function depending on whether the arg implements `Copy` - it will either be moved or copied.  

* Clone: creates a newly owned deep copy, so below is fine: 

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1, s2); // fine
```

* `fn to_owned(&self) -> Self::Owned` Creates new owned data (e.g. a new String) from borrowed data, usually by cloning. 

### Copyable Stack Types aren't moved 

* Stack Types (primatives, structs of primitives) are fine to copy, so below is fine, we don't need to clone since x & y are independent (i.e. is no difference between a deep & shallow copy for stack-types): 

```rust
let x = 5;
let y = x;
println!("x = {}, y = {}", x, y); // fine
```

* Copyable stack types are those that implement the `Copy` trait and include any simple scalar value e.g. all ints, floats, bool, chars, tuples provided they only contain types that implement `Copy` (so a tuple that contains a String isn't copyable).  
* Anything that requires allocation or any form of Resource can't implement Copy.

### References and functions

* Only positional args in Rust, no keyword args.
* Passing values to functions is similar to variable assignment - a variable will either be _moved_ or _copied_ into the fn depending on whether it implements `Copy`. 

* Moving ownership into a function:

```rust
fn main() {
    let s1 = gives_ownership();         // moves its return value into s1
    let s2 = String::from("hello");     // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 is moved into function. Note, 
                                        // no '&' on arg so passing by val, not borrowing). 
                                        // s2 moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.
```

## &Borrowing

* Borrowing: We call passing references as function parameters _borrowing_. 
* Borrowing References allow you to refer to some value without transferring ownership. 

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // pass &Ref without moving ownership 
                                     // into fn, so we can use below just fine
    println!("The length of '{}' is {}.", s1, len); // is ok
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

Pointer memory of above look like this:

```
  [s]        [s1]          [heap] 
key|val     key|val      index|value
-------------------------------------
ptr| -----> ptr |  ----> 0    | h
            len |5       1    | e
            cap |        2    | l
                         3    | l
                         4    | o
```

## No multiple muts or combining mut & immut in the same scope

* You can't have _multiple_ mutable references to a particular piece of mem/var in the same scope - prevents race conditions (and no synchronisation needed):

```rust
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;    // compile error 'cannot borrow `s` as mutable more than once at a time'
    println!("{}, {}", r1, r2);
```

* This code is ok due to the embedded scope:

```rust
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
```

* Can't _combine_ mut and immutable refs on a var in the same scope (but multple immutable refs are ok):

```rust
 let mut s = String::from("hello");
 let r1 = &s; // immutable ref no problem
 let r2 = &s; // immutable ref no problem
 let r3 = &mut s; 
 println!("{}, {}, and {}", r1, r2, r3); // error: 'cannot borrow `s` as mutable because it is also borrowed as immutable'
```

## Last used position of Ref defines scope boundary 

* A Ref's scope continues only to the place where it was last used. This is ok: 

```rust
   let mut s = String::from("hello");
   let r1 = &s; // no problem
   let r2 = &s; // no problem
   println!("{} and {}", r1, r2);
   // Refs r1 and r2 are no longer used after this point

   let r3 = &mut s; // no problem
   println!("{}", r3);
```

* Dangling Ref's aren't allowed, can no longer have a reference to an owned 
value/memory if that owned value has been dropped, dangling pointers are 
impossible in Rust, below causes error: 

```rust
fn main() {
    let reference_to_nothing = dangle();
}
fn dangle() -> &String {
    let s = String::from("hello");
    &s 
} // &s is created locally & then goes out of scope, so there's nothing to return. 
  // error: 'this function's return type contains a borrowed value, but there is no value
  // for it to be borrowed from.' 
  // You could return the string directly (remove &), that's ok because the 
  // string will be _moved_ out of the fn scope 
```

Pass by reference => &pointerRef (borrowing)
Pass by value => pass in value and transfer ownership to new context, can be used in API design to define end method calls. 

When specifying method args, using the following order helps (try 1 first): 
1) '&ref' (immutable borrow/pass-by-ref)
2) '&mut ref' (mutable borrow/pass-by-ref)
3) pass by value, ie transfer ownership of 'Copy' type eg straight 'String'

