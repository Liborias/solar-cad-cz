# Solar CAD CZ – Výuka v0.1.3

**První výukový (NEautorizovaný) plugin pro desktopový Open CAD Studio.**

## Funkce

- Vlastní záložka **Solar CAD CZ**, tlačítka **Navrhnout panely**, **Ukázkový výkres**, **Nápověda**.
- Český formulář: rozměry obdélníkové střechy; rozměry, výkon a orientace panelu; mezera, modelový odstup a souřadnice počátku.
- Reálné 2D CAD entity ve výkrese (uzavřené polylinie a popisy), hladiny `SOL_STRECHA`, `SOL_PANELY`, `SOL_POPISY`.
- Varianta **na výšku / na šířku / auto**. Auto porovná počet kusů v obou orientacích a vybere větší počet.
- Popisky P001...; součet kusů, instalovaný špičkový výkon v kWp a geometrické pokrytí střechy.
- Metadata XDATA `SOLARCAD_CZ` na obrysu střechy a obrysech modulů pro případná další rozšíření.
- Možnost vrátit vytvořené entity zpět jedním **Undo** (podle podpory hostitele).

## Ukázka

Výchozí vstup: střecha 10 000 × 6 500 mm, modul 1 134 × 1 722 mm, 450 Wp, mezera 20 mm, modelový odstup 350 mm. Automaticky vychází **25 panelů, 5 × 5, 11,25 kWp** (panely na šířku). Jen geometrický výsledek, nikoli doporučený skutečný návrh.

Ukázkový DXF, CSV a PNG byly dodány samostatně s první výukovou verzí; nejsou v tomto repozitáři. Reálné objekty se vytvářejí příkazem `SOL_DEMO` nebo `SOL_LAYOUT`.

## Instalace pomocí GitHubu (Open CAD Studio Plugin Manager)

Repozitář: **`Liborias/solar-cad-cz`**.

Open CAD Studio Plugin Manager instaluje nativní `.dll`/`.so`/`.dylib` zveřejněné jako **GitHub Release assets**, nikoliv samotný zdrojový kód. Tento projekt obsahuje `.github/workflows/release.yml` pro automatické sestavení na Windows/Linux/macOS a přiložení binárek i `plugin.toml` ke GitHub Release po vytvoření tagu `v*`.

**Status:** Verze 0.1.0 se zkompilovala na všech třech platformách, ale v Open CAD Studio 2026.44 při dispatchi hlásila `process died; skipping dispatch`. Verze 0.1.2 navíc opravuje asynchronní dialog. Původní `HostApi::show_dialog()` okamžitě vrací `Cancelled`; nyní se formulář spouští přes `InteractiveCommand::modal_dialog()` a odeslané hodnoty zpracuje `on_dialog_response()` přímo jako jednorázové vložení CAD objektů. Funkčnost v GUI čeká na ověření.

Podrobný postup: [INSTALACE_WINDOWS.md](INSTALACE_WINDOWS.md).

## Co výslovně chybí

- Mnohoúhelníkové střechy, vikýře, komíny, střešní okna, požární a servisní koridory.
- Kótování, podkladní DWG/DXF vrstvy, přesné rozměrové řetězce a automatická montážní dokumentace.
- Stringování, MPPT, střídače, kabely, jištění, proudy, napětí, měniče, simulace výroby.
- 3D stínění, posouzení konstrukcí, kotvení, vítr, sníh, normová kontrola.
- Výkazy celé FVE, cenové nabídky nebo kompletní dokumentace pro české stavební/elektro předpisy.

**NEDÁ SE POUŽÍT JAKO NÁHRADA PROJEKTOVÉ DOKUMENTACE.** Odstup 350 mm je názorná výchozí hodnota, nikoli doporučený nebo normový požadavek. Ověřit musí kvalifikovaný projektant. Tvar panelu se geometricky předpokládá obdélníkový a jeho skutečné montážní požadavky mohou rozmístění změnit.

## Technické poznámky

`src/layout.rs` obsahuje samostatně testovatelné geometrické jádro. `src/lib.rs` realizuje integraci s oficiálním plugin API v14. **Plugin při běhu nepotřebuje Python.**

V0.1.2 používá `ocs_plugin_api` z přesného release commitu Open CAD Studio **v2026.44** (`0e77cae652271d08c7619238e619f9b6668ae17c`). Původní v0.1.0 používala starší revizi `63045954` s odlišným revizním číslem knihovny `opencaddocument`. API v14 ověřuje také `OCS_WIRE_SCHEMA`; odlišný datový kontrakt může způsobit ukončení procesu pluginu. Příčina hlášení zatím nebyla z běhového logu potvrzena.

- Architektura: https://github.com/HakanSeven12/OpenCADStudio/blob/main/docs/plugin-architecture.md
- Vzor: https://github.com/HakanSeven12/opencad-example-plugin
- Zdrojový CAD: https://github.com/HakanSeven12/OpenCADStudio

Licence: GPL-3.0-only (viz soubor LICENSE). Přímé přebírání kódu komerčních nadstaveb AutoCADu zde není.

## Oprava pro Open CAD Studio 2026.44

Nejprve vytvoř a nainstaluj GitHub Release `v0.1.2`. V Plugin Manageru odeber staré vydání `v0.1.0`, restartuj CAD, nainstaluj `Liborias/solar-cad-cz` a znovu restartuj. Otestuj postupně příkazy `SOL_HELP`, `SOL_DEMO` a `SOL_LAYOUT`. Pokud se zobrazí `process died; skipping dispatch`, předej prosím přesný log z plugin runneru, protože tato hláška v příkazové řádce neobsahuje konkrétní důvod pádu.

## v0.1.3 – oprava barevného hlášení po vykreslení

V OCS 2026.44 se úspěšné vykreslení formulářem označovalo červeně jako `INVALID: [cz.solarcad.education stderr] ... Vytvarim 15 panelu ...`, ačkoli se výkres správně vytvořil. Příčinou bylo `eprintln!` pro úspěch. Úspěch se nyní zapisuje přes `println!` na stdout; skutečné chyby zůstávají na stderr. Nejde o změnu geometrie, elektrotechnických výpočtů ani opravu API.
