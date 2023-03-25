mod LinkedList;
mod cells;

struct Cursor{
    head: Cell,
    current: Cell}
impl Cursor {
    
    //Takes in a reference to a list and uses the cursor's internals to 
    //navigate said list. If the cursor's new curr node is the same as 
    //the node it started with, None is returned.
    fn next(&self, &list: LinkedList) -> Option<Cell>{
        
         self.current = list[self.current].next;
        
        if assert_eq!(self.current,self.head ){
        return None;}
        Some(self.current);
            } 
    fn prev(&self, &list:LinkedList) -> Option<Cell>{

        self.current = list[self.current].prev;

        if assert_eq!(self.current, self.head){
            return None;}
            Some(self.current);}
}
