use super::{
    egui, egui::RichText, update_input_key_state, PreviewFilterType, VSPreviewer, STATE_LABEL_COLOR,
};

use crate::vs_handler::{VSDitherAlgo, VSResizer};

pub struct UiPreferences {}

impl UiPreferences {
    pub fn ui(pv: &mut VSPreviewer, ui: &mut egui::Ui) {
        let old_vs_resizer = pv.state.frame_transform_opts.resizer;
        let old_enable_dithering = pv.state.frame_transform_opts.enable_dithering;
        let old_dither_algo = pv.state.frame_transform_opts.dither_algo;

        let old_upscale_flag = pv.state.upscale_to_window;
        let old_upsampling_filter = pv.state.upsampling_filter;
        let old_fit_window_flag = pv.state.fit_to_window;

        let header = RichText::new("Preferences").color(STATE_LABEL_COLOR);

        egui::CollapsingHeader::new(header).show(ui, |ui| {
            egui::Grid::new("prefs_grid")
                .num_columns(2)
                .spacing([8.0, 4.0])
                .show(ui, |ui| {
                    let new_vs_resizer = &mut pv.state.frame_transform_opts.resizer;

                    ui.label(RichText::new("Resizer (chroma)").color(STATE_LABEL_COLOR));
                    egui::ComboBox::from_id_source(egui::Id::new("vs_resizer_select"))
                        .selected_text(new_vs_resizer.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(new_vs_resizer, VSResizer::Bilinear, "Bilinear");
                            ui.selectable_value(new_vs_resizer, VSResizer::Bicubic, "Bicubic");
                            ui.selectable_value(new_vs_resizer, VSResizer::Point, "Point");
                            ui.selectable_value(new_vs_resizer, VSResizer::Lanczos, "Lanczos");
                            ui.selectable_value(new_vs_resizer, VSResizer::Spline16, "Spline16");
                            ui.selectable_value(new_vs_resizer, VSResizer::Spline36, "Spline36");
                            ui.selectable_value(new_vs_resizer, VSResizer::Spline64, "Spline64");
                        });
                    ui.end_row();

                    let new_enable_dithering = &mut pv.state.frame_transform_opts.enable_dithering;

                    ui.checkbox(new_enable_dithering, "Enable dithering");
                    if *new_enable_dithering {
                        let new_dither_algo = &mut pv.state.frame_transform_opts.dither_algo;

                        egui::ComboBox::from_id_source(egui::Id::new("vs_dither_algo_select"))
                            .selected_text(new_dither_algo.to_string())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(new_dither_algo, VSDitherAlgo::None, "None");
                                ui.selectable_value(
                                    new_dither_algo,
                                    VSDitherAlgo::Ordered,
                                    "Ordered",
                                );
                                ui.selectable_value(
                                    new_dither_algo,
                                    VSDitherAlgo::Random,
                                    "Random",
                                );
                                ui.selectable_value(
                                    new_dither_algo,
                                    VSDitherAlgo::ErrorDiffusion,
                                    "Error Diffusion",
                                );
                            });
                    }
                    ui.end_row();

                    ui.checkbox(&mut pv.state.upscale_to_window, "Upscale image to window");
                    ui.checkbox(&mut pv.state.fit_to_window, "Fit image to window");
                    ui.end_row();

                    if pv.state.upscale_to_window {
                        let new_upsampling_filter = &mut pv.state.upsampling_filter;

                        ui.label(RichText::new("Upsampling filter").color(STATE_LABEL_COLOR));
                        egui::ComboBox::from_id_source(egui::Id::new("upsampling_filter_select"))
                            .selected_text(new_upsampling_filter.to_string())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    new_upsampling_filter,
                                    PreviewFilterType::Gpu,
                                    "GPU",
                                );
                                ui.selectable_value(
                                    new_upsampling_filter,
                                    PreviewFilterType::Point,
                                    "Point",
                                );
                                ui.selectable_value(
                                    new_upsampling_filter,
                                    PreviewFilterType::Bilinear,
                                    "Bilinear",
                                );
                                ui.selectable_value(
                                    new_upsampling_filter,
                                    PreviewFilterType::Hamming,
                                    "Hamming",
                                );
                                ui.selectable_value(
                                    new_upsampling_filter,
                                    PreviewFilterType::CatmullRom,
                                    "CatmullRom",
                                );
                                ui.selectable_value(
                                    new_upsampling_filter,
                                    PreviewFilterType::Mitchell,
                                    "Mitchell",
                                );
                                ui.selectable_value(
                                    new_upsampling_filter,
                                    PreviewFilterType::Lanczos3,
                                    "Lanczos3",
                                );
                            });
                        ui.end_row();
                    }

                    let zoom_mult_dragval = egui::DragValue::new(&mut pv.state.zoom_multiplier)
                        .speed(0.01)
                        .clamp_range(1.0..=2.0)
                        .max_decimals(2);
                    ui.label(RichText::new("Zoom multiplier").color(STATE_LABEL_COLOR));
                    let res = ui.add(zoom_mult_dragval);
                    ui.end_row();

                    let in_use = res.has_focus() || res.drag_started();
                    update_input_key_state(
                        &mut pv.inputs_focused,
                        "zoom_mult_dragval",
                        in_use,
                        &res,
                    );

                    let scroll_mult_dragval = egui::DragValue::new(&mut pv.state.scroll_multiplier)
                        .speed(0.01)
                        .clamp_range(0.5..=4.0)
                        .max_decimals(2);
                    ui.label(RichText::new("Scroll multiplier").color(STATE_LABEL_COLOR));
                    let res = ui.add(scroll_mult_dragval);
                    ui.end_row();

                    let in_use = res.has_focus() || res.drag_started();
                    update_input_key_state(
                        &mut pv.inputs_focused,
                        "scroll_mult_dragval",
                        in_use,
                        &res,
                    );

                    let canvas_margin_dragval = egui::DragValue::new(&mut pv.state.canvas_margin)
                        .speed(1)
                        .clamp_range(0.0..=100.0)
                        .max_decimals(0);
                    ui.label(RichText::new("Canvas margin").color(STATE_LABEL_COLOR));
                    let res = ui.add(canvas_margin_dragval);
                    ui.end_row();

                    let in_use = res.has_focus() || res.drag_started();
                    let lost_focus = update_input_key_state(
                        &mut pv.inputs_focused,
                        "canvas_margin_dragval",
                        in_use,
                        &res,
                    );

                    if lost_focus {
                        pv.reprocess_outputs(false);
                    }
                });
        });

        let ft = pv.state.frame_transform_opts;

        // VS Processing setting changed
        if ft.resizer != old_vs_resizer
            || ft.enable_dithering != old_enable_dithering
            || ft.dither_algo != old_dither_algo
        {
            pv.rerender = true;
        } else if pv.state.upscale_to_window != old_upscale_flag
            || pv.state.upsampling_filter != old_upsampling_filter
            || pv.state.fit_to_window != old_fit_window_flag
        {
            pv.reprocess_outputs(false);
        }
    }
}