extern crate ndarray;
extern crate termcolor;
extern crate random_integer;
use serde::{Serialize, Deserialize};






impl matrix{

    pub fn zeros_matrix(cols: usize, rows: usize)-> matrix{
        matrix{cols:cols, rows:rows, data: vec![0.0; rows*cols]}
    }

    pub fn get(&self, y: usize, x:usize)->f32{
           assert!(self.cols > x);
          assert!(self.rows >y);
        self.data[self.cols*y+x]
    }
    pub fn set(&mut self, y:usize, x:usize, value:f32){
        assert!(self.cols > x);
        assert!(self.rows >y);
        self.data[self.cols*y+x] = value;
    }
    pub fn times(&self, b: &matrix) -> matrix{
        assert!(b.rows == self.cols);
        let mut c = matrix::zeros_matrix(self.rows, b.cols);
        for i in 0..self.rows{
            for j in 0..b.cols{
                let mut sum:f32 = 0.0;
                for k in 0..self.cols{
                    sum = sum + self.get(i, k)*b.get(k,j);
                }
                c.set(i,j,sum);
            }
        }
        return c;
    }
    pub fn times_a_vector(&self, b: &Vec<f32>) -> Vec<f32>{
        assert!(b.len() == self.cols);
        let mut vector:Vec<f32> = Vec::with_capacity(self.rows);
        for i in 0..self.rows{

                let mut sum:f32 = 0.0;
                for k in 0..self.cols{
                    sum = sum + self.get(i, k)*b[k];
                }
                vector.push(sum);

        }
        return vector;
    }
    pub fn print_matrix(&self){
        for y in 0..self.rows {
            for x in 0..self.cols {
                print!("({}) ",self.get(y,x));

            }

        }
        println!();
    }

}

pub fn matrix_from(columns: usize,cells: Vec<f32>) -> matrix{
    let length = cells.len();
    assert!(length%columns == 0);
    matrix{cols:columns, rows:length/columns, data:cells}


}
#[derive(Serialize, Deserialize, Debug)]

#[derive(Clone)]
pub struct matrix{
    pub cols: usize,
    pub rows: usize,
    pub data: Vec<f32>,
}

















//type Board = ArrayBase<OwnedRepr<Pieces>, Dim<[usize;2]>>;




