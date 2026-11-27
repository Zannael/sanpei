# Sanpei

Sanpei è un'app desktop personale per esplorare la pesca da riva, preparare le uscite e tenere un diario locale. Il progetto è in sviluppo: le funzioni effettivamente disponibili e le attività successive sono indicate in [docs/handoff.md](docs/handoff.md).

## Avvio su Linux

Servono Rust, Node.js, pnpm e le dipendenze di sistema Tauri 2, in particolare WebKitGTK 4.1. Dopo l'installazione:

```sh
pnpm install --frozen-lockfile
git config core.hooksPath scripts/hooks
pnpm tauri dev
```

Controlli: `pnpm test`, `pnpm build`, `cargo test --manifest-path src-tauri/Cargo.toml`, `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` e `pnpm privacy`. Per creare il pacchetto Debian: `pnpm tauri build --bundles deb`.

## Dati e fonti

Il diario è salvato nella directory dati dell'applicazione del sistema, fuori dal repository. La schermata dati crea e ripristina copie SQLite. Il ripristino verifica lo schema e conserva prima una copia di recupero nella cartella dati dell’app; le copie compatibili dello schema precedente vengono migrate su un duplicato. Sono disponibili JSON del diario, CSV separati per uscite, intervalli e catture e GeoJSON delle posizioni. I CSV antepongono un apostrofo ai testi che potrebbero essere interpretati come formule. Non inserire dati personali, foto, credenziali o il materiale originale della ricerca nel repository. Le schede sono sintesi preliminari di una ricerca critica del 24 novembre 2026, da verificare rispetto alle fonti originali. Le indicazioni locali non certificano accessibilità o sicurezza alimentare.

Il codice è distribuito con licenza MIT. Diritti e attribuzioni dei contenuti esterni sono verificati separatamente prima della distribuzione.

I pacchetti di ricerca sono documentati in [docs/research-packages.md](docs/research-packages.md).

## Roadmap e sviluppo

Il [backlog unico](roadmap/TODO.md) contiene attività, dipendenze e prove di completamento. La priorità è una proposta automatica di giorno, zona e assetto attraverso servizi ambientali e una mappa utilizzabile. Questa è la direzione di sviluppo, non una funzionalità già completata. Preferenze e inventario saranno riutilizzati; le condizioni manuali rimarranno un percorso facoltativo di osservazione/simulazione.

[Specifiche](docs/implementation-plan.md), [architettura](docs/architecture.md) e [registro dei fatti e delle decisioni](roadmap/docs/master.md) hanno ruoli distinti dal backlog. Per iniziare:

```sh
python3 scripts/check_roadmap.py
python3 scripts/check_roadmap.py --ready
python3 scripts/check_roadmap.py --layers
python3 -m unittest discover -s tests -v
```

Richiede Python 3.10+ senza dipendenze aggiuntive. Dopo una modifica al TODO, rigenerare con `python3 scripts/check_roadmap.py --write-graph`, poi validare; non modificare il JSON a mano. Gli hook in `scripts/hooks` controllano roadmap al commit e privacy al push. Il workflow completo è in [AGENTS.md](AGENTS.md).
