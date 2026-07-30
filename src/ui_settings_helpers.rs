use crate::*;

pub(crate) fn model_picker(
    app: &mut ErrorExplainerApp,
    ui: &mut egui::Ui,
    context: &egui::Context,
) {
    ui.label("Model");
    if app.available_models.is_empty() {
        ui.add_sized(
            [ui.available_width(), 30.0],
            TextEdit::singleline(&mut app.settings.model).hint_text("Model identifier"),
        );
    } else {
        egui::ComboBox::from_id_source("model_list")
            .selected_text(&app.settings.model)
            .show_ui(ui, |ui| {
                for model in &app.available_models {
                    ui.selectable_value(&mut app.settings.model, model.clone(), model);
                }
            });
    }
    if ui
        .add_enabled(
            !app.loading_models,
            egui::Button::new(if app.loading_models {
                "Loading models…"
            } else {
                "Load models"
            }),
        )
        .clicked()
    {
        app.load_models(context);
    }
}
