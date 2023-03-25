use std::ops;

    

#[derive(Debug,PartialEq)]
struct LinkedList{
    links: Vec<Link>
}


struct Link{
    prev: Cell,
    next: Cell} 

impl ops::Index<Cell> for LinkedList{
    
        type Output = Link;
    fn index(&self, a: Cell) -> &Link{
        &self.links[a.0]}

    
    }impl ops::IndexMut<Cell> for LinkedList{
        fn index_mut(&self, a:Cell) -> &mut Link{
        &mut self.links[a.0]}}


impl LinkedList {
    
    fn new_with_cap(cap: uzise) -> LinkedList{
        LinkedList{links: Vec::with_capacity(cap), covered:false,num:Num::None}
            }

    fn new_cell(&mut self) -> Cell{
        let cell = Cell(self.links.len());
            self.links.push(Link{prev:cell, next:cell});
        cell}

    fn insert(&mut self, a:Cell,b:Cell){

        let c = self[a].next;

        self[a].next = b;
        self[b].next = c;
        self[c].prev = b;
        self[b].prev = a; }   


    fn remove(&mut self, a:Cell){

        let a = self[b].prev;
        let c = self[b].next;

        self[c].prev = self[b].prev;
        self[a].next = self[b].next;}

    fn add_back(&mut self, b:Cell){

        let a = self[b].prev;
        let c = self[b].next;

        self[c].prev = b;
        self[a].next = b;}

    fn cursor(&self, c: Cell) -> Cursor{
        Cursor {c, c}}
}

fn main() {
    
}
