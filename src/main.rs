mod board;

use uuid::Uuid;
use chrono::Utc;

use board::entity::board::Board;
use board::repository::in_memory_board_repository::InMemoryBoardRepository;
use crate::board::controller::board_controller::BoardController;
use crate::board::controller::request_form::board_request_form::BoardRequestForm;
use crate::board::repository::board_repository::BoardRepository;
use crate::board::service::board_service_impl::BoardServiceImpl;
use crate::board::service::request::board_request::BoardRequest;

static mut BOARD_CONTROLLER: Option<BoardController> = None;

fn init_config() {
    unsafe {
        BOARD_CONTROLLER = Some(BoardController::new(
            Box::new(
                BoardServiceImpl::new(
                    Box::new(
                        InMemoryBoardRepository::new())))));
    }
}

fn main() {
    // let mut in_memory_repository = InMemoryBoardRepository::new();
    //
    // let new_board = Board::new(
    //     "Initial Title".to_string(),
    //     "Initial Writer".to_string(),
    //     "Initial Content".to_string(),
    // );
    //
    // in_memory_repository.save(new_board.clone());
    //
    // let all_boards = in_memory_repository.find_all();
    // println!("All boards in the repository: {:?}", all_boards);

    init_config();

    unsafe {
        if let Some(mut controller) = BOARD_CONTROLLER.take() {
            let board_request_form = BoardRequestForm::new(
                "Initial Title".to_string(),
                "Initial Writer".to_string(),
                "Initial Content".to_string(),
            );

            let board_response = controller.board_register(board_request_form);
            let board_id = board_response.get_board_id().clone();
            println!("Registered board with ID: {:?}", board_id);

            let retrieved_board = controller.board_read(board_id.clone());
            println!("Retrieved board: {:?}", retrieved_board);

            let modified_board_request_form = BoardRequestForm::new(
                "Modified Title".to_string(),
                "Modified Writer".to_string(),
                "Modified Content".to_string(),
            );
            let modified_board_response = controller.board_modify(
                board_id.clone(),
                modified_board_request_form);
            println!("Modified board: {:?}", modified_board_response);

            let second_board_request_form = BoardRequestForm::new(
                "Second Title".to_string(),
                "Second Writer".to_string(),
                "Second Content".to_string(),
            );
            let second_board_response = controller.board_register(
                second_board_request_form);
            let second_board_id = second_board_response.get_board_id().clone();
            println!("Registered second board with ID: {:?}", second_board_id);

            let board_list = controller.board_list();
            println!("List of boards: {:?}", board_list);

            controller.board_remove(board_id.clone());
            println!("Removed board with ID: {:?}", board_id);

            let board_list = controller.board_list();
            println!("List of boards: {:?}", board_list);
        }
    }
}
