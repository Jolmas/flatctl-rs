use crate::cli::ListFilter;
use crate::{flatpak, ui};
use anyhow::Result;
use colored::Colorize;
use inquire::Select;

pub fn run_menu() -> Result<()> {
    println!("{}", "\n╔═══════════════════════════════════════╗".cyan().bold());
    println!("{}", "║        Flatctl - Gestor Flatpak       ║".cyan().bold());
    println!("{}", "╚═══════════════════════════════════════╝".cyan().bold());

    loop {
        let menu_options = vec![
            "🔍 Buscar aplicaciones en Flathub",
            "📋 Listar paquetes instalados",
            "🔄 Actualizar todos los paquetes",
            "🗑️  Desinstalar aplicaciones",
            "🛠️  Mantenimiento del sistema",
            "ℹ️  Acerca de Flatctl",
            "🚪 Salir",
        ];

        let choice = Select::new("\n¿Qué deseas hacer?", menu_options).prompt();

        match choice {
            Ok("🔍 Buscar aplicaciones en Flathub") => {
                if let Some(term) = ui::prompt_search_term()? {
                    println!("Buscando '{}'...", term);
                    let results = flatpak::search(&term)?;
                    if results.is_empty() {
                        println!("{}", "No se encontraron resultados.".yellow());
                    } else {
                        ui::print_table(&results, &format!("Resultados para '{}'", term));
                        let to_install = ui::select_items_to_install(&results)?;
                        if !to_install.is_empty() {
                            flatpak::install(&to_install)?;
                            println!("{}", "✓ Instalación finalizada con éxito.".green().bold());
                        }
                    }
                }
            }
            Ok("📋 Listar paquetes instalados") => {
                let target_options = vec![
                    "Solo aplicaciones",
                    "Solo runtimes",
                    "Ambos (aplicaciones y runtimes)",
                ];
                let target = Select::new("¿Qué deseas listar?", target_options).prompt()?;
                let (filter, label) = match target {
                    "Solo runtimes" => (ListFilter::Runtime, "Runtimes instalados"),
                    "Ambos (aplicaciones y runtimes)" => (ListFilter::Both, "Todos los paquetes instalados"),
                    _ => (ListFilter::App, "Aplicaciones instaladas"),
                };

                let items = flatpak::list(filter)?;
                ui::print_table(&items, label);
            }
            Ok("🔄 Actualizar todos los paquetes") => {
                println!("Comprobando y aplicando actualizaciones...");
                flatpak::update()?;
                println!("{}", "✓ Actualizaciones completadas.".green().bold());
            }
            Ok("🗑️  Desinstalar aplicaciones") => {
                println!("Obteniendo lista de aplicaciones instaladas...");
                let items = flatpak::list(ListFilter::App)?;
                let to_uninstall = ui::select_items_to_uninstall(&items)?;
                if !to_uninstall.is_empty() {
                    flatpak::uninstall(&to_uninstall)?;
                    println!("{}", "✓ Desinstalación completada con éxito.".green().bold());
                }
            }
            Ok("🛠️  Mantenimiento del sistema") => {
                if let Some(destructive) = ui::prompt_maintenance_mode()? {
                    flatpak::repair()?;
                    flatpak::remove_unused()?;
                    if destructive {
                        flatpak::delete_data()?;
                    }
                    println!("{}", "✓ Mantenimiento finalizado con éxito.".green().bold());
                }
            }
            Ok("ℹ️  Acerca de Flatctl") => {
                println!("\n{}", "Flatctl (Rust Edition)".cyan().bold());
                println!("Herramienta CLI para la administración ágil de paquetes Flatpak.");
                println!("Basado en el proyecto original en Bash de Pedro Fernandes.");
                println!("Versión: 0.1.0\n");
            }
            Ok("🚪 Salir") | Err(_) => {
                println!("{}", "¡Hasta luego!".cyan());
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
