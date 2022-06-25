use super::game_state::GameState;

pub struct AttackState {}

impl GameState for AttackState {
    fn init(&mut self, game: &mut crate::handlers::game_manager::Game) {
        todo!()
    }

    fn run(&mut self, game: &mut crate::handlers::game_manager::Game) {
        todo!()
    }

    fn next(self: Box<Self>) -> Box<dyn GameState> {
        todo!()
    }
}
