/* pub enum Control {
    ButtonPrivate(),
    ToggleButtonPrivate(),
    Container(),
    ImageView(),
    Page(),
}
*/

pub trait Control {
    fn get_type() -> String;
}