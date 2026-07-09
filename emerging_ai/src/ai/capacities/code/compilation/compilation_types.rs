pub enum CompileTarget {
    // Avec rustc
    RustcLib, // Compile une bibliothèque
    RustcBin, // Compile un binaire
    // Avec cargo
    CargoBuild,    // Compile le projet entier
    CargoBuildLib, // Compile uniquement la lib
    CargoBuildBin, // Compile un binaire spécifique
}
