use crate::*;

pub(crate) fn custom_protocol_picker(app: &mut ErrorExplainerApp, ui: &mut egui::Ui) {
    if app.settings.provider != ProviderKind::CustomApi {
        return;
    }
    let old_protocol = app.settings.custom_protocol;
    egui::ComboBox::from_label("API protocol")
        .selected_text(app.settings.custom_protocol.label())
        .show_ui(ui, |ui| {
            for protocol in ApiProtocol::ALL {
                ui.selectable_value(
                    &mut app.settings.custom_protocol,
                    protocol,
                    protocol.label(),
                );
            }
        });
    if app.settings.custom_protocol != old_protocol {
        app.available_models.clear();
        app.settings.model.clear();
    }
    ui.label("Enter the provider's API root URL and exact model ID.");
}

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
