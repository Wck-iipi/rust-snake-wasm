#[derive (Debug, PartialEq, Copy, Clone)]
pub enum movement_direction {
    Up,
    Down,
    Left,
    Right,
}

impl movement_direction {
    pub fn is_in_opposite_direction(self, direction: movement_direction) -> bool {
        match self {
            movement_direction::Up => {
                if direction == movement_direction::Down {
                    return true;
                }
            },
            movement_direction::Down => {
                if direction == movement_direction::Up {
                    return true;
                }
            },
            movement_direction::Left => {
                if direction == movement_direction::Right {
                    return true;
                }
            },
            movement_direction::Right => {
                if direction == movement_direction::Left {
                    return true;
                }
            },
        }
        return false;
    }
}
