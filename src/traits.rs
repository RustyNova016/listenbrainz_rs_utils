pub trait Paginator<PageType, PositionType>: Iterator{
    fn set_position(&self, postion: PositionType);

    fn get_reader
}