pub use genmesh::{Line,Triangle};
pub use page_curl::vertex::Vertex;
use std::f32::consts::PI;
const RAD:f32 = 180.0 / PI;
const COLUMNS:u16 = 20;
const ROWS:u16 =20;
const N_VERTICES:usize = ((COLUMNS+1) * (ROWS+1)) as usize;
const N_STRIPS: usize =(((COLUMNS+1) * 2) * ((ROWS+1) - 1) + ((ROWS+1) - 2)) as usize;

pub struct Page{    
    width:f32,
    height:f32,
    flipping:bool,
    time:f32,
    theta:f32,
    rotation:f32,
    translation:f32,
    in_mesh:Vec<Line<f32>>,
    pub out_mesh:Vec<Vertex>,
    tex_coords:[(f32,f32);N_VERTICES],
   pub front_strip:[u16;N_STRIPS],
   pub back_strip:[u16;N_STRIPS],
}
impl Page{
    pub fn new()->Page{
        Page{
            width:0.8,
            height:1.0,
            flipping:false,
            time:0.0,
            theta:90.0,
            rotation:0.0,
            translation:0.0,
            in_mesh:vec![],
            out_mesh:vec![],
            tex_coords:[(0.0f32,0.0f32);N_VERTICES],
            front_strip:[0u16;N_STRIPS],
            back_strip:[0u16;N_STRIPS],
        }
    }
    pub fn create_mesh(&mut self){
        let cx =COLUMNS +1;
        let cy = ROWS +1;
        if  self.in_mesh.len() >0{
            self.in_mesh = vec![];
        }
        if  self.out_mesh.len() >0{
            self.out_mesh = vec![];
        }
        self.in_mesh = vec![];
        self.out_mesh = vec![];
        self.tex_coords = [(0.0f32,0.0f32);N_VERTICES];
        let mut i =0;
        for iy in 0..cy{
            let iiy =iy as f32;
            for ix in 0..cx{
                let iix = ix as f32;
                let px = iix* self.width/ (COLUMNS as f32);
                let py = iiy  * self.height/ (ROWS as f32);
                self.in_mesh.push(Line::new(px,py));
                let tx = (cx-ix ) as f32 / (COLUMNS as f32);
                let ty = (cy-iy ) as f32/(ROWS as f32);
                self.tex_coords[i] = (tx,ty);
                i+=1;
            }
        }
        self.time =0.0;
        println!("before strip");
        self.stripify();
        println!("aft strip");
        self.update_time();
    }
    pub fn stripify(&mut self){
        let cx =COLUMNS +1 ;      
        let mut offset;
        let mut i = 0;
        for iy in 0..ROWS{
            let last = iy == (ROWS -1);
            let odd = iy %2 ==1;
            offset = iy*cx;
            for ix in 0..(COLUMNS+1){
                if odd{
                    self.front_strip[i] = offset + COLUMNS - ix + cx;
                     self.back_strip[i] = offset + ix + cx;
                     i+=1;
                     self.front_strip[i] =offset + COLUMNS  - ix;
                     self.back_strip[i] =offset + ix;
                     i+=1;
                } else{
                    self.front_strip[i]= offset + ix + cx;
                    self.back_strip[i] =offset + COLUMNS  - ix + cx;
                    i+=1;
                    self.front_strip[i] =offset + ix;
                    self.back_strip[i] =offset + COLUMNS  - ix;
                    i+=1;
                }
                
            }
        if !last {
          if odd {
            self.front_strip[i]=offset + cx;
            self.back_strip[i]= offset + cx + COLUMNS;
            i+=1;
          } else {
            self.front_strip[i]= offset + cx + COLUMNS;
            self.back_strip[i]= offset + cx;
            i+=1;
          }
        }
        }
    }
    pub fn flip(&mut self){
        self.flipping = true;
    }
    pub fn update_time(&mut self){
        if !self.flipping{
            return;
        }
        
        let angle1 = 90.0/RAD;
        let angle2 = 9.0/RAD;
        let angle3 = 6.0/RAD;
        let _a1 = -15.0;
        let _a2 = -2.5;
        let _a3 = -3.5;
        let theta1 = 0.05;
        let theta2= 0.5;
        let theta3 = 10.0;
        let theta4= 2.0;
        self.time+=0.01;
        if self.time>=1.0{
            self.time =0.0;
            self.flipping = false;
            return;
        }
        let  dt;
        let  f1;
        let  f2;
        self.rotation = self.time * PI;
        if self.time <= 0.15{
            dt = self.time/0.15;
            f1 = (PI * dt.powf(theta1)/2.0).sin();
            f2 = (PI * dt.powf(theta2)/2.0).sin();
            self.theta = func_linear (f1, angle1, angle2);
        self.translation = func_linear (f2, _a1, _a2);
        }else if self.time <= 0.4 {
        dt = (self.time - 0.15) / 0.25;
        self.theta = func_linear (dt, angle2, angle3);
        self.translation = func_linear (dt, _a2, _a3);
      } else if self.time <= 1.0 {
        dt = (self.time - 0.4) / 0.6;
      f1 = (PI * dt.powf(theta3)/2.0).sin();
            f2 = (PI * dt.powf(theta4)/2.0).sin();
        self.theta = func_linear (f1, angle3, angle1);
        self.translation = func_linear (f2, _a3, _a1);
      }
      self.deform();
    }
    fn deform(&mut self){
        let mut ina;
        let mut tmp;
        let mut radius;
        let mut r;
        let mut beta;
        println!("n_vertices {},in_mesh {}",N_VERTICES,self.in_mesh.len());
        for i in 0..N_VERTICES{
            ina = self.in_mesh[i as usize];
            radius = (ina.y-self.translation).powf(2.0).sqrt();
            r = radius * self.theta.sin();
            beta = (ina.x/radius).asin() / self.theta.sin();

            let x = r*beta.sin();
            let y = radius + self.translation - r * (1.0 -beta.cos()) * self.theta.sin();
            let z = r*(1.0 - beta.cos()) * self.theta.sin();
            tmp = Triangle::new(x,y,z);

           // out = &self.out_mesh[i as usize];
            let xx = tmp.x * self.rotation.cos() - tmp.z *self.rotation.sin();
            let yy = tmp.y;
            let zz = tmp.x * self.rotation.sin() + tmp.z* self.rotation.cos();
            //out = &Triangle::new(xx,yy,zz);
            self.out_mesh.push(Vertex{
                position:(xx,yy,zz),
                tex_coords:(self.tex_coords[i].0,self.tex_coords[i].1)
            });
        }
    }
}
pub fn func_linear(ft:f32,f0:f32,f1:f32)->f32{ //created by genmesh
    f0 + (f1-f0)*ft
}