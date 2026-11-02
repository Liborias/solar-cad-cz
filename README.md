# Solar CAD CZ – Výuka v0.1.0

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

Ukázkové DXF, CSV a PNG jsou součástí samostatného výukového ZIP balíčku dodaného spolu s první verzí.n CAD Studiu bez instalace pluginu. `demo/solar_cad_cz_nahled.png` je ilustrativní náhled, `demo/vykaz_panelu.csv` ukázkový výkaz.

## Instalace pomocí GitHubu (Open CAD Studio Plugin Manager)

Repozitář: **`Liborias/solar-cad-cz`**.

Open CAD Studio Plugin Manager instaluje nativní `.dll`/`.so`/`.dylib` zveřejněné jako **GitHub Release assets**, nikoliv samotný zdrojový kód. Tento projekt obsahuje `.github/workflows/release.yml` pro automatické sestavení na Windows/Linux/macOS a přiložení binárek i `plugin.toml` ke GitHub Release po vytvoření tagu `v*`.

**V tuto chvíli jde o NEOVĚŘENÝ zdrojový prototyp.** Není doloženo, že se Rust plugin úspěšně kompiluje nebo že se načte v desktopové aplikaci Open CAD Studio. Nejdřív musí uspět GitHub Actions.

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

Pro tuto verzi byl použit vývojový kontrakt z oficiálního referenčního pluginu Open CAD Studio (`ocs_plugin_api` pinovaný na commit `63045954c4b75cbed764a5d044836c2d393de4f4`). Zdrojový kód není v tomto prostředí zkompilovaný ani spuštěný v GUI CADu; může být nutná úprava při konkrétní vydané verzi OCS.

- Architektura: https://github.com/HakanSeven12/OpenCADStudio/blob/main/docs/plugin-architecture.md
- Vzor: https://github.com/HakanSeven12/opencad-example-plugin
- Zdrojový CAD: https://github.com/HakanSeven12/OpenCADStudio

Licence: GPL-3.0-only (viz soubor LICENSE). Přímé přebírání kódu komerčních nadstaveb AutoCADu zde není.