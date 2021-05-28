
use crate::controls::list_view::ListDataModel;

#[derive(Clone)]
pub struct SimpleDataModel {
    data: Vec<String>,
}

impl ListDataModel for SimpleDataModel {

    fn item_count(&self) -> usize {
        self.data.len()
    }

    fn data(&self, index: usize) -> &str where Self : Sized {
        self.data.get(index).unwrap()
    }
}
