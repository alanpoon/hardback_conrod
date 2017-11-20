use conrod::{self, color, widget, Colorable, Positionable, Widget, Sizeable, image, Labelable};
use cardgame_widgets::custom_widget::full_cycle_sprite;
use cardgame_widgets::sprite::SpriteInfo;
use backend::codec_lib::codec::*;
use std::collections::HashMap;
use std::collections::HashSet;
use futures::sync::mpsc;
use futures::{Future, Sink};
use app::{GameData, Ids, Personal,OverlayStatus};
use backend::OwnedMessage;
use backend::SupportIdType;
use backend::meta::app::{AppData, ResourceEnum, Sprite};
use backend::meta::{cards,gameicon};
use backend::meta::local;
use logic::in_game;
use instruction::Instruction;
pub fn render(ui: &mut conrod::UiCell,
            w_id: tabview::Item,
            ids: &Ids,
            mut gamedata: &mut GameData,
            appdata: &AppData,
            result_map: &HashMap<ResourceEnum, SupportIdType>,
            action_tx: mpsc::Sender<OwnedMessage>) {
        let GameData{
            ref mut boardcodec,
            ref player_index,
            ref mut personal,
            ref mut overlay_receivedimage,
            ..
        } = *gamedata;
        //choose from the inked cards
        
         if let &mut Some(ref  boardcodec) = boardcodec {
                if let Some(ref  _player) = boardcodec.players.get(*player_index) {
                    let arranged = _player.arranged.clone();
                    let inked = arranged.filter(|(_ci,_inked,_optstr)|_inked).collect::<Vec<usize>>();
                    let inked_hash_set:HashSet<usize> = HashSet::new();
                    for card in inked.iter(){
                        inked_hash_set.insert(card);
                    }
                    let item_h = 230.0;
                    let (mut events, scrollbar) = widget::ListSelect::single(inked.len())
                    .flow_down()
                    .item_size(item_h)
                    .scrollbar_next_to()
                    .w_h(700.0, 260.0)
                    .middle_of(ids.overlaybody)
                    .set(ids.overlay_explainlistselect, ui);
             let card_images = in_game::card_images(result_map);
           
         // Handle the `ListSelect`s events.
                while let Some(event) = events.next(ui, |i| inked_hash_set.contains(&i)) {
                    use conrod::widget::list_select::Event;
                    match event {

                        // For the `Item` events we instantiate the `List`'s items.
                        Event::Item(item) => {
                            let card_index = &inked_hash_set[item.i];
                            let color= match list_selected.contains(&item.i) {
                                true => conrod::color::LIGHT_BLUE,
                                false => conrod::color::LIGHT_GREY,
                            };
                            let (_image_id, _rect, _) = in_game::get_card_widget_image_portrait(card_index.clone(), card_images, appdata);
                            let button = widget::Button::image(_image_id)
                                .source_rectangle(_rect)
                                .border(0.0)
                                .color(color);
                            item.set(button, ui);
                        },

                        // The selection has changed.
                        Event::Selection(selection) => {
                            selection.update_index_set(&mut inked);
                        },

                        // The remaining events indicate interactions with the `ListSelect` widget.
                        event => println!("{:?}", &event),
                    }
                }

                // Instantiate the scrollbar for the list.
                if let Some(s) = scrollbar { s.set(ui); }
            }
        }
  
        if let &mut Some(ref mut boardcodec) = boardcodec {
        if let Some(ref mut _player) = boardcodec.players.get_mut(*player_index) {
            if _player.remover>0{
                match overlay_receivedimage[1]{
                    &mut OverlayStatus::Received(ref _img,ref _rect,ref _theme)=>{
                         widget::Image::new(_over_receivedimage).source_rectangle(_rect.clone())
                            .w(150.0).h(150.0)
                            .mid_bottom_with_margins(ids.overlaybody,20.0)
                            .set(ids.overlay_receivedimage,ui);
                    },
                    &mut OverlayStatus::Loading=>{
                        if let Some(&SupportIdType::ImageId(dwn_img)) =
                            result_map.get(&ResourceEnum::Sprite(Sprite::DOWNLOAD)) {
                                let spin_info=get_spinner_spriteinfo();
                                FullCycleSprite::new(_over_receivedimage, spin_info)
                                .mid_bottom_with_margins(ids.overlaybody,20.0)
                                .w(100.0).h(100.0)
                                .set(ids.overlay_receivedimage, ui);
                        }
                    },
                    &mut OverlayStatus::None=>{
                        for _c in widget::Button::new().label(&appdata.texts.use_remover_but)
                        .mid_bottom_with_margins(ids.overlaybody,20.0)
                        .set(ids.overlay_okbut,ui){
                            *overlay_receivedimage[0] =OverlayStatus::Loading;
                            let action_tx_c = _action_tx.clone();
                            let mut h = ServerReceivedMsg::deserialize_receive("{}").unwrap();
                            let mut g = GameCommand::new();
                            g.use_remover = Some(true);
                            h.set_gamecommand(g);
                            action_tx_c.send(OwnedMessage::Text(ServerReceivedMsg::serialize_send(h).unwrap()))
                            .wait()
                            .unwrap();
                        }
                    }
                }

            }
        }
        }
  }
  fn get_spinner_spriteinfo()->SpriteInfo{
    SpriteInfo {
        first: (0.0, 400.0),
        num_in_row: 12,
        num_in_col: 4,
        w_h: (100.0, 100.0),
        pad: (0.0, 0.0, 0.0, 0.0),
    }
  }