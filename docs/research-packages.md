# Pacchetti di ricerca

Un pacchetto è un file JSON locale preparato e revisionato fuori dall'app. Per generare un modello preliminare dalla ricerca iniziale:

```sh
python3 scripts/create_package.py > /tmp/sanpei-pacchetto.json
```

Cambiare identificativo e versione, aggiornare le fonti e rivedere ciascuna scheda e regola. Solo dopo la revisione impostare `review_status` a `reviewed`. In Dati e impostazioni scegliere **Importa e attiva pacchetto JSON**: Sanpei controlla la struttura e chiede conferma prima dell'attivazione. La validazione non verifica la veridicità scientifica, l'attualità normativa o i diritti delle fonti.

Lo schema 1 supporta attualmente soltanto il Lazio, schede testuali di settori e montature, e regole per tecnica, condizione qualitativa, peso indicativo del piombo e montatura. Non supporta geometrie, nuovi operatori, script o parametri aggiuntivi. Campi e operatori non supportati sono respinti: un nuovo tipo di regola richiede una modifica dell'app e test. Non usare nomi o versioni già importati per contenuti diversi.

Ogni nuova versione resta distinta; si possono riattivare quelle precedentemente importate o tornare ai contenuti iniziali. I piani conservano scenario, risultati e copia dei cataloghi e delle regole usate. Il pacchetto iniziale è preliminare, senza posizioni di pesca verificate.
