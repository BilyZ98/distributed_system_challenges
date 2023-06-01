


pub trait Animal {
    fn baby_name(&self) -> String {
        String::from("baby")
    }
}


struct Dog;


struct Cat;

impl Animal for Dog {
    fn baby_name(&self) -> String {
        "Spot".to_string()
    }
}

impl Animal for Cat {
    fn baby_name(&self) -> String {
        "Kitten".to_string()
    }
}


pub fn name(ani: &impl Animal ) -> String {
    return ani.baby_name()
    // println!("Breaking news! {}", item.summarize());
}

pub fn name2<T: Animal>(ani: &T) -> String {
    return ani.baby_name()
    // println!("Breaking news! {}", item.summarize());
}
fn main() {
    let d = Dog;
    let c = Cat;
    // println!("A baby dog is called a {}", Dog::baby_name());
    // println!("A baby cat is called a {}", <Cat as Animal>::baby_name());
    println!("A baby dog is called a {}", d.baby_name());


    println!("A baby dog is called a {}", name(&d));
    println!("A baby cat is called a {}", name2(&c));



    let mut vec = vec![1,2,3];
    println!("vec = {:?}", vec);

    vec.push(4);
    println!("vec = {:?}", vec);

    let mut vec2 = vec![4,5];
    vec.append(&mut vec2);
    println!("vec = {:?}", vec);

}
