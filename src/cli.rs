use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "flatctl",
    author = "Flatctl Team",
    version = "0.1.0",
    about = "Herramienta CLI para gestionar paquetes Flatpak",
    long_about = "flatctl es una herramienta CLI para buscar, instalar, actualizar, desinstalar y realizar mantenimiento a paquetes Flatpak en Linux."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Buscar aplicaciones en los repositorios configurados (ej. Flathub)
    Search {
        /// Término de búsqueda
        query: String,
        /// Instalar interactivamente desde los resultados encontrados
        #[arg(short, long)]
        install: bool,
    },
    /// Listar aplicaciones o runtimes instalados
    List {
        /// Filtrar por tipo de paquete
        #[arg(short, long, value_enum, default_value_t = ListFilter::App)]
        target: ListFilter,
    },
    /// Instalar uno o más paquetes Flatpak
    Install {
        /// IDs de las aplicaciones a instalar (ej. org.mozilla.firefox)
        #[arg(required = true)]
        ids: Vec<String>,
    },
    /// Desinstalar aplicaciones
    Uninstall {
        /// IDs de las aplicaciones a desinstalar (si se omite, muestra lista interactiva)
        ids: Vec<String>,
    },
    /// Actualizar todos los paquetes Flatpak del sistema
    Update,
    /// Mantenimiento del sistema (reparar, limpiar paquetes huérfanos y datos residuales)
    Maintenance {
        /// Modo de mantenimiento (safe o destructive)
        #[arg(short, long, value_enum)]
        mode: Option<MaintenanceMode>,
    },
    /// Abrir menú interactivo en terminal
    Interactive,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ListFilter {
    /// Solo aplicaciones instaladas
    App,
    /// Solo runtimes instalados
    Runtime,
    /// Aplicaciones y runtimes instalados
    Both,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum MaintenanceMode {
    /// Seguro: reparar y eliminar paquetes en desuso (--unused)
    Safe,
    /// Completo: seguro + eliminar datos residuales (--delete-data)
    Destructive,
}
