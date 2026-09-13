use crate::cli::ListFilter;
use anyhow::{bail, Context, Result};
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct FlatpakItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub branch: String,
}

pub fn search(term: &str) -> Result<Vec<FlatpakItem>> {
    let output = Command::new("flatpak")
        .args([
            "search",
            term,
            "--columns=application,name,description,version,branch",
        ])
        .output()
        .context("No se pudo ejecutar 'flatpak search'. Asegúrate de que flatpak esté instalado.")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_tsv_output(&stdout)
}

pub fn list(filter: ListFilter) -> Result<Vec<FlatpakItem>> {
    let mut cmd = Command::new("flatpak");
    cmd.args(["list", "--system"]);

    match filter {
        ListFilter::App => {
            cmd.arg("--app");
        }
        ListFilter::Runtime => {
            cmd.arg("--runtime");
        }
        ListFilter::Both => {}
    }

    cmd.arg("--columns=application,name,description,version,branch");

    let output = cmd
        .output()
        .context("No se pudo ejecutar 'flatpak list'.")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_tsv_output(&stdout)
}

pub fn install(ids: &[String]) -> Result<()> {
    if ids.is_empty() {
        bail!("No se especificó ninguna aplicación para instalar.");
    }

    let mut cmd = Command::new("flatpak");
    cmd.args(["install", "--system", "--assumeyes", "flathub"])
        .args(ids)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = cmd.status().context("Error al ejecutar 'flatpak install'")?;
    if !status.success() {
        bail!("La instalación falló con código: {:?}", status.code());
    }

    Ok(())
}

pub fn uninstall(ids: &[String]) -> Result<()> {
    if ids.is_empty() {
        bail!("No se especificó ninguna aplicación para desinstalar.");
    }

    let mut cmd = Command::new("flatpak");
    cmd.args(["uninstall", "--system", "--assumeyes"])
        .args(ids)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = cmd
        .status()
        .context("Error al ejecutar 'flatpak uninstall'")?;
    if !status.success() {
        bail!("La desinstalación falló con código: {:?}", status.code());
    }

    Ok(())
}

pub fn update() -> Result<()> {
    let mut cmd = Command::new("flatpak");
    cmd.args(["update", "--system", "--assumeyes"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = cmd.status().context("Error al ejecutar 'flatpak update'")?;
    if !status.success() {
        bail!("La actualización falló con código: {:?}", status.code());
    }

    Ok(())
}

pub fn repair() -> Result<()> {
    println!("-> Reparando instalación del sistema (requiere privilegios)...");
    let status = Command::new("pkexec")
        .args(["flatpak", "repair", "--system"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("✓ Reparación completada.");
        }
        Ok(s) => {
            println!("⚠ 'flatpak repair' finalizó con código: {:?} (puede haber sido cancelado).", s.code());
        }
        Err(e) => {
            println!("⚠ No se pudo invocar pkexec para reparar: {}", e);
        }
    }
    Ok(())
}

pub fn remove_unused() -> Result<()> {
    println!("-> Eliminando paquetes en desuso (runtimes huérfanos)...");
    let status = Command::new("flatpak")
        .args(["uninstall", "--system", "--unused", "--assumeyes"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Error al limpiar paquetes no utilizados")?;

    if !status.success() {
        println!("⚠ Limpieza de paquetes huérfanos finalizada con advertencias.");
    } else {
        println!("✓ Paquetes en desuso eliminados.");
    }
    Ok(())
}

pub fn delete_data() -> Result<()> {
    println!("-> Eliminando datos residuales de aplicaciones desinstaladas...");
    let status = Command::new("flatpak")
        .args(["uninstall", "--system", "--delete-data", "--assumeyes"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Error al eliminar datos de aplicaciones")?;

    if !status.success() {
        println!("⚠ Limpieza de datos residuales finalizada con advertencias.");
    } else {
        println!("✓ Datos residuales eliminados.");
    }
    Ok(())
}

fn parse_tsv_output(raw: &str) -> Result<Vec<FlatpakItem>> {
    let mut items = Vec::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let cols: Vec<&str> = trimmed.split('\t').collect();
        if cols.len() >= 5 {
            items.push(FlatpakItem {
                id: cols[0].trim().to_string(),
                name: cols[1].trim().to_string(),
                description: cols[2].trim().to_string(),
                version: cols[3].trim().to_string(),
                branch: cols[4].trim().to_string(),
            });
        }
    }
    Ok(items)
}
