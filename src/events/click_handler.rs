//pub type ClickHandler = fn(&mut page::Page, &MouseEvent);

pub trait ClickHandler<T : ?Sized> {
    fn do_things(&mut self, event : T);
}
