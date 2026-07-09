use systemstat::{Platform, System as StatSystem};

pub fn get_max_temperature(stat_sys: &StatSystem, ia_name: &str) -> f32 {
    // Variable statique pour limiter les messages d'erreur
    static mut ERROR_MSG_COUNT: u32 = 0;
    static mut LAST_ERROR_TIME: u64 = 0;

    let temp_result = stat_sys.cpu_temp();

    match temp_result {
        Ok(temp) => {
            println!("[{}] Température CPU réelle: {:.1}°C", ia_name, temp);
            // Réinitialiser le compteur d'erreurs
            unsafe {
                ERROR_MSG_COUNT = 0;
            }
            temp
        }
        Err(e) => {
            // Limiter la fréquence des messages d'erreur
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            let should_print = unsafe {
                if now - LAST_ERROR_TIME > 60 || ERROR_MSG_COUNT < 1 {
                    ERROR_MSG_COUNT += 1;
                    LAST_ERROR_TIME = now;
                    true
                } else {
                    false
                }
            };

            if should_print {
                println!(
                    "[{}] Impossible de lire la température CPU: {} (messages limités)",
                    ia_name, e
                );
            }

            #[cfg(target_os = "linux")]
            {
                if let Ok(temps) = stat_sys.temperatures() {
                    let max_temp = temps
                        .iter()
                        .map(|(name, temp)| {
                            let celsius = temp.current.celsius();
                            if celsius > 50.0 {
                                println!("[{}] Capteur {}: {:.1}°C", ia_name, name, celsius);
                            }
                            celsius
                        })
                        .fold(0.0, f32::max);

                    if max_temp > 60.0 {
                        println!("[{}] Température maximale : {:.1}°C", ia_name, max_temp);
                    }

                    return max_temp;
                }
            }

            #[cfg(windows)]
            {
                // Méthode alternative sous Windows: OpenHardwareMonitorLib ou WMI
                let result = super::windows::try_get_temperature_windows();
                if let Some(temp) = result {
                    println!("[{}] Température CPU Windows: {:.1}°C", ia_name, temp);
                    return temp;
                } else {
                    // Fallback: CPU usage comme indicateur indirect
                    let cpu_usage = super::windows::check_cpu_usage_as_temp_indicator();
                    if cpu_usage > 80.0 {
                        println!(
                            "[{}] AVERTISSEMENT: Charge CPU élevée ({:.1}%) pendant une période prolongée - risque de surchauffe",
                            ia_name, cpu_usage
                        );
                        return 75.0; // Simuler une température élevée pour déclencher le refroidissement
                    }
                }
            }

            // Protection proactive en surveillant l'utilisation CPU sous Windows
            #[cfg(windows)]
            {
                return super::windows::get_temperature_from_cpu_usage(ia_name);
            }

            // Valeur par défaut si aucune méthode ne fonctionne - utiliser une valeur sûre
            40.0 // Valeur par défaut plus basse pour garantir la sécurité
        }
    }
}
