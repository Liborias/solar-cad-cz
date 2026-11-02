# Instalace Solar CAD CZ do Open CAD Studia přes GitHub

## Instalace uživatelem (doporučeno)

**Podmínka:** Musí existovat veřejný GitHub repozitář `Liborias/solar-cad-cz` a v něm alespoň jedno úspěšné **GitHub Release** s DLL a `plugin.toml`.

1. Otevři desktopový **Open CAD Studio** → **Plugin Manager**.
2. Vyber instalaci pluginu **z GitHub repozitáře** a zadej `Liborias/solar-cad-cz` (přesné označení příkazu se může v jednotlivých verzích lišit).
3. Nainstaluj nejnovější dostupný Release a restartuj OCS.
4. Ve výchozím testovacím výkresu hledej záložku **Solar CAD CZ**. Nebo zadej příkaz `SOL_DEMO` / `SOL_LAYOUT`.

**Nestačí pouze mít zdrojové soubory v GitHub repozitáři.** Plugin Manager potřebuje zkompilovaný nativní soubor `.dll` jako asset ve vydání GitHub Release.

## První vydání pro vývojáře repozitáře

1. Vytvoř na GitHubu veřejný repozitář `Liborias/solar-cad-cz` s výchozí větví `main`.
2. Nahraj **obsah složky tohoto balíčku**, nikoli jen ZIP jako jediný soubor; cesta `.github/workflows/release.yml` musí být zachována.
3. V záložce **Actions** ověř, že automatická sestavení na `main` skončila úspěšně. Pokud selžou, je nutné nejprve opravit chybu v kódu nebo závislostech.
4. Po úspěšném sestavení vytvoř tag `v0.1.0` směřující na ověřený commit (lze přes GitHub Releases → Draft a new release → Choose a tag → Create new tag).
5. Nový tag spustí workflow. Až dokončí všechny čtyři platformy a krok `Publish GitHub Release`, zkontroluj v sekci **Releases** soubory `cz.solarcad.education-windows-x86_64.dll` a `plugin.toml`.
6. Teprve poté lze instalovat v OCS pomocí GitHub repozitáře. Pokud se neinstaluje, porovnej požadovanou verzi API s použitým OCS. Aktuální prototyp cílí API 14.

## Vývojářský lokální test (volitelný)

```powershell
rustc --version
cargo test
cargo build --release
```

DLL na Windows: `target\release\solar_cad_cz.dll`. Tyto příkazy vyžadují Rust a build nástroje MSVC; návod na instalaci Rustu: https://rustup.rs/.

## Co dělat, když sestavení selže

Na GitHubu otevři **Actions → Build and release Open CAD Studio plugin → neúspěšný běh → Build / windows-x86_64**. Zkopíruj první konkrétní Rust compiler error (nikoli pouze poslední `Process completed with exit code 101`).

## Bez pluginu

Soubor `demo/solar_cad_cz_demo.dxf` otevřeš v CADu přímo, bez pluginu. Obsahuje ukázkový výukový návrh 25 panelů.

**Bezpečnost:** Program je výukový, nenahrazuje elektroprojekt, požární posouzení ani statiku. Neověřený návrh nepoužívej jako realizační dokumentaci.