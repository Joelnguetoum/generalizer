/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::cmp;
use std::collections::{BTreeSet, HashMap};

use image::{Rgb, RgbImage};
use image_colored_text::draw::single_line::{draw_line_of_colored_text, DrawCoord};
use image_colored_text::ttp::TextToPrint;
use imageproc::drawing::draw_line_segment_mut;
use crate::interactions::io::output::draw_commons::font::{get_font, HIBOU_FONT_SCALE};
use crate::interactions::io::output::draw_commons::hibou_color_palette::HCP_Black;
use crate::interactions::io::output::draw_commons::sd_drawing_conf::{FRAGMENT_PADDING, FRAGMENT_TITLE_MARGIN, VERTICAL_SIZE};
use crate::interactions::io::output::draw_interactions::as_sd::action_repr::draw_action::draw_action;
use crate::interactions::io::output::draw_interactions::as_sd::action_repr::draw_vp::draw_vp;
use crate::interactions::io::output::draw_interactions::as_sd::util::dimensions_tools::get_y_pos_from_yshift;
use crate::interactions::io::output::draw_interactions::as_sd::util::lf_coords::DrawingLifelineCoords;
use crate::interactions::io::textual_convention::{SYNTAX_ALT, SYNTAX_LOOP_S, SYNTAX_PAR};
use crate::interactions::syntax::general_context::GeneralContext;
use crate::interactions::syntax::interaction::Interaction;
use crate::interactions::syntax::util::get_recursive_frag::{get_recursive_alt_frags, get_recursive_par_frags};
// **********
// CODE TO IMPROVE

pub fn draw_interaction_rec(    image : &mut RgbImage,
                                gen_ctx : &GeneralContext,
                                interaction : &Interaction,
                                lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                                lf_num : usize,
                                nest_shift : &mut u32,
                                yshift : &mut u32)
                        -> ([usize;2],u32) { // returns left and right borders of the interaction
    match interaction {
        &Interaction::Empty => {
            return ([lf_num,0],0); // because when going up we keep the minimum on the left and maximum on the right
        },
        &Interaction::Action(ref em_act) => {
            let lr_bounds = draw_action(image,gen_ctx,em_act,lf_x_widths,*yshift);
            *yshift = *yshift + 3;
            return (lr_bounds,3);
        },
        &Interaction::Vp(ref act1, ref act2) => {
            let lr_bounds = draw_vp(image,gen_ctx,act1,act2,lf_x_widths,*yshift);
            *yshift = *yshift + 3;
            return (lr_bounds,3);
        },
        &Interaction::Seq(ref i1,ref i2) => {
            let wr1  = draw_interaction_rec(image, gen_ctx,i1, lf_x_widths,  lf_num,nest_shift, yshift);
            *yshift = *yshift +1;
            let wr2 = draw_interaction_rec(image,  gen_ctx,i2, lf_x_widths,  lf_num,nest_shift, yshift);
            return ([ std::cmp::min(wr1.0[0],wr2.0[0]) , std::cmp::max(wr1.0[1],wr2.0[1]) ],wr1.1 + wr2.1 +1);
        },
        &Interaction::Tensor(ref i1,ref i2) => {
            let wr1   = draw_interaction_rec(image, gen_ctx,i1, lf_x_widths,  lf_num,nest_shift, yshift);
            let temp = *yshift;
            *yshift = *yshift - wr1.1;
            let wr2  = draw_interaction_rec(image,  gen_ctx,i2, lf_x_widths,  lf_num,nest_shift, yshift);
            *yshift = std::cmp::max(*yshift, temp);
            return ([ std::cmp::min(wr1.0[0],wr2.0[0]) , std::cmp::max(wr1.0[1],wr2.0[1]) ], std::cmp::max(wr1.1, wr2.1));
        },
        /*
        &Interaction::Strict(ref i1,ref i2) => {
            let mut frags = get_recursive_strict_frags(i1);
            frags.extend( get_recursive_strict_frags(i2) );
            let label = vec![TextToPrint::new(SYNTAX_STRICT.to_string(),Rgb(HCP_Black))];
            return draw_n_ary_combined_fragment(image, gen_ctx,frags,lf_x_widths, lf_num,label, nest_shift, yshift);
        },

         */
        &Interaction::Alt(ref i1,ref i2) => {
            let mut frags = get_recursive_alt_frags(i1);
            frags.extend( get_recursive_alt_frags(i2) );
            let label = vec![TextToPrint::new(SYNTAX_ALT.to_string(),Rgb(HCP_Black))];
            return draw_n_ary_combined_fragment(image, gen_ctx,frags,lf_x_widths, lf_num,label, nest_shift, yshift);
        },
        &Interaction::Par(ref i1,ref i2) => {
            let mut frags = get_recursive_par_frags(i1);
            frags.extend( get_recursive_par_frags(i2) );
            let label = vec![TextToPrint::new(SYNTAX_PAR.to_string(),Rgb(HCP_Black))];
            return draw_n_ary_combined_fragment(image, gen_ctx,frags,lf_x_widths, lf_num,label, nest_shift, yshift);
        },
        &Interaction::LoopS( ref i1) => {
            let label = vec![TextToPrint::new(SYNTAX_LOOP_S.to_string(),Rgb(HCP_Black))];
            return draw_unary_combined_fragment(image,  gen_ctx,i1,lf_x_widths, lf_num,label, nest_shift, yshift);
        },
        _ => {
            panic!("non-conform interaction");
        }
    }
}

fn draw_unary_combined_fragment(    image : &mut RgbImage,
                                    gen_ctx : &GeneralContext,
                                    i1 : &Interaction,
                                    lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                                    lf_num : usize,
                                    label : Vec<TextToPrint>,
                                    nest_shift : &mut u32,
                                    yshift : &mut u32) -> ([usize;2],u32) {
    // draw content and gather data
    let mut unshift:u32 = 0;
    *nest_shift += 1;
    let start_y : u32 = *yshift;
    *yshift += 3;
    unshift += 3;
    let lr_bounds  = draw_interaction_rec(image,  gen_ctx,i1, lf_x_widths,  lf_num,nest_shift, yshift);
    *yshift += 1;
    unshift += lr_bounds.1 + 1;
    let end_y : u32 = *yshift;
    *nest_shift -= 1;
    // draw frame
    let mut y_drafts : Vec<u32> = [start_y,end_y].to_vec();
    draw_combined_fragment_frame(image, label, *nest_shift,lf_x_widths,lr_bounds.0[0],lr_bounds.0[1],y_drafts);
    return (lr_bounds.0,unshift);
}

fn draw_n_ary_combined_fragment(  image : &mut RgbImage,
                                  gen_ctx : &GeneralContext,
                                  sub_ints : Vec<&Interaction>,
                                  lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                                  lf_num : usize,
                                  label : Vec<TextToPrint>,
                                  nest_shift : &mut u32,
                                  yshift : &mut u32) -> ([usize;2],u32) {
    let mut y_drafts : Vec<u32> = Vec::new();
    // draw content and gather data
    let mut unshift:u32 = 0;
    *nest_shift += 1;
    y_drafts.push(*yshift);
    *yshift += 2;
    unshift += 2;
    //
    let mut min_lf_id : usize = gen_ctx.get_lf_num();
    let mut max_lf_id : usize = 0;
    for my_int in sub_ints {
        *yshift += 1;
        let lr_bounds = draw_interaction_rec(image,  gen_ctx,my_int, lf_x_widths,  lf_num,nest_shift, yshift);
        min_lf_id = cmp::min( min_lf_id, lr_bounds.0[0]);
        max_lf_id = cmp::max( max_lf_id, lr_bounds.0[1]);
        *yshift += 1;
        unshift += lr_bounds.1 + 1;
        y_drafts.push(*yshift);
    }
    *nest_shift -= 1;
    //
    let lr_bounds: [usize;2] = [ min_lf_id, max_lf_id ];
    // draw frame
    draw_combined_fragment_frame(image,label,*nest_shift,lf_x_widths,lr_bounds[0],lr_bounds[1],y_drafts);
    return (lr_bounds,unshift);
}


fn draw_combined_fragment_frame(    image : &mut RgbImage,
                                    label : Vec<TextToPrint>,
                                    nest_shift : u32,
                                    lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                                    left_bound : usize,
                                    right_bound : usize,
                                    y_drafts : Vec<u32>) {
    match (lf_x_widths.get(&left_bound), lf_x_widths.get(&right_bound)) {
        (Some(left_lf_coords),Some(right_lf_coords)) => {
            let x_left : f32 = left_lf_coords.x_start + (nest_shift as f32)*FRAGMENT_PADDING;
            let x_right : f32 = (right_lf_coords.x_start + right_lf_coords.x_span_outer) - (nest_shift as f32)*FRAGMENT_PADDING;

            let mut y_coords : Vec<f32> = y_drafts.into_iter().map(|y| get_y_pos_from_yshift(y) ).collect::< Vec<f32> >();
            let y_start : f32 = y_coords.remove(0);
            let y_end : f32 = y_coords.pop().unwrap();// - (nest_shift as f32)*FRAGMENT_PADDING;
            draw_line_segment_mut(image,
                                  (x_left, y_start),
                                  (x_left, y_end),
                                  Rgb(HCP_Black));
            draw_line_segment_mut(image,
                                  (x_right, y_start),
                                  (x_right, y_end),
                                  Rgb(HCP_Black));
            draw_line_segment_mut(image,
                                  (x_left, y_start),
                                  (x_right, y_start),
                                  Rgb(HCP_Black));
            draw_line_segment_mut(image,
                                  (x_left, y_end),
                                  (x_right, y_end),
                                  Rgb(HCP_Black));
            for y_coord in y_coords {
                draw_line_segment_mut(image,
                                      (x_left, y_coord),
                                      (x_right, y_coord),
                                      Rgb(HCP_Black));
            }
            draw_line_of_colored_text(image,
                                      &DrawCoord::StartingAt(x_left + FRAGMENT_TITLE_MARGIN),
                                      &DrawCoord::CenteredAround(y_start + VERTICAL_SIZE+ FRAGMENT_TITLE_MARGIN),
                                      &label,
                                      &get_font(),
                                      &HIBOU_FONT_SCALE);
        },
        _ => {}
    }
}







