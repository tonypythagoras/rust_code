
/*This simulate health monitoring system. 
Keeps track of patient's health statistics as he visits doctor 
*/

#![allow(dead_code)]
pub struct User{
    name:String,
    age: u32,
    height: f64,
    num_visit: u32,
    last_bp:Option<(u32,u32)>
}

pub struct Measurements{
    height: f64,
    bp:(u32,u32)
}
pub struct Report<'a>{
    name: &'a str,
    num_visit:u32,
    height_change:f64,
    bp_change:Option<(i32,i32)>,
}

impl User {
    pub fn new(name:String, age:u32,height:f64)->Self{
        Self { name, age, height, num_visit: 0, last_bp: None }
    }

    pub fn visit_doctor(&mut self, measurements:Measurements)->Report<'_>{

        self.num_visit +=1;
        let bp=measurements.bp;
        let report=Report{
            name:&self.name,
            num_visit:self.num_visit,
            height_change: measurements.height-self.height,
            bp_change: self.last_bp.map(|lbp| (bp.0 as i32 - lbp.0 as i32, bp.1 as i32 - lbp.1 as i32)),
        };
        self.height=measurements.height;
        self.last_bp=Some(bp);
        report

    }
    
}

#[test]
fn test_patient(){
    let  patient=User::new(String::from("Patrick"), 30,5.7);
    assert_eq!(patient.last_bp,None);
    assert_eq!(patient.num_visit,0);
    assert_eq!(patient.age,30);
    assert_eq!(patient.height,5.7);

}
#[test]
fn test_visit(){
 let  mut patient=User::new(String::from("Patrick"), 30,5.7);
 let report=patient.visit_doctor(Measurements { height: 5.7, bp: (200,180) });
assert_eq!(report.num_visit,1);
assert_eq!(report.name,"Patrick");
assert_eq!(report.height_change,0.0);
assert_eq!(report.bp_change,None);
 let report=patient.visit_doctor(Measurements { height: 5.7, bp: (195,181) });
assert_eq!(report.num_visit,2);
assert_eq!(report.height_change,0.0);
assert_eq!(report.bp_change,Some((-5,1)));
    

}


fn main() {
    println!("Hello, world!");
}
