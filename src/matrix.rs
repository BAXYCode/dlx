mod cells;
mod cursor;
mod LinkedList;


struct Matrix{
    horizontal: LinkedList,
    vertical: LinkedList,

    access: Vec<Cell>,

    sizes: Vec<usize>,
   } 

impl Matrix{

    pub fn new(n: usize, rows:usize,cols:usize) -> Matrix{
        
        let mut  matrix =  Matrix{
            horizontal : Vec::with_capacity(cols+1),
            vertical : Vec::with_capacity(rows+1),
            size: Vec::new(),
            access: Vec::with_capacity(cols+1),
        }
        for _ in  0..cols{
            matrix.add_column();
        }
        matrix
    } 

    fn allocate_cell(&mut self, cell: Cell) -> Cell{

        self.access.push(cell);
        let x_cell = self.horizontal.new_cell();
        let y_cell = self.vertical.new_cell();
        assert_eq!(x_cell,y_cell );
            x_cell
    }
    fn allocate_column(&mut self) -> Cell{
            
        let cell = self.allocate_cell(CERO);
        self.access[cell] = cell;
        self.size.push(0);
        cell 
    }

    fn add_column(&mut self) -> Cell {
        
        let cell = self.allocate_column();

        // add column in last position
        self.horizontal.insert(self.horizontal[CERO].prev,cell);

        cell
    }
    pub fn add_row(&mut self, row: Vec<u32>) {


        let mut col = CERO;
        
        let mut prev = None;
        for val in row{
        col = self.horizontal[col].next;  
        match val{
            Some(0) => _,
            Some(1) =>{
                self.size[col] += 1;
                let new_cell = self.allocate_cell();
                //fetch column and add new cell as cols new previous cell
                self.vertical.insert(self.vertical[col].prev,new_cell);
                if let Some(prev) =prev{
                    self.horizontal.insert(prev,new_cell);
                    }
                prev = Some(new_cell);}                
            }
        }
    }
    fn cover(&mut self, cell:Cell){
       //remove acces cell front x axis list 
        self.horizontal.remove(cell);
        //get y axis cursor to iterate on rows to cover
        let mut cur = self.y.cursor(cell);
        //here we only cover the rows but we leave the covered columns nodes
        //for us to cover later outside of this function
        while let Some(c_axis_cell) = cur.next(&self.vertical){
            let mut curr = self.horizontal.cursor(c_axis_cell);
            while let Some(r_axis_cell) = curr.next(&self.horizontal){
                
                self.vertical.remove(r_axis_cell);
                self.size[self.acces[r_axis_cell]] -= 1;

            }
             
        }
    }


    fn uncover(&mut self, cell:Cell){
        
        let mut cur = self.vertical.cursor(cell);
        while let Some(c_axis_cell) = cur.prev(&self.vertical){
        
            let mut r_axis_cell = self.horizontal.cursor(c_axis_cell);
            while let Some(current_cell) = r_axis_cell.prev(&self.horizontal){
                
                self.size[self.access[current_cell]] += 1;
                self.vertical.add_back(current_cell);

            }

        }
        self.horizontal.add_back(cell);

    }

}
