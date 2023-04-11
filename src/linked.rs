use std::ops;
use crate::cells::Cell;
use crate::cursor::Cursor;

    
pub(crate) struct Linked{
   pub(crate)  links: Vec<Link>
}


pub struct Link{
   pub(crate)  prev: Cell,
    pub(crate) next: Cell} 

impl ops::Index<Cell> for Linked{
    
        type Output = Link;
    fn index(&self, a: Cell) -> &Link{
        &self.links[a.0]}

    
    }impl ops::IndexMut<Cell> for Linked{
        fn index_mut(&mut self, a:Cell) -> &mut Link{
        &mut self.links[a.0]}}


impl Linked {
    
    pub(crate) fn new_with_cap(cap: usize) -> Linked{
        Linked{links: Vec::with_capacity(cap)}
            }

    pub(crate) fn new_cell(&mut self) -> Cell{
        let cell = Cell(self.links.len());
            self.links.push(Link{prev:cell, next:cell});
        cell}

    pub(crate) fn insert(&mut self, a:Cell,b:Cell){

        let c = self[a].next;

        self[a].next = b;
        self[b].next = c;
        self[c].prev = b;
        self[b].prev = a; }   


   pub(crate)  fn remove(&mut self, b:Cell){

        let a = self[b].prev;
        let c = self[b].next;

        self[c].prev = self[b].prev;
        self[a].next = self[b].next;}

   pub(crate)  fn add_back(&mut self, b:Cell){

        let a = self[b].prev;
        let c = self[b].next;

        self[c].prev = b;
        self[a].next = b;}

   pub(crate) fn cursor(&self, c: Cell) -> Cursor{
        Cursor {head: c, current:c}}
}


