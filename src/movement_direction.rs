#[derive (Debug, PartialEq, Copy, Clone)]
pub enum Movement_direction {
    Up,
    Down,
    Left,
    Right,
}

impl Movement_direction {
    pub fn is_in_opposite_direction(self, direction: Movement_direction) -> bool {
        match self {
            Movement_direction::Up => {
                if direction == Movement_direction::Down {
                    return true;
                }
            },
            Movement_direction::Down => {
                if direction == Movement_direction::Up {
                    return true;
                }
            },
            Movement_direction::Left => {
                if direction == Movement_direction::Right {
                    return true;
                }
            },
            Movement_direction::Right => {
                if direction == Movement_direction::Left {
                    return true;
                }
            },
        }
        return false;
    }
}
