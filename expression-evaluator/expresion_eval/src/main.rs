

#[derive(Debug)]
enum Optor {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Debug)]
enum Expr {

    Op {op:Optor, left: Box<Expr>, right:Box<Expr>},
    Value(i64),
}

fn eval(e:Expr)->i64{
    match e {
        Expr::Op { op, left, right }=>{
            let left=eval(*left);
            let right=eval(*right);
            match op {
                Optor::Add=>left + right,
                Optor::Sub=>left - right,
                Optor::Div => left / right,
                Optor::Mul => left * right,

            }

        },
        Expr::Value(x)=>x,
        
    }
}


#[test]
fn test_value_literal(){
    assert_eq!(60,eval(Expr::Value(60)));
}

#[test]
fn test_add(){
    assert_eq!(eval(Expr::Op { op: Optor::Add, left: Box::new(Expr::Value(20)), right: Box::new(Expr::Value(10)), }),30);
}


#[test]
fn test_sub(){
    assert_eq!(eval(Expr::Op { op: Optor::Sub, left: Box::new(Expr::Value(20)), right: Box::new(Expr::Value(10)), }),10);
}


#[test]
fn test_division(){
    assert_eq!(eval(Expr::Op { op: Optor::Div, left: Box::new(Expr::Value(100)), right: Box::new(Expr::Value(5)), }),20);
}

#[test]
fn test_multiply(){
    assert_eq!(eval(Expr::Op { op: Optor::Mul, left: Box::new(Expr::Value(12)), right: Box::new(Expr::Value(5)), }),60);
}

#[test]
fn test_zeros(){
    assert_eq!(eval(Expr::Op { op: Optor::Add, left: Box::new(Expr::Value(0)), right: Box::new(Expr::Value(0)) }),0);
    assert_eq!(eval(Expr::Op { op: Optor::Sub, left: Box::new(Expr::Value(0)), right: Box::new(Expr::Value(0)) }),0);
    assert_eq!(eval(Expr::Op { op: Optor::Mul, left: Box::new(Expr::Value(0)), right: Box::new(Expr::Value(0)) }),0);
    assert_eq!(eval(Expr::Op { op: Optor::Div, left: Box::new(Expr::Value(0)), right: Box::new(Expr::Value(7)) }),0);

}


#[test]
fn test_recursion(){
    let first=Expr::Op { 
        op: Optor::Add,
         left: Box::new(Expr::Value(2)),
          right: Box::new(Expr::Value(4)),
         };

         let second=Expr::Op { 
            op: Optor::Mul,
             left: Box::new(Expr::Op { op: Optor::Add,
                 left: Box::new(Expr::Value(3)),
                  right: Box::new(Expr::Value(1)) }),

              right: Box::new(Expr::Value(10)),
             };

             assert_eq!(eval(Expr::Op { op: Optor::Sub,
                 left: Box::new(first),
                  right: Box::new(second) }),-34);

}
fn main() {
}
