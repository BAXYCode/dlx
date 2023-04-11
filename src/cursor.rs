use crate::linked::Linked;
use crate::cells::Cell;

pub(crate) struct Cursor{
   pub(crate)  head: Cell,
    pub(crate) current: Cell}
impl Cursor {
    
    //Takes in a reference to a list and uses the cursor's internals to 
    //navigate said list. If the cursor's new curr node is the same as 
    //the node it started with, None is returned.
   pub(crate)  fn next(&mut self, list: &Linked) -> Option<Cell>{
        
         self.current = list[self.current].next;
        
        if self.current == self.head{
        return None;}
        Some(self.current)
            } 
    pub(crate)fn prev(&mut self, list:&Linked) -> Option<Cell>{

        self.current = list[self.current].prev;

        if self.current == self.head{
            return None;}
            Some(self.current)}
}
