pub mod page;
pub mod vertex;

pub fn render(_page:&mut page::Page){
     _page.flip();
     _page.create_mesh();
     println!("ff{:?} ",_page.front_strip);
     println!("ff len{:?}",_page.front_strip.len());
     println!("bb{:?}",_page.back_strip);
       println!("bb len{:?}",_page.back_strip.len());
  
}
