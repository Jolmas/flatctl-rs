mod cli;
mod flatpak;
mod interactive;
mod ui;

use clap::Parser;
use cli::{Cli, Commands, ListFilter, MaintenanceMode};
use colored::Colorize;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Search { query, install }) => {
            println!("Buscando '{}'...", query);
            let results = flatpak::search(&query)?;
            if results.is_empty() {
                println!("{}", "No se encontraron paquetes para esa búsqueda.".yellow());
            } else {
                ui::print_table(&results, &format!("Resultados de búsqueda para '{}'", query));
                if install {
                    let to_install = ui::select_items_to_install(&results)?;
                    if !to_install.is_empty() {
                        flatpak::install(&to_install)?;
                        println!("{}", "✓ Instalación completada con éxito.".green().bold());
                    }
                }
            }
        }
        Some(Commands::List { target }) => {
            let label = match target {
                ListFilter::App => "Aplicaciones instaladas",
                ListFilter::Runtime => "Runtimes instalados",
                ListFilter::Both => "Paquetes instalados (Aplicaciones y Runtimes)",
            };
            let items = flatpak::list(target)?;
            ui::print_table(&items, label);
        }
        Some(Commands::Install { ids }) => {
            flatpak::install(&ids)?;
            println!("{}", "✓ Instalación completada con éxito.".green().bold());
        }
        Some(Commands::Uninstall { ids }) => {
            let target_ids = if ids.is_empty() {
                println!("Obteniendo lista de aplicaciones instaladas...");
                let installed = flatpak::list(ListFilter::App)?;
                ui::select_items_to_uninstall(&installed)?
            } else {
                ids
            };

            if target_ids.is_empty() {
                println!("{}", "No se seleccionó ninguna aplicación para desinstalar.".yellow());
            } else {
                flatpak::uninstall(&target_ids)?;
                println!("{}", "✓ Desinstalación completada con éxito.".green().bold());
            }
        }
        Some(Commands::Update) => {
            println!("Buscando y aplicando actualizaciones del sistema...");
            flatpak::update()?;
            println!("{}", "✓ Sistema actualizado con éxito.".green().bold());
        }
        Some(Commands::Maintenance { mode }) => {
            let destructive = match mode {
                Some(MaintenanceMode::Destructive) => true,
                Some(MaintenanceMode::Safe) => false,
                None => match ui::prompt_maintenance_mode()? {
                    Some(val) => val,
                    None => {
                        println!("{}", "Operación de mantenimiento cancelada.".yellow());
                        return Ok(());
                    }
                },
            };

            flatpak::repair()?;
            flatpak::remove_unused()?;
            if destructive {
                flatpak::delete_data()?;
            }
            println!("{}", "✓ Mantenimiento finalizado con éxito.".green().bold());
        }
        Some(Commands::Interactive) | None => {
            interactive::run_menu()?;
        }
    }

    Ok(())
}
