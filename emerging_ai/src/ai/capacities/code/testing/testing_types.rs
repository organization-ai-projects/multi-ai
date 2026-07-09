pub enum TestingStrategy {
    CargoCheck,   // Vérifie la compilation sans produire de binaire
    CargoTest,    // Lance les tests unitaires
    CargoTestLib, // Teste uniquement la bibliothèque
    CargoTestBin, // Teste un binaire spécifique
    CargoBench,   // Lance les benchmarks
}
