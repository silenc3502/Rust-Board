use uuid::Uuid;
use crate::board::controller::request_form::board_request_form::BoardRequestForm;
use crate::board::entity::board::Board;
use crate::board::service::board_service::BoardService;

pub struct BoardController {
    board_service: Box<dyn BoardService>,
}

impl BoardController {
    pub fn new(board_service: Box<dyn BoardService>) -> BoardController {
        BoardController {
            board_service,
        }
    }

    pub fn board_register(&mut self, board_request_form: BoardRequestForm) -> Board {
        self.board_service.register(board_request_form.to_board_request())
    }

    pub fn board_list(&self) -> Vec<Board> {
        self.board_service.list()
    }

    pub fn board_read(&self, board_id: Uuid) -> Option<Board> {
        self.board_service.read(board_id)
    }

    pub fn board_remove(&mut self, board_id: Uuid) {
        self.board_service.remove(board_id);
    }

    pub fn board_modify(&mut self, board_id: Uuid, board_request_form: BoardRequestForm) -> Option<Board> {
        self.board_service.modify(board_id, board_request_form.to_board_request())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::service::board_service_impl::BoardServiceImpl;
    use crate::board::repository::in_memory_board_repository::InMemoryBoardRepository;

    #[test]
    fn test_board_register() {
        let mut controller = BoardController::new(
            Box::new(
                BoardServiceImpl::new(
                    Box::new(
                        InMemoryBoardRepository::new()
                    )
                )));

        let board_request_form = BoardRequestForm::new(
            "Test Title".to_string(),
            "Test Writer".to_string(),
            "Test Content".to_string());

        controller.board_register(board_request_form);

        let boards = controller.board_list();
        assert_eq!(boards.len(), 1);
        assert_eq!(boards[0].get_title(), "Test Title");
    }

    #[test]
    fn test_board_list() {
        let controller = BoardController::new(
            Box::new(
                BoardServiceImpl::new(
                    Box::new(
                        InMemoryBoardRepository::new()
                    )
                )));

        let boards = controller.board_list();
        assert_eq!(boards.len(), 0);
    }

    #[test]
    fn test_board_read() {
        let mut controller = BoardController::new(
            Box::new(
                BoardServiceImpl::new(
                    Box::new(
                        InMemoryBoardRepository::new()
                    )
                )));

        let board_request_form = BoardRequestForm::new(
            "Test Title".to_string(),
            "Test Writer".to_string(),
            "Test Content".to_string());

        let board_response = controller.board_register(board_request_form);
        let board_id = board_response.get_board_id().clone();

        let retrieved_board = controller.board_read(board_id.clone());
        assert_eq!(retrieved_board.unwrap().get_title(), "Test Title");
    }

    #[test]
    fn test_board_remove() {
        let mut controller = BoardController::new(
            Box::new(
                BoardServiceImpl::new(
                    Box::new(
                        InMemoryBoardRepository::new()
                    )
                )));

        let board_request_form = BoardRequestForm::new(
            "Test Title".to_string(),
            "Test Writer".to_string(),
            "Test Content".to_string());
        let board_id = controller.board_register(board_request_form).get_board_id().clone();

        controller.board_remove(board_id.clone());

        let boards = controller.board_list();
        assert_eq!(boards.len(), 0);
    }

    #[test]
    fn test_modify() {
        let mut controller = BoardController::new(
            Box::new(
                BoardServiceImpl::new(
                    Box::new(
                        InMemoryBoardRepository::new()
                    )
                )));

        let board_request_form = BoardRequestForm::new(
            "Test Title".to_string(),
            "Test Writer".to_string(),
            "Test Content".to_string());

        let board_id = controller.board_register(board_request_form).get_board_id().clone();
        let updated_board_request_form = BoardRequestForm::new(
            "Updated Title".to_string(),
            "Updated Writer".to_string(),
            "Updated Content".to_string());

        let modified_board = controller.board_modify(board_id.clone(), updated_board_request_form);
        assert_eq!(modified_board.unwrap().get_title(), "Updated Title");

        let retrieved_board = controller.board_read(board_id.clone());
        assert_eq!(retrieved_board.unwrap().get_title(), "Updated Title");
    }
}
