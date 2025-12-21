

/*
Evaluate arithmetical expression with error handling; like division by zero.

*/
//The arithmetical operations we support represented as enum
// With debug formating {:?}
#[derive(Debug)]
enum Optor {
    Add,
    Sub,
    Mul,
    Div,
}

//This enum represents expression. And expression is recursive
//The expression has left and right separated by an Operator
// The simplest expression is a value which is immediately retrurned
#[derive(Debug)]
enum Expr {

    Op {op:Optor, left: Box<Expr>, right:Box<Expr>},
    Value(i64),
}

#[derive(PartialEq, Eq, Debug)]
struct DivideByZeroError;

//expression evaluator
//It can go left or right
//returns on encountering a value
fn eval(e:Expr)->Result<i64, DivideByZeroError> {
    match e {
        Expr::Op { op, left, right }=>{
            let left=eval(*left)?;
            let right=eval(*right)?;
            Ok(match op {
                Optor::Add=>left + right,
                Optor::Sub=>left - right,
                Optor::Div => if right !=0{left / right}else{

                    return Err(DivideByZeroError);
                },
                Optor::Mul => left * right,

            })

        },
        Expr::Value(x)=>Ok(x),
    }
        
    }


//Test the error condition. ie division by zero
#[test]
fn test_error() {
assert_eq!(
eval(Expr::Op {op: Optor::Div,left: Box::new(Expr::Value(99)),right: Box::new(Expr::Value(0)),}),Err(DivideByZeroError));
}

//Test okay condition. Normal arithmetics
#[test]
fn test_ok() {
let expr = Expr::Op {op: Optor::Sub,left: Box::new(Expr::Value(20)),right: Box::new(Expr::Value(10)),};
assert_eq!(eval(expr), Ok(10));
}



#[test]
fn test_value_literal(){
    assert_eq!(Ok(60),eval(Expr::Value(60)));
}

#[test]
fn test_add(){
    assert_eq!(eval(Expr::Op { op: Optor::Add, left: Box::new(Expr::Value(20)), right: Box::new(Expr::Value(10)), }),Ok(30));
}


#[test]
fn test_sub(){
    assert_eq!(eval(Expr::Op { op: Optor::Sub, left: Box::new(Expr::Value(20)), right: Box::new(Expr::Value(10)), }),Ok(10));
}


#[test]
fn test_division(){
    assert_eq!(eval(Expr::Op { op: Optor::Div, left: Box::new(Expr::Value(100)), right: Box::new(Expr::Value(5)), }),Ok(20));
}

#[test]
fn test_multiply(){
    assert_eq!(eval(Expr::Op { op: Optor::Mul, left: Box::new(Expr::Value(12)), right: Box::new(Expr::Value(5)), }),Ok(60));
}

#[test]
fn test_zeros(){
    assert_eq!(eval(Expr::Op { op: Optor::Add, left: Box::new(Expr::Value(0)), right: Box::new(Expr::Value(0)) }),Ok(0));
    assert_eq!(eval(Expr::Op { op: Optor::Sub, left: Box::new(Expr::Value(0)), right: Box::new(Expr::Value(0)) }),Ok(0));
    assert_eq!(eval(Expr::Op { op: Optor::Mul, left: Box::new(Expr::Value(0)), right: Box::new(Expr::Value(0)) }),Ok(0));
    assert_eq!(eval(Expr::Op { op: Optor::Div, left: Box::new(Expr::Value(0)), right: Box::new(Expr::Value(7)) }),Ok(0));

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
                  right: Box::new(second) }),Ok(-34));

}
fn main() {
}
