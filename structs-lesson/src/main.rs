
trait Run{
    fn run(&self){
        println!("I'm running");
    }
}

struct Robot{}

impl Run for Robot{}

fn main() {
    let robot = Robot{};
    robot.run();
}