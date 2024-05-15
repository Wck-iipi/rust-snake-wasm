#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

impl MovementDirection {
    pub fn is_in_opposite_direction(self, direction: MovementDirection) -> bool {
        match self {
            MovementDirection::Up => {
                if direction == MovementDirection::Down {
                    return true;
                }
            }
            MovementDirection::Down => {
                if direction == MovementDirection::Up {
                    return true;
                }
            }
            MovementDirection::Left => {
                if direction == MovementDirection::Right {
                    return true;
                }
            }
            MovementDirection::Right => {
                if direction == MovementDirection::Left {
                    return true;
                }
            }
        }
        return false;
    }
}
