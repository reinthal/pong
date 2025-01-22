use std::ops::Add;

// Define the enum
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Na,
}

// Implement Add for Direction
impl Add for Direction {
    type Output = Direction;

    fn add(self, rhs: Direction) -> Direction {
        match (self, rhs) {
            // Define rules
            (Direction::Up, Direction::Up) => Direction::Na,
            (Direction::Up, Direction::Down) => Direction::Na,
            (Direction::Down, Direction::Up) => Direction::Na,
            (Direction::Down, Direction::Down) => Direction::Na,
            
            // Interaction with `Na`
            (Direction::Up, Direction::Na) => Direction::Up,
            (Direction::Na, Direction::Up) => Direction::Up,
            (Direction::Down, Direction::Na) => Direction::Down,
            (Direction::Na, Direction::Down) => Direction::Down,
            
            // Na + Na
            (Direction::Na, Direction::Na) => Direction::Na,
        }
    }
}

// Main function to test the behavior
fn main() {
    let result1 = Direction::Up + Direction::Up;
    let result2 = Direction::Up + Direction::Down;
    let result3 = Direction::Up + Direction::Na;
    let result4 = Direction::Na + Direction::Na;

    println!("Up + Up = {:?}", result1); // Expected: Na
    println!("Up + Down = {:?}", result2); // Expected: Na
    println!("Up + Na = {:?}", result3); // Expected: Up
    println!("Na + Na = {:?}", result4); // Expected: Na
}