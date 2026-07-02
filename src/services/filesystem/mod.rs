use std::path::Path;
use std::process::Command;
use std::{fs, io};

use crate::models::FsEntry;

pub fn list_dir(path: &Path) -> std::io::Result<Vec<FsEntry>> {
    let entries = fs::read_dir(path)?;
    let dir_entries: Vec<fs::DirEntry> = entries.collect::<std::io::Result<Vec<_>>>()?;

    let mut fs_entries: Vec<FsEntry> = dir_entries
        .iter()
        .map(|entry| {
            let path = entry.path();
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.to_string_lossy().into_owned());
            let is_dir = path.is_dir();

            FsEntry { name, path, is_dir }
        })
        .collect();

    fs_entries.sort_by(|a, b| {
        let rank = |e: &FsEntry| if e.is_dir { 0u8 } else { 1u8 };
        rank(a)
            .cmp(&rank(b))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    Ok(fs_entries)
}

pub fn rename(old_path: &Path, new_name: &str) -> io::Result<()> {
    if new_name.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "el nombre no puede estar vacío",
        ));
    };

    if new_name.contains(['/', '\\']) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "el nombre no puede contener separadores de ruta: {:?}",
                new_name
            ),
        ));
    };

    let parent = old_path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "el path no tiene directorio padre - no se puede renombrar",
        )
    })?;
    let new_path = parent.join(new_name);

    if new_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("ya existe algo llamado {:?} en esta carpeta", new_name),
        ));
    };

    std::fs::rename(old_path, &new_path)
}

pub fn open_file(path: &Path) -> io::Result<()> {
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Path does not exist",
        ));
    }

    #[cfg(target_os = "windows")]
    {
        if path.is_dir() {
            if Command::new("cmd")
                .args(["/C", "code", &path.display().to_string()])
                .status()
                .is_ok_and(|s| s.success())
            {
                return Ok(());
            }
        }

        Command::new("cmd")
            .args(["/C", "start", "", &path.display().to_string()])
            .spawn()?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(path).spawn()?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open").arg(path).spawn()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_rename_success() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let old_path = dir.path().join("old.txt");

        std::fs::write(&old_path, b"contenido de prueba")?;
        rename(&old_path, "new.txt")?;

        assert!(
            dir.path().join("new.txt").exists(),
            "al archivo con el nombre nuevo no existe en el directorio"
        );

        assert!(
            !old_path.exists(),
            "el archivo viejo todavía existe, rename debería haberlo movido"
        );

        Ok(())
    }

    #[test]
    fn test_rename_empty_name() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let old_path = dir.path().join("file.txt");
        std::fs::write(&old_path, b"x")?;

        let result = rename(&old_path, "");

        assert!(result.is_err(), "nombre vacío tiene que devolver error");

        Ok(())
    }

    #[test]
    fn test_rename_name_with_separator() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let old_path = dir.path().join("file.txt");
        std::fs::write(&old_path, b"x")?;

        let invalid_names = ["sub/other.txt", r"sub\other.txt", "../outside.txt"];

        for name in &invalid_names {
            let result = rename(&old_path, name);
            assert!(
                result.is_err(),
                "se esperaba error para el nombre {:?} pero rename devlovió OK",
                name
            );
        }

        assert!(
            old_path.exists(),
            "el archivo original desapareció tras intentos de rename fallidos"
        );

        Ok(())
    }

    #[test]
    fn test_rename_existing_name() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let origin = dir.path().join("origin.txt");
        let dest = dir.path().join("destination.txt");

        std::fs::write(&origin, b"contenido del origen")?;
        std::fs::write(&dest, b"contenido destino")?;

        let restult = rename(&origin, "destination.txt");

        assert!(
            restult.is_err(),
            "rename a un nombre existente tiene que devolver error"
        );

        let content = std::fs::read_to_string(&dest)?;
        assert_eq!(
            content, "contenido destino",
            "el contenido del destino cambió - rename pisó un archivo existente"
        );

        Ok(())
    }

    #[test]
    fn test_list_dir_devuelve_entradas() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;

        std::fs::write(dir.path().join("archivo.txt"), b"x")?;
        std::fs::create_dir(dir.path().join("subcarpeta"))?;

        let entradas = list_dir(dir.path())?;

        assert_eq!(
            entradas.len(),
            2,
            "se esperaban 2 entradas, se obtuvieron {}",
            entradas.len()
        );

        Ok(())
    }

    #[test]
    fn test_list_dir_distingue_tipo() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        std::fs::write(dir.path().join("un_archivo.txt"), b"x")?;
        std::fs::create_dir(dir.path().join("una_carpeta"))?;

        let entradas = list_dir(dir.path())?;

        assert_eq!(entradas.len(), 2);
        assert!(
            entradas[0].is_dir,
            "la primera entrada debería ser la carpeta (is_dir = true)"
        );
        assert!(
            !entradas[1].is_dir,
            "la segunda entrada debería ser el archivo (is_dir = false)"
        );

        assert_eq!(entradas[0].name, "una_carpeta");
        assert_eq!(entradas[1].name, "un_archivo.txt");

        Ok(())
    }

    #[test]
    fn test_list_dir_carpetas_primero() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;

        std::fs::write(dir.path().join("a_archivo.txt"), b"x")?;
        std::fs::write(dir.path().join("b_archivo.txt"), b"x")?;
        std::fs::create_dir(dir.path().join("z_carpeta"))?;

        let entradas = list_dir(dir.path())?;

        assert_eq!(entradas.len(), 3);

        assert!(
            entradas[0].is_dir,
            "la carpeta tiene que ir primero, incluso si su nombre va después alfabéticamente. \
             Primera entrada: {:?} (is_dir: {})",
            entradas[0].name, entradas[0].is_dir
        );

        assert!(!entradas[1].is_dir);
        assert!(!entradas[2].is_dir);

        Ok(())
    }

    #[test]
    fn test_list_dir_orden_alfabetico_case_insensitive() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;

        std::fs::write(dir.path().join("Gamma.txt"), b"x")?;
        std::fs::write(dir.path().join("alpha.txt"), b"x")?;
        std::fs::write(dir.path().join("Beta.txt"), b"x")?;
        std::fs::write(dir.path().join("delta.txt"), b"x")?;

        let entradas = list_dir(dir.path())?;

        assert_eq!(entradas.len(), 4);

        assert_eq!(entradas[0].name, "alpha.txt");
        assert_eq!(entradas[1].name, "Beta.txt");
        assert_eq!(entradas[2].name, "delta.txt");
        assert_eq!(entradas[3].name, "Gamma.txt");

        Ok(())
    }
}
