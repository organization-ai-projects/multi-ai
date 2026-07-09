use std::{fs, io};
use rand::{thread_rng, Rng}; 
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
struct Objective {
    name: String,
    success_count: u32,  // Changé en u32 car toujours positif
    fail_count: u32,     // Changé en u32 car toujours positif
}

#[derive(Deserialize)]
struct LogEntry {
    timestamp: u64,
    status: String,
    details: Option<String>,
}

#[derive(Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
struct Objectives {
    objectives: Vec<Objective>
}

const OBJECTIVES_RON: &str = "goals/objectives.ron";
const OBJECTIVES_BIN: &str = "goals/objectives.bin";

fn main() -> io::Result<()> {
    let mut rng = rng();

    loop {
        // Génère du contenu totalement aléatoire
        let size = rng.random_range(1..1000); // Taille aléatoire
        let content: String = (0..size)
            .map(|_| {
                // Mélange de tous les caractères possibles
                let choices = [
                    (32u8..127).collect::<Vec<_>>(), // ASCII imprimable
                    b"{[()]}".to_vec(),              // Structures
                    b"\"'".to_vec(),                 // Strings
                    b"\n\t ".to_vec(),              // Whitespace
                ].concat();
                
                choices[rng.random_range(0..choices.len())] as char
            })
            .collect();

        // Choisit une extension aléatoire
        let ext = [
            "rs", "toml", "ron", "bin", "", "txt", "json"
        ][rng.random_range(0..7)];

        // Crée un chemin aléatoire (pour tester la sécurité)
        let dirs = ["", ".", "..", "src", "sandbox", "../sandbox"];
        let path = format!("{}/{}.{}", 
            dirs[rng.random_range(0..dirs.len())],
            rng.gen::<u64>(),
            ext
        );

        // Tente d'écrire
        let _ = fs::write(&path, &content);

        // Apprend de ses résultats via les logs
        if let Ok(content) = fs::read_to_string("../logs/system.ron") {
            if let Ok(logs) = ron::from_str::<Vec<LogEntry>>(&content) {
                if let Some(last_log) = logs.last() {
                    let mut objectives_changed = false;
                    let len = state.objectives.len();
                    
                    // Met à jour ses scores selon succès/échecs
                    for i in 0..len {
                        if last_log.status == "success" {
                            state.objectives[i].success_count += 1;
                            
                            // Vérifie si on peut évoluer (hors de la boucle pour éviter le double borrowing)
                            if state.objectives[i].success_count > 10 && len < 3 {
                                objectives_changed = true;
                            }
                        } else {
                            state.objectives[i].fail_count += 1;
                        }
                    }

                    // Ajoute un nouvel objectif si nécessaire
                    if objectives_changed {
                        state.objectives.push(Objective {
                            name: format!("evolved_goal_{}", len),
                            success_count: 0,
                            fail_count: 0,
                        });
                        
                        // Sauvegarde car les objectifs ont changé
                        let ron_string = ron::ser::to_string_pretty(&state.objectives, Default::default())
                            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                        fs::write(OBJECTIVES_RON, ron_string)?;
                        
                        // Sauvegarde en binaire uniquement pour l'IA
                        let bin_content = bincode_next::encode_to_vec(&state.objectives, bincode_next::config::standard())
                            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                        fs::write(OBJECTIVES_BIN, bin_content)?;
                    }
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
