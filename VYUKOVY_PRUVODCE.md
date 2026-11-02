# Výukový průvodce – co vlastně Solar CAD CZ počítá?

Tento jednoduchý nástroj používá **geometrické rozmístění obdélníkových panelů** na obdélníkovou plochu. Nehodnotí, jestli je reálná střecha konstrukčně, požárně ani elektro-technicky vhodná.

## Jednotky ve formuláři

| Symbol | Název | Jednotka | Význam |
|---|---|---|---|
| `W` | šířka střechy | mm | vodorovný rozměr modelové střechy na ose X |
| `H` | výška střechy | mm | svislý rozměr modelové střechy v půdorysu na ose Y; **nikoliv nadmořská výška či výška budovy** |
| `w` | šířka FV panelu | mm | rozměr jednoho panelu ve zvolené orientaci |
| `h` | výška FV panelu | mm | druhý rozměr jednoho panelu |
| `s` | modelový odstup od okraje | mm | pouze geometrická hodnota pro cvičení, neslouží jako ověření požárních či montážních odstupů |
| `g` | mezera mezi panely | mm | opakovaná vodorovná/svislá mezera v modelu |
| `P_m` | jmenovitý výkon panelu | Wp | výkon uvedený za standardních testovacích podmínek (STC) |
| `P_total` | instalovaný špičkový výkon | kWp | aritmetický součet Wp všech panelů dělený 1000 |

`mm` znamená milimetr, `Wp` watt-peak (jmenovitý špičkový výkon FV modulu), `kWp` kilowatt-peak, tedy 1 kWp = 1 000 Wp. Hodnota v kWp není údaj o roční výrobě v kWh.

## Výpočet počtu sloupců a řad

Pro šířku střechy `W` a jednoho panelu `w` při okrajovém odstupu `s` a mezeře `g`:

`n_x = floor((W − 2s + g) / (w + g))`

Pro výšku střechy `H` a výšku panelu `h`:

`n_y = floor((H − 2s + g) / (h + g))`

`floor` znamená **zaokrouhlení dolů na celé číslo**. Přičítá se jedna mezera v čitateli, protože mezi `n` panely je právě `n−1` mezer. Algoritmus také ošetřuje případ, kdy se panel nevejde vůbec.

Počet panelů je `N = n_x × n_y`. Výkon celé skupiny je `P_total = N × P_m / 1000` v kWp. **Nejde o elektrickou konfiguraci stringů.**

## Konkrétní příklad z balíčku

Výchozí střecha má rozměry 10 000 × 6 500 mm. Modelový okrajový odstup je 350 mm, mezera mezi panely 20 mm, panel 1 134 × 1 722 mm a výkon 450 Wp.

**Panely na výšku:**
- Výška jednotlivého panelu = 1 722 mm a šířka = 1 134 mm.
- Počet sloupců: `floor((10000−700+20)/(1134+20)) = 8`.
- Počet řad: `floor((6500−700+20)/(1722+20)) = 3`.
- Výsledek = **24 panelů = 10,80 kWp**.

**Panely na šířku:**
- Šířka jednotlivého panelu = 1 722 mm a výška = 1 134 mm.
- Počet sloupců: `floor((10000−700+20)/(1722+20)) = 5`.
- Počet řad: `floor((6500−700+20)/(1134+20)) = 5`.
- Výsledek = **25 panelů = 11,25 kWp**.

Automatický režim proto vybere **panely na šířku**. Přebytečný prostor se rozdělí symetricky kolem celé soustavy (soustava je vycentrovaná).

## Co znamená plošné pokrytí?

`A_pokryti (%) = N × (w × h) / (W × H) × 100`.

Je to podíl geometrické plochy samotných modulů na zadané ploše střechy. **Není** to využitelná plocha střechy, koeficient účinnosti, ani energetický výnos.

## Proč zatím neřešíme stringy?

Z pouhého počtu panelů nelze bezpečně odvodit počet a zapojení stringů. Je nutné znát mimo jiné Voc, Vmpp, teplotní koeficienty, proudy, limitní hodnoty MPPT a střídače, minimální/maximální teploty, ochrany, případné optimalizéry a další podmínky. To bude až samostatná výuková etapa s vysvětlenými předpoklady.