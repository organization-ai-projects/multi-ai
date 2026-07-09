use std::process::Command;
use std::thread;
use std::time::Duration;

/// Essaie d'obtenir la température CPU sous Windows via WMI
pub fn try_get_temperature_windows() -> Option<f32> {
    // 1. Essayer avec la méthode spécifique aux CPU Intel sous Windows
    let intel_output = Command::new("powershell")
        .args(&[
            "-Command",
            "Get-WmiObject MSAcpi_ThermalZoneTemperature -Namespace \"root\\wmi\" | ForEach-Object { $_.CurrentTemperature / 10.0 - 273.15 }"
        ])
        .output()
        .ok()?;
    
    if intel_output.status.success() {
        let stdout = String::from_utf8_lossy(&intel_output.stdout);
        for line in stdout.lines() {
            if let Ok(temp) = line.trim().parse::<f32>() {
                return Some(temp);
            }
        }
    }
    
    // 2. Essayer avec la méthode spécifique aux cartes mères ASUS (WMI)
    let asus_output = Command::new("powershell")
        .args(&[
            "-Command",
            "Get-WmiObject -Namespace \"root\\wmi\" -Class \"ASUSHW_DLSC_SMC_Temperature\""
        ])
        .output()
        .ok()?;
        
    if asus_output.status.success() {
        let stdout = String::from_utf8_lossy(&asus_output.stdout);
        // Analyser la sortie pour extraire la température
        for line in stdout.lines() {
            if line.contains("Temperature") && line.contains(":") {
                if let Some(temp_str) = line.split(':').nth(1) {
                    if let Ok(temp) = temp_str.trim().parse::<f32>() {
                        return Some(temp);
                    }
                }
            }
        }
    }
    
    // 3. Essayer avec la méthode spécifique à Windows 11 (PowerShell)
    let win11_output = Command::new("powershell")
        .args(&[
            "-Command",
            "(Get-CimInstance -ClassName Win32_TemperatureProbe).CurrentReading"
        ])
        .output()
        .ok()?;
        
    if win11_output.status.success() {
        let stdout = String::from_utf8_lossy(&win11_output.stdout);
        if let Ok(temp) = stdout.trim().parse::<f32>() {
            return Some(temp);
        }
    }
    
    // Si toutes les méthodes spécifiques échouent, revenir à la méthode générique
    None
}

/// Utilise la charge CPU comme indicateur indirect de température
pub fn check_cpu_usage_as_temp_indicator() -> f32 {
    // Prendre plusieurs échantillons pour éviter les pics temporaires
    let mut samples = Vec::new();
    
    for _ in 0..3 {
        let output = Command::new("powershell")
            .args(&[
                "-Command",
                "(Get-Counter '\\Processor(_Total)\\% Processor Time').CounterSamples[0].CookedValue"
            ])
            .output();
            
        if let Ok(output) = output {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Ok(usage) = stdout.trim().parse::<f32>() {
                    samples.push(usage);
                }
            }
        }
        
        thread::sleep(Duration::from_millis(500));
    }
    
    // Ajouter une vérification spécifique pour les CPU Intel
    if let Ok(output) = Command::new("powershell")
        .args(&[
            "-Command",
            "(Get-CimInstance -ClassName Win32_Processor).Name"
        ])
        .output()
    {
        let cpu_model = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        // Ajustement basé sur la génération de processeur Intel
        let is_high_end = cpu_model.contains("i9") || cpu_model.contains("i7");
        let is_recent_gen = cpu_model.contains("11th") || cpu_model.contains("12th") || cpu_model.contains("13th");
        
        // Les modèles plus récents et haut de gamme tolèrent mieux les charges élevées
        if is_high_end && is_recent_gen && !samples.is_empty() {
            let avg_usage = samples.iter().sum::<f32>() / samples.len() as f32;
            println!("CPU Intel haut de gamme détecté - ajustement des seuils de température");
            
            // Ajuster le résultat pour les CPU haute performance qui supportent mieux les charges élevées
            return avg_usage * 0.9; // 10% de tolérance additionnelle
        }
    }

    // Moyenne des échantillons
    if !samples.is_empty() {
        return samples.iter().sum::<f32>() / samples.len() as f32;
    }
    
    50.0 // Valeur par défaut modérée
}

/// Convertit l'utilisation CPU en température équivalente
pub fn get_temperature_from_cpu_usage(ia_name: &str) -> f32 {
    static mut COUNTER: u32 = 0;
    
    // Protection basée sur l'utilisation CPU lorsque la température n'est pas accessible
    let cpu_usage = check_cpu_usage_as_temp_indicator();
    
    // Système de protection par niveaux
    if cpu_usage > 85.0 {
        println!("[{}] ⚠️ ALERTE CRITIQUE: Charge CPU très élevée ({:.1}%) - ARRÊT IMMÉDIAT", ia_name, cpu_usage);
        // Convertir la charge CPU en valeur de température équivalente
        // Une charge de 85%+ équivaut à une température critique (100°C)
        return 100.0;
    } else if cpu_usage > 75.0 {
        println!("[{}] ⚠️ ALERTE: Charge CPU élevée ({:.1}%) - Protection activée", ia_name, cpu_usage);
        // Une charge de 75%+ équivaut à une température élevée (76°C)
        return 76.0; 
    } else if cpu_usage > 60.0 {
        println!("[{}] ⚠️ AVERTISSEMENT: Charge CPU significative ({:.1}%) - Ralentissement préventif", ia_name, cpu_usage);
        // Une charge de 60%+ équivaut à une température modérément élevée (65°C)
        return 65.0;
    }
    
    // Même avec une charge CPU normale, implémenter un mécanisme de sécurité cyclique
    // Toutes les ~10 vérifications, simuler une température plus élevée pour forcer des pauses régulières
    unsafe {
        COUNTER += 1;
        if COUNTER % 10 == 0 {
            println!("[{}] Pause préventive programmée (cycle de refroidissement)", ia_name);
            return 67.0; // Température qui causera une courte pause
        }
    }
    
    // Utilisation normale
    45.0 + (cpu_usage * 0.25) // Formule de conversion simple entre usage CPU et température
}
