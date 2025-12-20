
use std::cmp::Ordering;


//This is a generic method that returns the lesser of two values
//Leveraging the Ordering trait
fn min<T:Ord>(left:T, right:T)->T{
    match left.cmp(&right) {
        Ordering::Less |Ordering::Equal => left,
        Ordering::Greater => right,
        
    }
}

//testing numbers
#[test]
fn test_integer() {
    assert_eq!(min(2,6),2);
    assert_eq!(min(69, 400),69);
    
}

// testing characters
#[test]
fn test_characters(){
    assert_eq!(min('a','c'),'a');
    assert_eq!(min('Z', 'H'),'H');
}


// testing strings
#[test]
fn test_strings(){
    assert_eq!(min("left", "alright"),"alright");
    assert_eq!(min("NIG", "USA"),"NIG");
}

fn main() {
 let country1="USA";
 let country2="NIGERIA";
 let lesser=min(country1, country2);

 println!("The lesser is {}", lesser);

}
