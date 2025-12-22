
/*
This simulates the famous dining philosopher problem

*/

//synchronization mechanism
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

struct Chopstick;



struct Philosopher{
    name:String,
    left_stick:Arc<Mutex<Chopstick>>,
    right_stick:Arc<Mutex<Chopstick>>,
    thinking:mpsc::SyncSender<String>
}


// philosopher actions
impl Philosopher {

    fn think(&self){
        self.thinking.send(format!("Got it! {} has a new idea!", &self.name)).unwrap();
     }

     fn eat(&self){
        println!("{} is trying to eat",&self.name);
        let _left=self.left_stick.lock().unwrap();
        let _right=self.right_stick.lock().unwrap();
        println!("{} is eating... ",&self.name);
        thread::sleep(Duration::from_secs(1));
     }   
}

// names of philosophers
static PHILOSOPHERS: &[&str]=&["Xenophone","Musashi","Socrates","Nnaemeka"];

fn main() {
  let (tx, rx)= mpsc::sync_channel(10);
   //creates chopsticks
   let chopsticks=PHILOSOPHERS.iter()
                                               .map(|_| Arc::new(Mutex::new(Chopstick)))
                                               .collect::<Vec<_>>();


                                            for i in 0..chopsticks.len(){
                                               let tx=tx.clone();
                                               let mut left_stick=Arc::clone(&chopsticks[i]);
                                               let mut right_stick=Arc::clone(&chopsticks[(i+1) % chopsticks.len()]);

                                               if i == chopsticks.len() - 1{
                                                std::mem::swap(&mut left_stick, &mut right_stick);
                                               }
//creates philosopher
                                               let philosopher = Philosopher{
                                                                       name:PHILOSOPHERS[i].to_string(),
                                                                       thinking:tx,
                                                                       left_stick,
                                                                       right_stick,
                                               };

// aAphilosopher tries to eat and think for 100 times
                                            thread::spawn(move ||{

                                               for _ in 0..100{
                                                philosopher.eat();
                                                philosopher.think();
                                               } 
                                            });
 }

 drop(tx);
 // print thoughts
 for tink in rx{
    println!("{tink}");
 }


}
