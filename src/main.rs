use crate::creature::Creature;

mod creature;

fn main() {
   let john = Creature::new("John");
   let goblin = Creature::new("Goblin");
   println!("{} killed {}",john,goblin);
   drop(goblin);
}
