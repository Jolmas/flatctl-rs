# Flatctl (Rust CLI)

Versión CLI en Rust de **`flatctl`**, diseñada para gestionar aplicaciones y runtimes Flatpak de forma rápida, eficiente e interactiva en la terminal.

## Características

- 🔍 **Búsqueda e instalación interactiva:** Busca paquetes en Flathub e instálalos directamente.
- 📋 **Listado formateado:** Muestra aplicaciones y runtimes instalados en tablas limpias y adaptativas.
- 🗑️ **Desinstalación guiada:** Selección múltiple mediante checklist en la terminal para desinstalar uno o varios paquetes.
- 🔄 **Actualizaciones:** Actualiza todos los Flatpaks con un solo comando.
- 🛠️ **Mantenimiento del sistema:** Reparación del repositorio, limpieza de runtimes sin uso (`--unused`) y purga de datos residuales (`--delete-data`).
- 🧭 **Modo menú interactivo:** Interfaz guiada por menús cuando se ejecuta sin argumentos.

---

## Compilación e Instalación

### Requisitos
- Rust y Cargo (1.70+)
- Flatpak instalado en el sistema

### Compilar en modo Release
```bash
cargo build --release
```
El ejecutable se generará en `target/release/flatctl`.

### Instalar en el sistema
Para usar `flatctl` globalmente:
```bash
cargo install --path .
```
O copiar el binario a tu ruta local (por ejemplo `~/.local/bin`):
```bash
cp target/release/flatctl ~/.local/bin/
```

---

## Uso de la CLI

### 1. Modo Interactivo (Menú principal)
Si ejecutas `flatctl` sin argumentos o con el comando `interactive`:
```bash
flatctl
# o bien
flatctl interactive
```

### 2. Buscar aplicaciones
```bash
# Búsqueda simple
flatctl search firefox

# Búsqueda con selección interactiva para instalar
flatctl search vlc --install
```

### 3. Listar paquetes instalados
```bash
# Solo aplicaciones (por defecto)
flatctl list

# Solo runtimes
flatctl list --target runtime

# Aplicaciones y runtimes
flatctl list --target both
```

### 4. Instalar aplicaciones
```bash
flatctl install org.mozilla.firefox com.spotify.Client
```

### 5. Desinstalar aplicaciones
```bash
# Modo interactivo (muestra lista con casillas de selección)
flatctl uninstall

# Desinstalación directa por ID
flatctl uninstall com.github.Flacon
```

### 6. Actualizar el sistema
```bash
flatctl update
```

### 7. Mantenimiento del sistema
```bash
# Mantenimiento seguro (reparar + eliminar runtimes no utilizados)
flatctl maintenance --mode safe

# Mantenimiento completo (incluye borrado de datos residuales)
flatctl maintenance --mode destructive

# Preguntar interactivamente qué modo ejecutar
flatctl maintenance
```
