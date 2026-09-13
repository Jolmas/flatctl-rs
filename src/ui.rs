use crate::flatpak::FlatpakItem;
use anyhow::Result;
use colored::Colorize;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use inquire::{Confirm, MultiSelect, Select, Text};

pub fn print_table(items: &[FlatpakItem], title: &str) {
    if items.is_empty() {
        println!("{}", "No se encontraron elementos.".yellow());
        return;
    }

    println!("\n{}", format!("=== {} (Total: {}) ===", title, items.len()).cyan().bold());

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("ID").add_attribute(Attribute::Bold).fg(Color::Cyan),
            Cell::new("Nombre").add_attribute(Attribute::Bold).fg(Color::Green),
            Cell::new("Versión").fg(Color::Yellow),
            Cell::new("Rama").fg(Color::Magenta),
            Cell::new("Descripción"),
        ]);

    for item in items {
        table.add_row(vec![
            Cell::new(&item.id),
            Cell::new(&item.name),
            Cell::new(&item.version),
            Cell::new(&item.branch),
            Cell::new(&item.description),
        ]);
    }

    println!("{table}\n");
}

pub fn select_items_to_uninstall(items: &[FlatpakItem]) -> Result<Vec<String>> {
    if items.is_empty() {
        println!("{}", "No hay aplicaciones instaladas disponibles para desinstalar.".yellow());
        return Ok(Vec::new());
    }

    let options: Vec<String> = items
        .iter()
        .map(|i| format!("{} | {} ({})", i.id, i.name, i.version))
        .collect();

    let ans = MultiSelect::new(
        "Selecciona las aplicaciones a desinstalar (Espacio para marcar, Enter para confirmar):",
        options,
    )
    .with_page_size(15)
    .prompt()?;

    if ans.is_empty() {
        return Ok(Vec::new());
    }

    let confirm = Confirm::new(&format!(
        "¿Estás seguro de que deseas desinstalar {} paquete(s)?",
        ans.len()
    ))
    .with_default(false)
    .prompt()?;

    if !confirm {
        println!("{}", "Operación cancelada.".yellow());
        return Ok(Vec::new());
    }

    let selected_ids = items
        .iter()
        .filter(|i| ans.iter().any(|choice| choice.starts_with(&i.id)))
        .map(|i| i.id.clone())
        .collect();

    Ok(selected_ids)
}

pub fn select_items_to_install(items: &[FlatpakItem]) -> Result<Vec<String>> {
    if items.is_empty() {
        return Ok(Vec::new());
    }

    let options: Vec<String> = items
        .iter()
        .map(|i| format!("{} | {} ({})", i.id, i.name, i.version))
        .collect();

    let ans = MultiSelect::new(
        "Selecciona las aplicaciones que deseas instalar:",
        options,
    )
    .with_page_size(15)
    .prompt()?;

    let selected_ids = items
        .iter()
        .filter(|i| ans.iter().any(|choice| choice.starts_with(&i.id)))
        .map(|i| i.id.clone())
        .collect();

    Ok(selected_ids)
}

pub fn prompt_search_term() -> Result<Option<String>> {
    let term = Text::new("Ingresa el término o nombre de aplicación a buscar:")
        .prompt()?;
    let trimmed = term.trim().to_string();
    if trimmed.is_empty() {
        Ok(None)
    } else {
        Ok(Some(trimmed))
    }
}

pub fn prompt_maintenance_mode() -> Result<Option<bool>> {
    let options = vec![
        "Mantenimiento Seguro (reparar y eliminar paquetes no utilizados)",
        "Mantenimiento Completo (seguro + eliminar datos residuales de apps desinstaladas)",
        "Cancelar",
    ];

    let choice = Select::new("Selecciona el modo de mantenimiento:", options).prompt()?;

    if choice.starts_with("Mantenimiento Seguro") {
        Ok(Some(false))
    } else if choice.starts_with("Mantenimiento Completo") {
        let confirm = Confirm::new(
            "El modo completo eliminará los datos residuales de aplicaciones desinstaladas. ¿Continuar?",
        )
        .with_default(false)
        .prompt()?;

        if confirm {
            Ok(Some(true))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
