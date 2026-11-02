# Diagnostika - chyba `process died; skipping dispatch`

## Konkrétní problém (Open CAD Studio 2026.44, Windows)

Záložka Solar CAD CZ - Výuka se zobrazila, ale při odeslání `SOL_DEMO` a `SOL_LAYOUT` CAD hlásil:

```text
INVALID: Plugin 'cz.solarcad.education' process died; skipping dispatch
INVALID: Unknown command: SOL_LAYOUT
```

**Příčina zatím není potvrzená, pouze nejpravděpodobnější:** balíček `v0.1.0` byl kompilován proti starému `ocs_plugin_api` (`63045954`, `opencaddocument` `e6135a7`). Open CAD Studio `v2026.44` používá novější `ocs_plugin_api` a `opencaddocument` `d657f94`. Od API v14 runner porovnává hodnotu `OCS_WIRE_SCHEMA` a při nekompatibilitě ukončí proces.

V `v0.1.1` je SDK přímo připnutý na release commit `0e77cae652271d08c7619238e619f9b6668ae17c` (v2026.44). To je cílená oprava, ne důkaz odstranění chyby.

## Test po instalaci v0.1.1

1. Odebrat starou verzi v Plugin Manageru a restartovat OCS.
2. Nainstalovat nejnovější vydání `Liborias/solar-cad-cz` (GitHub Release v0.1.1) a znovu spustit OCS.
3. Ve výkresu zadat `SOL_HELP` (ověření komunikace bez úpravy výkresu).
4. Zadat `SOL_DEMO` (25 panelů, 11,25 kWp) a použít Zoom Extents, pokud výkres není v záběru.
5. Zadat `SOL_LAYOUT` (ověření formuláře).

Pokud plugin stále padá, uchovat screenshot a chybový výpis z běhu Open CAD Studia/runneru. Běžná hláška `process died` sama o sobě neukazuje konkrétní důvod. Nezměňovat náhodně parametry projektu ani neinstalovat zpět v0.1.0.

**Bezpečnost:** Výukový plugin není určen pro skutečné elektrotechnické projekty.

## Oprava dialogu v0.1.2

Na Windows OCS 2026.44 po úspěšné instalaci v0.1.1 fungovaly `SOL_HELP` a `SOL_DEMO`, avšak `SOL_LAYOUT` zobrazil parametrický dialog a po potvrzení psal do konzole `Solar CAD CZ: zruseno.`.

Příčina potvrzena v `src/app/plugin_host.rs` OCS: `HostSession::show_dialog` otevře dialog, ale bez čekání vrací `DialogResponse::Cancelled`. Pro zpracování skutečného odeslání je potřeba `HostApi::start_interactive` a `InteractiveCommand::modal_dialog / on_dialog_response`.

V0.1.2 používá správný interaktivní životní cyklus. Kliknutí na `Vytvorit vykres` by mělo ihned vložit všechny entity do výkresu (není potřeba žádný klik do pracovní plochy). Ověřte po vydání a přeinstalování nové DLL. `SOL_DEMO` zůstává beze změny.

## v0.1.3 – červená hláška i při úspěšném návrhu

OCS v `src/app/update/mod.rs` mapuje výstup podprocesu ze `stderr` na `push_error` (červený `INVALID`) a výstup ze `stdout` na `push_output`. V původní implementaci `SolarLayoutCommand::on_dialog_response` se informace `Vytvarim ... panelu, ... kWp` vypisovala přes `eprintln!`; výkres přitom vznikl v pořádku. V0.1.3 mění úspěšnou zprávu na `println!`. Chybové výpisy `Chybne parametry` / `Nelze vytvorit navrh` zůstávají na `stderr`. Ověření uvnitř GUI po vydání je ještě potřeba.
