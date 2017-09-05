macro_rules! image_map {
     ($(($idx:expr,$type:expr, $path:expr)),* $(,)*) => {
        use std::path::{Path};
        struct Vala{
            source_type:&'static str,
            path:&'static str
        }
        pub struct ImageIds{
            map:HashMap<ResourceEnum,Vala>
        }

        impl ImageIds{
            pub fn new()->ImageIds{
                let mut map = HashMap::<ResourceEnum,Vala>::new();
                $(map.insert($idx,Vala{source_type:$type,path:$path});)*
                ImageIds{
                    map:map
                }
            }

            pub fn pump(&self,result_map:&mut HashMap<ResourceEnum,SupportIdType>,
            display:&glium::Display,ui:&mut conrod::Ui,image_m:&mut conrod::image::Map< glium::texture::Texture2d>){
                let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
                for (k,v) in &self.map{
                    let  kk = k.clone();
                    if v.source_type =="image"{
                      let rust_logo = load_image(display, assets.join(v.path));
                      let id_i = image_m.insert(rust_logo);
                       result_map.insert(kk,SupportIdType::ImageId(id_i));
                    } else {
                      let id_f= ui.fonts.insert_from_file( assets.join(v.path)).unwrap();
                        result_map.insert(kk,SupportIdType::FontId(id_f));
                    }
                   
                } 
            }
        }
     fn load_image<P>(display: &glium::Display, path: P) -> glium::texture::Texture2d
        where P: AsRef<Path>,
    {
        let path = path.as_ref();
        let rgba_image = image::open(&Path::new(&path)).unwrap().to_rgba();
       let image_dimensions = rgba_image.dimensions();
       let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
        texture
    }
    };
}
