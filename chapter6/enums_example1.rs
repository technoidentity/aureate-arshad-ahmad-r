enum TrafficLight{
    Red,
    Yellow,
    Green,
}

fn main(){
    let light=TrafficLight::Red;

    match light{
        TrafficLight::Red => println!("stop"),
        TrafficLight::Yellow => println!("Ready to Stop"),
        TrafficLight::Green => println!("Go"),
    }
}
