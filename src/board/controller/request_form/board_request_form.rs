use crate::board::entity::board::Board;
use crate::board::service::request::board_request::BoardRequest;

#[derive(Debug)]
pub struct BoardRequestForm {
    title: String,
    writer: String,
    content: String,
}

impl BoardRequestForm {
    pub fn new(title: String, writer: String, content: String) -> BoardRequestForm {
        BoardRequestForm { title, writer, content }
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_writer(&self) -> &str {
        &self.writer
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn to_board_request(&self) -> BoardRequest {
        BoardRequest::new(self.title.clone(), self.writer.clone(), self.content.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_board() {
        let title = String::from("Sample Title");
        let writer = String::from("John Doe");
        let content = String::from("Sample Content");

        let board_request = BoardRequest::new(title.clone(), writer.clone(), content.clone());
        let board = board_request.to_board();

        assert_eq!(board.get_title(), &title);
        assert_eq!(board.get_writer(), &writer);
        assert_eq!(board.get_content(), &content);
    }
}
